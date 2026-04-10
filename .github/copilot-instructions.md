Review LevitateOS with a policy-first mindset.

- Prioritize bugs, regressions, policy violations, reproducibility gaps, and missing validation over style.
- Treat `distro-variants/*`, `distro-builder`, `distro-contract`, `distro-spec`, `testing/*`, and `xtask` as the canonical ownership areas.
- Treat `leviso/`, `AcornOS/`, `IuppiterOS/`, and `RalphOS/` as legacy compatibility surfaces unless the pull request explicitly targets them.
- Flag any wiring that points checkpoint, rootfs, or tooling paths at legacy download outputs such as `*/downloads/rootfs` or `*/downloads/.tools`.
- Prefer true root-cause fixes. Flag masking, suppression, fallback paths, or ad-hoc artifact surgery that only make checks pass.
- Check that executable entrypoints enforce policy guards themselves. Wrapper-only enforcement in `just` is not sufficient.
- Check the recipe/orchestrator boundary strictly: recipes own package and source facts; Rust/TOML orchestration owns sequencing, policy, contracts, and artifact topology.
- Flag Rust or TOML changes that duplicate recipe-owned facts such as package lists, upstream URLs, checksums, torrent links, trust-marker names, or large required define maps.
- Check checkpoint vocabulary and ownership. Rings, products, releases, and scenarios own manifests and orchestration; checkpoint names are only canonical where the conformance ladder owns the concept.
- Flag cross-checkpoint reuse of non-kernel artifacts in release or hardening paths.
- Flag subtractive shaping of checkpoint payloads. Checkpoint progression should be additive by producers, not post-copy pruning.
- Enforce the build versus scenario boundary. Scenario wrappers should consume existing artifacts, not rebuild implicitly.
- Expect explicit acceptance commands or a concrete verification path on the pull request.
- For review output, prefer concrete findings with file references and explain the user-visible or policy risk.
