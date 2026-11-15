# LevitateOS variant

LevitateOS is the daily-driver workstation. Its userland is built from Fedora 45 RPM packages: glibc and systemd, Plasma as the desktop, the latest kernel line. Software built for mainstream Linux runs on it.

- `distro.toml`: the variant contract. It holds the identity, the profile (init, libc, admin tool, desktop, kernel line, and the mode the desktop starts in), the colours in light and dark mode (`[look.light]`, `[look.dark]`), the package sources, and what the live and installed systems add to the shared lists. Its shape is `distro_contract::ConformanceContract`; `distro_contract::load` reads and validates it.
- `kconfig`: the kernel options of this variant, applied after `../_shared/kconfig` (the kernel policy both variants share).
- `branding/`: `wallpaper-light.jpg` and `wallpaper-dark.jpg` (3840x2160), the desktop background in each mode, and `logo-*.svg`, the mark for the start button (24px and 16px, ink and paper). See [docs/desktop-theming.md](../../../docs/desktop-theming.md).
- `keys/`: the Fedora RPM signing key that `base.signing_key` names.
