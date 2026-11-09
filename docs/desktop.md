# Optional desktop: KDE Plasma (LevitateOS), LXQt (AcornOS)

Each variant gets a second ISO with a graphical desktop: it boots live into
the desktop and installs a desktop system. The base ISO does not change.

## Status (2026-11-07)

Both desktops were built on the host the way the recipes build a rootfs, then
booted under libvirt with the distribution's own kernel, and both reach an
autologged-in desktop. The real pipeline cannot build them yet: no live ISO
currently builds, because Recipe fails to extract the Fedora RPMs inside Ginux
(see `tools/recab/README.md`).

| | LevitateOS | AcornOS |
|---|---|---|
| Desktop | Plasma on Wayland, SDDM | LXQt on X11 (Alpine has no `lxqt-wayland-session`), SDDM |
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
- Branding: the tree shows Fedora's wallpaper and start-menu logo. Replace
  them with LevitateOS's; [desktop-theming.md](desktop-theming.md) lists every
  piece for both desktops.

### AcornOS (LXQt)

Packages: the base set plus `lxqt-desktop lximage-qt pavucontrol-qt screengrab
sddm xorg-server xf86-input-libinput eudev udev-init-scripts
udev-init-scripts-openrc mesa-dri-gallium elogind polkit-elogind dbus dbus-x11
pipewire wireplumber pipewire-pulse pipewire-alsa networkmanager
networkmanager-wifi network-manager-applet breeze-icons font-noto firefox`.

- apk solves dependencies and runs package scripts and triggers even in
  usermode. Owners and special modes, however, live only in
  `lib/apk/db/installed` (`M:`/`a:` records; 1,641 apply, all `777` records are
  symlinks); the recipe's target metadata must carry them (setuid
  `dbus-daemon-launch-helper`, `polkit-agent-helper-1`, `doas`, …).
- Runlevels: sysinit `devfs dmesg udev udev-trigger udev-settle` (libinput and
  Xorg need udev, not mdev); boot `modules sysctl hostname bootmisc syslog
  hwclock localmount root`; default `dbus elogind polkit networkmanager sddm`.
  NetworkManager replaces dhcpcd/networking on the desktop.
- The live user needs groups `wheel video input audio netdev plugdev`.
- PipeWire starts from `/etc/xdg/autostart/pipewire.desktop`
  (`pipewire-launcher`); nothing else is needed.
- Alpine's `/etc/init.d/sddm` waits 2 s in `start_post` for
  `/run/user/<sddm uid>`. With autologin the greeter never runs, so OpenRC marks
  sddm failed while the session runs. Acorn needs its own start_post for
  autologin, or the scenarios' `ERROR:` pattern will flag it.

## In the pipeline (2026-11-08, not built or booted yet)

- `live-desktop` builds the **desktop system** (`REC_FLAVOUR=desktop` in its
  os-release): the variant's rootfs recipe installs the base lock and the
  desktop lock (`sources/<id>-<backend>-desktop-lock.rhai`, downloaded by
  `cargo xtask build rootfs-source <id> --desktop`) into one tree, and
  `installed-system.rhai`'s desktop policy does the packages' setup (caches,
  accounts, user units), enables the display manager and boots to it. That
  tree is the ISO's `filesystem.erofs`, so a desktop install is that system.
  The overlay adds the live user `live` (password `live`) and SDDM's autologin
  (`/etc/sddm.conf.d/live-autologin.conf`, the same on both inits); tty1 is
  the display manager's on both, so the install screen runs on ttyS0 and from
  a terminal. On OpenRC the sddm service's conf.d makes its start_post wait
  for the greeter succeed, so an autologin boot leaves no failed service.
- `recab stage` refuses an image of the other flavour unless `--change-flavour`.
- `cargo xtask test desktop-boot <id>` boots it in 6 GiB (it is not in
  `test all`: its ISO is a build of its own).

The plan below is what the prototypes taught.

## Fitting it into the pipeline

- **Product**: a `live-desktop` product per variant (it builds its own rootfs, the desktop system, rather than extending `live-tools`).
  Desktop packages go into `filesystem.erofs` (the installer copies only that),
  live-session policy (autologin, live user) into the overlay.
- **Locks**: desktop packages declared beside `rpm_base`; `cargo xtask base lock
  levitate` solves the closure with dnf. Acorn needs a lock in the same shape
  (it pins `extra_apks` by hand today).
- **Recipe limits**: the rootfs recipe's domain needs `storage_max_objects` of
  about 150,000 (LevitateOS) and 80,000 (AcornOS); the default is 16,384. The
  `memory_mib` budget holds the whole tree and its inputs: about 6 GiB for
  Plasma (2.6 GB tree, 0.8 GB RPMs).
- **Live overlay**: on the desktop ISO, tty1 belongs to the display manager;
  the install screen stays on ttyS0 and in a terminal. The overlay's OpenRC
  service list (`live-overlay.rhai`) only allows sshd/networking/dhcpcd.
- **A/B**: `recab stage` does not check the flavour; staging a base image onto a
  desktop install turns it back into the base system.
- **Sizes**: libvirt VMs have 4 GiB RAM with a 50 % tmpfs upper; recstrap's
  free-space precheck is 2 GiB. Both need raising for the desktop.
- **Kernel**: both kconfigs start from `x86_64_defconfig` (evdev on). The
  prebuilt kernel in `.artifacts/kernel/levitate/current` (6.12.71) has no
  evdev, so it would boot the desktop without keyboard or mouse; it predates the
  current pin and needs rebuilding anyway.
