# TUI Workspace Migration Plan (No Regression)

Date: 2026-02-24

## Invariant

`docs/tui` and `shared/tui-kit` behavior must migrate into `tui` workspace without changing runtime behavior, keybinds, snapshots, or CLI outputs.

## Ownership Layer

- `tui/kit/core`: reusable runtime/primitives/components/chrome/theme only.
- `tui/kit/presets-install`: cross-app install UX composition presets (no domain data access).
- `tui/kit/testing`: shared test harness and snapshot utilities.
- `tui/apps/s02-live-tools/install-docs`: docs domain, content adapters, navigation, CLI, rendering plan, app wiring.

Why this layer can enforce it:
- `apps/*` keeps domain behavior local.
- `kit/*` stays dependency-safe and reusable.
- Direction is one-way: `apps -> kit`.

Proof artifact path:
- `tui/MIGRATION_PLAN.md`

Complexity budget:
- Add only package boundary moves and import rewrites.
- No semantic rewrites in Phase 0-2.

## MODEL RECLASSIFICATION GATE

Invariant sentence:
- "Install docs behavior and shared TUI primitives must be identical before and after path/package migration."

Current model/layer:
- Split ownership in `docs/tui` and `shared/tui-kit`, wired by local file dependency.

Alternative models:

1. Keep split repos and patch paths
- determinism: low
- complexity: high ongoing
- ownership fit: weak
- testability: medium
- compatibility impact: low short-term

2. Single `tui` workspace with app/kit package boundaries (chosen)
- determinism: high
- complexity: medium one-time
- ownership fit: strong
- testability: high
- compatibility impact: medium during migration

3. Collapse all code into one package
- determinism: medium
- complexity: medium
- ownership fit: weak
- testability: medium
- compatibility impact: medium

Chosen model:
- Option 2, because it preserves domain boundaries while enabling atomic refactors and stable dependency direction.

## Target Package Layout

```text
/tui
  /apps
    /s02-live-tools/install-docs
    /s03-install/*
    /s04-login-gate/*
    /s06-runtime/*
  /kit
    /core
    /presets-install
    /testing
```

## File Ownership Mapping

### 1) shared/tui-kit -> tui/kit/core

Move as-is first (no API change):
- `src/app/*`
- `src/runtime/*`
- `src/chrome/*`
- `src/surfaces/*`
- `src/primitives/*`
- `src/components/*`
- `src/hooks/*`
- `src/theme*`
- `src/utils/*`
- `src/index.ts`, `src/globals.d.ts`
- unit tests under `tests/unit/*`

Do not move into core:
- docs-domain rendering/navigation/data mapping

### 2) docs/tui -> tui/apps/s02-live-tools/install-docs

Move as-is first:
- `src/adapters/*`
- `src/domain/*`
- `src/rendering/*`
- `src/presentation/*`
- `src/app/*`
- `src/cli/*`
- `src/main.ts`
- `src/*.test.ts`
- `bin/*`, `scripts/*`, `snapshots/*`

Keep behavior exactly the same:
- CLI contract (`levitate-install-docs`)
- keybinds
- inspect snapshot outputs

### 3) docs/tui local primitives that may be promoted later

Candidate for later extraction to `tui/kit/presets-install` after parity is proven:
- `src/presentation/ink/components/{sidebar,status-bar,content-pane}.tsx`
- `src/presentation/ink/screens/install-viewer.tsx`
- `src/presentation/ink/theme/index.ts`

Rules:
- Promote only if reused by at least one more app.
- Promotion must include unchanged snapshot proof in both consumers.

### 4) test infrastructure -> tui/kit/testing

Initial contents to add during migration:
- PTY runner utilities used by inspect/snapshot tests.
- shared snapshot assertions and fixture builders.

No app-domain logic allowed in `kit/testing`.

## No Regression Gates (must pass before cutover)

From migrated paths:
1. install-docs typecheck and tests pass.
2. install-docs inspect snapshot check matches baseline.
3. `tui/kit/core` unit tests pass.
4. superproject wrappers resolve to new paths and still run.

Required command equivalents after migration:
- `just docs-tui-check`
- `just docs-tui-inspect-check`
- `bun run test` in `tui/kit/core`

## Migration Phases

### Phase 0: Path Mirror (no behavior change)

1. Copy `shared/tui-kit` into `tui/kit/core`.
2. Copy `docs/tui` into `tui/apps/s02-live-tools/install-docs`.
3. Keep old paths active with deprecation warnings.

### Phase 1: Workspace Wiring

1. Add new workspace paths in root `package.json`.
2. Rewire `just docs-tui*` to new path.
3. Keep compatibility wrappers for one transition window.

### Phase 2: Import and Dependency Cutover

1. Update install-docs imports from `../../shared/tui-kit` to `tui/kit/core` package reference.
2. Ensure no reverse imports from `kit` into `apps`.
3. Run parity checks and snapshot diff.

### Phase 3: Optional Extraction

1. Move reusable install compositions into `kit/presets-install`.
2. Add shared test helpers into `kit/testing`.
3. Prove no behavior changes via snapshot parity.

### Phase 4: Legacy Freeze and Archive Readiness

1. Old repos become read-only compatibility.
2. Remove old workspace paths after stable release window.
3. Archive when no runtime references remain.

## Cutover Checklist

- [ ] `tui/apps/s02-live-tools/install-docs` matches old `docs/tui` snapshots.
- [ ] `tui/kit/core` exports match old `shared/tui-kit` public API.
- [ ] `just docs-tui` and `just docs-tui-split` run from new path.
- [ ] No direct imports from `apps/*` into any `kit/*` package.
- [ ] `tools/recpart` remains backend owner; S03 UI stays in `tui/apps/s03-install/disk-plan`.
