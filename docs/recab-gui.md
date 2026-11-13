# recab GUI: plan (2026-11-10, not started)

A graphical app beside `recab-tui` for the desktop images: KDE Plasma on
LevitateOS and LXQt on AcornOS. Nothing below is built. Its three decisions are made
(toolkit, how changes get root, where the source lives); work has not started.

## What it is

One window for recab's system update and continuation lifecycle: show both
systems, prepare the inactive continuation, apply changes and downloaded updates
without losing pending work, finalize, trial-boot, commit, roll back and reboot.
A and B have identical capabilities. Today's TUI only exposes release-image
staging and boot operations; that limited surface is not the GUI's final scope.
It ships only in the desktop system (`live-desktop`); the base system keeps the
command line, and on LevitateOS the TUI.

It is not a second implementation of recab. recab stays the only thing that
reads and changes slot state.

## What exists to build on

- `recab status --json` carries the existing boot-operation view: the slots,
  the next boot, the trial state, and for each action recab's own verdict, its
  reason when refused, and the state it would leave (`ActionView`, `After` in
  `tools/recab/src/status.rs`). It has a `schema_version` and a JSON Schema
  (the `contracts` feature). A failing command prints its error as one JSON
  object on stderr.
- `recab-tui` (`tui/apps/runtime/recab-tui`, about 1,600 lines of TypeScript on
  ink) is a starting point for existing actions and wording: its pages, its
  before/after table, and typing the slot's name before a write that formats it.
- Both desktop locks carry the same toolkit: Qt 6.11 (`qt6-qtbase`,
  `qt6-qtdeclarative`, `qt6-qtwayland`), polkit 127 with an authentication
  agent (`polkit-kde`, `lxqt-policykit`) and `polkit-qt6`.
- Plasma runs on Wayland, LXQt on X11 (Alpine 3.24 has no LXQt Wayland
  session). The app must run on both.

## Toolkit

Decided (2026-11-10): **Qt 6 Widgets, in C++**.

- It is the one toolkit both desktops are made of. The app takes each desktop's
  own style (Breeze on Plasma, LXQt's configured style), file dialogs and
  fonts, and runs on Wayland and X11 with no code for either.
- It adds nothing to either image but the binary: every library it needs is
  already in both desktop locks. Widgets needs only `qt6-qtbase`.
- Widgets: LXQt is a Widgets desktop, and status tables and update dialogs fit
  the existing toolkit.

Its cost: a C++ program in a Rust and TypeScript repository, and a build per
distro against that distro's Qt and libc (see Build).

Not chosen: **Slint, in Rust**. It stays in the cargo workspace and needs no
Qt at build time, but it does not look or behave like either desktop, and a
musl build that loads the system's Wayland and X11 libraries at run time is its
own toolchain problem (the Rust tools are static musl today and cannot load
libraries).

Also set aside: Python with PyQt (AcornOS's desktop has no Python; it would
add the interpreter and bindings), a local web page (a root-capable service on
localhost, and no window of its own in Firefox), GTK (foreign to both
desktops).

## How it works

- **Reads:** runs `recab status --json` as the desktop user, on start, after
  every change and when the window regains focus. It checks `schema_version`
  and refuses to guess on another one.
- **What each action would do:** from the status' own action views (verdict,
  reason, after-state). The GUI holds labels and layout only, no slot rules.
- **Changes:** runs `recab <command>` through `pkexec`, with a polkit policy
  file (`org.levitateos.recab.policy`) so the desktop's own agent asks for the
  password and says what for. Decided (2026-11-10): `pkexec`, not a D-Bus
  helper started by the system bus. It is far less to build and audit; the
  helper would only pay off if actions should be allowed without a prompt.
- **Errors:** recab's structured JSON class, exit code and message shown as is.
- **Preparation and updates:** render recab's generation, pending changes,
  conflicts, progress and readiness once its contract exposes them. Do not erase
  an existing preparation when adding an update. Distinguish ordinary incremental
  preparation from an explicit destructive replacement or discard.
- **Running from a checkout:** the TUI's stand-ins (`fixtures/recab`,
  `fixtures/reboot`) on PATH, so every state can be shown without a machine.

## Gaps in recab to close first

The backend first needs matching initial A/B systems, incremental continuation,
preparation sessions, update discovery/download/application and Recipe targeting
of the inactive system, with conflict handling and candidate finalization.
Current destructive `stage` and the shared Recipe profile do not supply this.
See [the lifecycle requirements](../tools/recab/REQUIREMENTS.md). The remaining
items concern frontend integration and do not replace those backend requirements.

1. **Progress for `stage`.** It runs `recstrap --quiet` and prints nothing while
   it formats and extracts. recstrap already has JSON events; `recab stage`
   needs to pass them on (`--json` on stderr), or the GUI shows an indefinite
   bar for minutes.
2. **Reading without root.** By reading the code it should work: `status` has
   no root check (only the changing commands take `require_root`), the ESP is
   mounted with `defaults` and recab creates its state directory with the
   default mode. Not yet run as a desktop user on either distro. If it fails
   there, fix the permissions or read through the same polkit action with
   `allow_active=yes`.
3. **`pkexec`.** Both locked packages ship it setuid: Fedora's `polkit` and
   Alpine's `polkit-common` (`/usr/bin/pkexec`). Still to confirm: that it
   authorizes under elogind on AcornOS.
4. **Types.** `cargo xtask gen` writes the TUI's TypeScript types from recab's
   Rust types. Add the same for the GUI (field names as a generated header), so
   a changed status shape fails the GUI's compile instead of its first run.

## Build

The Rust tools are one static musl binary for both distros and the TUI is
compiled by bun per libc. A Qt program fits neither: it must be compiled and
linked against each distro's own Qt 6.11 and C++ runtime (glibc and libstdc++
16 on Fedora 45, musl and libstdc++ 15 on Alpine 3.24).

Plan: a recipe that builds it inside a tree made from the distro's own
packages: the desktop lock plus a small build lock (compiler, CMake, Qt
development files), the way the rootfs recipes already assemble a tree and run
the packages' setup in it. The binary is copied out; the build tree is not
shipped. This is the largest piece of new work and should reuse whatever the
Recipe and Ginux integration settles on for building in a distro's
environment, not a second mechanism beside it.

### State of the build (2026-11-11)

Four programs exist in `gui/` now: the apps `recipe-gui`, `recab-gui` and
`disk-gui`, and the helper `levitate-agent`. None of this has been built or
run: no image carries any of it yet. What is done and what is not, for getting
them into both desktop images:

- **Done: one build for all four.** `gui/CMakeLists.txt` builds them together;
  `cmake --install` with a `DESTDIR` writes the four programs to `usr/bin`,
  three menu entries (the apps', not the helper's) to `usr/share/applications`
  and the two polkit actions (Recipe's and recab's) to
  `usr/share/polkit-1/actions`. Run on the development machine only.
- **The helper.** `levitate-agent` has no window and no menu entry. It is built
  at `agent/levitate-agent` in the build tree and installed as
  `usr/bin/levitate-agent`. `levitate-agent --version` prints
  `levitate-agent 0.1`.
- **What the helper needs at run time:** `gdbus` (GLib's tool, for desktop
  notices) and one agent program, `codex` or `claude`. Recdirstat also runs `lsblk`.
- **No agent program is in either image, and no recipe for one exists.** For a
  person this means the apps start and their own pages work, but everything
  that hands work to the agent fails until they install `codex` or `claude`
  themselves.
- **What the images carry of that** (from the package locks; the files inside
  the packages were not opened): Fedora has `gdbus` in `glib2` and `lsblk` in
  `util-linux-core`, both in the base lock. Alpine has `lsblk` in the base
  lock; its `gdbus` is in `glib-bin`, which neither Alpine lock has, so Acorn
  lacks `gdbus` until `glib-bin` is added to `[packages]` in
  `distro/variants/acorn/distro.toml` and the lock is made again. A polkit
  agent for `pkexec` is in the desktop lock of each desktop: `polkit-kde`
  (Plasma) and `lxqt-policykit` (LXQt, through `lxqt-desktop`). Whether each
  one starts in the session has not been seen.
- **Done: the apps need nothing else added to either image.** The programs
  link Qt 6 Widgets, Gui, Core and Svg, the C++ runtime and OpenGL. Both
  desktop locks carry them already (`qt6-qtbase`, `qt6-qtbase-gui` and
  `qt6-qtsvg` 6.11.2 on Fedora 45; `qt6-qtbase`, `qt6-qtbase-x11` and
  `qt6-qtsvg` 6.11.1 on Alpine 3.24), and `pkexec` (`polkit`, `polkit-common`).
  The polkit actions name `/usr/bin/recipe` and `/usr/bin/recab`, where the
  builder puts both.
- **Written 2026-11-10, run 2026-11-12: the build inside the image pipeline**
  (the route chosen 2026-11-10: each distro's own root). Both distros' app
  trees were compiled this way, published as prebuilts and taken into the
  desktop ISOs built on 2026-11-12. The same root now also compiles AcornOS's
  X server (XLibre, `os-recipes/xlibre.rhai`).
  - `[desktop].apps_build_packages` in each `distro.toml` names what compiling
    needs beyond the base; `cargo xtask base lock` writes it as a third lock,
    `sources/*-apps-build-lock.rhai`, in the same run as the desktop lock, so
    both name the same Qt (6.11.2 on Fedora 45, 6.11.1 on Alpine 3.24).
  - The desktop flavour's preseed also downloads that lock.
  - `fedora-rpm-apps-build-root.rhai` and `alpine-apps-build-root.rhai` unpack
    the base lock and the apps-build lock into one tree, without firmware.
  - The builder (`distro/builder/src/desktop_apps.rs`) makes one root image of
    that tree and runs `desktop-apps.rhai` in it instead of Recipe's usual
    root, with the static Recipe as the worker, since an Alpine root cannot
    run the host's. The recipe runs `cmake` there and publishes the installed
    tree, which is merged into the desktop system's rootfs.
  - The contract lists the apps (`DESKTOP_APPS`, with `DESKTOP_APP_ENTRIES`)
    and the helper (`DESKTOP_HELPERS`, no menu entry), and a desktop rootfs
    without any of them is refused.
- **Written, never run: the `desktop-boot` check** (`desktop-boot.sh`). In the
  booted desktop each app is on PATH, has its menu entry and answers
  `--version` with Qt's offscreen platform; the helper is on PATH and answers
  `--version`.
- **Answered by the 2026-11-12 build:** the unpacked trees work as roots
  (nothing ran the packages' scripts; only the linker's name on Fedora and the
  shell's on Alpine are set by hand) and CMake and the compiler run under
  Ginux there, with make and one job (parallel make and Ninja's use of file
  times failed or were doubtful under Ginux before; meson with samurai, which
  XLibre needs, has not run there yet). Not measured: how long copying a root
  of that size takes for each run.

## In the pipeline

- Source: a `gui/` folder in this repository, beside `tui/` (decided
  2026-11-10; not a repository of its own like the `tools/*` submodules). When
  it exists, `AGENTS.md` names it among the places active work lives.
- Contract: a list of desktop apps beside `SYSTEM_TUI_APPS`, the same on every
  distro; the desktop product carries the binary, a `.desktop` entry, an icon
  and the polkit policy.
- Check: `desktop-boot` starts the app in the logged-in session and takes a
  screenshot of the status page, on both distros. From a checkout it runs
  against the stand-ins with Qt's offscreen platform.

## Order of work

1. Implement the required recab continuation/update lifecycle and its frontend
   status contract. Run recab gaps 2 and 3 on a booted desktop of each distro
   (a failure there reopens the `pkexec` decision), and close gap 1.
2. The build recipe, proven with a window that only prints `recab status`.
3. Status page and the four quick actions (try, commit, roll back, reboot).
4. Continuation and updates: prepare/reopen the inactive system, retain pending
   changes while obtaining and applying an update, report conflicts and progress,
   then finalize. Expose destructive replacement/discard separately.
5. Contract, desktop product and the `desktop-boot` check.

Later, not in the first version: a notification after a trial boot ("kept" or
"rolled back"), a tray icon, finding an inserted release medium by itself,
translations.
