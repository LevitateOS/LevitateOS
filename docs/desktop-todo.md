# Desktop: what is left to configure and implement

Everything still needed for a full desktop on LevitateOS (Plasma, Fedora 45
packages, systemd) and AcornOS (LXQt, Alpine 3.24 packages, OpenRC). Building
and booting are left out: this list is only what has to be written, declared
or decided. Companions: [desktop.md](desktop.md) (how the desktop system is
built), [desktop-theming.md](desktop-theming.md) (the look),
[recab-gui.md](recab-gui.md) (the update app).

How it was made (2026-11-10): the two desktop package requests in
`distro/variants/<id>/distro.toml`, the four lock files in
`distro/builder/os-recipes/sources/`, `installed-system.rhai`, `live-overlay.rhai`
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
- **Software the user adds later**: the top-level `recipes/` catalog, using the
  same Recipe engine as `distro/builder/os-recipes/`. The catalog is currently
  empty; activation and independent package updates/removal remain to be
  implemented in Recipe (`AGENTS.md`, "Recipe is the product").

Marks: `[x]` done (committed, unbuilt), `[ ]` to do, **L** LevitateOS,
**A** AcornOS, **decide** needs the user's choice first.

## 1. Before anything else works

- [x] **L A** Firmware is in both base locks (2026-11-10, builder `59e7065`):
  graphics, Wi-Fi, Bluetooth and laptop audio (`alsa-sof-firmware` on L,
  `sof-firmware` on A). AcornOS leaves NVIDIA's firmware out on purpose.
- [ ] **L A** CPU microcode is in neither image: nothing puts it in front of
  the initramfs yet (recinit / recuki).
- [ ] **L A** recstrap's free-space check is 2 GiB; the desktop system is
  larger. Take the needed size from the image instead of a constant.
- [ ] **L A** Lock generation needs an exclude list (`--exclude` for dnf), used
  below to drop Fedora's art packages.
- [x] **A** The accounts Alpine's install scripts would add are read from
  those scripts (apk keeps them in `lib/apk/db/scripts.tar.gz`): every
  `addgroup`/`adduser` is carried out, and a form the recipe does not read
  stops the build. Fedora's packages carry `sysusers.d` files, which the RPM
  recipe reads. Also `pinentry`'s link and the X core font indexes.
- [ ] **A** Owners and special modes of apk's files (setuid
  `dbus-daemon-launch-helper`, `polkit-agent-helper-1`, `doas`): Recipe's
  `target_capture_apks` reads them from each package's own archive, so this
  is likely done; check the three files' modes in a built image.

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
- [x] **L** `/etc/xdg/kdeglobals`: dark by default (decision 1) through
  Fedora's dark Global Theme (Breeze Dark colours and icons, our wallpaper)
  until our own Global Theme replaces it.
- [ ] **L** A launcher icon that follows a light or dark panel
  (`distributor-logo-levitateos-symbolic`); approved, to be drawn and
  reviewed (decision 12).
- [ ] **L** Drop Fedora's branding from the lock: `fedora-logos` (ask for
  `generic-logos`), `plasma-lookandfeel-fedora`, `kde-settings`,
  `kde-settings-plasma`, `kde-settings-sddm`, `desktop-backgrounds-*`,
  `f45-backgrounds-*`, `fedora-release*` if the desktop request pulls them in.
  When `kde-settings-plasma` goes, the start button step must set the icon
  through our own Global Theme instead of Fedora's scripts.
- [x] **L** Remove the two Fedora files inside packages we keep:
  `usr/share/sddm/themes/01-breeze-fedora` (removed) and Firefox's
  `firefox-redhat-default-prefs.js` (rewritten without Fedora's start page).
- [ ] **L** Own SDDM theme `levitateos` (Breeze's QML with the lockup,
  `color=#14221A`), replacing the override of Breeze.
- [x] **L** About page: `/etc/xdg/kcm-about-distrorc` (the mark, the website,
  "Desktop").
- [ ] **A** LXQt themes `acornos-light` and `acornos-dark` (panel, runner,
  notification and leave-dialog QSS; `mainmenu.svg`; `wallpaper.cfg` so the
  wallpaper follows the theme; preview).
- [x] **A** LXQt's settings: its packaged `lxqt.conf` already is the decided
  look (light theme, Fusion widgets, Breeze icons, Sans 11); left as packaged
  until our LXQt themes exist.
- [ ] **A** Qt palette `/usr/share/lxqt/palettes/AcornOS`.
- [ ] **A** Openbox window decoration `AcornOS` and `/etc/xdg/openbox/lxqt-rc.xml`.
- [x] **A** GTK settings (`/etc/gtk-3.0/` and `gtk-4.0/settings.ini`):
  Adwaita with Breeze icons, the Adwaita cursor and Sans 11.
- [ ] **A** Login screen: add the acorn lockup to the Maldives-based
  theme that is there now (decision 9).
- [x] **L A** `LOGO=distributor-logo-<id>` in os-release (the icon the desktop
  installs the mark as), and `HOME_URL` from the contract's new
  `identity.home_url` (LevitateOS only).
- [x] **L A** Firefox's default prefs file for each system: the OS as Firefox's
  distribution, Fedora's start page gone from the packaged prefs (renamed
  `<id>-default-prefs.js`).
- [x] **L A** Firefox homepage: `HOME_URL` where the OS has one
  (`https://levitateos.org` on L); Firefox's own on AcornOS (decision 11).
- [ ] **L A** Default user picture (keep Breeze's, or supply one).
- [x] **L A** One cursor theme for SDDM, the session, GTK and Qt (the `default`
  icon theme): Breeze's on L, Adwaita's on A (`adwaita-icon-theme` added; A
  had none).
- [ ] **L A** Boot splash: Plymouth is in neither lock; a theme also needs a
  hook in recinit. Wanted, with the mark (decision 7).

## 3. The session itself

### LevitateOS (Plasma on Wayland)

- [x] `upower` is not in the lock: no battery state, no low-battery action in
  PowerDevil.
- [x] `power-profiles-daemon` is in the lock and enabled: the power profile
  switch.
- [x] `udisks2` is in the lock: Dolphin and the device notifier mount USB
  drives. (`plasma-disks` for disk health is left out.)
- [x] `kwallet-pam` is in the lock; Fedora's SDDM PAM files already load
  `pam_kwallet5`, so the wallet unlocks at login.
- [ ] `plasma-systemmonitor`, `kscreen`, `powerdevil`, `plasma-nm`, `plasma-pa`
  are requested; check after the lock that `kde-gtk-config`'s daemon,
  `xdg-desktop-portal-kde` and `polkit-kde` start with the session.
- [ ] Decide what replaces `kde-settings`' environment scripts once it is
  dropped (ssh-agent start; none is required).
- [x] Baloo file indexing: on, full content (Plasma's default; decision 10).
- [x] KDE crash handler (`plasma-drkonqi`) ships; `khelpcenter` does not.
- [ ] Remove or hide menu entries that do nothing here (PackageKit is present
  only as a library; make sure no "Software" or "Updates" entry shows).

### AcornOS (LXQt on X11)

- [x] `udisks2` and `gvfs` are in the lock: the removable-media button and
  pcmanfm-qt mount drives, with trash. (`gvfs-smb` / `gvfs-nfs` for network
  browsing are left out; see section 4.)
  Alpine's `gvfs` links against `gnome-online-accounts`, which brings GTK 4,
  libadwaita and GStreamer's bad plug-ins (with `flite`): about 40 MB of the
  202 packages this added. Part of decision 2: keep it, or drop `gvfs` and
  lose trash, phones and cameras in pcmanfm-qt.
- [x] `setxkbmap` is not in the lock: LXQt's keyboard layout settings and the
  layout indicator need it.
- [x] `xrandr` is in the lock. Still to check: `libkscreen`'s X11 backend for
  LXQt's monitor settings.
- [x] Compositor: `picom`, as KWin composites on LevitateOS (same features,
  decision 2); its own autostart entry starts it with the session.
- [x] Screen lock: `xscreensaver` is in the lock and `lxqt-session`'s autostart
  entry starts it with the session; LXQt locks through it.
- [ ] xscreensaver's unlock dialog in the brown look instead of its default.
- [ ] Session autostart: confirm `lxqt-policykit`, `lxqt-powermanagement`,
  `lxqt-notificationd`, `nm-applet` and PipeWire start (their autostart files
  come from the packages; `nm-applet`'s must not be hidden in LXQt).
- [x] `xdg-desktop-portal` and `xdg-desktop-portal-lxqt` are in the lock.
- [x] `libnotify` is not in the lock (`notify-send` for scripts).
- [x] gnome-keyring unlocks at login: Alpine's SDDM PAM files already load
  `pam_gnome_keyring` (from `gnome-keyring-pam`, in the lock).
- [x] X11 input defaults: tap-to-click and natural scrolling for touchpads
  (`/etc/X11/xorg.conf.d/40-touchpad.conf`).
- [ ] Default panel layout (`/etc/xdg/lxqt/panel.conf`): decide the plug-ins
  (the desktop switcher and quick-launch defaults are upstream's).
- [x] Global shortcuts: `lxqt-globalkeys`' packaged defaults already have
  brightness, screenshot (Print), terminal (Ctrl+Alt+T) and lock
  (Ctrl+Alt+L); the panel's volume plug-in takes the volume keys.
- [x] OpenRC services for the desktop: `bluetooth`, `avahi-daemon`, `cupsd`,
  `firewalld` and `vm-guest-agents` enabled; udisks, UPower, bolt and fwupd
  are D-Bus activated.
- [ ] Wayland: Alpine 3.24 has no `lxqt-wayland-session`; revisit at the next
  Alpine release.

## 4. Hardware

- [x] **L** Bluetooth: `bluez` and `bluedevil`, `bluetooth.service` enabled.
- [x] **A** Bluetooth: `bluez` and `blueman`, the `bluetooth` service enabled.
- [x] **L** Printing: `cups`, `cups-filters`, `ipp-usb`, Avahi for finding
  driverless printers, `print-manager`; `cups` and `avahi-daemon` enabled.
- [x] **A** Printing: `cups`, `cups-filters`, Avahi, `system-config-printer`;
  `cupsd` and `avahi-daemon` enabled. (Alpine has no `ipp-usb`.)
- [x] **L A** Scanning: `sane-backends` with Skanlite (L), Simple Scan (A).
- [x] **L A** Video acceleration: `libva` is present but no driver. Add
  `mesa-va-drivers` and `intel-media-driver` (L), `mesa-va-gallium` and
  `intel-media-driver` (A).
- [x] **A** Vulkan: `vulkan-loader` with Mesa's Intel and AMD drivers.
- [x] **L A** NVIDIA's GPU firmware in both base locks, for nouveau.
- [ ] **L A** NVIDIA's proprietary driver as a Recipe recipe (decision 5).
- [x] **A** `fwupd` is in the lock (L had it); firmware updates for laptops
  and docks.
- [x] **L** Thunderbolt (`bolt`, `plasma-thunderbolt`).
- [x] **A** Thunderbolt (`bolt`, D-Bus activated; LXQt has no settings page,
  `boltctl` authorizes devices).
- [x] **L A** Phones and cameras: `libmtp` on L through `kio-extras`;
  `gvfs-mtp` and `gvfs-gphoto2` on A.
- [x] **L A** `exfatprogs` and `ntfs-3g` in both locks.
- [x] **L A** The kernel configs have exFAT (`=y`) and NTFS3 (`=m`).
- [x] **L A** Network shares: `gvfs-smb` and `gvfs-nfs` on A; L has SMB
  through `kio-extras`.
- [x] **L A** Mobile broadband: not supported (decision 3).
- [x] **L A** VPN: OpenVPN through `NetworkManager-openvpn` and
  `plasma-nm-openvpn` (L), `networkmanager-openvpn` (A); WireGuard is in
  NetworkManager itself.
- [x] **L A** Laptop basics: no hibernation (recinit cannot resume from it;
  `sleep.conf.d` for logind and elogind), lid close and suspend keep their
  defaults, brightness keys come with PowerDevil and LXQt's shortcuts.
- [ ] **L A** Battery charge thresholds (PowerDevil has a setting on L; none on
  A).
- [ ] **L A** Webcam: `libcamera` is on A only; check PipeWire camera support
  on both.
- [x] **L** Virtual machine guests: `spice-vdagent` and `qemu-guest-agent`,
  started by their udev rules when the VM has the port.
- [x] **A** Virtual machine guests: `spice-vdagent` and `qemu-guest-agent`,
  started by the `vm-guest-agents` service only when the VM has their port.
- [x] **L A** Kernel configs: i915, xe, amdgpu, nouveau, HID multitouch,
  Bluetooth, USB audio, UVC, exFAT, NTFS3 and CIFS/NFS are on (evdev from the
  defconfig).
- [ ] **L A** Kernel configs: the Wi-Fi drivers the firmware above is for.

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

- [x] **L** KCalc added; Gwenview, Okular and a media player are Recipe's.
- [x] **A** Mousepad (Alpine has no FeatherPad), Qalculate (the calculator,
  as KCalc on L), `qps`, `unzip` and `7zip` added; PDF viewer and media
  player are Recipe's, as on L.
- [x] **L A** Codecs: each image ships its distribution's (decision 4).
- [ ] **L A** H.264, AAC and the rest as Recipe recipes.
- [x] **L** Fonts: Noto Serif, Noto Sans CJK and Liberation added.
- [x] **A** Fonts: Liberation added (`font-noto` already has Serif).
- [x] **A** CJK fonts: `font-noto-cjk`.
- [x] **L A** Thumbnails: add `kdegraphics-thumbnailers` and
  `ffmpegthumbs` (L); pcmanfm-qt needs `ffmpegthumbnailer` (A).
- [x] **L A** Spell checking: `hunspell-en-US` on L, `hunspell-en` on A.
- [x] **L A** Default apps: `/usr/share/applications/mimeapps.list` (browser,
  text on L, images, PDF, archives, folders). Images open in Firefox on L
  until an image viewer ships; A has no text editor.
- [x] **L A** Home folders: `xdg-user-dirs` is in both locks and its own
  autostart entry runs `xdg-user-dirs-update` at login on both desktops.
- [x] **L A** Menu clean-up: every visible entry on L is a real app; on A,
  CUPS's web page, Hibernate and the bare XScreenSaver daemon are hidden
  (`NoDisplay`). LXQt's Users and Groups stays.
- [x] **L A** Manual pages: `man-db` (L), `mandoc` and `man-pages` (A).

## 6. Software through Recipe

The desktop adds no other package manager, store or bundle format.

- [ ] **L A** Recipes for the apps people expect first (office suite, mail,
  chat, media, graphics, development tools); first set: OnlyOffice (not LibreOffice), mail, media and
  graphics, chat, developer tools (decision 13).
- [ ] **L A** Desktop integration of what Recipe installs: `.desktop` entries,
  icons, MIME types and D-Bus services must land where the menus look, and the
  caches (`update-desktop-database`, icon cache, MIME database) must be
  refreshed after an install or removal.
- [ ] **L A** Per-user or system-wide installs from the desktop: how a desktop
  user is asked for the administrator password (polkit action for Recipe).
- [ ] **L A** Update notice: something in the session that says a Recipe
  update is available.
- [ ] **L A** A graphical front-end for Recipe beside the command line
  (decision 7); plan it like the recab GUI.
- [ ] **L A** Proprietary drivers and codecs as recipes (NVIDIA, H.264).
- [ ] **L A** State what Recipe lacks for each point above and extend Recipe.

## 7. Installing from the desktop ISO

- [x] **L A** "Install LevitateOS / AcornOS" in the live desktop's menu
  (`<id>-install.desktop`, live overlay only): Konsole or QTerminal runs the
  install screen through sudo or doas.
- [ ] **L A** The launcher on the desktop itself, and its own icon
  `<id>-install` (mark plus an arrow into a disk, as SVG; the theme's
  installer icon stands in).
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
  software with Recipe, how updates work); wanted (decision 7).
- [x] **L A** The live session must not sleep or lock during an install
  (`stay_awake` in `live-overlay.rhai`; lid close still follows logind).
- [ ] **L A** Live user's password `live`: the Install entry's description
  shows it; also put it in the welcome or on the lock screen hint.

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

- [x] **L** Languages: the 13 glibc language packs in the lock (decision 8).
- [ ] **A** `musl-locales` is present; LXQt's translations come with the
  packages. Check the locale list recsetup offers.
- [x] **L A** Input methods: none (decision 8).
- [ ] **L A** Time zone: recsetup sets it; check the panel clocks follow, and
  decide on automatic time zone (geoclue is on L only).
- [x] **L A** Time sync: chronyd is enabled on every system
  (`capability_services`).
- [ ] **L A** Clock format, first day of the week and paper size follow the
  locale.

## 10. Security and privacy

- [x] **L A** Firewall: firewalld enabled on both desktops; the public zone
  refuses incoming connections but ssh, DHCPv6 and mDNS (for printers).
  Settings: Plasma's firewall page (L), `firewall-config` (A).
- [ ] **L** Screen locker (`kscreenlocker`) is present; check it works with
  the PAM stack the recipe writes (`system-auth`), including after suspend.
- [ ] **A** Screen lock on suspend and on lid close (xscreensaver with
  `xss-lock` or LXQt's own setting).
- [x] **L A** sshd is off on installed systems (it already was: only the live
  ISO enables it).
- [x] **L A** Polkit: mounting removable drives, connecting to networks and
  suspending are the packages' defaults for a user at the machine; a rule
  adds setting the time zone and saving a network for every user.
- [ ] **L A** Encrypted installs: unlock prompt at boot is console text
  without Plymouth.
- [x] **L A** Firefox: `distribution/policies.json` turns off telemetry, studies,
  first-run and what's-new pages and the built-in updater.

## 11. Accessibility

- [x] **L** Screen reader: Orca.
- [x] **A** Screen reader: Orca.
- [x] **L A** Orca's speech: `speech-dispatcher` and `espeak-ng` (named, since
  the lock takes no weak dependencies).
- [x] **L A** On-screen keyboard: `plasma-keyboard` (L), Onboard (A).
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

## Decisions (the user, 2026-11-10)

The items above that needed a decision follow these; the marks are updated as each
is carried out.

1. **Look:** LevitateOS starts dark, AcornOS starts light.
2. **Same features on both desktops:** AcornOS's desktop does everything
   LevitateOS's does, on its own stack (musl, OpenRC, LXQt, X11); only the
   parts differ. So AcornOS also gets Bluetooth, printing, scanning, Orca,
   CJK fonts, an on-screen keyboard, a compositor, Thunderbolt, the VM guest
   agents, NVIDIA's firmware and a calculator. `gvfs` stays, with the GTK 4
   and GStreamer it brings. "Small and strict" describes AcornOS's base
   system and its choice of parts, not fewer desktop features.
3. **Apps in the image:** LevitateOS adds KCalc (Gwenview, Okular and a media
   player are Recipe's). AcornOS adds `unzip` and `7zip`, a text editor
   (Mousepad: GTK 3 is already in for Firefox; KWrite would bring KDE
   Frameworks), Qalculate and `qps`. KDE's crash handler (`drkonqi`) ships on
   LevitateOS; `khelpcenter` does not. Scanning ships on LevitateOS
   (`sane-backends`, Skanlite); not on AcornOS (decision 2). No mobile
   broadband.
4. **Codecs:** each image ships what its distribution ships; H.264, AAC and
   the rest come as Recipe recipes.
5. **NVIDIA:** supported as a Recipe recipe (the proprietary module built per
   kernel); nouveau stays the default.
6. **Firewall:** firewalld on both desktops, incoming connections blocked
   except what a zone allows; Plasma's firewall settings page on LevitateOS.
   sshd is off on installed desktops (still on for the live ISO).
7. **Extras:** a graphical Recipe front-end (planned like the recab GUI), a
   first-login welcome, and a Plymouth boot splash with the mark (needs the
   recinit hook; also gives encrypted installs a graphical unlock prompt).
8. **Languages:** the 13 glibc language packs LevitateOS has; no input
   methods.
9. **AcornOS style:** Fusion widgets with an Adwaita GTK look, and the own
   Maldives-based login theme with the acorn added.
10. **Baloo:** on, full content (Plasma's default).
11. **Firefox homepage:** `https://levitateos.org` on LevitateOS, Firefox's
    own on AcornOS.
12. **Symbolic launcher icon:** drawn, for the user to review.
13. **First recipes:** office (OnlyOffice, not LibreOffice), mail, media and
    graphics, chat, developer tools. These are what a person can install,
    and what the Recipe front-end lists.

Still open:

- Whether the LevitateOS light and dark wallpapers are right, or the
  original half-and-half picture should be one of them.
- XLibre in place of the X.Org server on AcornOS (the user's idea). Alpine
  has no XLibre package in 3.24 or edge, so it would be a Recipe-built X
  server, with `xf86-input-libinput` and any video driver rebuilt against
  it. LevitateOS is unaffected (Wayland; Xwayland is a separate program).
