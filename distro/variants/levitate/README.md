# LevitateOS variant

The daily-driver workstation: glibc and systemd, so software built for mainstream Linux runs.

- `distro.toml`: the whole variant contract (identity, profile with its kernel line, sources, and what its live and installed systems add to the shared lists). Its shape is `distro_contract::ConformanceContract`; `distro_contract::load` reads and validates it.
- `kconfig`: the kernel options of its kernel line only, applied after
  `../_shared/kconfig` (the kernel policy every variant shares).
- `branding/`: `wallpaper-light.jpg` and `wallpaper-dark.jpg` (3840×2160), the desktop system's background under a light and a dark look ([docs/desktop-theming.md](../../../docs/desktop-theming.md)).
- `keys/`: the Fedora RPM signing key `base.signing_key` names.
