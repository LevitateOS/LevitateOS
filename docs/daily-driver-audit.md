# Daily-driver audit: how far are the two desktops?

Audit of 2026-11-11, superproject `4e1c0c8`. The question: how far are
LevitateOS (Plasma) and AcornOS (LXQt) from a desktop a person can install on
their own computer and live on for months.

How it was made: the declarations (both `distro.toml` files, the six lock
files, `installed-system.rhai`, `live-overlay.rhai`, the shared kconfig), the
tools' READMEs and sources, the run manifests under `.artifacts/out`, CI's run
list and the published releases were read. Nothing was built or booted for
this audit. Where a consequence is reasoned from the code and not observed, it
says "expected".

## The answer in short

Neither system is a daily driver, and nobody could have tried: **no desktop
image has ever come out of the pipeline.** The desktop exists as about 960
locked packages and a configuration recipe per system, all committed unbuilt.
What has run in a virtual machine is the text-mode system of 2026-11-09, which
is 165 commits behind and has the old kernels and no firmware.

Each area sits on one of five rungs:

| Rung | Meaning |
|---|---|
| 0 | not written |
| 1 | written, never built |
| 2 | built and run in a VM |
| 3 | run on a real computer |
| 4 | lived on for weeks, updates included |

| Area | LevitateOS | AcornOS |
|---|---|---|
| Base system, text install, A/B switch | 2 (build of 11-09; code has moved since) | 2 (same) |
| Kernel and drivers | 2 (QEMU boot test only) | 2 (same) |
| Firmware | 1 | 1 |
| Desktop session (login screen, Plasma / LXQt, audio, network) | 1 | 1 |
| Install from the desktop ISO | 1 | 1 |
| Settings that must outlive a reboot (Wi-Fi, password, printer) | 0 on an A/B install | 0 on an A/B install |
| Everyday apps beyond the browser | 0 | 0 |
| Adding desktop software (Recipe) | 0 for desktop programs; 2 for command-line tools, outside an image | same, and upstream desktop builds cannot run on musl yet |
| The three maintenance apps (Recipe, System, Disk) | 1, against stand-ins | 1, and the agent helper lacks `gdbus` |
| System updates reaching an installed computer | 0 (no channel, no keys) | 0 |
| Disk encryption, Secure Boot, microcode | 0 | 0 |

Nothing is on rung 3 or 4. LevitateOS is the nearer of the two: on glibc an
upstream program runs as shipped, and its session has fewer holes.

## What is solid

- **The pipeline builds and installs a text system.** On 2026-11-09 both
  distros built `base-rootfs`, `live-boot` and `live-tools`, and every scenario
  passed under libvirt: live boot, the 44 live tools with a real installer
  terminal, a five-step install, boot of the installed system, and an A/B
  update "verified across 7 boots", fallback and rollback included
  (`.artifacts/out/<id>/scenarios/*/run-manifest.json`).
- **The kernels are built, published and pinned**: 7.3.3 for LevitateOS, 6.18.62
  for AcornOS (2026-11-10). The built configs carry every option the kconfig
  asks for; `linux-prebuilt.rhai` pins the digests and they match the published
  files.
- **Driver coverage is wide** (`distro/variants/_shared/kconfig`): Wi-Fi (Intel,
  Atheros 9k–12k, Realtek rtw88/89, MediaTek, Broadcom), Intel/AMD/nouveau
  graphics with a simple-framebuffer fallback, HDA and SOF audio, NVMe, AHCI,
  VMD, USB4, SD readers, touchpads, the common laptop platform drivers, both
  p-state drivers, Bluetooth, UVC, Ethernet up to 2.5G, USB tethering, dm-crypt,
  KVM, containers.
- **The desktop is complete on paper.** Every package either `distro.toml` asks
  for is in its lock (961 unique packages on L, 971 on A), every enabled service
  has its package, and no configuration is written for a program that is
  missing.
- **recab's code is whole**: no stubs; install with two equal slots,
  continuation by changed ranges, merge of an update with pending changes,
  finalize, trial, fallback, rollback. Proven by native loop-device tests.
- **Recipe installs command-line tools**: twelve tools each installed, ran on
  the glibc host and in an Alpine root, and were removed cleanly
  (`.artifacts/recipe-cli-batch/results.json`). The catalog repository is public
  and serves its index.
- **The website says the truth**: "Not released yet", first releases planned
  for 2027.

## Blockers

Each of these alone keeps the system from being used daily. In the order they
would be met.

### 1. No desktop image exists

- `.artifacts/out/*/releases` has no `live-desktop`; no desktop ISO or rootfs is
  anywhere on disk. The two screenshots of 2026-11-07 are host-built
  prototypes, not pipeline output.
- `desktop-boot` has never run. It is left out of `cargo xtask test all` and of
  CI, so nothing would notice a broken desktop.
- Unrun on both systems: `desktop_system` in `installed-system.rhai` (theme,
  services, default apps, firewall zone, polkit rule), the live overlay's
  install entry, the step that builds the three Qt apps in the distro's own
  root (`desktop_apps.rs`, `desktop-apps.rhai`).
- Expect several rounds of fixes. About 750 desktop packages per system land
  through recipes that have only ever unpacked about 210.

### 2. Everything tested is stale, and CI is red

- Newest build: 2026-11-09 10:24 UTC. Since then: 165 superproject commits,
  38 in the builder, 20 in the contract, 34 in recab.
- The tested LevitateOS image has kernel 6.12.116, the tested AcornOS image has
  7.3.3: the lines were swapped afterwards. Both have only `regulatory.db`
  under the firmware directory.
- So no image has been built with the current kernels, kconfig or firmware.
- recab was rewritten after the last `ab-update` pass. Its README calls those
  passes "historical evidence" and says installed boots with the read-only
  `/etc` "remain unverified".
- CI's "Products and scenarios" job failed for both distros in about 38 runs
  from 2026-11-08 to 11-11; the only green runs skipped it. Two runs were in
  progress when this was written.

### 3. Never started on a real computer

- All evidence is QEMU with OVMF. Suspend, lid, brightness keys, Wi-Fi, sound,
  Bluetooth, the touchpad, an external monitor and battery life are untested.
- No CPU microcode: no package in any lock, and nothing in recinit to load it
  early.
- UEFI only, and Secure Boot must be switched off: the boot files are unsigned
  (no shim, no signing step; `recuki` "does not sign").
- NVIDIA: nouveau only. The proprietary driver is a planned recipe, not
  written.

### 4. An installed A/B system cannot keep everyday settings

- The running root is read-only, `/etc` with it, and the writable overlay was
  removed. The user docs say it: "Commands that write ordinary `/etc` cannot
  change the running root" (`07-post-installation.ts:27`). The only route is
  `recab edit`, finalize, restart into the other slot.
- recab's runtime policy redirects only the mount table, DNS, a few systemd
  first-boot writers and the OpenRC clock (`tools/recab/src/runtime.rs`).
- Expected to fail on a running system, because each writes under `/etc`:
  - saving a new Wi-Fi network or VPN (`/etc/NetworkManager/system-connections`);
  - changing a password, adding a user (`/etc/shadow`, `/etc/passwd`);
  - adding a printer (`/etc/cups`);
  - changing the time zone or the hostname;
  - a lasting firewall rule (`/etc/firewalld`);
  - SDDM's autologin setting from System Settings.
- The desktop recipe's own polkit rule lets every user set the time zone and
  save a network (`installed-system.rhai:696`); on an A/B install both would be
  permitted and then fail.
- This is a design gap, not a bug to patch: it needs a decision on which state
  belongs to the computer (and lives on `/var`) and which belongs to the system
  (and lives in the slot). `desktop-todo.md` section 8 still lists it as "confirm
  what is shared and what is per slot".
- A mutable install has none of this, and no A/B either.

### 5. There is almost no software to add, and it would not show up

- The catalog holds 18 recipes: 13 command-line tools and 5 build tools. No
  recipe installs a program with a window.
- Planned and unwritten (`recipe-catalog.md`, layer 4): a PDF viewer, an image
  viewer, mpv, OnlyOffice, Thunderbird, Element, VSCodium, Krita. Not even
  planned: Chromium, Steam, Signal, Telegram, Discord, Slack, Zoom, Spotify,
  GIMP, Inkscape, OBS, Podman or Docker, a VM manager, Wine.
- Recipe's activation only puts `bin` on `PATH`. The profile's `share` is not in
  `XDG_DATA_DIRS`, and no menu, icon or MIME cache is refreshed. An installed
  desktop program would not appear in the menu or open its files.
- Self-contained packages are proven for htop and a hand-built Qt "hello" drawn
  offscreen. Open: the automatic step, a real program on a booted desktop, a
  Qt built by a recipe, the locale on AcornOS.
- AcornOS: the catalog plan says upstream desktop builds "cannot run on AcornOS
  until Recipe can settle a program it did not link".
- Without a recipe a person has two routes: write one by hand, or have an agent
  write it. No image ships Codex or Claude Code, and no recipe installs either.
- A system-wide install on an A/B system goes into the other slot and needs a
  restart.

### 6. No update can reach an installed computer

- No channel is published: no signing keys, no Pages site, no release payloads.
  Both `distro.toml` files say `channel = "development"`. The workflows exist
  behind three switches that are off (`release/updates/README.md`).
- The only public release is "Boot to shell" of January 2026.
- Nothing re-locks or rebuilds on a schedule, and nothing watches advisories.
  The daily refresh job only renews the freshness of an index that already
  exists. A security fix to any of the ~960 packages needs a person to lock,
  build, sign and publish.
- Firefox is in the image with its own updater turned off by policy. Browser
  security fixes therefore arrive only as a whole new system. For a daily
  driver this sets the cadence: Firefox ships fixes about every two weeks.
- `recipe update` installs what the catalog pins; nothing notices a new
  upstream release, and nothing in the session says an update is waiting.

### 7. The install is still a typed procedure

- Partition, extract, fstab, configure, boot step, unmount: typed commands
  beside a guide, with TUIs for three of the steps. The system-setup TUI was
  "written 2026-11-10, not yet run on an ISO"; the boot step has no page.
- No Wi-Fi picker (typed `nmcli`), one administrator only, no graphical
  installer, and the whole disk is erased by default.
- From the desktop ISO the same TUI runs in Konsole or QTerminal; its size,
  colours and getting root there are unchecked.

## Serious gaps

Usable without them, but a person would notice within a week.

**Both systems**

- No disk encryption: recpart has none, recinit loads no LUKS or LVM modules,
  the base locks have no `cryptsetup`. On a laptop many would call this a
  blocker.
- No hibernation (decided); no hang watchdog, so a trial that freezes does not
  fall back; no rescue path when both slots fail other than reinstalling; no
  backup or snapshot of `/home`.
- The three maintenance apps have never run in an image or against the real
  `recipe` and `recab`. Of 231 listed gaps, 61 are done, 91 partly, 79 open
  (`gui-ux-gaps.md`). Everything that hands work to an agent fails until the
  person installs one.
- No PDF viewer, media player, office suite or mail program in either image.
  Firefox stands in for PDFs.
- No fingerprint reader support, no input methods (decided), no Plymouth, no
  first-login welcome, no notice of updates or of a restart that is due.
- The default language is `C.UTF-8` until recsetup changes it; whether Plasma
  and LXQt follow recsetup's locale, clock format and paper size is unchecked.
- The user docs have no page on the desktop.

**LevitateOS only**

- H.264 and HEVC: Fedora's `libavcodec-free` with the `noopenh264` stub and no
  `gstreamer1-plugins-good`. Expected: much web and local video will not play
  until the codec recipe exists. AcornOS ships Alpine's full codecs and does
  not have this problem.
- Fedora's branding is still in the lock (`fedora-logos`,
  `plasma-lookandfeel-fedora`, `kde-settings*`, `f45-backgrounds-*`), and the
  dark look, wallpaper and start-button steps patch Fedora's files. It has to
  go before anything is published under the LevitateOS name.
- No image viewer (pictures open in Firefox); no libcamera plug-in for
  PipeWire.

**AcornOS only**

- Bluetooth headphones: no PipeWire Bluetooth plug-in is in the lock, so
  pairing works and audio does not.
- No emoji font, and no `-lang` packages, so LXQt and its apps are in English
  whatever the locale.
- The screen does not lock on suspend or lid close (no `xss-lock`).
- No screen sharing (no screen-cast portal on X11 here), no night light, no
  clipboard manager, no power profiles, no HiDPI setting.
- Mousepad is installed but is not the default for text files.
- The agent helper needs `gdbus`, which neither Alpine lock has.
- `networkmanager-wwan` is in the lock without ModemManager (mobile broadband
  is decided as unsupported; the package is dead weight).

## Polish

Open in `desktop-todo.md` and not repeated here: own Global Theme and SDDM
theme (L), LXQt themes, palette and window decoration (A), the panel layout
(A), the xscreensaver dialog (A), battery charge thresholds, accessibility at
the login screen, the install icon, screenshots for the website.

## The distance, as stages

Each stage ends in something observable. The order matters: every later stage
needs the image of the first.

1. **It boots.** Build both desktop ISOs from today's locks, run
   `desktop-boot`, install from the desktop ISO in a VM and log in. These run
   locally (CI runs only the quick checks). Get CI green.
2. **It works on a real laptop.** Microcode. One Intel and one AMD laptop each:
   Wi-Fi, sound, Bluetooth, suspend, lid, brightness, an external screen.
3. **It survives a week.** Decide and build where computer state lives so
   Wi-Fi, passwords, printers and the time zone can be changed (blocker 4).
   Run the current recab through `ab-update` on both. Close AcornOS's session
   holes.
4. **It has the software.** Recipe puts menus, icons and file types where the
   desktop looks; the first desktop recipes (PDF, pictures, media, office,
   mail, chat); codecs on LevitateOS; an agent that can be installed. For
   AcornOS, the step that lets it run a program it did not link.
5. **It stays safe for months.** A published, signed channel; a cadence that
   keeps Firefox current; update notices; encryption; Secure Boot signing.

LevitateOS can reach stage 3 on work that is mostly already written and needs
running. Stages 4 and 5 are new work on Recipe and on release operations.
AcornOS shares all of it and adds the musl problem in stage 4.

## Stale statements found on the way

- `desktop-todo.md`: the Wi-Fi kconfig item is done; section 5's table
  predates KCalc, Mousepad, Qalculate and `qps`; section 8 says the recab GUI
  is "not started" and that `stage` is destructive; several "not in the lock"
  lines name packages that now are; the default-apps file is
  `/etc/xdg/mimeapps.list`, not the path given; `kwallet-pam` is Fedora's
  `pam-kwallet`; `mesa-va-drivers` is inside `mesa-dri-drivers`.
- `install-tui-todo.md` says neither distro ships Wi-Fi firmware and that recab
  populates only A; `03-installation-disk.ts:119` repeats the second.
- `tools/recab/REQUIREMENTS.md`'s header still says the implementation does not
  meet the requirements; the README says it does, unverified on a booted
  system.
- `docs/desktop.md:30-32` lists the old short package set.
- `recipe-sources.md:343` says the recipes repository is archived; it is public
  and serving `index.json`.
