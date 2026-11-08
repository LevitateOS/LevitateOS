# Builds and scenarios

LevitateOS and AcornOS each declare themselves in one file, `distro/variants/<distro>/distro.toml`, beside the kernel configuration (`kconfig`) and signing keys (`keys/`). `distro_contract::load` reads and validates it; the file's shape is the `ConformanceContract` type.

| Section | Responsibility |
|---|---|
| `identity` | OS name, id and version (the ISO label, hostname, login prompt and kernel localversion are derived from the ids) |
| `profile` | The base facts the variants differ in: package backend, init, libc, privilege tool, desktop, kernel line (written into the image's os-release as `REC_*`) |
| `build.kernel` | Kernel version and checksum |
| `sources` | Recipe selection and source acquisition |
| `install_ux` | The live ISO's install screen |
| `live`, `installed` | What the variant's live and installed systems add to the shared lists: services, kernel command line, tools, the installed initramfs |

The products (`base-rootfs`, `live-boot`, `live-tools`, `installed-boot`) and the scenarios are one registry, `distro_contract::registry`, the same for every variant; artifact names follow from `os_id` (`<os_id>-x86_64-<product>.iso`, `<os_id>-live.efi`, `filesystem.erofs`, ...), and what each scenario expects follows from the scenario, the init system and the identity (`install_tests::expect`).

Recipes own package/source facts and acquire/build/install behavior. Orchestration owns product dependencies, output locations and execution. Product dependencies form a build graph; they do not impose an ordered test ladder.

Privilege escalation is an intentional platform difference: Levitate uses `sudo`, and Acorn uses `doas`. Both provide password-authenticated administration for members of `wheel`; parity checks must exercise the variant's declared tool.

## Build a product

```sh
cargo xtask build live-tools levitate
cargo xtask build live-tools acorn
```

LevitateOS's rootfs starts from a locked set of signed Fedora RPMs: the variant declares the Fedora release, signing key and packages (`sources.rootfs_source.rpm_base` in `distro.toml`), and `cargo xtask base lock levitate` resolves the newest build of each from the release and updates repositories into `distro/builder/recipes/sources/levitate-rpm-lock.rhai`. Rerun it to take Fedora's security updates; `cargo xtask doctor` says when the lock is a month old.

Canonical ISO products are `base-rootfs`, `live-boot` and `live-tools`. `installed-boot` is a product preparation target, not a release ISO target. An explicit release build can realize missing parent products from the dependency graph. Kernel rebuilds stay explicit:

```sh
cargo xtask build kernel levitate             # compile (23:00-10:00 unless --now)
cargo xtask build kernel levitate --prebuilt  # or download
```

Release runs live under `.artifacts/out/<distro>/releases/<product>/run-*/`. Installed test disks live under `.artifacts/out/<distro>/scenarios/install/run-*/`. Kernel artifacts remain under `.artifacts/kernel/<distro>/current/`. There are no numbered checkpoint output families.

## Run a scenario

```sh
cargo xtask test live-tools levitate
cargo xtask test install levitate
cargo xtask test runtime levitate --force
cargo xtask test all levitate
cargo xtask run live-tools levitate     # interactive
cargo xtask status levitate
```

`cargo xtask test --help` lists the scenarios; `all` runs every one, each on its own, and does not stop at a failure. Select any scenario directly. Each consumes existing artifacts and performs its required preflight; it does not build missing artifacts or require earlier scenario pass records. Installed-system scenarios require a completed install disk because that is their actual input. Create that input explicitly with the `install` scenario when needed.

Each cached result is bound to its own resolved input fingerprint. A changed input invalidates that result, and status reports stale results separately from passes. `--force` reruns only the selected scenario. `cargo xtask reset <distro>` removes saved results. Set `LEVITATE_SCENARIO_STATE_DIR` to keep separate state for CI or another local run.

## Correctness

Validate declared products, artifact provenance, required runtime payload and real behavior. Keep protections for destructive disk operations. Build payload through canonical producers; do not patch generated artifacts to obtain a passing result. Release preparation needs relevant integration/e2e evidence, including update/rollback coverage for A/B behavior; a cached pass or successful build alone is insufficient.
