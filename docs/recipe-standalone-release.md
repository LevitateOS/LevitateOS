# Recipe as a standalone release

Plan of 2026-11-13; decisions taken the same day (end of file).

## Update of 2026-11-23: the public-lifecycle cutover changes the privilege story

Written, never built or run. Recipe now runs recipes through Ginux's public
lifecycle (design `tools/recipe/docs/ginux-cutover-design.md`, plan
`tools/recipe/docs/ginux-public-lifecycle-migration.md`, section 6), not the
downstream `ginux launch`. For a standalone release that means:

- **There is no ordinary-user path and no AppArmor profile.** The
  user-namespace launch (Ginux `ccc492a2`, `7b9dec91`) that let an ordinary user
  install on Fedora and Ubuntu is gone; so is `recipe runtime stage --apparmor`,
  `data/apparmor-ginux`, the AppArmor load in `install.sh` and the `.deb`
  `postinst`. Every privileged step of a Ginux run is the Ginux helper run as
  root; `sudo` runs only Recipe's guard, which runs the helper.
- **The sudoers requirement.** To run anything in Ginux a user must be root or
  a member of the group named by the rule `recipe runtime stage --sudoers GROUP`
  writes to `/etc/sudoers.d/recipe-ginux` (mode 0440; the rule lets `%GROUP` run
  `/usr/libexec/recipe/recipe-ginux-guard --protocol 1 --state-root /var/lib/recipe/ginux
  --run-root /run/recipe/ginux *` as root without a password). The rule names the
  guard, never the helper: the guard (root-owned, 0755, in a root-owned
  directory) allows only Recipe's verbs (`import`, `inject`, `start`, `status`,
  `stop`, `delete`, `extract`) on instances named `recipe-<uid>-*`, re-renders
  the instance config Recipe imports from validated fields, clears the
  environment, and for a caller other than root accepts an image only if its
  exact reference is in the root-owned allowlist
  `/usr/libexec/recipe/runtime-images` (written by `recipe runtime stage`,
  `sudo recipe runtime fetch` and `install.sh`); then it runs the helper.
  `import-image` is not available through the guard: `install.sh` imports the
  image as root (`sudo recipe runtime fetch`), and a non-root run without an
  imported image stops and says to run that command. A group member can
  therefore run Recipe-shaped sandbox instances from trusted images without a
  password. That is not a guarantee of harmlessness to the host:
  the code that unpacks and prepares a trusted image runs as root and follows
  symlinks in it (R13, R14: a hostile image could delete host files; the
  allowlist keeps untrusted images away, but root callers and the OS builder
  still use that code), every user's workload runs as the same host uid 65534
  (R15), and there are no per-uid instance, disk or cache caps (the free-space
  watchdog and the byte bounds are Recipe's, in the client). See
  `tools/recipe/docs/runtime-limits.md`, "What a shared host still exposes". `install.sh` (already run as root) writes the rule for
  the group it picks: `%wheel` on Fedora, Rocky, LevitateOS and AcornOS, `%sudo`
  on Ubuntu. Other users cannot run a recipe in Ginux. Recipes that start no program still run on the host for
  everyone (`--execution auto`). Option A of the cutover design (decided by the
  owner); options B (a `recipe` group) and C (root only) were not taken.
- **What a release now installs** (all as root): the `ginux` CLI at
  `libexec/recipe/ginux`, Recipe's guard at
  `/usr/libexec/recipe/recipe-ginux-guard`, the helper, image tool and runtime at
  `/usr/libexec/ginux/{ginux-helper,ginux-image,ginux-runtime}`, the allocator
  at `/usr/lib64/ginux/libginux_allocator.so`, the sudoers rule, and the image
  layout downloaded as `image.oci.tar.zst` (identity: the OCI
  manifest digest; the installer's `sudo recipe runtime fetch` imports it once
  per host into the root-owned state root `/var/lib/recipe/ginux`, about 1.3 GB
  more on disk, and lists it in `/usr/libexec/recipe/runtime-images`; root's
  first run does the same, a non-root run does not import). The files must be
  root-owned and not writable by anyone else: a user-writable copy reached
  through the passwordless rule would be a root escalation. `GINUX_BIN`,
  `GINUX_ROOT*` and `RECIPE_*` replace nothing the old way: Recipe stops on the
  old variables and names the new ones.
- **Supported hosts, honestly.** Rocky/RHEL 10 is the only host Ginux itself
  packages for (an RPM). Fedora 44 and Ubuntu 24.04 are Recipe's own packaging
  (the CLI and image tool are to be built static musl so one build runs on both,
  not yet checked; the helper needs GNU `mv --no-copy`, `cp --reflink`,
  `script -E never`, `flock`, `gawk`, `stat -c`, all present on both), and
  **neither has been run on the public lifecycle**: the verification pass must
  install on each and run a recipe. SELinux on Fedora (the helper runs
  unconfined through sudo) and `sudo`'s `use_pty` with binary stdin/stdout are
  unproven. AcornOS needs `sudo`, util-linux, GNU coreutils and gawk in its
  image, which sits badly with "small and strict". The kernel floor is
  unestablished.
- **Accepted regressions that ship with it**, stated in
  `tools/recipe/docs/runtime-limits.md` and the release notes: no storage limit
  inside a run, network grants per port (programs can reach any address on a
  granted port, no connection or byte cap), live roots copied, no hard links
  through the instance, each run costs a full image expansion where the state
  root has no reflinks.

The findings below are history of the retired runtime. Where they speak of
user namespaces, AppArmor, an ordinary user's install, `/usr/libexec/recipe/ginux`
as the launcher or `rootfs.tar`, they no longer apply.

Status (2026-11-13), all compile-checked, none of it built into an image,
run under Ginux or published (retired runtime):
- Done: Recipe step 1 and step 2 (tools/recipe `540e9c1`). `recipe runtime
  stage|fetch|status` exist, runtime paths resolve against the install
  prefix, and the image downloads on first use (checked against a local
  server). The builder stages through `recipe runtime stage`, and
  `cargo xtask prebuilt publish-runtime` uploads the image (distro/builder
  `dac0f8f`). The window's Qt floor is 6.4 (desktop-apps `b21ce94`; source
  audit, not built against 6.4).
- Published 2026-11-13: the pinned compiler image
  `oci:recipe-compiler@sha256:4ec20be0…` (product-ci.json) is the release
  `recipe-runtime-4ec20be0f93bdd65` of `LevitateOS/prebuilt`. That is
  1 289 105 408 bytes compressed to 332 MB. `recipe runtime fetch` downloaded
  it back and the result had that digest. A standalone package must ship
  `root-reference` naming this image: the built-in default reference in
  `runtime.rs` (the bare Rocky base `b93bfd0f…`) is not published.
- Tried on Fedora 44 (2026-11-13; kernel 6.19, glibc 2.43, SELinux
  enforcing). This was a plain cloud VM under libvirt session
  (`fedora-recipe-standalone`; disk in
  `~/.local/share/libvirt/images/recipe-standalone`). Recipe was installed
  from a 5.6 MB `/usr/local` tarball made with `recipe runtime stage`, plus a
  glibc release `recipe` and Ginux at `d777b0ad`. See "Step 3: what the
  Fedora run showed" below.
- Fixed and checked on the same Fedora VM (2026-11-13). Ginux `ccc492a2`
  gives ordinary users a user-namespace launch with no capabilities left
  (root path unchanged). Recipe `9b36fd4` puts scratch on disk, exports
  under state, and root's software on everyone's `PATH`. An ordinary user
  installed ripgrep (30 s) and root installed fd (17 s); each was found on
  `PATH`, and no `.artifacts` was left behind.
- Ubuntu 24.04.5 (kernel 6.8, AppArmor userns restriction on), 2026-11-13:
  - **Ordinary user, no profile:** Ginux stops with exit 133 "user
    namespace refused" and names the Ubuntu setting. AppArmor moves the
    namespace into `unprivileged_userns` and denies `sys_admin`.
  - **With an AppArmor profile** allowing `userns` for the launcher, two more
    faults surfaced, both fixed:
    - Ginux published exports with `linkat(AT_EMPTY_PATH)`, which needs
      `CAP_DAC_READ_SEARCH` before kernel 6.10. It now falls back to
      `/proc/self/fd` (Ginux `7b9dec91`).
    - Ubuntu's umask 002 made the store group-writable, which activation
      refused. Recipe now tightens its own store (`10c6875`).
  - **After both fixes**, ordinary-user installs of ripgrep, fd (fresh store)
    and jq succeed, and root installs work without the profile.
    `recipe runtime stage --apparmor` writes that profile, and packages and
    `install.sh` must load it on AppArmor hosts (Recipe `edee1b9`; the
    superproject pin waits for the ISO builds running on 2026-11-13).
  - **Kernel floor:** kernels older than 6.10 need the `/proc/self/fd` path;
    5.x is untried.
- Release command written 2026-11-13 (`cargo xtask release recipe`, xtask
  `recipe_release.rs`; compile-checked and run with a glibc recipe, which marks the
  release unofficial; never run with the static recipe, never published). Its
  RPM installed with `dnf` on Fedora 44 (`rpm -V` clean), and its `.deb` installed
  with `apt` on Ubuntu 24.04, where `postinst` loaded the AppArmor profile. On both,
  an ordinary user then installed ripgrep. Each first install took 18 minutes,
  with the host at load 18–21 from two ISO builds; Fedora also warned "cannot clone the
  cached image, unpacking it instead", which still needs a look. The APK (no
  Alpine run; needs the static recipe), the AUR files and the Void template are
  checked by reading them only.
- 2026-11-13: whether Ginux should wrap whole recipe runs is open; see
  `docs/recipe-ginux-boundary.md`.
- Next: the
  rename to `recipe-pm`; the window's packaging; mark OS-only recipes.

The goal is to release Recipe for other Linux systems, not only as part of
LevitateOS and AcornOS. Each release comes in two forms:

- **Recipe**: the command line only.
- **Recipe with the window**: the same command line plus `recipe-gui`.

Both forms come from the same version and the same runtime. The window has no
package rules of its own; it runs `recipe` and shows the JSON it returns.

## What a release has today, and why it cannot run a recipe

`LevitateOS/recipe` release 0.1.4 (2026-03-13) holds
`levitate-recipe-0.1.4-1.x86_64.rpm` and `…-r0.x86_64.apk`. These packages
contain `recipe`, `recipe-image`, the manual pages and the docs. Since then the
pin has moved about 275 commits ahead (`0.1.3-275-gdbe2e17`).

Recipe never runs a recipe on the host. Every run goes through Ginux, which
needs three things that only the OS image ships today. They are put there by
`distro/builder/os-recipes/install-tools.rhai` (`recipe_runtime`):

| What | Where Recipe looks | Who provides it today |
|---|---|---|
| The Ginux files (2026-11-23) | the `ginux` CLI at `/usr/libexec/recipe/ginux` or `RECIPE_GINUX`; the helper, image tool, runtime and allocator root-owned at `/usr/libexec/ginux`, `/usr/lib64/ginux`; the sudoers rule `/etc/sudoers.d/recipe-ginux` | the builder, built from `tools/ginux` at its pin; `recipe runtime stage` lays them out |
| The runtime image, a pinned Rocky OCI image layout | `/usr/share/recipe/runtime/compiler.oci.tar` with `root-reference`, or `RECIPE_IMAGE_LAYOUT` with `RECIPE_IMAGE`, or `$XDG_CACHE_HOME/recipe/images/<manifest digest>.oci.tar` (`runtime.rs` `layout_path`) | the builder, from `recipe-image` (images built here are 429 MB to 1.2 GB) |
| The official source of recipes | `/usr/share/recipe/sources/levitate.toml` and a copy of `levitate/index.json` (`src/sources.rs` `SHIPPED`) | the builder, from `recipes/` at its pin |
| `PATH` for installed commands | `/etc/profile.d/recipe.sh` | the builder |

Read from the code, not run: on a host with only the package, every recipe run
stops at startup with a message that names `RECIPE_GINUX` and the missing
Ginux file (`ginux.rs` `Ginux::from_env`), and there is no official
source to list from. You can only get further by hand, with
`docs/setup-linux.md`: build Ginux, produce an image and set the environment
variables (since 2026-11-23: `RECIPE_GINUX`, `RECIPE_IMAGE`, `RECIPE_IMAGE_LAYOUT`
and the worker).

So the main work is to have **Recipe itself own its runtime**, so that a
release can contain it. The builder would then use the same layout instead of
writing its own.

## What off-LevitateOS already behaves correctly

- **Package stores.** Root uses `/var/lib/recipe` and everyone else
  `~/.local/share/recipe` (`metadata.rs` `default_package_root`). Neither
  assumes recab.
- **The system route.** `recipe --system` and recab are used only when the
  kernel command line contains `recab.immutable` (`commands.rs` `system_route`).
  On another distro it never triggers.
- **What installs produce.** Packages from the catalog are self-contained and
  have run on a glibc host and in a musl root
  (`docs/recipe-self-contained-packages.md`).
- **The `recipe` binary.** The distro build already compiles it as static musl,
  so one binary runs on any x86_64 Linux.
- **The window without recab.** It handles a missing `recab` (`shots.cpp`
  checks the strip with `/nonexistent/recab`), and its own CMake folder builds
  `recipe-gui` alone with `kit/`.

## The work

### Recipe (`tools/recipe`)

1. **The runtime is laid out by Recipe.** A Recipe command, or a script kept
   in `tools/recipe`, stages the launcher, the runtime reference, the official
   source file with its index copy, and the profile snippet into a tree. The
   release packs that tree, and `install-tools.rhai` calls the same step
   instead of writing these files itself. One owner, two consumers.
2. **The runtime image is downloaded, not packed.** `rootfs_path` already
   looks in `$XDG_CACHE_HOME/recipe/images/<digest>.tar`; only the download is
   missing. Recipe downloads the image by digest, verifies it, and caches it.
   The source is a release in `LevitateOS/prebuilt` (written only by
   `prebuilt publish`, see `docs/prebuilt-downloads.md`). This keeps each
   package small and keeps the image under its digest pin. The OS image keeps
   shipping the tar so that it works offline.
3. **Privileges are settled on a foreign host (settled 2026-11-23).** The public
   lifecycle needs root: every privileged step is the Ginux helper run through
   `sudo`. A standalone host therefore needs the sudoers rule
   (`recipe runtime stage --sudoers GROUP`, written by `install.sh` for the
   group it picks) for any user who is not root, and the window's "For me"
   needs it too; without it Recipe says to run as root or to have an
   administrator install the rule (see the update at the top).
4. **The profile snippet covers both cases.** The snippet that is shipped now
   already handles a system without recab. It moves into Recipe's runtime
   tree unchanged.
5. **Name and wording.** Rename the crate `levitate-recipe` to `recipe-pm`
   and change its description so it no longer says "for LevitateOS"
   (decision 4). The `generate-rpm` metadata and the doc paths under
   `/usr/share/doc` follow the rename.
6. **A local release command.** `cargo xtask release recipe <version>` takes the
   static `recipe` through `distro-builder prebuilt get tools levitate-recipe`
   (the local cache when it holds the current key, else the published release
   `tools-<key16>` of `LevitateOS/prebuilt`, verified like an image build does, else
   a compile; `provenance.json` records the key in `recipe_prebuilt_key` and which of
   the three in `recipe_prebuilt_source`), builds `recipe-image` and `recipe-image` and the launcher at the Ginux pin, stages
   the runtime tree (since 2026-11-23: `--ginux-dir`, the five Ginux files
   built from the fork, CLI and image tool built static musl so one build runs
   on Fedora and Ubuntu, the image as an OCI layout whose release asset is
   `image.oci.tar.zst`, and the sudoers rule; `CONFFILES` gains
   `etc/sudoers.d/recipe-ginux` and loses `etc/apparmor.d/recipe-ginux`, and the
   `.deb` `postinst` no longer loads AppArmor), packs the formats of decision 2 (tarball, RPM, APK,
   `.deb`; the AUR and Void build files get the new checksums), and publishes
   the GitHub release of `LevitateOS/recipe`. Products are built locally, not in CI
   (CI is for quick checks only). It must be reproducible and record its source
   pins, following the release rules in `AGENTS.md`.

### The window (`gui`, `LevitateOS/desktop-apps`)

1. **Package `recipe-gui` alone.** `gui/recipe` with `kit/`, its menu entry
   and its polkit action, depending on the Recipe package.
2. **No agent inside.** The window has no agent of its own (the agent subsystem
   was removed): failure views have "Copy for agent", which only copies text for
   the person's own agent.
3. **Qt comes from the host** (decision 5). Build the window once per distro
   family against that family's Qt. Check whether the 6.5 floor can drop to
   6.4.
4. **"For this computer" needs a polkit agent** on the desktop. Most desktops
   run one. Without one, the window says why the change did not start.
5. **The window has never run against the real `recipe`** (see "Not done" in
   `gui/recipe/README.md`). The first standalone run on a real desktop is also
   its first real run.

### The catalog (`recipes/`)

`desktop-runtime` belongs to the OS images and does nothing on another
distro. Such recipes must say they are OS-only, and the standalone window and
`recipe list` must not offer them. Every other recipe in the catalog is a
self-contained program and fits a standalone release.

## Step 3: what the Fedora run showed (retired runtime, 2026-11-13)

- **What works.** The prefix lookup found the launcher, reference and
  sources under `/usr/local`, and `recipe list` listed the catalog.
  `recipe runtime fetch` downloaded the image from GitHub in 33 s.
- **Root works.** `sudo -i recipe install ripgrep` installed and activated
  ripgrep 15.2.0 in 47 s, with the image download included, and root's
  `rg` runs.
- **An ordinary user does not (and since 2026-11-23 cannot, by design, without
  the sudoers rule).** Ginux stopped with "launch bootstrap
  unavailable: chroot failed (host privilege required)". The launch uses
  plain `chroot(2)` (`tools/ginux/crates/ginux-init/src/launch.rs`), which
  needs `CAP_SYS_CHROOT`. "For me" installs therefore fail for every
  ordinary user, on LevitateOS as well. The window's "For me" has only ever
  run against a stand-in. A way that needs no root is for the launcher to
  enter its own user namespace (`CLONE_NEWUSER | CLONE_NEWNS`, with the
  caller's uid mapped to 0) before the chroot. Fedora allows that. Ubuntu
  24.04 restricts unprivileged user namespaces through AppArmor unless a
  profile allows the program.
- **`/tmp` on tmpfs.** Recipe's scratch pool is `$TMPDIR/recipe-scratch-<uid>`
  (`src/temporary.rs`), and each run unpacks the 1.3 GB root there.
  Fedora's `/tmp` is a tmpfs of half the RAM with per-user quotas, so the
  user's run failed first with "Disk quota exceeded". Root is exempt from
  the quota, but the unpacked root still occupies RAM; a machine with 4 GB
  has a 2 GB `/tmp`. The pool belongs on disk, under the cache directory. On
  btrfs (Fedora's `/home`) the reflink root cache then works too.
- **Root's software is not on users' `PATH`.** `profile.d/recipe.sh` gives a
  non-root user only their own store, so `rg`, installed by root into
  `/var/lib/recipe`, is not found by `tester`. This is fine on LevitateOS,
  where the locked system has its own path, but on a standalone host
  "for this computer" must also add `/var/lib/recipe/profile/bin`.
- **A stray `.artifacts`.** Publication writes
  `<current directory>/.artifacts/ginux-export` (`src/publication.rs`
  line 58), so `recipe install` leaves that folder in whatever directory it
  runs from. It belongs under the state or cache directory.
- **Static musl.** A static musl `recipe` cannot be built on this host: the C
  parts of zstd, xz and bzip2 need `x86_64-linux-musl-gcc`. The OS builder
  compiles it inside Ginux. The release command needs one of those two.

## Decisions (user, 2026-11-13)

1. **Target hosts: any x86_64 Linux.** The command line is static `recipe`
   plus the launcher plus the downloaded image, so it depends on no
   distribution. Still to establish: the oldest kernel the Ginux launcher
   accepts. The release states it.
2. **Formats: a tarball with `install.sh`, RPM, APK, `.deb`, an AUR package
   and a Void Linux template.** The tarball is the reference layout; every
   other format packs the same staged tree. RPM and APK exist (0.1.4). `.deb`,
   the AUR `PKGBUILD` and the Void `xbps-src` template are new. The AUR and
   Void ones are recipes for those systems' own build tools, published to the
   AUR and proposed to void-packages, not files on the GitHub release.
3. **Runtime image: downloaded by `sudo recipe runtime fetch`** (the installer
   runs it; root's first run does too), by digest from
   `LevitateOS/prebuilt`, verified and cached in
   `$XDG_CACHE_HOME/recipe/images`. No package contains it. The OS images
   keep shipping the tar.
4. **Name: `recipe-pm`.** This is the package name in every format, and the
   tarball is named `recipe-pm-<version>-x86_64.tar.zst`. The command stays
   `recipe`. The window's package is `recipe-pm-gui`, with the command
   `recipe-gui`. The crate `levitate-recipe` is renamed with the first
   release, and its description loses "for LevitateOS".
5. **Qt for the window: the host's Qt, one build per distro family** (RPM,
   `.deb`, APK, AUR, Void). There is no window in the tarball. The window
   requires Qt 6.5 (`gui/recipe/CMakeLists.txt`), but Debian 12 and
   Ubuntu 24.04 LTS ship Qt 6.4. Find out whether anything really needs 6.5.
   If nothing does, lower the floor to 6.4. Otherwise those two get the
   command line only.
