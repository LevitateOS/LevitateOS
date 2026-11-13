# Recipe as a standalone release

Plan of 2026-11-13; decisions taken the same day (end of file).

Status (2026-11-13), all compile-checked, none of it built into an image,
run under Ginux or published:
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
| The Ginux launcher | `/usr/libexec/recipe/ginux` (`tools/recipe/src/runtime.rs` `LAUNCHER`), or `GINUX_BIN` | the builder, built from `tools/ginux` at its pin |
| The runtime image, a pinned Rocky rootfs tar | `/usr/share/recipe/runtime/rootfs.tar` with `root-reference`, or `GINUX_ROOTFS`, or `$XDG_CACHE_HOME/recipe/images/<digest>.tar` (`runtime.rs` `rootfs_path`) | the builder, from `recipe-image` (images built here are 429 MB to 1.2 GB) |
| The official source of recipes | `/usr/share/recipe/sources/levitate.toml` and a copy of `levitate/index.json` (`src/sources.rs` `SHIPPED`) | the builder, from `recipes/` at its pin |
| `PATH` for installed commands | `/etc/profile.d/recipe.sh` | the builder |

Read from the code, not run: on a host with only the package, every recipe run
stops at startup with "ginux launcher /usr/libexec/recipe/ginux is missing or
inaccessible" (`backend.rs` `resolve_launcher`), and there is no official
source to list from. You can only get further by hand, with
`docs/setup-linux.md`: build Ginux, produce an image and set four environment
variables.

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
3. **Privileges are settled on a foreign host.** `docs/setup-linux.md` says
   the coordinator needs root for Ginux's chroot bootstrap. Upstream Ginux
   says it runs as an ordinary process. The window's "For me" runs `recipe`
   without the root helper. One run on a plain Fedora or Rocky host shows
   which is true. If root is needed, installing "for me" needs the helper too,
   and the docs must say so.
4. **The profile snippet covers both cases.** The snippet that is shipped now
   already handles a system without recab. It moves into Recipe's runtime
   tree unchanged.
5. **Name and wording.** Rename the crate `levitate-recipe` to `recipe-pm`
   and change its description so it no longer says "for LevitateOS"
   (decision 4). The `generate-rpm` metadata and the doc paths under
   `/usr/share/doc` follow the rename.
6. **A local release command.** `cargo xtask release recipe <version>` builds
   static `recipe` and `recipe-image` and the launcher at the Ginux pin, stages
   the runtime tree, packs the formats of decision 2 (tarball, RPM, APK,
   `.deb`; the AUR and Void build files get the new checksums), and publishes
   the GitHub release of `LevitateOS/recipe`. Products are built locally, not in CI
   (CI is for quick checks only). It must be reproducible and record its source
   pins, following the release rules in `AGENTS.md`.

### The window (`gui`, `LevitateOS/desktop-apps`)

1. **Package `recipe-gui` alone.** `gui/recipe` with `kit/`, its menu entry
   and its polkit action, depending on the Recipe package.
   `levitate-agent` is optional.
2. **Without an agent.** When `levitate-agent` is missing, "Ask for software"
   and repair say so in one sentence and point to the way forward. Check this;
   it has not been seen yet.
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

## Step 3: what the Fedora run showed

- **What works.** The prefix lookup found the launcher, reference and
  sources under `/usr/local`, and `recipe list` listed the catalog.
  `recipe runtime fetch` downloaded the image from GitHub in 33 s.
- **Root works.** `sudo -i recipe install ripgrep` installed and activated
  ripgrep 15.2.0 in 47 s, with the image download included, and root's
  `rg` runs.
- **An ordinary user does not.** Ginux stops with "launch bootstrap
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
3. **Runtime image: downloaded on first use**, by digest from
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
