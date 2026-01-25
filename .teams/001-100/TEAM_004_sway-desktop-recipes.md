# TEAM 004: Sway Desktop Environment Recipes

## Objective
Create recipes to build a complete Sway-based Wayland desktop environment from source.

## Status: COMPLETE (recipes written, SHA256s need filling)

## Components Needed

### Core Stack (must-have)
- wayland, wayland-protocols
- wlroots
- sway, swaybg, swaylock, swayidle
- libinput, libxkbcommon
- mesa (or minimal GL)
- seatd

### Desktop Apps (essential)
- foot (terminal)
- waybar (bar)
- wofi/fuzzel (launcher)
- mako (notifications)

### Supporting Libraries
- Many... see sway-desktop.md

## Decisions Made
- Using seatd over elogind (simpler, no systemd dependency)
- Using foot over alacritty (smaller, fewer deps)
- Using wofi over rofi (native wayland)

## Progress

### Recipes Created
- [x] Wayland core (wayland, wayland-protocols, libxkbcommon, libinput)
- [x] Session (seatd)
- [x] wlroots
- [x] Sway ecosystem (sway, swaybg, swaylock, swayidle)
- [x] Desktop apps (foot, waybar, wofi, mako)
- [x] Utilities (grim, slurp, wl-clipboard)
- [x] Support libs (gtk-layer-shell)

### Total: 17 recipes

## Notes
- Build order matters due to dependencies
- Some packages may need patches for musl if targeting musl
- Mesa is the big one - complex build
