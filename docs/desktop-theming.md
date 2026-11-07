# Desktop theming: what to brand, and how

Companion to [desktop.md](desktop.md). This document traces the branding in the
2026-11-07 prototype trees (Fedora 45 Plasma, Alpine 3.24 LXQt) to the package
and file that sets it, then lists what LevitateOS and AcornOS ship in its place.

## Starting point

- **Marks** exist and are final: `docs/website/public/brand/` (hovering leaf,
  sculpted acorn; standard 24px, micro 16px, lockup; `-ink` for light surfaces,
  `-paper` for dark). The identity kit (`docs/website/design/field-notes-v3/`)
  says to use them as supplied: never generate a new mark, and never add
  texture, glow or a wordmark to a small launcher icon.
- **Colours** come from `docs/website/src/styles/field-notes/tokens.json`:
  paper `#F2EEE4`, ink `#14221A`, pine `#214438` (Levitate), acorn brown
  `#644532`, red `#B22F27` (one primary action), night surface `#14221A`, night
  Levitate `#C7D9BA`, night Acorn `#E4CBA6`.
- **Art** that has to be made: wallpapers only (see
  [desktop-art-brief.md](desktop-art-brief.md)). Everything else is
  configuration, QML/QSS edited from upstream themes, or the existing SVG marks.
- **App icons** stay Breeze, on both desktops. A full icon theme is thousands of
  icons and would add nothing. The new icons are only the distribution's own,
  all drawn from the existing marks as SVG (see "Icons" below).

## LevitateOS (Plasma 6)

### Fedora branding to remove

The desktop closure pulls in Fedora's art and look. Recipes unpack the locked
RPMs without a solver, so the lock simply leaves these packages out and our
branding ships files in their place:

| Package | Pulled in by | What it sets |
|---|---|---|
| `fedora-logos` | provides `system-logos` (needed by `breeze-icon-theme`, `sddm`, `kde-settings-plasma`) | `start-here` and `fedora-logo-icon` in hicolor, `/usr/share/pixmaps/*` (About page logo), Plymouth art |
| `fedora-release`, `-common`, `-identity-basic` | the desktop request | a second os-release in `/usr/lib/os-release` (`LOGO=fedora-logo-icon`) |
| `plasma-lookandfeel-fedora` | `plasma-workspace` (hard requirement, Fedora patch) | the default Global Theme `org.fedoraproject.fedora.desktop` |
| `kde-settings`, `kde-settings-plasma` | `plasma-workspace` | `LookAndFeelPackage=org.fedoraproject.fedora.desktop`, wired in via `XDG_CONFIG_DIRS` from `/etc/profile.d/kde.sh`; `kcm-about-distrorc`; start-here launcher script |
| `desktop-backgrounds-kde`, `-compat`, `f45-backgrounds-*` | `sddm`, `sddm-breeze` | `/usr/share/wallpapers/{Default,Fedora}` → F45, `/usr/share/backgrounds/default.jxl` |
| (sddm-breeze file) `usr/share/sddm/themes/01-breeze-fedora` | `sddm-breeze` | login screen with Fedora logo and F45 background |
| (firefox file) `firefox-redhat-default-prefs.js` | `firefox` | homepage and pinned tab `start.fedoraproject.org` |

Fedora offers `generic-logos` and `generic-release` for remixes (both in the F45
repository). The lock request names `generic-logos` so `system-logos` resolves
without Fedora's art. The Fedora packages above, which only hold art, are left
out of the lock. Lock generation therefore needs an exclude list beside the
request: `cargo xtask base lock levitate` passes it to dnf (`--exclude`), and
the recipe drops the two Fedora files that live inside packages we keep
(`01-breeze-fedora`, the Firefox prefs file).

`kde-settings` also carries non-branding environment scripts (ssh-agent,
GTK scrolling). None of them is required. Leave it out.

### What LevitateOS ships

All in one recipe-built tree (a `levitate-branding` step of the desktop rootfs
recipe), owned by root, mode 0644:

| Item | Path | Content |
|---|---|---|
| Global Theme | `/usr/share/plasma/look-and-feel/org.levitateos.desktop/` | `metadata.json`; `contents/defaults` (colour scheme, icons `breeze`, cursor `breeze_cursors`, Plasma style `default`, window decoration Breeze, `[Wallpaper] Image=LevitateOS`, `[ksplashrc] Theme=org.levitateos.desktop`); `contents/layouts/org.kde.plasma.desktop-layout.js` (bottom panel, Kickoff with `icon=distributor-logo-levitateos`); `contents/splash/Splash.qml` (Breeze splash with our mark); `contents/previews/*` |
| Default theme | `/etc/xdg/kdeglobals` | `[KDE] LookAndFeelPackage=org.levitateos.desktop`, `ColorScheme=LevitateOSDark`, Noto fonts |
| Colour schemes | `/usr/share/color-schemes/LevitateOSLight.colors`, `LevitateOSDark.colors` | Breeze Light/Dark with the accent set to pine `#214438` / `#C7D9BA`; selection and focus follow the accent |
| Wallpaper | `/usr/share/wallpapers/LevitateOS/` | `metadata.json` (`Id: LevitateOS`, licence), `contents/images/<W>x<H>.jpg` and `contents/images_dark/<W>x<H>.jpg`; Plasma picks the closest size and the dark set under a dark colour scheme. Also link `/usr/share/wallpapers/Default` → `LevitateOS` |
| Login screen | `/usr/share/sddm/themes/levitateos/` + `/etc/sddm.conf.d/10-theme.conf` (`[Theme] Current=levitateos`) | copy of `sddm-breeze`'s QML with `theme.conf`: `showlogo=shown`, `logo=` the paper lockup, `background=` the dark wallpaper, `color=#14221A` |
| Lock screen | — | follows the wallpaper; nothing extra |
| Icons | `/usr/share/icons/hicolor/scalable/apps/distributor-logo-levitateos.svg`, `levitateos-logo-icon.svg`, `…/16x16/apps/…` (micro mark) | see "Icons" |
| About page | `/etc/xdg/kcm-about-distrorc` | `LogoPath=/usr/share/pixmaps/levitateos-logo.svg`, `Website=https://levitateos.org`, `Variant=Desktop` |
| os-release | builder (`distro/builder/src/pipeline/plan.rs`) | add `LOGO=levitateos-logo-icon` (and `HOME_URL`); KInfoCenter and some apps read it |
| Firefox | `/usr/lib64/firefox/browser/defaults/preferences/levitateos-prefs.js` | homepage `https://levitateos.org` (or blank); remove the Fedora prefs file |

### Setting the default for new users

Plasma applies the Global Theme's `defaults` and layout only to a fresh
profile. The live user is created in the image with no `~/.config`, so the
first login takes the defaults. Do not write per-user `kdeglobals`/`plasmarc`
into `/etc/skel`: those freeze the settings and stop the Global Theme from
working in System Settings.

## AcornOS (LXQt)

Alpine adds no distribution branding. LXQt's own defaults are the "Clearlooks"
LXQt theme, the Fusion widget style, the `waves-logo` wallpaper and the
Openbox "Clearlooks" window decoration. Alpine 3.24 also has `breeze`
(Qt widget style), `breeze-gtk`, `kvantum`, `sddm-breeze` and `lxqt-themes`.

| Item | Path | Content |
|---|---|---|
| LXQt theme | `/usr/share/lxqt/themes/acornos/` | panel, runner, notification and leave-dialog QSS edited from `light`/`dark` to the Acorn colours; `mainmenu.svg` = acorn mark (this is the menu button icon); `wallpaper.cfg`; `preview.png` |
| Defaults | `/etc/xdg/lxqt/lxqt.conf` | `theme=acornos`, `icon_theme=breeze`, `[Qt] style=Breeze`, font Noto Sans |
| Palette | `/usr/share/lxqt/palettes/AcornOS` | Qt palette in Acorn browns and paper (LXQt applies it with `palette_override`) |
| Wallpaper | `/usr/share/wallpapers/AcornOS/…` + `/etc/xdg/pcmanfm-qt/lxqt/settings.conf` | `Wallpaper=` the Acorn wallpaper, `WallpaperMode=zoom` |
| Window decoration | `/usr/share/themes/AcornOS/openbox-3/themerc` + `/etc/xdg/openbox/lxqt-rc.xml` (`<theme><name>AcornOS</name>`) | Openbox themes are plain text (colours, fonts, button bitmaps); start from `Clearlooks` |
| GTK apps (Firefox) | `/etc/gtk-3.0/settings.ini` | `gtk-theme-name=Breeze`, `gtk-icon-theme-name=breeze` |
| Login screen | `/usr/share/sddm/themes/acornos/` + `/etc/sddm.conf.d/10-theme.conf` | from `sddm-breeze` (add it to the Acorn desktop set), with the Acorn lockup and wallpaper |
| Icons | hicolor `distributor-logo-acornos`, `acornos-logo-icon` | see "Icons" |
| os-release | builder | `LOGO=acornos-logo-icon` |

LXQt has no "global theme" package: the defaults live in `/etc/xdg`, which is
read for every user without a `~/.config/lxqt` of its own.

## Icons

Plasma, LXQt and GTK find icons by name through the icon theme (Breeze, which
falls back to hicolor). The distribution needs only:

| Name | Used by | Source |
|---|---|---|
| `distributor-logo-<distro>` | launcher button (Kickoff, LXQt menu), About pages | standard mark; micro mark at 16/22px |
| `<distro>-logo-icon` | os-release `LOGO=` | same as above |
| `<distro>-install` | the "Install <Distro>" launcher on the live desktop (opens a terminal running the installer) | mark plus a down-arrow-into-disk glyph, drawn as SVG |

Qt renders SVG icons itself (`qt6-qtsvg` is in both sets), so a scalable SVG in
`hicolor/scalable/apps/` plus the micro SVG in `hicolor/16x16/apps/` is enough.
Neither desktop needs PNG icons. The ink marks are the right colour on a light
panel. On a dark panel use the paper version. The kit says these fixed-colour
files are not symbolic icons, so check them at native size on the real panel.

The marks are a fixed colour, but Plasma colours "symbolic" icons from the
colour scheme. If a launcher icon that follows the panel's light/dark scheme is
wanted, add `distributor-logo-<distro>-symbolic.svg` using
`fill="currentColor"` with Plasma's `ColorScheme-Text` class. That would be a
new variant of the mark, so it needs the identity owner's sign-off.

## Raster work

The only images that are not SVG are the wallpapers (and theme previews, which
are screenshots). The host has no SVG rasteriser (`rsvg-convert`, ImageMagick,
Inkscape and Pillow are all missing). None is needed: icons ship as SVG; the
splash and login screens draw the SVG lockup in QML; and the wallpaper sizes
come from the generated master. Resize it at build time with the target's own
tools, or commit the few sizes Plasma needs (3840×2160, 2560×1440,
1920×1080, 1920×1200, 1366×768, 1080×1920) as JPEG at quality 90. Committed
sizes are simpler and make the build reproducible.

Where the sources live: `distro/variants/<distro>/branding/` (SVGs, wallpaper
masters, QML, QSS, colour schemes), copied in by the desktop rootfs recipe.
The website keeps its own copies of the marks. The brand files are small, so
copying is simpler than sharing a path across repositories.

## Not branded (yet)

- **Boot**: the UKI boots through systemd-boot and recinit with no splash. A
  Plymouth theme (both distributions have `plymouth`) would hide the console
  during boot but needs an initramfs hook in recinit. Leave it for later.
- **systemd-boot menu**: text only. `loader.conf` takes no images.
- **Default avatar**: SDDM and Plasma use Breeze's default face; fine as is.
