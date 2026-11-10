# Self-contained packages: design (2026-11-10, accepted, three trials run)

How a program Recipe builds finds its shared libraries when it runs, so that
libraries need no agreement between packages, one catalog serves both
distributions, and keeping the catalog up is mechanical. The core was tried
on a terminal program and on a Qt program ("The first trial" to "The third
trial"); the automatic form described here is not built, and "Open
questions" says what is still to settle.

It adds to [the format design](recipe-front-end-format.md), which defines the
`package` output, and to [the sources design](recipe-sources.md), which
defines how recipes and the recipes they need reach a computer.

## The problem

A recipe that builds from source needs more than its own file:

1. **Toolchains and build tools.** Rust, Go, Node, CMake and the rest. This
   part is solved by what exists: a recipe that publishes a directory output
   named `tools` is put on the path of any recipe that lists it under
   `build_deps`, and a source fetches needed recipes with the one installed.
2. **Libraries.** A program built against a shared library needs it again when
   it runs. If libraries are packages others share, every package using one
   must agree on its version, and the catalog has to be moved as a whole.
3. **Two C libraries.** LevitateOS is glibc, AcornOS is musl
   (`distro/variants/*/distro.toml`). A program built in the build image is
   tied to that image's C library (Rocky 10.2, glibc 2.39), so it runs on at
   most one of the two.
4. **Upkeep.** Every recipe pins versions somebody has to move.

The static ripgrep recipe has none of 2 and 3, which is why it is the only
recipe so far.

## What exists today

- A recipe's hooks run in Ginux. The root is a build image the OS ships at
  `/usr/share/recipe/runtime/rootfs.tar`: a Rocky 10.2 root with GCC 14,
  glibc 2.39 and its headers, Make, pkg-config, and Rust 1.99.0
  (`tools/recipe/bootstrap/ginux-build-image.json`). Both distributions ship
  the same image.
- A dependency's declared outputs are available to the recipe that lists it,
  at `RUNTIME.paths.dependencies/DEP/OUTPUT`; an output named `tools` is also
  put on its path.
- The `package` output may hold only `bin/` and `share/`
  (`copy_package` in `tools/recipe/src/packages.rs`), regular files only.
  Activation copies it to `.packages/NAME/` in a new profile generation and
  links every file into the profile; the profile is a symlink to the current
  generation. A package kept from one generation to the next is hard-linked,
  not copied.
- Nothing looks at what a built program links against. A program that needs a
  shared library finds it in the host's own folders or not at all.
- Recipe builds on the computer that installs. No built package is passed
  between computers.

## The rule

**A package carries every shared library it uses, down to the C library.**
When a program from a package runs, nothing it loads comes from the host's
library folders or from another package.

What a package may still take from the host is a short fixed list, the host
contract (below): the kernel, a few files under `/etc`, fonts, time zones and
the desktop's sockets.

## What follows

| Problem | With the rule |
|---|---|
| Libraries | A library is a build input only. Nothing installed is shared, so two packages may use two versions and no version has to be agreed. |
| Two C libraries | The package brings the build image's C library and its loader. The host's is not used, so one build runs on both distributions. |
| Upkeep | One catalog, not one per C library. Moving a library means rebuilding what carries it, with nothing to coordinate. |

Programs linked statically (Rust and Go by default, upstream's static
releases) already satisfy the rule and are not touched.

## Why it can be simple here

Other systems that carry libraries must make packages work from any folder,
because they hand built packages to other computers. Recipe does not: it
builds where it installs, so **the folder a package will live in is known
when it is built**. A program can therefore be told the exact path of its own
loader and libraries, as a path written into the program file. It then starts
like any other program: no wrapper script, no environment variable, and it
sees its own real path.

The path is written in after the build, not at the link (the trials showed
why). The link only reserves the room: every program and library is linked
with one long stand-in for its library folder. Afterwards Recipe writes the
real library folder and the real loader's path into that room. Nothing in a
file has to move, so Recipe does it itself in a few lines and no tool is
needed for it.

## 1. Where libraries come from at build time

- **The build image is the base set.** What a program links against from the
  image (the C library, libstdc++, and whatever else the image holds) is
  carried from there.
- **Everything else is a recipe that publishes `sdk`:** a directory of headers
  and libraries, as the format design already names it for kind `library`.
  A program lists such recipes under `build_deps`. An `sdk` is never
  activated; nobody installs a library.
- A library recipe is in the catalog but is not something a person browses.
  A front end hides kinds `library` and `toolchain` unless asked.

`deps` (needed when it runs) is then for programs a package calls, never for
libraries.

## 2. The carry step

After `install` has written the `package` output, and still inside the
sandbox, Recipe does this itself. No recipe writes code for it.

1. Read every program and library in `package` and list what each loads.
2. Find each of those in the sandbox: first in the `sdk` outputs of the
   recipe's `build_deps`, then in the build image.
3. Copy them into `package/lib/`, with the loader, and repeat for what the
   copied files load in turn.
4. Write into every program and library the paths it will have once
   activated: its loader, and `lib/` as the only place to look for libraries.
   They go into the room the stand-in reserved at the link. A program that
   was linked without it is refused, with the flags to use.
5. Refuse a program that still needs something not found, naming it. The
   install fails; the active version stays.
6. Record what was carried: each file, the recipe or image it came from, and
   that one's version or digest. This goes in the receipt.

A static program has nothing to list and is left as it is.

Libraries a program opens by name while running (toolkit plug-ins, drivers)
are not found by step 1. The recipe places those itself, under `lib/`, where
the toolkit will look; the carry step treats every program and library it
finds there as in steps 1 to 6. (An earlier draft named them in a `CARRY`
block. The third trial showed the recipe has to choose the folder anyway,
because where a toolkit looks is the toolkit's own rule.)

## 3. The package tree

```
package/
  bin/      programs, linked into the profile          (as today)
  share/    data, menus, icons, linked into the profile (as today)
  lib/      what the package carries; not linked anywhere (new)
```

`lib/` belongs to the package alone. Two packages never conflict over it, and
nothing in it is on anyone's path.

A program may live under `lib/` too, with a two-line starter in `bin/` that
names it by its full path. A toolkit that looks for its plug-ins beside the
program needs that: `bin/` is one flat folder shared by all packages.

## 4. The folder is known at build time

Recipe passes the package's final folder into the build as
`RUNTIME.paths.prefix`: `PROFILE/.packages/NAME`, where `PROFILE` is the
profile of the store being installed into (`/var/lib/recipe/profile` for
root, a person's own otherwise). The path goes through the profile symlink,
so it stays the same from one generation to the next.

Root's software on a system recab prepares is laid out differently: a
`package` output's files go straight into `/usr/lib/recipe/profile`, with no
folder per package (`payloads` in `tools/recipe/src/system.rs`). Two
packages' `lib/` would meet there. That route needs a folder per package for
`lib/`, such as `/usr/lib/recipe/profile/.packages/NAME/lib`, and that folder
is then the prefix. It is known before the build as well.

What this costs:

- A built package belongs to one store. The same recipe installed into a
  person's own store is built again (the carry step only, where the build is
  kept).
- What Recipe keeps of a finished build is keyed by the prefix as well.
- With `--no-activate` (the OS builder) there is no prefix and no carry step.

## 5. The host contract

What a carried library may expect from the host, on both distributions, and
nothing else:

| What | Where |
|---|---|
| Kernel interfaces | system calls, `/proc`, `/sys`, `/dev` |
| Users, names, addresses | `/etc/passwd`, `/etc/group`, `/etc/hosts`, `/etc/resolv.conf` |
| Certificates | one bundle, reachable at the path each carried TLS library looks in |
| Time zones | `/usr/share/zoneinfo`, `/etc/localtime` |
| Fonts | `/usr/share/fonts`, `/etc/fonts` |
| Terminal descriptions | `/usr/share/terminfo` or `/etc/terminfo` |
| The session | the D-Bus, Wayland and audio sockets under `$XDG_RUNTIME_DIR` |
| Graphics devices | `/dev/dri` |

`distro/contract` checks this list on both images, as it checks the rest of
what an image must hold. Where the two differ today (the certificate bundle's
path is one known case), the image provides both paths.

## 6. Keeping it up

- **Finding out.** The receipt says what each package carries. When a library
  recipe or the build image changes, the folder a recipe is fetched into
  changes too, because it is named by every recipe in it; `update` already
  treats that as a change. A package that carries something from the build
  image records the image's digest, so a new image shows as an update as
  well.
- **Doing it.** An update is a rebuild. Nothing has to be changed by hand in
  the packages that carry a moved library.
- **Prefer what upstream built.** Where upstream publishes a checked, static
  or self-contained release, the recipe uses it, as ripgrep does. Building
  from source is for what has none.
- **What is still missing** is noticing that upstream moved. That is a
  scheduled job for an agent (read the recipe's pinned version, ask upstream,
  propose the new pin with its checksums) and belongs with the `SOURCES`
  block of the format design, not here.

## What changes where

| Where | Change |
|---|---|
| `tools/recipe` | `lib/` in the package tree; `RUNTIME.paths.prefix`; the carry step in the worker; carried files in the receipt; the build key includes the prefix |
| `tools/recipe` `src/system.rs`, `tools/recab` | a folder per package for `lib/` on a prepared system, and its receipt's record of what is carried |
| `recipes` | library recipes publishing `sdk`; toolchain recipes publishing `tools`; `kind` on both |
| `distro/contract`, `distro/builder` | the host contract and what each image adds to meet it |
| `gui/recipe` | hide libraries and toolchains; show what a package carries |

## Order of work

1. `lib/` in the package tree and `RUNTIME.paths.prefix`. Changes nothing for
   existing recipes. `lib/` is built; the prefix is not.
2. The carry step, tried on a small C program with one library (`htop` and
   `ncurses`), run on both distributions. Tried as helpers a recipe calls;
   see the two trials. Writing the place into files after the build is
   done. Still to do: Recipe adding the link flags and running the step
   itself, and a run on booted systems.
3. The receipt's record of what is carried, and update on a changed image.
4. The host contract in `distro/contract`.
5. Plug-ins, tried on a desktop program with a toolkit: done for loading,
   plug-ins and fonts (the third trial). Still to do: the same on a running
   desktop on both systems, which needs images built with this Recipe.

## The first trial (2026-11-10)

`htop` 3.5.3 with `ncurses` 6.6 as a shared library, both built from source
in the sandbox and installed into a store. The recipes, logs and store are in
`.artifacts/recipe-carry-trial`.

What it showed:

- The install carried four files into the package's `lib/`: the loader, the C
  library and the maths library from the build image, and ncurses from the
  `ncurses` recipe's `sdk`. `lib/` is in the package and linked nowhere.
- On this computer (glibc) the program runs and loads all four from its own
  package, not from the system.
- Inside an AcornOS root (Alpine 3.24.4, musl, no glibc anywhere) the same
  files run: `htop --version`, and its full screen on a terminal, which found
  the terminal descriptions in `/etc/terminfo`.

How far that goes:

- The AcornOS root was entered with `chroot` on this computer's kernel. It is
  the tree an AcornOS image is made from, not a booted AcornOS.
- One terminal program. No toolkit, no network name lookups, no plug-ins.
- The whole run took ten and a half minutes, nine of them building ncurses.

What was built is the trial's form of steps 1 and 2, not the design's:

| Design | Trial |
|---|---|
| `lib/` in the package tree | built (`tools/recipe` `src/packages.rs`) |
| `RUNTIME.paths.prefix` | not built; the folder was given with `--define PREFIX=` |
| Recipe runs the carry step itself after `install` | the recipe calls `carry_libraries(package, prefix, search)` |
| Recipe tells every link the package's place | first trial: the recipe passes flags to the final link only. Second trial: the recipe passes `carry_link_flags()` to every link, from `configure` on |
| what was carried is in the receipt | only as files of the package |

`RUNTIME` was left alone on purpose: it is part of what every kept build is
matched by, and changing it under builds other work depends on needs a
decision of its own.

What it found:

1. **The place cannot be told at every link.** `configure` runs the small
   programs it builds. Told a loader in a folder that does not exist inside
   the sandbox, they cannot start, and `configure` stops. The trial gave the
   flags to the final link only. A build that runs its own products needs
   either the package's folder to exist in the sandbox while it builds, or
   the place written into the files after the build. This decides open
   question 2: writing it afterwards is not optional.
2. **The sandbox refused a large link.** ncurses' own shared build links some
   190 object files in one step and failed with "Too many open files". The
   trial built the archive position-independent and made the shared library
   from that one file. This is a limit of the sandbox, not of the design, and
   belongs with the other sandbox gaps.
3. **Terminal descriptions are host data.** AcornOS keeps them in
   `/etc/terminfo`, LevitateOS in `/usr/share/terminfo`. They join the host
   contract, and a carried ncurses must be built to look in both.
4. **The carried C library needed nothing beside it for this program.**
   Whether that holds for name lookups and locales (open question 6) was not
   exercised.

## The second trial (2026-11-11)

The step the first trial showed to be missing: writing a program's place
into its files after the build.

The first form failed, and that settled how it has to work. Programs were
linked for a stand-in folder inside the sandbox holding a copy of the loader,
to be overwritten afterwards. `configure` stopped again. A probe recipe
showed why: **the sandbox starts a program only with the build image's own
loader.** A loader anywhere else (under `/work`, under `/out`, by any name,
with any mode) gives "Operation not supported"; a symlink to the image's
loader gives "required file not found". The same loader run as a command
works.

The form that works leaves the loader alone at the link:

- Every link gets one addition, a 512-character stand-in library folder
  (`carry_link_flags()`). The program keeps the image's loader, so it runs in
  the sandbox like any other. `configure`'s test programs ran.
- The carry step writes the real library folder at the end of the stand-in's
  room and the real loader's path at its start, and points the program's two
  references there. The room is in a part of the file that is loaded, which
  the loader needs to read its own name.
- A package's folder may be 220 characters at most, so both always fit.

What it showed, with the same `htop` and shared `ncurses`:

- Built from `configure` on with the stand-in, then settled: `htop` names the
  loader and `lib/` of its own package folder.
- It runs on this computer and inside the AcornOS root, loading all four
  files from its own package; its full screen drew in the AcornOS root.
- A two-line C program went the same way in nine seconds, which is how the
  step was worked out (`.artifacts/recipe-carry-trial/quick`).

What this form cannot do:

- A program somebody else linked (a prebuilt release that is not static) has
  no reserved room. It needs its files rebuilt around a longer path, which is
  the larger job this form avoids; not designed.
- A build that adds library folders of its own to a program is refused, not
  repaired.

## The third trial (2026-11-11): a Qt program

The trial meant to decide whether the design holds for a toolkit. A small Qt
Widgets program (a window, a label, a button) with Qt 6.10.1 and everything
it needs carried: 75 files, 91 MB, of which 65 are shared libraries and 9
are Qt plug-ins. The recipe, inputs, logs, pictures and store are in
`.artifacts/recipe-carry-trial/qt`.

What it showed:

- On this computer the program starts, loads 41 libraries and plug-ins, all
  from its own package, draws its window and saves it as a picture.
- Inside the AcornOS root it does the same, with the same Qt, the same glibc
  and the same files. No library came from the system, which has none it
  could have used.
- With AcornOS's own font configuration and fonts in place (Alpine's
  `fontconfig` and `font-noto` packages, as a desktop AcornOS has), the
  carried Qt reads them and the picture is the same as on this computer.
  Without them the window is drawn and the text is empty boxes.

How far that goes:

- No display. Qt drew into memory (its `offscreen` platform). Wayland, X11,
  the graphics drivers, themes and input were not exercised; they need a
  running desktop on both systems.
- The AcornOS root was entered with `chroot`, as in the earlier trials.
- **The program was compiled on this computer, not in the sandbox.** The
  build image has no C++ compiler or headers. This computer has the same
  distribution as the image (Rocky 10.2), so the result is what the image
  would build if it could. Assembling the package and the carry step ran in
  the sandbox, in thirteen seconds.
- **Qt was not built by a recipe.** Its libraries and plug-ins were copied
  from this computer's own Qt packages and given to the recipe as a folder,
  standing in for the `sdk` of a Qt recipe.

What it found:

1. **A toolkit decides where its plug-ins are, and a distribution's build
   decides it for good.** This Qt was built to look in
   `/usr/lib64/qt6/plugins` and beside the program, nowhere else. The room Qt
   keeps for that folder cannot be lengthened: a longer path written into it
   is read back cut to the old length. So:
   - The program and its plug-ins went together into the package's own part
     (`lib/helloqt/`), with a two-line starter in `bin/`. That is Qt's
     ordinary layout for a program shipped with its own Qt.
   - The built-in folder had to be pointed at a folder that does not exist
     (same length). Otherwise this Qt also loads plug-ins from a Qt the host
     happens to have there, which is the leak the rule forbids. It did
     exactly that before the change.
   A Qt built by its own recipe can be built to find its plug-ins relative
   to its libraries, and then needs neither. That is the form to aim for;
   the two steps above are what carrying a distribution's Qt costs.
2. **Plug-ins are placed by the recipe, not listed.** The carry step now
   looks at everything under `lib/`, follows what plug-ins load, and treats a
   program found there like one in `bin/`.
3. **The carried C library has no UTF-8 locale on AcornOS.** Qt warned that
   it could not switch to one and went on. glibc reads its locales from
   `/usr/lib/locale`, which a musl system does not have. The data has to
   travel with the package or be named to glibc when the program starts;
   not solved here.
4. **Fonts work across the two.** The host's font configuration and fonts
   are enough for a carried Qt and fontconfig, which is what the host
   contract assumes.
5. **The build image cannot build C++.** No recipe can compile a Qt, GTK or
   any C++ program in it today. That is a matter of what the image holds or
   of a toolchain recipe, and it comes before any real desktop recipe.

What it decides: the library side of the design holds for a toolkit program
on both C libraries. What it leaves open is everything that needs a running
desktop, and the locale.

## Open questions

1. **Does a toolkit program work this way on both distributions?** For
   loading, plug-ins and fonts: yes (the third trial). On a running desktop
   (Wayland or X11, graphics drivers, themes, input): not tried, and it needs
   both systems booted.
2. **Who rewrites the program files.** Settled by the second trial: Recipe
   itself, in the sandbox, into room reserved at the link. No tool is added
   to the build image. Open again only for programs linked elsewhere.
3. **Graphics drivers.** The open drivers are libraries like any other and
   can be carried. NVIDIA's own driver cannot: it must match the kernel
   module on the host, and it exists for glibc only. Proposed: carried
   programs use the open drivers; NVIDIA's driver is a later design and will
   not reach AcornOS.
4. **Disk space.** Each toolkit program carries its toolkit, on the order of
   100 MB. Proposed: accept it at first. Identical files in different
   packages can later be stored once, as kept packages already are between
   generations, without changing the rule.
5. **A fix in a library many packages carry.** Every one of them is rebuilt.
   Proposed: that is the price of the rule; the receipt makes the list of
   affected packages exact, and the rebuild needs no hand.
6. **The carried C library and the host's configuration.** glibc reads
   `/etc/nsswitch.conf` and loads name-service modules by name; AcornOS, being
   musl, need not have either. Proposed: carry glibc's `files` and `dns` modules always, and let
   it fall back to them. To be confirmed in step 2.
7. **`CARRY` as its own block or a field of `NEEDS`.** Dropped: the recipe
   places plug-ins under `lib/` itself (the third trial).
8. **The carried C library's locales.** glibc looks in `/usr/lib/locale`,
   which AcornOS does not have. Proposed: carry the one UTF-8 locale every
   program needs, and have the starter name its folder to glibc. This makes
   a starter the rule for programs that need a locale, which is most
   desktop programs.
9. **A C++ compiler for recipes.** In the build image, or as a toolchain
   recipe under `build_deps`. Proposed: a toolchain recipe, like any other
   compiler, so the image stays the small base.

## Not in this design

- A program installed by a person into their own store without root: it
  works with this design (their profile's path is the prefix), but the
  install path itself is still unsettled.
- Running an installed program inside Ginux. The rule makes it possible
  later, because a package no longer depends on the host's libraries.
- Passing built packages between computers.
- The job that notices new upstream versions.
- NVIDIA's driver.
