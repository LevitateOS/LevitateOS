# TEAM_214: Port All Kernel Builds to Recipe System

**Date:** 2026-02-10
**Status:** Complete

## What was done

Ported all three distros (AcornOS, IuppiterOS, LevitateOS) to use the recipe system
for kernel builds, eliminating the parallel Rust kernel build implementation.

### Changes

1. **distro-builder/src/recipe/mod.rs** — Added `run_recipe_json_with_defines()` for
   injecting `--define` constants into recipe execution (ported from leviso).

2. **distro-builder/src/recipe/linux.rs** — Changed `linux()` signature to accept
   `&KernelSource`, injecting KERNEL_VERSION/SHA256/LOCALVERSION as defines.

3. **AcornOS/deps/linux.rhai** & **IuppiterOS/deps/linux.rhai** — Parameterized with
   `KERNEL_VERSION` scope constant (computed from `--define`). Added SHA256 verification
   via `verify_sha256()`. Removed hardcoded version strings.

4. **leviso/deps/linux.rhai** — Added SHA256 verification after download.

5. **AcornOS/src/main.rs** — Replaced all `distro_builder::build::kernel::*` calls with
   `distro_builder::recipe::linux::linux()`. Removed `steal_kernel()` and
   `leviso_kernel_available()` (theft is handled by the .rhai recipe).

6. **IuppiterOS/src/main.rs** — Same as AcornOS. Removed `has_kernel()`,
   `leviso_kernel_available()`, and inline theft logic.

7. **leviso/src/recipe/linux.rs** — Replaced full implementation with thin wrapper
   calling `distro_builder::recipe::linux::linux(base_dir, &KERNEL_SOURCE)`.

8. **leviso/src/recipe/mod.rs** — Replaced duplicated `find_recipe`, `run_recipe_json`,
   `run_recipe_json_with_defines`, `RecipeBinary`, `build_from_source` with re-exports
   from `distro_builder::recipe::*`.

9. **distro-builder/src/build/kernel.rs** — Marked all public functions as
   `#[deprecated]` with message pointing to recipe system.

## Phase 2: Recipe consolidation

Consolidated duplicate `linux.rhai` from AcornOS and IuppiterOS into a single shared
recipe at `distro-builder/recipes/linux.rhai`.

- AcornOS and IuppiterOS linux.rhai were identical (only variable names differed,
  and both just used `dirname(BUILD_DIR)`)
- leviso's linux.rhai is genuinely different (no theft, defconfig + scripts/config
  merge, UsrMerge module install) — kept separate
- `linux()` in distro-builder now checks `{distro}/deps/linux.rhai` first, falls
  back to `distro-builder/recipes/linux.rhai`
- Deleted `AcornOS/deps/linux.rhai` and `IuppiterOS/deps/linux.rhai`

## Key decisions

- Kept theft mode in shared .rhai file (recipe handles it internally)
- Used `#[deprecated]` instead of deleting Rust kernel build code (safer rollback)
- Re-exported shared types from distro-builder in leviso to avoid breaking leviso internals
- leviso keeps its own linux.rhai because it has genuinely different build logic
- Distro-specific recipes override shared recipes (fallback chain)

## Files modified

| File | Change |
|------|--------|
| distro-builder/src/recipe/mod.rs | Added `run_recipe_json_with_defines` |
| distro-builder/src/recipe/linux.rs | Accept `KernelSource` param, fallback to shared recipe |
| distro-builder/recipes/linux.rhai | NEW: shared recipe for Alpine-based distros |
| distro-builder/src/build/kernel.rs | Deprecated all public functions |
| AcornOS/deps/linux.rhai | DELETED (uses shared recipe) |
| IuppiterOS/deps/linux.rhai | DELETED (uses shared recipe) |
| leviso/deps/linux.rhai | Added SHA256 verify (kept separate) |
| AcornOS/src/main.rs | Switched to recipe system |
| IuppiterOS/src/main.rs | Switched to recipe system |
| leviso/src/recipe/linux.rs | Thin wrapper over distro-builder |
| leviso/src/recipe/mod.rs | Re-exports from distro-builder |
