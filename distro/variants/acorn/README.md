# AcornOS variant

- `distro.toml`: the whole variant contract (identity, profile with its kernel line, sources, and what its live and installed systems add to the shared lists). Its shape is `distro_contract::ConformanceContract`; `distro_contract::load` reads and validates it.
- `kconfig`: the kernel options of its kernel line only, applied after
  `../_shared/kconfig` (the kernel policy every variant shares).
- `hooks/`: the release hooks the builder runs for each release ISO (they call `../_shared/hooks/build-release.sh`).
