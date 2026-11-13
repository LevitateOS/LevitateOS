# Optional desktop: KDE Plasma (LevitateOS), LXQt (AcornOS)

Each variant gets a second ISO that installs a desktop system. It boots like
every live ISO, into the install screen on tty1 (decided 2026-11-12: the live
session is the installer, not a desktop); the desktop starts once installed.
The base ISO does not change.

What is still to configure and implement, for both systems, is in
[desktop-todo.md](desktop-todo.md).

## Status (2026-11-07)

Both desktops were built on the host the way the recipes build a rootfs, then
booted under libvirt with the distribution's own kernel, and both reach an
autologged-in desktop. The real pipeline cannot build them yet: no live ISO
currently builds, because Recipe fails to extract the Fedora RPMs inside Ginux
(see `tools/recab/README.md`).

| | LevitateOS | AcornOS |
|---|---|---|
| Desktop | Plasma on Wayland, SDDM | LXQt on X11 (Alpine has no `lxqt-wayland-session`), SDDM |
| X server | Xwayland only (`xorg-x11-server-Xwayland`) | XLibre 25.1, compiled from source (since 2026-11-13; booted desktops above ran Alpine's `xorg-server`) |
| Result | No failed units; logind session type `wayland` | Desktop up; OpenRC marks sddm failed (see below) |

## What the desktop recipes must do

### LevitateOS (Plasma)

Packages: the base set plus `plasma-desktop plasma-workspace kwin sddm
sddm-breeze sddm-wayland-plasma xorg-x11-server-Xwayland konsole dolphin kwrite
ark spectacle plasma-systemmonitor plasma-nm plasma-pa powerdevil kscreen
kinfocenter kde-gtk-config breeze-gtk breeze-icon-theme xdg-desktop-portal-kde
polkit-kde kwalletmanager5 pipewire pipewire-pulseaudio pipewire-alsa
wireplumber mesa-dri-drivers mesa-vulkan-drivers google-noto-sans-fonts
google-noto-sans-mono-fonts google-noto-color-emoji-fonts NetworkManager-wifi
wpa_supplicant firefox systemd-pam`, resolved without weak dependencies.

- `systemd-pam` is required and nothing pulls it in. Without `pam_systemd.so`,
  logind registers no session and Wayland gets no seat.
- The lock needs dependency solving. Fedora's solver (`dnf download --resolve
  --alldeps --url`, unprivileged, with `--repofrompath` for the release and
  updates repositories and `excludepkgs=*.i686`) gives the closure.
- Steps the scriptlets would take, run in the tree: `systemd-sysusers` (about 25
  accounts: sddm, polkitd, pipewire, rtkit, geoclue, …), `ldconfig`,
  `glib-compile-schemas`, `gdk-pixbuf-query-loaders-64 --update-cache`,
  `gio-querymodules-64`, `update-mime-database`, `fc-cache -s`,
  `gtk-update-icon-cache` per icon theme, `update-desktop-database`.
- Services: enable `sddm.service` (display-manager) and NetworkManager, default
  target `graphical.target`; per user (`systemctl --global`): enable
  `dbus-broker.service` (the user bus, `dbus.service` alias; without it the
  PipeWire sockets fail and there is no audio), `pipewire.socket`,
  `pipewire-pulse.socket`, `wireplumber.service`; mask `unity-gtk-module.service`.
- Branding: the wallpaper, login background and start button are
  LevitateOS's; the rest still shows Fedora's. [desktop-theming.md](desktop-theming.md) lists every
  piece for both desktops.

### AcornOS (LXQt)

Packages: the base set plus `lxqt-desktop lximage-qt pavucontrol-qt screengrab
sddm eudev udev-init-scripts udev-init-scripts-openrc mesa-dri-gallium elogind
polkit-elogind dbus dbus-x11 pipewire wireplumber pipewire-pulse pipewire-alsa
networkmanager networkmanager-wifi network-manager-applet breeze-icons
font-noto firefox` and the rest `[desktop].packages` lists, among them the X
server's run-time libraries, `xkbcomp`, `xkeyboard-config` and the cursor and
misc fonts.

- The X server is XLibre, the fork of the X.Org server, not Alpine's
  `xorg-server` (desktop-todo.md, decision 14). Alpine has no package for it,
  so `distro/builder/os-recipes/xlibre.rhai` compiles the server (tag
  `xlibre-xserver-25.1.11`) and XLibre's `xf86-input-libinput` (tag
  `xlibre-xf86-input-libinput-25.0.1`), each pinned by commit, with meson in
  the Alpine apps-build root, with Alpine's own `xorg-server` options, and
  publishes a tree laid out like Alpine's package: `/usr/bin/X` → `Xorg` (a sh
  script) → `/usr/libexec/Xorg.wrap` (setuid root through the target
  metadata) → `/usr/libexec/Xorg`, modules in
  `/usr/lib/xorg/modules/xlibre-25` (modesetting is built in),
  `/usr/share/X11/xorg.conf.d/{10-quirks,40-libinput}.conf`. The builder
  (`distro/builder/src/display_server.rs`) caches it as the prebuilt family
  `display-server` and merges it into the desktop system, which it refuses
  without the server, the setuid wrapper, `/usr/bin/X` and both drivers.
  SDDM starts `/usr/bin/X` by its compiled-in default, unchanged. Keeping
  XLibre current is this project's job: its fixes are announced only in its
  release notes, so each 25.1 point release is a security release, and the
  pins are at the top of `xlibre.rhai`. X.Org-built drivers (`xf86-video-*`,
  `xf86-input-*` packages) do not load in XLibre and must not be added.

- apk solves dependencies and runs package scripts and triggers even in
  usermode. Owners and special modes, however, live only in
  `lib/apk/db/installed` (`M:`/`a:` records; 1,641 apply, all `777` records are
  symlinks); the recipe's target metadata must carry them (setuid
  `dbus-daemon-launch-helper`, `polkit-agent-helper-1`, `doas`, …).
- Runlevels: sysinit `devfs dmesg udev udev-trigger udev-settle` (libinput and
  the X server, XLibre built with `-Dudev=true`, need udev, not mdev); boot `modules sysctl hostname bootmisc syslog
  hwclock localmount root`; default `dbus elogind polkit networkmanager sddm`.
  NetworkManager replaces dhcpcd/networking on the desktop.
- The live user needs groups `wheel video input audio netdev plugdev`.
- PipeWire starts from `/etc/xdg/autostart/pipewire.desktop`
  (`pipewire-launcher`); nothing else is needed.
- Alpine's `/etc/init.d/sddm` waits 2 s in `start_post` for
  `/run/user/<sddm uid>`. With autologin the greeter never runs, so OpenRC marks
  sddm failed while the session runs. Acorn needs its own start_post for
  autologin, or the scenarios' `ERROR:` pattern will flag it.

## Software on the desktop

Apps come through Recipe, as on the base system; the desktop adds no other
package manager, store or bundle format (see `AGENTS.md`, "Recipe is the
product"). Apps bundled by the variant's desktop package list are part of the
OS image and its update lifecycle. Their producers live in
`distro/builder/os-recipes/`. Independently user-managed software belongs in the
top-level `recipes/` catalog, starting with ripgrep. Both collections use
the same Recipe format and engine. Recipe activates a published `package` tree
in its persistent profile, updates it from the selected catalog's pin and removes
it using the installed receipt. The current package layout supports commands and
shared data files; it does not yet integrate desktop services or application menus.

What the desktop still needs from Recipe: recipes for the apps
people expect, installed apps showing up in the menu with their icons and file
associations, a notice when something has an update, and, if wanted, a
graphical front-end beside the command line.

## In the pipeline (2026-11-08, not built or booted yet)

- `live-desktop` builds the **desktop system** (`REC_FLAVOUR=desktop` in its
  os-release): the variant's rootfs recipe installs the base lock and the
  desktop lock (`sources/<id>-<backend>-desktop-lock.rhai`, downloaded by
  `cargo xtask build rootfs-source <id> --desktop`) into one tree, and
  `installed-system.rhai`'s desktop policy does the packages' setup (caches,
  accounts, user units), enables the display manager and boots to it. That
  tree is the ISO's `filesystem.erofs`, so a desktop install is that system.
  The live overlay keeps the display manager off and gives tty1 to the install
  screen, as on every live ISO: systemd boots `multi-user.target`, OpenRC a
  `live` runlevel holding the installed system's `default` runlevel without
  the display manager (which `rc-service sddm start` still starts by hand).
  There is no live user. On OpenRC the sddm service's conf.d makes its
  start_post wait for the greeter succeed, so an autologin boot of the
  installed system leaves no failed service.
- `recab stage` refuses an image of the other flavour unless `--change-flavour`.
- `cargo xtask test desktop-boot <id>` boots it in 6 GiB (it is not in
  `test all`: its ISO is a build of its own).

The plan below is what the prototypes taught.

## Fitting it into the pipeline

- **Product**: a `live-desktop` product per variant (it builds its own rootfs, the desktop system, rather than extending `live-tools`).
  Desktop packages go into `filesystem.erofs` (the installer copies only that),
  live-session policy (the display manager kept off, the install screen on
  tty1) into the overlay.
- **Locks**: desktop packages declared beside `rpm_base`; `cargo xtask base lock
  levitate` solves the closure with dnf. Acorn needs a lock in the same shape
  (it pins `extra_apks` by hand today).
- **Recipe limits**: the rootfs recipe's domain needs `storage_max_objects` of
  about 150,000 (LevitateOS) and 80,000 (AcornOS); the default is 16,384. The
  `memory_mib` budget holds the whole tree and its inputs: about 6 GiB for
  Plasma (2.6 GB tree, 0.8 GB RPMs).
- **Live overlay**: on the desktop ISO, as on the others, the install screen
  holds tty1 and ttyS0; the display manager does not start in the live
  session. The overlay's OpenRC service list (`live-overlay.rhai`) only allows
  sshd/networking/dhcpcd.
- **A/B**: current `recab stage` checks the flavour and requires
  `--change-flavour` to switch deliberately. It still replaces the inactive
  system. The required update lifecycle must preserve local changes and updates
  in one prepared continuation, in either direction, with equivalent initial
  A/B systems. See [recab's contract](../tools/recab/REQUIREMENTS.md) for what
  remains missing.
- **Sizes**: libvirt VMs have 4 GiB RAM with a 50 % tmpfs upper; recstrap's
  free-space precheck is 2 GiB. Both need raising for the desktop.
- **Kernel**: both kconfigs start from `x86_64_defconfig` (evdev on). The
  prebuilt kernel in `.artifacts/kernel/levitate/current` (6.12.71) has no
  evdev, so it would boot the desktop without keyboard or mouse; it predates the
  current pin and needs rebuilding anyway.
