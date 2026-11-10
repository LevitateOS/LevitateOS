# Desktop: what is left to configure and implement

Everything still needed for a full desktop on LevitateOS (Plasma, Fedora 45
packages, systemd) and AcornOS (LXQt, Alpine 3.24 packages, OpenRC). Building
and booting are left out: this list is only what has to be written, declared
or decided. Companions: [desktop.md](desktop.md) (how the desktop system is
built), [desktop-theming.md](desktop-theming.md) (the look),
[recab-gui.md](recab-gui.md) (the update app).

How it was made (2026-11-10): the two desktop package requests in
`distro/variants/<id>/distro.toml`, the four lock files in
`distro/builder/recipes/sources/`, `installed-system.rhai`, `live-overlay.rhai`
and recsetup were read. "Not in the lock" means no package of that name is in
the variant's base or desktop lock today. Nothing here was checked on a running
desktop.

Where things go:

- **A package in the image**: `[desktop].packages` in the variant's
  `distro.toml`, then the lock is regenerated (`cargo xtask base lock <id>`).
- **Configuration of the image**: `installed-system.rhai` (`desktop_system` and
  the functions it calls), from files in `distro/variants/<id>/branding/` where
  it needs any.
- **Live session only**: `live-overlay.rhai`.
- **Software the user adds later**: Recipe, and nothing else (`AGENTS.md`,
  "Recipe is the product").

Marks: `[x]` done (committed, unbuilt), `[ ]` to do, **L** LevitateOS,
**A** AcornOS, **decide** needs the user's choice first.

## 1. Before anything else works

- [ ] **L** Regenerate the locks. `distro.toml` asks for `linux-firmware`, the
  GPU firmware packages and the Intel Wi-Fi firmware, but no firmware package
  is in the lock, so the image has none.
- [ ] **A** Firmware: the request names none and the lock has none. Add
  `linux-firmware` (or the split packages for Intel, AMD, Realtek, Atheros,
  MediaTek Wi-Fi and Bluetooth) and `sof-firmware` for laptop audio.
- [ ] **L** `sof-firmware` and `alsa-sof-firmware` for laptop audio are not in
  the lock (check after the lock is regenerated).
- [ ] **L A** recstrap's free-space check is 2 GiB; the desktop system is
  larger. Take the needed size from the image instead of a constant.
- [ ] **L A** Lock generation needs an exclude list (`--exclude` for dnf), used
  below to drop Fedora's art packages.
- [ ] **A** Carry owners and special modes from apk's database into the
  recipe's target metadata (setuid `dbus-daemon-launch-helper`,
  `polkit-agent-helper-1`, `doas`), if the apk backend does not already.
  Without them polkit prompts and D-Bus activation fail.

## 2. Look and branding

Details and paths are in [desktop-theming.md](desktop-theming.md).

- [x] **L A** Wallpapers, light and dark, as the default.
- [x] **L A** Login screen background.
- [x] **L A** Start button shows the leaf / the acorn.
- [ ] **L** Own Global Theme `org.levitateos.desktop`: defaults (colour scheme,
  icons, cursor, Plasma style, window decoration, wallpaper, splash), panel
  layout, splash screen with the leaf, previews.
- [ ] **L** Colour schemes `LevitateOSLight` and `LevitateOSDark` (Breeze with
  the pine accent).
- [ ] **L** `/etc/xdg/kdeglobals`: the Global Theme and colour scheme as the
  default. Decide whether light or dark is the default (**decide**).
- [ ] **L** A launcher icon that follows a light or dark panel
  (`distributor-logo-levitateos-symbolic`); needs sign-off as a new variant of
  the mark (**decide**).
- [ ] **L** Drop Fedora's branding from the lock: `fedora-logos` (ask for
  `generic-logos`), `plasma-lookandfeel-fedora`, `kde-settings`,
  `kde-settings-plasma`, `kde-settings-sddm`, `desktop-backgrounds-*`,
  `f45-backgrounds-*`, `fedora-release*` if the desktop request pulls them in.
  When `kde-settings-plasma` goes, the start button step must set the icon
  through our own Global Theme instead of Fedora's scripts.
- [ ] **L** Remove the two Fedora files inside packages we keep:
  `usr/share/sddm/themes/01-breeze-fedora` and Firefox's
  `firefox-redhat-default-prefs.js`.
- [ ] **L** Own SDDM theme `levitateos` (Breeze's QML with the lockup,
  `color=#14221A`), replacing the override of Breeze.
- [ ] **L** About page: `/etc/xdg/kcm-about-distrorc` (logo, website, variant).
- [ ] **A** LXQt themes `acornos-light` and `acornos-dark` (panel, runner,
  notification and leave-dialog QSS; `mainmenu.svg`; `wallpaper.cfg` so the
  wallpaper follows the theme; preview).
- [ ] **A** `/etc/xdg/lxqt/lxqt.conf`: theme, icon theme, widget style, font.
  The lock has `breeze-icons` but no Breeze widget style and no GTK theme: add
  `breeze` and `breeze-gtk`, or pick Fusion and Adwaita (**decide**).
- [ ] **A** Qt palette `/usr/share/lxqt/palettes/AcornOS`.
- [ ] **A** Openbox window decoration `AcornOS` and `/etc/xdg/openbox/lxqt-rc.xml`.
- [ ] **A** GTK settings (`/etc/gtk-3.0/settings.ini`) so Firefox matches.
- [ ] **A** Login screen: add the acorn lockup; decide between the Maldives
  copy that is there now and `sddm-breeze` (**decide**).
- [ ] **L A** `LOGO=<id>-logo-icon` and `HOME_URL` in os-release (builder,
  `pipeline/source.rs`), and the icon `<id>-logo-icon` in hicolor.
- [ ] **L A** Firefox: homepage and default prefs file for each system.
- [ ] **L A** Default user picture (keep Breeze's, or supply one).
- [ ] **L A** Cursor theme set the same in SDDM, the session and GTK apps.
- [ ] **L A** Boot splash: Plymouth is in neither lock; a theme also needs a
  hook in recinit. Until then the boot is console text (**decide** if wanted).

## 3. The session itself

### LevitateOS (Plasma on Wayland)

- [ ] `upower` is not in the lock: no battery state, no low-battery action in
  PowerDevil.
- [ ] `power-profiles-daemon` (or `tuned-ppd`) is not in the lock: no power
  profile switch.
- [ ] `udisks2` is not in the lock: Dolphin and the device notifier cannot
  mount USB drives. Add it, with `plasma-disks` for disk health if wanted.
- [ ] `kwallet-pam` is not in the lock: the wallet does not unlock at login, so
  Wi-Fi passwords prompt each time. Add it and its PAM lines for SDDM.
- [ ] `plasma-systemmonitor`, `kscreen`, `powerdevil`, `plasma-nm`, `plasma-pa`
  are requested; check after the lock that `kde-gtk-config`'s daemon,
  `xdg-desktop-portal-kde` and `polkit-kde` start with the session.
- [ ] Decide what replaces `kde-settings`' environment scripts once it is
  dropped (ssh-agent start; none is required).
- [ ] Baloo file indexing: on by default; decide on or off (**decide**).
- [ ] KDE crash handler (`drkonqi`) and help centre (`khelpcenter`) are not in
  the lock; decide whether to ship them (**decide**).
- [ ] Remove or hide menu entries that do nothing here (PackageKit is present
  only as a library; make sure no "Software" or "Updates" entry shows).

### AcornOS (LXQt on X11)

- [ ] `udisks2` and `gvfs` are not in the lock: the panel's removable-media
  button and pcmanfm-qt cannot mount drives, and there is no trash or network
  browsing.
- [ ] `setxkbmap` is not in the lock: LXQt's keyboard layout settings and the
  layout indicator need it.
- [ ] `xrandr` is not in the lock; `libkscreen` is, which LXQt's monitor
  settings use. Check that its X11 backend is installed.
- [ ] No compositor (`picom`) in the lock: no shadows or transparency, and
  tearing on some hardware (**decide**: add, or stay without).
- [ ] Screen lock: `xscreensaver` is in the lock; set it as LXQt's lock command,
  start it with the session, and give it the brown look instead of its default
  dialog.
- [ ] Session autostart: confirm `lxqt-policykit`, `lxqt-powermanagement`,
  `lxqt-notificationd`, `nm-applet` and PipeWire start (their autostart files
  come from the packages; `nm-applet`'s must not be hidden in LXQt).
- [ ] `xdg-desktop-portal` and a back-end (`xdg-desktop-portal-lxqt` or `-gtk`)
  are not in the lock: Firefox's file dialogs and screen sharing fall back or
  fail.
- [ ] `libnotify` is not in the lock (`notify-send` for scripts).
- [ ] gnome-keyring is in the lock: add its PAM lines for SDDM so it unlocks at
  login (NetworkManager's applet stores Wi-Fi passwords in it).
- [ ] X11 input defaults: tap-to-click and natural scrolling for touchpads
  (`/etc/X11/xorg.conf.d/` for libinput).
- [ ] Default panel layout (`/etc/xdg/lxqt/panel.conf`): decide the plug-ins
  (the desktop switcher and quick-launch defaults are upstream's).
- [ ] Global shortcuts (`lxqt-globalkeys`): volume, brightness, screenshot,
  terminal, lock.
- [ ] OpenRC services for the desktop beyond `dbus elogind polkit
  networkmanager sddm`: whatever sections 4 and 6 add (bluetooth, cupsd,
  udisks is D-Bus activated).
- [ ] Wayland: Alpine 3.24 has no `lxqt-wayland-session`; revisit at the next
  Alpine release.

## 4. Hardware

- [ ] **L A** Bluetooth: no `bluez` daemon in either lock. Add `bluez`, plus
  `bluedevil` (L) or `blueman` (A), and enable the service (**decide** for A:
  it is a large addition to a small system).
- [ ] **L A** Printing: only the CUPS libraries are in the locks. Add `cups`,
  `cups-filters`, driverless (IPP Everywhere) support, and `print-manager` (L)
  or `system-config-printer` (A); enable `cups.socket` / `cupsd` (**decide**
  for A).
- [ ] **L A** Scanning: `sane-backends` with `skanlite` (L) or `simple-scan`
  (**decide**).
- [ ] **L A** Video acceleration: `libva` is present but no driver. Add
  `mesa-va-drivers` and `intel-media-driver` (L), `mesa-va-gallium` and
  `intel-media-driver` (A).
- [ ] **A** Vulkan: no loader or Mesa Vulkan drivers in the lock.
- [ ] **L A** NVIDIA: only the open kernel driver (nouveau) and, on L, the GPU
  firmware once the lock is regenerated. The proprietary driver is a Recipe
  job (**decide** whether to support it).
- [ ] **A** `fwupd` is not in the lock (L has it); firmware updates for laptops
  and docks.
- [ ] **L A** Thunderbolt (`bolt`, `plasma-thunderbolt` on L).
- [ ] **L A** Phones and cameras: `libmtp` is on L through `kio-extras`; A has
  nothing (needs `gvfs-mtp`, `gvfs-gphoto2`).
- [ ] **L A** File systems on USB drives: neither lock has `exfatprogs` or
  `ntfs-3g`; the kernel configs need exFAT and NTFS3 on.
- [ ] **L A** Network shares: A has no SMB or NFS support; L has the Samba
  client libraries through `kio-extras`.
- [ ] **L A** Mobile broadband: only the ModemManager libraries on L, nothing
  on A (**decide**).
- [ ] **L A** VPN: no NetworkManager VPN plug-ins (WireGuard works in
  NetworkManager itself; OpenVPN needs `NetworkManager-openvpn`).
- [ ] **L A** Laptop basics: brightness keys, lid close, suspend and hibernate
  settings (logind / elogind configuration), battery thresholds.
- [ ] **L A** Webcam: `libcamera` is on A only; check PipeWire camera support
  on both.
- [ ] **L A** Virtual machine guests: neither lock has `spice-vdagent` or
  `qemu-guest-agent`, so clipboard sharing and display resize do not work in
  the libvirt test VMs.
- [ ] **L A** Kernel configs: confirm DRM drivers, evdev, HID multitouch,
  Bluetooth, USB audio, webcam (UVC), exFAT, NTFS3 and the Wi-Fi drivers the
  firmware above is for.

## 5. Apps that come in the image

Small on purpose; everything else comes through Recipe (section 6).

| Need | LevitateOS today | AcornOS today |
|---|---|---|
| Browser | Firefox | Firefox |
| Files | Dolphin | pcmanfm-qt |
| Terminal | Konsole | QTerminal |
| Text editor | KWrite | none |
| Archives | Ark, unzip, 7zip | lxqt-archiver, no unzip or 7zip |
| Screenshots | Spectacle | screengrab |
| Image viewer | none | LXImage-Qt |
| PDF viewer | none | none |
| Media player | none | none |
| Calculator | none | none |
| System monitor | Plasma System Monitor | none (htop in a terminal) |
| Disk tool | none | none |
| Settings | System Settings | LXQt configuration centre |

- [ ] **L** Add Gwenview, Okular, a media player (Haruna or Dragon) and KCalc,
  or leave them to Recipe (**decide**).
- [ ] **A** Add FeatherPad, a PDF viewer (qpdfview), a media player (mpv with a
  small front-end), a calculator, `unzip` and `7zip`, qps or a similar task
  manager, or leave them to Recipe (**decide**).
- [ ] **L A** Codecs: L has Fedora's `libavcodec-free` and `noopenh264`, so
  H.264 and AAC do not play; A has full ffmpeg libraries but no GStreamer.
  Decide what ships and what comes through Recipe (**decide**).
- [ ] **L A** Fonts: Noto Sans, Mono and emoji on L, `font-noto` on A. Add
  Noto Serif, a CJK set and metric-compatible fonts for documents
  (Liberation or Carlito/Caladea).
- [ ] **L A** Thumbnails: add `kdegraphics-thumbnailers` and
  `ffmpegthumbs` (L); pcmanfm-qt needs `ffmpegthumbnailer` (A).
- [ ] **L A** Spell checking: `hunspell-en-US` is on L only.
- [ ] **L A** Default apps: `/usr/share/applications/mimeapps.list` (browser,
  text, images, PDF, archives, folders).
- [ ] **L A** Home folders: `xdg-user-dirs` is in both locks; make sure
  `xdg-user-dirs-update` runs at first login (LXQt needs an autostart entry).
- [ ] **L A** Menu clean-up: hide entries that make no sense for a user
  (duplicate settings modules, `lxqt-admin`'s user tool if recsetup owns users,
  Avahi browsers and the like).
- [ ] **L A** Manual pages: neither lock has `man-db` or `mandoc`.

## 6. Software through Recipe

The desktop adds no other package manager, store or bundle format.

- [ ] **L A** Recipes for the apps people expect first (office suite, mail,
  chat, media, graphics, development tools); pick the first set (**decide**).
- [ ] **L A** Desktop integration of what Recipe installs: `.desktop` entries,
  icons, MIME types and D-Bus services must land where the menus look, and the
  caches (`update-desktop-database`, icon cache, MIME database) must be
  refreshed after an install or removal.
- [ ] **L A** Per-user or system-wide installs from the desktop: how a desktop
  user is asked for the administrator password (polkit action for Recipe).
- [ ] **L A** Update notice: something in the session that says a Recipe
  update is available.
- [ ] **L A** A graphical front-end for Recipe beside the command line, if
  wanted (**decide**).
- [ ] **L A** Proprietary drivers and codecs as recipes (NVIDIA, H.264).
- [ ] **L A** State what Recipe lacks for each point above and extend Recipe.

## 7. Installing from the desktop ISO

- [ ] **L A** "Install LevitateOS / AcornOS" launcher on the live desktop and
  in the menu: opens a terminal running the installer; icon `<id>-install`
  (mark plus an arrow into a disk, drawn as SVG). Live overlay only.
- [ ] **L A** The installer inside a desktop terminal: check the TUI's size
  and colours in Konsole and QTerminal, and that it gets root (sudo / doas for
  the live user).
- [ ] **L A** recsetup's user step for a desktop install: put the user in the
  groups the session needs (`wheel video input audio netdev plugdev`, those
  the image has), as the live overlay does for `live`.
- [ ] **L A** recsetup sets the console keymap only. On a desktop install also
  write the X11/Wayland layout: `/etc/X11/xorg.conf.d/00-keyboard.conf` (A,
  and SDDM on both) and Plasma's default layout (L).
- [ ] **L A** recsetup's locale step: Plasma and LXQt take the language from
  it; check both read `/etc/locale.conf` / the profile on their init.
- [ ] **L A** Autologin choice at install (off by default), written to
  `/etc/sddm.conf.d/`.
- [ ] **L A** recsetup's service list shows SDDM without a `.wants` link
  (it is enabled by alias); show it as enabled.
- [ ] **L A** First login: a short welcome (what the system is, how to get
  software with Recipe, how updates work) (**decide** if wanted).
- [ ] **L A** The live session must not sleep or lock during an install.
- [ ] **L A** Live user's password `live` is shown nowhere on the desktop; put
  it in the welcome or on the lock screen hint.

## 8. System updates on the desktop

- [ ] **L A** recab GUI: planned in [recab-gui.md](recab-gui.md) (Qt 6
  Widgets, pkexec), not started; waits for the user's go.
- [ ] **L A** A notice in the session when a new system image is available,
  and when a restart is needed to switch slots.
- [ ] **L A** polkit rules for recab's actions.
- [ ] **L A** What a flavour change means for the user (`recab stage
  --change-flavour`): wording in the GUI and docs.
- [ ] **L A** Home directories and user settings across a slot switch:
  confirm what is shared and what is per slot.

## 9. Language, input and time

- [ ] **L** Languages: glibc language packs for cs, de, en, es, fi, fr, it,
  nb, nl and a few more are in the lock; KDE's translations come with each
  package. Decide the supported set and add the missing packs (**decide**).
- [ ] **A** `musl-locales` is present; LXQt's translations come with the
  packages. Check the locale list recsetup offers.
- [ ] **L A** Input methods for CJK and others: only `ibus-libs` on L, nothing
  on A. Add fcitx5 or ibus if those languages are supported (**decide**).
- [ ] **L A** Time zone: recsetup sets it; check the panel clocks follow, and
  decide on automatic time zone (geoclue is on L only).
- [ ] **L A** Time sync: chrony is in both locks; make sure it is enabled on
  the desktop system.
- [ ] **L A** Clock format, first day of the week and paper size follow the
  locale.

## 10. Security and privacy

- [ ] **L A** Firewall: neither lock has firewalld, nftables or ufw. Decide
  whether the desktop ships one, with a default that blocks incoming
  connections (**decide**).
- [ ] **L** Screen locker (`kscreenlocker`) is present; check it works with
  the PAM stack the recipe writes (`system-auth`), including after suspend.
- [ ] **A** Screen lock on suspend and on lid close (xscreensaver with
  `xss-lock` or LXQt's own setting).
- [ ] **L A** sshd is enabled on the live ISO; decide whether an installed
  desktop has it off by default (**decide**).
- [ ] **L A** Polkit rules for the things a desktop user does without a
  password (mount a USB drive, connect to Wi-Fi, suspend, change time zone).
- [ ] **L A** Encrypted installs: unlock prompt at boot is console text
  without Plymouth.
- [ ] **L A** Firefox: telemetry and first-run pages (policy file).

## 11. Accessibility

- [ ] **L A** Screen reader: Orca is in neither lock (`at-spi2-core` is in
  both).
- [ ] **L A** On-screen keyboard: `plasma-keyboard` is on L; A has none.
- [ ] **L A** High-contrast and large-text settings reachable from the login
  screen.

## 12. Documentation and checks

- [ ] **L A** User docs: what the desktop ISO is, installing from it, getting
  software with Recipe, updating with recab, switching between light and dark.
- [ ] **L A** Re-record the install docs stories that show the desktop
  section.
- [ ] **L A** `desktop-boot` scenario: add checks for the pieces above as they
  land (wallpaper file present and set, SDDM theme, the start-button icon
  resolves, user services for audio running, NetworkManager up, a removable
  drive mounts, no failed unit or service).
- [ ] **L A** A scenario for an install from the desktop ISO that boots to the
  login screen and logs in.
- [ ] **L A** Screenshots of both desktops, light and dark, for the website.

## Decisions waiting for the user

1. Light or dark as the default look on each system.
2. How small AcornOS stays: Bluetooth, printing, scanning, a compositor, a
   media player and input methods each add a lot.
3. Which apps ship in the image and which come only through Recipe
   (section 5), and the first set of app recipes (section 6).
4. Codecs: what ships, what is a recipe.
5. NVIDIA's proprietary driver: supported or not.
6. A firewall on the desktop, and sshd off on installed desktops.
7. A graphical front-end for Recipe, and a first-login welcome.
8. Boot splash (Plymouth) or console text.
9. Supported languages and input methods.
10. Breeze or Fusion as AcornOS's widget style, and which login theme.
11. Whether the LevitateOS light and dark wallpapers are right, or the
    original half-and-half picture should be one of them.
