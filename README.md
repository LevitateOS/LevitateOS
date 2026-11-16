# LevitateOS

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

This repository builds two operating systems, LevitateOS and AcornOS, and the
tools they are made with.

## 1. What this is

LevitateOS and AcornOS are independent operating systems. Their userlands are
built from Fedora 45 RPM packages (LevitateOS) and Alpine 3.24 APK packages
(AcornOS). Those package sets are our parts supplier and nothing more. The
kernel (built from pinned source), the installer, the `rec*` tools, A/B booting
and Recipe are our own. `dnf` and `apk` never run, here or on an installed
system.

What differs is declared in each variant's `distro.toml`
([LevitateOS](distro/variants/levitate/distro.toml),
[AcornOS](distro/variants/acorn/distro.toml)):

| | LevitateOS | AcornOS |
| --- | --- | --- |
| Packages | Fedora RPM | Alpine APK |
| C library | glibc | musl |
| Init | systemd | OpenRC |
| Admin tool | sudo | doas |
| Desktop | Plasma | LXQt |
| Kernel line | latest | LTS |

LevitateOS is the daily-driver workstation: software built for mainstream
Linux runs on it. AcornOS is the small, strict system: the fewest parts that do
the job. Each OS ships one release ISO.

## 2. Recipe

Software is acquired, built, installed and removed through Recipe
([`tools/recipe`](tools/recipe)). There is no other package manager on either
system.

A recipe is a readable Rhai file that says where a program comes from, how it
is built and what it needs. You can read it before you run it.
Recipes that start programs run in an isolated Ginux domain. The others run on
the host. Ginux is Greg Kurtzer's upstream project
([`gmkurtzer/ginux`](https://github.com/gmkurtzer/ginux)), which we follow; our
fork is [`tools/ginux`](tools/ginux).

Every install is either for me or for everyone:

- For me: the program is installed for one user.
- For everyone: the program is installed into the system. On an A/B
  installation that change is written into the other root (see below), never
  into the booted one.

The recipe catalog is [`recipes/`](recipes/). The recipes that build the
operating systems themselves are in
[`distro/builder/os-recipes/`](distro/builder/os-recipes/). Both use the same
format: see [`tools/recipe/WRITING_RECIPES.md`](tools/recipe/WRITING_RECIPES.md)
and [`docs/package-recipe-format.md`](docs/package-recipe-format.md).

Software that ships above the minimal base, the desktop included, is
installed through Recipe too, so it can be removed or swapped like any other.
The installer installs the default set; see section 7 for what has run.

## 3. The two roots

An A/B installation has two root filesystems, root A and root B
([`tools/recab`](tools/recab)).

- The booted root is locked: it is mounted read-only.
- The other root is unlocked. System changes are written there.
- Boot into the other root and the roles swap.

`recab copy` makes the other root a copy of the booted root, which discards
whatever was in it. `recab boot-other` starts the other root once. The boot
default is the root that last finished starting, so a root that fails to start
is left behind and the machine boots the previous one.

A mutable installation has one root and no recab; it is updated by the
Updater app (`recupdate apply`).

## 4. Installing

The ISO boots into the install screen: a root terminal on the left and the
installation guide on the right. The guide offers the helper apps
(`disk-plan`, `base-system`, `system-setup`) or the manual commands. Use one or
the other: when a helper has done a step, the manual section for it says to
skip it.

Running the helpers is not an installed system. A system counts as installed
when boot files and boot entries exist on the EFI partition and the
verification has passed. The same pages are in [`docs/content`](docs/content)
and on the website.

## 5. Where things are

| Path | What is there |
| --- | --- |
| [`distro/builder`](distro/builder) | Builds products: rootfs, EROFS images, ISOs. Holds `os-recipes/` and the install screen's split pane. |
| [`distro/contract`](distro/contract) | The variant contract: its types, the loader and the product and scenario registry. |
| [`distro/variants`](distro/variants) | `levitate` and `acorn`: one `distro.toml`, kernel config and branding each. |
| [`tools/recipe`](tools/recipe) | Recipe. |
| [`tools/ginux`](tools/ginux) | Ginux, the isolation Recipe runs programs in (upstream's project, our fork). |
| [`tools/recab`](tools/recab) | The two roots. |
| [`tools/recstrap`](tools/recstrap), [`tools/recpart`](tools/recpart) | Install tools: extract the system and configure boot; plan and apply partitions. |
| [`tools/recinit`](tools/recinit), [`tools/recuki`](tools/recuki), [`tools/reciso`](tools/reciso) | Build the initramfs, unified kernel images and the ISO. |
| [`tools/recqemu`](tools/recqemu), [`tools/recguest`](tools/recguest), [`tools/recshuttle`](tools/recshuttle) | Scenario VMs under libvirt, the guest agent, and the SSH transport to a guest. |
| [`tools/reclogin`](tools/reclogin) | The login screen (written, not in an image yet; see [`docs/reclogin.md`](docs/reclogin.md)). |
| [`tools/gui`](tools/gui) | The Qt desktop apps: Recipe, Recab, Recdirstat (in `disk/`) and Updater. |
| [`tools/recupdate`](tools/recupdate) | Checks for, downloads and (on a mutable installation) applies a release. |
| [`tools/tui`](tools/tui) | The terminal apps of the live ISO and installer. |
| [`testing/install-tests`](testing/install-tests) | Boot and install scenarios. |
| [`testing/ginux-boot`](testing/ginux-boot) | Boots an OS root under Ginux and judges its startup. |
| [`recipes`](recipes) | The software catalog. |
| [`docs`](docs) | Design notes, the product handoff, `content` (live docs) and `website`. |
| [`xtask`](xtask) | `cargo xtask`, the front door for every workflow. |

Several of these are Git submodules (see `.gitmodules`); each has its own
README.

## 6. Working here

Everything runs through `cargo xtask`; `just` has a short alias for each
command. `cargo xtask --help` lists them. The ones you will use:

```sh
cargo xtask setup                         # once: QEMU tools, firmware, SSH key
cargo xtask doctor                        # what is missing, and the command that fixes it
cargo xtask check                         # quick checks: cargo check, generated files, TypeScript
cargo xtask check tui                     # the terminal apps
cargo xtask build release <distro>        # the release ISO
cargo xtask test all <distro>             # the scenarios
cargo xtask status <distro>               # which scenarios pass, fail or are stale
```

`<distro>` is `levitate` or `acorn`. With `just`: `just check`, `just build
...`, `just test ...`, `just website-dev`.

Image, kernel and ISO builds take hours. They happen only when the owner asks
for one; a change to a declaration is done when it is edited and committed.
Read [`AGENTS.md`](AGENTS.md) for the working rules, and
[`docs/product-handoff.md`](docs/product-handoff.md) before changing the
installer, recab, Recipe, the desktop apps or public wording about the OSes.
[`xtask/README.md`](xtask/README.md) says what xtask owns.

## 7. State of things

Everything written since 2026-11-15 is unbuilt and has not run on a machine:
no image has been built since, no installed system has used it, and by the
owner's rule nothing was run to prove it. It compiles; that is all that was
checked.

In source, unbuilt:

- recab with the locked/unlocked contract and the commands in section 3
  (`status`, `copy`, `boot-other`, `started`, `compare`, `health`, `init`,
  `verify-installation`, `install-unit`, `update`). Its old lifecycle and
  updater are removed.
- Shipped software is installed through Recipe. The image holds only the
  base. The desktop, Firefox, audio, network, printing and the rest are group
  recipes ([`recipes/`](recipes/)) with their own configuration, and the
  installer installs each OS's default set (`[desktop].recipes` in
  `distro.toml`) from a package cache on the install medium. Each group has a
  receipt and can be removed.
- Recipe writes "for everyone" installs into the other root on an A/B
  installation and into the one root on a mutable one.
- Releases and updates, as designed in
  [`docs/release-engineering.md`](docs/release-engineering.md): a signed
  manifest per release, `recupdate` (check, download, daily notice), `recab
  update` for A/B installations, `recupdate apply` and the Updater app for
  mutable ones, "Check for updates" in every app, and
  `cargo xtask release keygen|sign|publish`. The base is an entry Recipe owns,
  and system groups change only with a release.
- Steam for LevitateOS (`steam`, with the 32-bit libraries in `lib32`).
- A `codecs` recipe for LevitateOS (the full FFmpeg codec library; written 2026-11-16, never run).
- reclogin ([`tools/reclogin`](tools/reclogin)), the login screen for both
  OSes: a root program and a greeter with no rights that draws straight to
  the display, password login through PAM, Wayland and X sessions, the guest
  account, automatic login, an on-screen keyboard. Designed in
  [`docs/reclogin.md`](docs/reclogin.md) and
  [`docs/reclogin-technical.md`](docs/reclogin-technical.md). Three
  independent reads of the code led to 47 fixes; none of it has
  run on a console or display.
- recab and the Recab app reach only an A/B installation, the Updater app
  only a mutable one.
- Recipe pinned to a Ginux that includes upstream's work through gates 25 and
  26; Recipe's acceptance suite passed on it before the work above.
- The LevitateOS image generates its CA trust bundles; Firefox ships one
  bookmark; the Plasma panel has the Recipe app where Discover was.
- Recipe, Recab, Recdirstat and the Updater have their own icons.
- The agent app is removed from the desktop apps; they keep only a "Copy for
  agent" button. The install hub is gone; the install screen is the terminal
  plus the guide.

Not there yet:

- A published release. The repository
  [`LevitateOS/releases`](https://github.com/LevitateOS/releases) exists and is
  empty.
- A recipe for NVIDIA's proprietary driver.
- reclogin as the login screen of an image. It is written (see above), but
  SDDM stays the login screen until reclogin has been built and published for
  each OS; the steps of the switch are in
  `tools/reclogin/dist/recipe/SWITCH.md`.

The defects recorded in section 5 of the handoff (the install's Boot step on
both OSes among them) were seen on ISOs built before this work. Whether it
clears them is unknown until an image is built and installed. A system
installed before releases existed cannot be updated; it is installed again
from a release ISO.

## 8. Credits and licences

- Ginux is Greg Kurtzer's project, [`gmkurtzer/ginux`](https://github.com/gmkurtzer/ginux).
  The `.hdae` skills come from his [`gmkurtzer/hdae`](https://github.com/gmkurtzer/hdae).
- Fedora and Alpine supply the packages the two userlands are built from.
- Our code is MIT OR Apache-2.0: [`LICENSE`](LICENSE) explains what is under
  which licence, with [`LICENSE-MIT`](LICENSE-MIT),
  [`LICENSE-APACHE`](LICENSE-APACHE) and the other texts in
  [`LICENSES/`](LICENSES). Built images contain packages under their own
  licences.

Website: [levitateos.org](https://levitateos.org).
