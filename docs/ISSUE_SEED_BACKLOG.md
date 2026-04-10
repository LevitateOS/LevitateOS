# Issue Seed Backlog

This file is a temporary migration aid.

Purpose:

- convert the currently known active backlog into GitHub Issues
- stop treating migration docs and audits as the live backlog

After the corresponding issues exist, replace each section below with issue
links or delete this file.

## Created GitHub Issues

GitHub Issues are now the live source of truth for this backlog.

- `#28` epic: production readiness for LevitateOS and AcornOS with a pristine installation environment
  Child issues: `#29`, `#30`, `#31`, `#1`, `#2`, `#3`, `#4`, `#5`, `#6`
- `#29` task: define production-readiness gates for canonical levitate and acorn releases
- `#30` task: complete production-readiness sweep for distro-variants/levitate on the canonical release path
- `#31` task: complete production-readiness sweep for distro-variants/acorn on the canonical release path
- `#1` epic: remove stage/checkpoint residue from canonical validation and reporting
  Child issues: `#7`, `#8`, `#9`, `#10`
- `#2` epic: complete the recipe A/B composition model
  Child issues: `#11`, `#12`, `#13`, `#14`
- `#3` epic: harden the recipe engine
  Child issues: `#15`, `#16`, `#17`, `#18`
- `#4` epic: unblock manual install through canonical recipe ownership
  Child issues: `#19`, `#20`, `#21`
- `#5` epic: align operator-facing docs with the canonical ring model
  Child issues: `#22`, `#23`, `#24`
- `#6` epic: small active cleanup and optional legacy compatibility debt
  Child issues: `#25`, `#26`, `#27`
- `#32` task: migrate LevitateOS default-branch delivery to a PR-gated workflow
- `#34` task: enable automatic AI review for pull requests

## Epic 1: Remove Stage/Checkpoint Residue From Canonical Validation And Reporting

Primary owner: `distro-contract`

Why:

- the migration index says the main remaining repo-level work is that canonical
  validation/runtime/error surfaces still center `CheckpointId`
- ring ownership and canonical ring execution are materially landed already

Source references:

- `distro-builder/docs/00_MIGRATION_INDEX.md`
- `distro-builder/docs/05_MIGRATION_RING_EXECUTION_MODEL.md`
- `distro-contract/src/error.rs`
- `distro-contract/src/validate.rs`
- `distro-contract/src/runtime.rs`

Suggested child issues:

1. `task: replace canonical CheckpointId attribution with owner/ring/product-native vocabulary`
   - owner: `distro-contract`
   - kind: `migration`
   - priority: `p0`
   - acceptance:
     - `cargo check -p distro-contract -p levitate-xtask -p install-tests`
     - `cargo test -p distro-contract`
     - canonical validation/reporting no longer centers `CheckpointId`

2. `task: reduce stage-era compatibility aliases on the default operator path`
   - owner: `xtask`
   - related paths:
     - `justfile`
     - `xtask/README.md`
     - `testing/install-tests/test-scripts/README.md`
   - kind: `migration`
   - priority: `p1`
   - acceptance:
     - canonical docs teach product/scenario names first
     - remaining stage aliases are explicit compatibility-only paths

3. `task: remove transition-only path-layout metadata from distro-contract`
   - owner: `distro-contract`
   - kind: `debt`
   - priority: `p2`
   - acceptance:
     - downstream callers compile without the transition metadata
     - variant/layout tests still pass

4. `task: decide which migration docs stay archival and rewrite the rest`
   - owner: `docs/content`
   - related paths:
     - `distro-builder/docs/03_MIGRATION_STAGELESS.md`
     - `distro-builder/docs/04_MIGRATION_RING_MODEL.md`
     - `distro-builder/docs/05_MIGRATION_RING_EXECUTION_MODEL.md`
     - `distro-builder/docs/06_MIGRATION_VARIANT_LAYOUT.md`
   - kind: `docs`
   - priority: `p2`
   - acceptance:
     - each migration doc is clearly marked current vs archival

## Epic 2: Complete The Recipe A/B Composition Model

Primary owner: `tools/recipe`

Why:

- the repo still lacks the recipe features needed for safe inactive-slot
  composition and atomic A/B behavior

Source references:

- `tools/recipe/README.md`
- `tools/recipe/HELPERS_AUDIT.md`

Suggested child issues:

1. `task: implement sysroot confinement and no-escape path joining in recipe`
   - owner: `tools/recipe`
   - kind: `migration`
   - priority: `p0`
   - acceptance:
     - sysroot-aware path handling exists
     - helpers cannot escape the target sysroot

2. `task: add installed_files tracking and atomic staging/commit to recipe`
   - owner: `tools/recipe`
   - kind: `migration`
   - priority: `p0`
   - acceptance:
     - recipe install flow tracks installed files
     - staging/commit behavior exists for A/B composition

3. `task: add high-level install helpers for recipe`
   - owner: `tools/recipe`
   - kind: `task`
   - priority: `p1`
   - acceptance:
     - helper surface includes install helpers such as `install_bin` or an
       equivalent lower-level installer

4. `task: add update and upgrade lifecycle commands to recipe`
   - owner: `tools/recipe`
   - kind: `task`
   - priority: `p2`
   - acceptance:
     - update/upgrade lifecycle is implemented or explicitly retired from the
       spec/docs in the same change set

## Epic 3: Harden The Recipe Engine

Primary owner: `tools/recipe`

Why:

- the recipe audit still carries unresolved correctness and security debt

Source references:

- `docs/recipe-engine-audit.md`

Suggested child issues:

1. `task: add argv-safe shell helper and audit recipes away from shell concatenation`
   - owner: `tools/recipe`
   - kind: `debt`
   - priority: `p0`

2. `task: prevent archive extraction path and symlink escape`
   - owner: `tools/recipe`
   - kind: `bug`
   - priority: `p0`

3. `task: add end-to-end extends integration test coverage`
   - owner: `tools/recipe`
   - kind: `task`
   - priority: `p1`

4. `task: lock or atomically update .packages-version`
   - owner: `tools/recipe`
   - kind: `bug`
   - priority: `p2`

## Epic 4: Unblock Manual Install Through Canonical Recipe Ownership

Primary owner: `tools/recipe`

Why:

- manual install docs still depend on missing recipe/bootstrap/live-ISO work

Source references:

- `docs/manual-install-plan.md`

Suggested child issues:

1. `task: implement recipe bootstrap`
   - owner: `tools/recipe`
   - kind: `migration`
   - priority: `p0`

2. `task: define the base recipe set for bootstrap`
   - owner: `tools/recipe`
   - kind: `task`
   - priority: `p1`

3. `task: ensure recipe binary and base recipes are present on the live ISO`
   - owner: `distro-builder`
   - related paths:
     - `docs/manual-install-plan.md`
     - live-ISO build/product paths
   - kind: `migration`
   - priority: `p1`

## Epic 5: Align Operator-Facing Docs With The Canonical Ring Model

Primary owner: `docs/content`

Why:

- the docs still teach legacy paths and contain stale tool status

Source references:

- `README.md`
- `docs/content/src/content/05-rec-tooling/03-recab.ts`
- `tools/recab/README.md`

Suggested child issues:

1. `task: rewrite top-level README around canonical variant ownership and current build flows`
   - owner: `docs/content`
   - kind: `docs`
   - priority: `p1`

2. `bug: recab docs page says scaffold but implementation is live`
   - owner: `docs/content`
   - kind: `bug`
   - priority: `p1`
   - acceptance:
     - docs page matches implemented `recab` capabilities and current stability

3. `task: audit canonical docs for stale stage/staged wording`
   - owner: `docs/content`
   - related paths:
     - `docs/content`
     - `xtask/README.md`
     - `testing/install-tests/README.md`
   - kind: `docs`
   - priority: `p2`

## Epic 6: Small Active Cleanup And Optional Legacy Compatibility Debt

Primary owner: `testing/install-tests`

Why:

- a few active TODOs remain in canonical code
- there is also legacy compatibility drift outside the canonical owners

Source references:

- `testing/install-tests/src/preflight.rs`
- `distro-spec/src/acorn/verification.rs`
- `AcornOS/src/artifact/iso.rs`
- `IuppiterOS/src/artifact/iso.rs`

Suggested child issues:

1. `task: pass distro_id through fsdbg verify path once verify_distro exists`
   - owner: `testing/install-tests`
   - kind: `task`
   - priority: `p2`

2. `task: resolve or retire stale Acorn verification TODO`
   - owner: `distro-spec`
   - kind: `debt`
   - priority: `p3`

3. `bug: full workspace compile fails in legacy AcornOS and IuppiterOS crates`
   - owner: `legacy-compat`
   - kind: `bug`
   - priority: `p3`
   - acceptance:
     - `cargo check --workspace --all-targets` passes
