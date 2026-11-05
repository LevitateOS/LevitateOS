# Builds and scenarios

LevitateOS and AcornOS use ring-owned manifests under `distro-variants/<distro>/`.

| Owner | Responsibility | Manifest |
|---|---|---|
| Ring 3: sources | Recipe selection and source acquisition | `ring3/sources.toml` |
| Ring 2: products | Filesystem composition and runtime payloads | `ring2/products.toml` |
| Ring 1: transforms | EROFS, initramfs and UKI artifacts | `ring1/transforms.toml` |
| Ring 0: releases | ISO/disk packaging and release metadata | `ring0/release.toml` |
| Build host | Host prerequisites and kernel build wiring | `build-host/build-host.toml` |
| Scenarios | Boot, installation and runtime expectations | `scenarios/scenarios.toml` |

Recipes own package/source facts and acquire/build/install behavior. Orchestration owns product dependencies, output locations and execution. Product dependencies form a build graph; they do not impose an ordered test ladder.

## Build a product

```sh
just release-build live-tools levitate
just release-build live-tools acorn
```

Canonical ISO products are `base-rootfs`, `live-boot` and `live-tools`. `installed-boot` is a product preparation target, not a release ISO target. An explicit release build can realize missing parent products from the dependency graph. Kernel rebuilds stay explicit:

```sh
cargo xtask kernels build levitate
```

Release runs live under `.artifacts/out/<distro>/releases/<product>/run-*/`. Installed test disks live under `.artifacts/out/<distro>/scenarios/install/run-*/`. Kernel artifacts remain under `.artifacts/kernel/<distro>/current/`. There are no numbered checkpoint output families.

## Run a scenario

```sh
just scenario-test live-tools levitate
just scenario-test install levitate
just scenario-test runtime levitate
just scenario live-tools levitate
cargo xtask scenarios test runtime levitate --force
cargo xtask scenarios status levitate
```

Available scenarios are `build-preflight`, `live-boot`, `live-tools`, `install`, `installed-boot`, `automated-login` and `runtime`. Select any scenario directly. Each consumes existing artifacts and performs its required preflight; it does not build missing artifacts or require earlier scenario pass records. Installed-system scenarios require a completed install disk because that is their actual input. Create that input explicitly with the `install` scenario when needed.

Each cached result is bound to its own resolved input fingerprint. A changed input invalidates that result, and status reports stale results separately from passes. `--force` reruns only the selected scenario. `cargo xtask scenarios reset <distro>` removes saved results. Set `LEVITATE_SCENARIO_STATE_DIR` to keep separate state for CI or another local run.

## Correctness

Validate declared products, artifact provenance, required runtime payload and real behavior. Keep protections for destructive disk operations. Build payload through canonical producers; do not patch generated artifacts to obtain a passing result. Release preparation needs relevant integration/e2e evidence, including update/rollback coverage for A/B behavior; a cached pass or successful build alone is insufficient.
