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
- **Colours** are declared in each variant's `distro.toml`: ink `#14221A`,
  pine `#214438` (Levitate), acorn brown `#644532`, night surface `#14221A`,
  night Levitate `#C7D9BA`, night Acorn `#E4CBA6`. The light surfaces are sage
  paper `#DDE2D2` on LevitateOS and parchment `#E6DFCF` on AcornOS (2026-11-13:
  the earlier near-white paper `#F2EEE4` was rejected as too white; it remains
  only as the colour of text on dark bands and on the red action). The site
  keeps its own red `#B22F27` for the one primary action. Each `distro.toml`
  carries the colours as two palettes, `[look.light]` and `[look.dark]` (surface,
  raised and sunken surface, text in three strengths, rule, hairline, accent,
  highlight, band and its text, ok, warn, bad).
- **Mode** (decision 1 in [desktop-todo.md](desktop-todo.md), 2026-11-13):
  both distros start dark, LevitateOS in its green and AcornOS in its brown
  (the light LevitateOS start of earlier that day was withdrawn the same
  day; a light palette stays for those who switch) (`profile.appearance`, which
  reaches the image as `REC_APPEARANCE` in os-release). The builder gives the
  desktop system recipe both palettes as its `look` input (one
  `<mode>.<role>=#RRGGBB` line each, part of the configured tree's key), and
  `installed-system.rhai` keys every look choice on the mode, never on the
  desktop. All of it is written and unbuilt: no image with it has been built
  or booted.
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
| Default theme (done, unbuilt) | `/etc/xdg/kdeglobals` | today: `[KDE] LookAndFeelPackage=org.fedoraproject.fedoralight.desktop` (`fedoradark` when the mode is dark), `[General] ColorScheme=LevitateOSLight`, `accentColorFromWallpaper=false`, `[Icons] Theme=breeze` (`breeze-dark` on dark), and the scheme's colour groups. Fedora's `fedora`, `fedoralight` and `fedoradark` Global Themes name `LevitateOSLight`/`LevitateOSDark` in their `contents/defaults` in place of Breeze's, because Plasma copies the Global Theme's defaults into each user's `~/.config/kdedefaults`, which is read ahead of `/etc/xdg`. Later: `org.levitateos.desktop`, Noto fonts |
| Colour schemes (done, unbuilt) | `/usr/share/color-schemes/LevitateOSLight.colors`, `LevitateOSDark.colors` | generated from the two palettes, with every group of Breeze's schemes: window = surface, view and button and tooltip = raised surface, alternate rows = sunken surface, text = text, inactive text = muted text, selection = accent with surface text, focus, hover and links = accent, visited = highlight, positive/neutral/negative = ok/warn/bad, header, complementary and active title bar = band with on_band text (pine `#214438` on both modes), inactive header and title bar = sunken surface; effects as Breeze's. The scheme carries the brand accent itself; no `AccentColor` is set |
| Konsole (done, unbuilt) | `/usr/share/konsole/LevitateOSLight.colorscheme`, `LevitateOSDark.colorscheme`, `LevitateOS.profile`, `/etc/xdg/konsolerc` (`[Desktop Entry] DefaultProfile=LevitateOS.profile`) | surface background, text foreground; ANSI black = text on light (on dark, the muted text dimmed towards the surface), bright black = muted text, white = secondary text, bright white = text, red/green/yellow = bad/ok/warn, and blue, magenta, cyan fixed per mode for both distros (`terminal_hues`: light `#2A5C8A`, `#7A3F6E`, `#1F6F6B`; dark `#9BB8D9`, `#D4A5C9`, `#9ACFC4`), since the palette's accent and highlight are too close to green and yellow; bright leans to the text, faint to the surface. Every normal and bright colour is at least 3:1 on the background in all four distro and mode combinations (lowest: bright black on AcornOS light, 3.45:1). The profile uses the dark scheme on both distros, whatever mode the desktop starts in: a terminal is dark (owner, 2026-11-13). The light scheme is installed for those who pick it |
| Wallpaper (done, unbuilt) | `/usr/share/wallpapers/LevitateOS/` | `metadata.json` (`Id: LevitateOS`), `contents/images/3840x2160.jpg` and `contents/images_dark/3840x2160.jpg`; Plasma scales the one size and shows the dark one under a dark colour scheme, the light one under a light one, so LevitateOS starts on the light one. `/usr/share/wallpapers/Default` links to `LevitateOS`, so Fedora's Global Theme (`Image=Fedora` → `Default`) already shows it; the Fedora background packages are still in the lock |
| Login screen (background done, unbuilt) | `/etc/sddm.conf.d/10-theme.conf` (`[Theme] Current=breeze`) + `/usr/share/sddm/themes/breeze/theme.conf.user` | KDE's Breeze theme (no logo) in place of `01-breeze-fedora`, with `background=` the starting mode's wallpaper (light on LevitateOS) and `color=` that mode's surface; SDDM reads `theme.conf.user` over `theme.conf`. The greeter draws in the scheme of `/etc/xdg/kdeglobals` (its complementary group, the band). Still to do: an own `levitateos` theme with `showlogo=shown` and `logo=` the lockup in the tone of the mode |
| Lock screen | — | follows the wallpaper; nothing extra |
| Icons | `/usr/share/icons/hicolor/scalable/apps/distributor-logo-levitateos.svg`, `levitateos-logo-icon.svg`, `…/16x16/apps/…` (micro mark) | see "Icons" |
| About page (done, unbuilt) | `/etc/xdg/kcm-about-distrorc` | `LogoPath=` the start button's icon (`distributor-logo-levitateos.svg`, ink on light, paper on dark), `Website=https://levitateos.org`, `Variant=Desktop` |
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
| LXQt themes | `/usr/share/lxqt/themes/acornos-light/`, `acornos-dark/` | panel, runner, notification and leave-dialog QSS edited from LXQt's `light`/`dark` to the Acorn colours; `mainmenu.svg` = acorn mark (this is the menu button icon); `wallpaper.cfg` naming that theme's light or dark wallpaper, so switching the LXQt theme switches the wallpaper too; `preview.png` |
| Defaults (done, unbuilt) | `/etc/xdg/lxqt/lxqt.conf` | the packaged `/usr/share/lxqt/lxqt.conf` with `theme=dark` (LXQt's packaged `light`/`dark` theme, named after the mode), `icon_theme=breeze-dark` (`breeze` on light), `[Qt] style=Fusion`, and `[Palette]` from the mode's colours: `window_color` surface, `window_text_color` and `text_color` text, `base_color` and `tooltip_base_color` raised surface, `highlight_color` and `link_color` accent, `highlighted_text_color` surface, `link_visited_color` highlight, `tooltip_text_color` text (the keys LXQt's Qt platform plugin reads). Later: `theme=acornos-dark`, font Noto Sans |
| Palette | `/usr/share/lxqt/palettes/AcornOS` | still to do: the same palette as a file the appearance settings can offer |
| Wallpaper (done, unbuilt) | `/usr/share/wallpapers/AcornOS/` (same layout as Plasma's) + `/etc/xdg/pcmanfm-qt/lxqt/settings.conf` | the packaged `/usr/share/pcmanfm-qt/lxqt/settings.conf` with `Wallpaper=` the starting mode's wallpaper (dark on AcornOS), `WallpaperMode=zoom`, desktop icon text `FgColor=` the mode's text with `ShadowColor=` and `BgColor=` its surface, and `IconThemeName=` the mode's Breeze. pcmanfm-qt reads one whole settings file, the first on the configuration path, so the file is a full copy. LXQt does not switch wallpaper with the look; the other one waits for our themes' `wallpaper.cfg` |
| Window decoration (packaged theme, unbuilt) | `/etc/xdg/openbox/rc.xml` | Openbox reads it when the user has none (nothing in this LXQt passes `lxqt-rc.xml`). `<theme><name>` is `Onyx` on dark (dark active title bars and menus; its inactive title bars are light) and `Clearlooks` on light, the packaged themes nearest the mode. Still to do: an `AcornOS` theme (`/usr/share/themes/AcornOS/openbox-3/themerc`) from the palette |
| GTK apps (Firefox) (done, unbuilt) | `/etc/gtk-3.0/settings.ini`, `/etc/gtk-4.0/settings.ini` | `gtk-theme-name=Adwaita`, `gtk-application-prefer-dark-theme=true` on dark (`false` on light), `gtk-icon-theme-name=` the mode's Breeze, Adwaita cursor, Sans 11 |
| QTerminal (done, unbuilt) | `/usr/share/qtermwidget6/color-schemes/AcornOSLight.colorscheme`, `AcornOSDark.colorscheme`, `/etc/xdg/qterminal.org/qterminal.ini` (`colorScheme=AcornOSDark`) | the same terminal scheme as Konsole's on LevitateOS, from the AcornOS palettes |
| Login screen (background done, unbuilt) | `/usr/share/sddm/themes/acornos/` + `/etc/sddm.conf.d/10-theme.conf` (`Current=acornos`) | SDDM's own Maldives theme (`Main.qml` and its two images, copied from the package) with `background=` the starting mode's wallpaper (dark on AcornOS) and `QtVersion=6` in `metadata.desktop`: Alpine has only `sddm-greeter-qt6`, and its Maldives does not name a Qt version. Its `theme.conf` has no colour settings (the QML's own white, black and red stay). Still to do: the Acorn lockup, and `sddm-breeze` if the look should match Plasma's |
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

Done (unbuilt): `start_button()` in `installed-system.rhai` installs
`distributor-logo-<os id>` (the standard mark in `hicolor/scalable/apps/`, the
micro mark in `hicolor/16x16/apps/`) from `branding/logo-*.svg`, and puts it on
the start button. The panel follows the starting mode, so the mark is ink in
light mode (LevitateOS) and paper in dark mode (AcornOS), on either desktop.
On Plasma, Fedora's look-and-feel scripts that set a new launcher's icon to
`start-here` set ours instead; on LXQt, `ownIcon` and `icon` in the
`[fancymenu]` section of `/etc/xdg/lxqt/panel.conf`. A user who switches mode
keeps the starting mode's mark until the symbolic icon exists. `<distro>-logo-icon`, `LOGO=` and the install launcher are not done.

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

The wallpapers are there now: `branding/wallpaper-light.jpg` and
`branding/wallpaper-dark.jpg`, 3840×2160 JPEG, the only size shipped. The
contract requires both files (`distro_contract::WALLPAPERS`), the builder
passes them to the rootfs recipe as `wallpaper_light` and `wallpaper_dark`, and
`wallpaper()` in `installed-system.rhai` installs them. How they were made:
the user picked one generated picture per distribution (kept, with a contact
sheet, in `.artifacts/desktop-art/picked/`); Qwen Image 2.1 edit made the
light and dark versions (AcornOS's dark one is the pick itself); Ultimate SD
Upscale with Z-Image-Turbo and 4x-UltraSharp (2×, denoise 0.12, no content in
the prompt) upscaled each; ImageMagick put the source's colours back and cut
them to size.
The website keeps its own copies of the marks. The brand files are small, so
copying is simpler than sharing a path across repositories.

## Not branded (yet)

- **Boot**: the UKI boots through systemd-boot and recinit with no splash. A
  Plymouth theme (both distributions have `plymouth`) would hide the console
  during boot but needs an initramfs hook in recinit. Leave it for later.
- **systemd-boot menu**: text only. `loader.conf` takes no images.
- **Default avatar**: SDDM and Plasma use Breeze's default face; fine as is.
