# Where each fact lives

- **A distro's declarations:** `distro/variants/<id>/distro.toml`, beside its `kconfig` and `keys/`. `distro_contract::load` reads and validates it; the shape is the `ConformanceContract` type, and `[profile]` holds the facts the distros differ in (also written into the image's os-release as `REC_*`, read through the `distro-profile` crate). Artifact names follow from the distro's `os_id`.
- **Products and scenarios:** one registry, `distro_contract::registry`, the same for every distro. What a scenario expects follows from the scenario, the init system and the identity (`install_tests::expect`).
- **Commands:** `cargo xtask --help`; `xtask/README.md` says what xtask owns.
- **Building products:** `distro/builder` (its README maps the crate).
- **Running scenarios:** `testing/install-tests` (its README maps the scenarios, state and VM launcher).
- **Packages:** recipes own package and source facts and acquire/build/install behaviour. The base-package lock is written by `cargo xtask base lock`; the variant declares what to lock.
- **Output locations:** under `.artifacts`, chosen by the builder's `recipe/outputs.rs` and the install-tests state module, not documented here.

Rules that hold everywhere: recipes own package facts, Rust/TOML owns orchestration; scenarios consume existing artifacts and do not build them; product dependencies form a build graph, not an ordered test ladder; destructive disk operations keep their protections; a cached pass or a successful build is not release evidence for A/B behaviour (update and rollback need their scenario). The admin tool differs by design (sudo or doas), so parity checks exercise the distro's declared tool.
