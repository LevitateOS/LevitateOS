# TEAM_215: Recipe Composition via `//! extends:` Directive

**Date:** 2026-02-10
**Status:** Complete

## What Was Implemented

Added recipe inheritance via `//! extends: <path>` directive, allowing child recipes to extend a base recipe and override only the phases that differ.

### Phase 1: Recipe Engine (tools/recipe)
- `parse_extends()` — extracts base path from leading `//!` comments
- `resolve_base_path()` — searches child dir, then `--recipes-path`
- `compile_recipe()` — compiles base + child ASTs, merges via Rhai `+=`
- Recursive extends rejected with clear error
- `BASE_RECIPE_DIR` constant set when extends is active
- `--recipes-path` CLI flag wired through to engine (was parsed but unused)
- All 188 tests pass including 4 new extends-specific tests

### Phase 2: linux.rhai Refactoring
- Created `distro-builder/recipes/linux-base.rhai` — shared kernel build with hookable helpers:
  - `_apply_kconfig()` — kconfig application strategy
  - `_install_modules()` — module installation method
  - `_cleanup_modules()` — post-install cleanup
- Rewrote `distro-builder/recipes/linux.rhai` — Alpine theft overlay (extends linux-base)
- Rewrote `leviso/deps/linux.rhai` — LevitateOS overlay with defconfig merge + UsrMerge

### Phase 3: Shared Recipe Consolidation — SKIPPED
Cannot move per-distro recipes to shared location because `ctx::persist` writes state back to the recipe file. Multiple distros sharing one file would stomp each other's persisted paths.

### Phase 4: distro-builder Command Wiring
- `run_recipe_json_with_defines()` now accepts `recipes_path: Option<&Path>`
- Passes `--recipes-path` to recipe binary Command
- `linux()` passes `distro-builder/recipes/` so extends resolution works

## Key Decisions
- **Template method pattern** for partial overrides: base calls `_apply_kconfig()`, child overrides it
- **No recursive extends** for v1 — explicit error if attempted
- **Child MUST define `let ctx = #{...}`** — required by `ctx::persist`
- **Skipped Phase 3** — ctx persistence model prevents file sharing

## Files Modified
- `tools/recipe/src/core/executor.rs` — extends parsing + AST merge
- `tools/recipe/src/lib.rs` — thread recipes_path
- `tools/recipe/src/bin/recipe.rs` — wire --recipes-path
- `distro-builder/recipes/linux-base.rhai` — NEW shared base
- `distro-builder/recipes/linux.rhai` — REWRITTEN as extends overlay
- `leviso/deps/linux.rhai` — REWRITTEN as extends overlay
- `distro-builder/src/recipe/mod.rs` — recipes_path in Command
- `distro-builder/src/recipe/linux.rs` — pass recipes_path
