# Development Guidelines

Keep development fast. Fix the actual behavior, use judgment, and keep process proportional to the change.

These rules apply to every directory and submodule, recursively, across Git repository boundaries. Nested guides may add domain details but must not reintroduce unit tests or mandatory development bureaucracy.

## Working style
- Act on authorized work without repeated permission requests, plans, audits, checklists, decision matrices, or approval gates.
- Bureaucracy is prohibited: do not introduce mandatory paperwork, review ceremonies, coverage targets, or process-only blockers.
- Use the existing owner of a behavior. Prefer a small direct change over a new framework or parallel implementation.
- Preserve unrelated edits. Do not commit secrets, caches, or generated junk.
- Development submodules may follow their working branches. A checkout differing from the superproject pin is normal, not a development failure. Require exact pins only when reproducing or preparing a release.

## Validation
- Unit testing, including assertion-based unit doctests, is prohibited. Do not add, retain, run, require, or replace unit tests with renamed equivalents, snapshots of isolated helpers, or mock-only unit suites.
- Remove unit tests and their runners rather than marking them ignored or reporting them as passing.
- Use compilation, typechecking, and focused integration, CLI, boot, install, or runtime checks when they help establish the changed behavior. Choose checks appropriate to the work; do not run a full suite for every edit or commit.
- No mandatory build, lint, test, or policy-audit pipeline on local commits. Keep those commands available explicitly.
- Report failures and unverified behavior honestly. Fix causes; do not fabricate success, suppress failures, or silently substitute outputs.

## Build and runtime
- Filesystem iteration uses `02LiveTools`; scenarios may run in any order. Full checkpoint validation belongs to release preparation.
- Recipes own source/package facts and acquisition/build/install behavior. Rust/TOML owns orchestration, output locations, and contracts. Normal public recipes run without required defines.
- Build payload through its producers. Keep build commands explicit; scenario commands consume existing artifacts.
- Use `.artifacts` for current outputs and tooling. Do not bind active builds to legacy crates' download directories.
- Reuse existing kernel artifacts during ordinary development. Rebuild kernels explicitly with `cargo xtask kernels build <distro>` or `build-all`.
- Keep runtime checks that prevent invalid artifacts or destructive operations. Fail with useful errors.
- For releases, produce reproducible artifacts and validate checkpoint payloads and isolation. Development convenience is not release evidence.

## Code ownership
- Prefer `distro-variants/*`, `distro-builder`, `distro-contract`, `distro-spec`, `testing/*`, `tools/*`, `tui`, and `xtask` for active work. Edit legacy crates when the requested work requires it.
- Keep JavaScript/TypeScript ESM-only.
- Prefer `just` wrappers when useful; direct commands are fine.
- When asked to commit everything, commit changed submodules first, then the superproject pointers.
