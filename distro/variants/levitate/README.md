# LevitateOS variant

- `distro.toml`: the whole variant contract (identity, kernel pin, sources, products, transforms, scenarios, release). Its shape is `distro_contract::ConformanceContract`; `distro_contract::load` reads and validates it.
- `kconfig`: the kernel configuration.
- `keys/`: the Fedora RPM signing key `rpm_base.signing_key` names.
- `hooks/`: the release hooks the builder runs for each release ISO (they call `../_shared/hooks/build-release.sh`).
