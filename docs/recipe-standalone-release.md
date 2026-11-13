# Recipe as a standalone release

Plan of 2026-11-13; decisions taken the same day (end of file). Nothing in it is built yet.

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
