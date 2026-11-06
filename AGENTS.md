# Development Guidelines

Keep development fast. Fix the actual behavior, use judgment, and keep process proportional to the change.

These rules apply to every directory and submodule, recursively, across Git repository boundaries. Nested guides may add domain details but must not reintroduce unit tests or mandatory development bureaucracy, except for the ginux unit-testing exception below.

## Working style
- ginux (`tools/ginux` and all of its branches and worktrees) is Greg Kurtzer's external upstream project (`gmkurtzer/ginux`); `LevitateOS/ginux` is our contribution fork. Respect upstream ownership, conventions, existing tests, and CI coverage. Keep LevitateOS-specific policies and exceptions in this superproject's instructions; do not put them in ginux files, commits, or pull requests. Do not remove upstream tests or weaken validation to impose LevitateOS workspace rules.
- Act on authorized work without repeated permission requests, plans, audits, checklists, decision matrices, or approval gates.
- Bureaucracy is prohibited: do not introduce mandatory paperwork, review ceremonies, coverage targets, or process-only blockers.
- Use the existing owner of a behavior. Prefer a small direct change over a new framework or parallel implementation.
- Preserve unrelated edits. Do not commit secrets, caches, or generated junk.
- Development submodules may follow their working branches. A checkout differing from the superproject pin is normal, not a development failure. Require exact pins only when reproducing or preparing a release.
- Never create, switch, or rename Git branches without the user's explicit permission. Commit on whatever branch is checked out, including `master`/`main`, in the superproject and in every submodule.
- Commit early and often. Commit each coherent, working step as it lands rather than batching a session's work into one large commit at the end.

## Validation
- Exception: the ginux repository (`tools/ginux`, including its branches and worktrees) permits unit tests, assertion-based unit doctests, their runners, and unit-test CI coverage. Preserve its existing tests and coverage; add or update focused tests when useful. The following unit-test prohibition and removal requirement do not apply to ginux.
- Unit testing, including assertion-based unit doctests, is prohibited. Do not add, retain, run, require, or replace unit tests with renamed equivalents, snapshots of isolated helpers, or mock-only unit suites.
- Remove unit tests and their runners rather than marking them ignored or reporting them as passing.
- Use compilation, typechecking, and focused integration, CLI, boot, install, or runtime checks when they help establish the changed behavior. Choose checks appropriate to the work; do not run a full suite for every edit or commit.
- No mandatory build, lint, test, or policy-audit pipeline on local commits. Keep those commands available explicitly.
- Report failures and unverified behavior honestly. Fix causes; do not fabricate success, suppress failures, or silently substitute outputs.

## Build and runtime
- Filesystem iteration uses the `live-tools` product. Named integration/e2e scenarios run independently against existing artifacts; do not reintroduce numbered composition, numeric aliases, or ordered test gates.
- Recipes own source/package facts and acquisition/build/install behavior. Rust/TOML owns orchestration, output locations, and contracts. Normal public recipes run without required defines.
- Build payload through its producers. Keep build commands explicit; scenario commands consume existing artifacts.
- Use `.artifacts` for current outputs and tooling.
- Reuse existing kernel artifacts during ordinary development. Rebuild kernels explicitly with `cargo xtask build kernel <distro>` (or `--all`).
- Keep runtime checks that prevent invalid artifacts or destructive operations. Fail with useful errors.
- For releases, produce reproducible artifacts and validate declared products, provenance, and relevant integration/e2e behavior. Development convenience is not release evidence.

## Code ownership
- Prefer `distro-variants/*`, `distro-builder`, `distro-contract`, `distro-spec`, `testing/*`, `tools/*`, `tui`, and `xtask` for active work. Edit legacy crates when the requested work requires it.
- Keep JavaScript/TypeScript ESM-only.
- Run workflows through `cargo xtask` (`cargo xtask --help` lists them; `just` has short aliases). Direct commands are fine.
- When asked to commit everything, commit changed submodules first, then the superproject pointers.

<!-- hdae:context-recovery:start -->
## HDAE context recovery

For every task that uses HDAE, and after any context compaction, reset, resume,
or handoff, read `.agents/skills/hdae-development/SKILL.md` completely before
continuing. Then read the skill-directed references and authoritative HDAE
project checkpoint. Do not rely on compacted chat as a substitute.
<!-- hdae:context-recovery:end -->
