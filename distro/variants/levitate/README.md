# LevitateOS variant

The daily-driver workstation: glibc and systemd, so software built for mainstream Linux runs.

- `distro.toml`: the whole variant contract (identity, profile with its kernel line and the mode its desktop starts in (`appearance`), its colours in light and dark mode (`[look.light]`, `[look.dark]`, which the desktop system's look is made from), sources, and what its live and installed systems add to the shared lists). Its shape is `distro_contract::ConformanceContract`; `distro_contract::load` reads and validates it.
- `kconfig`: the kernel options of its kernel line only, applied after
  `../_shared/kconfig` (the kernel policy every variant shares).
- `branding/`: `wallpaper-light.jpg` and `wallpaper-dark.jpg` (3840×2160), the desktop system's background under a light and a dark look, and `logo-*.svg`, the mark (24px and 16px micro, ink and paper) for its start button ([docs/desktop-theming.md](../../../docs/desktop-theming.md)).
- `keys/`: the Fedora RPM signing key `base.signing_key` names.
