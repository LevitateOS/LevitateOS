# Self-contained packages: design (2026-11-10, accepted, first trial run)

How a program Recipe builds finds its shared libraries when it runs, so that
libraries need no agreement between packages, one catalog serves both
distributions, and keeping the catalog up is mechanical. The core was tried
once on a terminal program ("The first trial"); the automatic form described
here is not built, and "Open questions" says what is still to settle.

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
5. Refuse a program that still needs something not found, naming it. The
   install fails; the active version stays.
6. Record what was carried: each file, the recipe or image it came from, and
   that one's version or digest. This goes in the receipt.

A static program has nothing to list and is left as it is.

Libraries a program opens by name while running (toolkit plug-ins, drivers)
are not found by step 1. A recipe names those folders in a literal block:

```rhai
const CARRY = ["qt6/plugins/platforms", "qt6/plugins/imageformats"];
```

Each is a path under an `sdk` or the image, copied whole and treated as in
steps 3 to 6.

## 3. The package tree

```
package/
  bin/      programs, linked into the profile          (as today)
  share/    data, menus, icons, linked into the profile (as today)
  lib/      what the package carries; not linked anywhere (new)
```

`lib/` belongs to the package alone. Two packages never conflict over it, and
nothing in it is on anyone's path.

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
| `tools/recipe` | `lib/` in the package tree; `RUNTIME.paths.prefix`; the carry step in the worker; `CARRY`; carried files in the receipt; the build key includes the prefix |
| `tools/recipe` `src/system.rs`, `tools/recab` | a folder per package for `lib/` on a prepared system, and its receipt's record of what is carried |
| the build image | a tool to rewrite a program's loader and library paths, if Recipe does not do it itself (open question 2) |
| `recipes` | library recipes publishing `sdk`; toolchain recipes publishing `tools`; `kind` on both |
| `distro/contract`, `distro/builder` | the host contract and what each image adds to meet it |
| `gui/recipe` | hide libraries and toolchains; show what a package carries |

## Order of work

1. `lib/` in the package tree and `RUNTIME.paths.prefix`. Changes nothing for
   existing recipes. `lib/` is built; the prefix is not.
2. The carry step, tried on a small C program with one library (`htop` and
   `ncurses`), run on both distributions. Tried as helpers a recipe calls;
   see "The first trial". Still to do: the automatic step, writing the place
   into files after the build, and a run on booted systems.
3. The receipt's record of what is carried, and update on a changed image.
4. The host contract in `distro/contract`.
5. `CARRY`, tried on one real desktop program with a toolkit. This is the
   trial that decides whether the design holds.

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
| Recipe tells every link the package's place | the recipe passes `carry_link_flags(prefix)` to the final link |
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

## Open questions

1. **Does a toolkit program work this way on both distributions?** Not tried.
   Toolkits open plug-ins by name, read data from folders they work out from
   their own location, and talk to the desktop. Proposed: step 5 above, on a
   Qt program, before any more of the catalog is written.
2. **Who rewrites the program files.** The trial showed it has to happen
   after the build (finding 1). Proposed: a small existing tool added to the
   build image and run by Recipe in the sandbox, where the files are already
   untrusted input. The other choice is doing it in Recipe itself.
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
7. **`CARRY` as its own block or a field of `NEEDS`.** Proposed: its own
   block, since it grants nothing.

## Not in this design

- A program installed by a person into their own store without root: it
  works with this design (their profile's path is the prefix), but the
  install path itself is still unsettled.
- Running an installed program inside Ginux. The rule makes it possible
  later, because a package no longer depends on the host's libraries.
- Passing built packages between computers.
- The job that notices new upstream versions.
- NVIDIA's driver.
