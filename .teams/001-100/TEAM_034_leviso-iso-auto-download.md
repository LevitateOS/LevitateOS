# TEAM_034: Leviso Rocky Linux ISO Auto-Download

## Objective
Implement the plan from Phase 3 of TEAM_033 - add missing helpers to recipe crate and create ISO download recipe for leviso.

## Changes

### Recipe Crate (recipe/src/engine.rs)
Add missing helpers:
- File checks: `exists`, `file_exists`, `dir_exists`
- File operations: `mkdir`, `rm`, `mv`, `ln`, `chmod`
- Reading: `read_file`, `glob_list`
- Environment: `env`, `set_env`
- Command variants: `run_output`, `run_status`

### Leviso Crate
- Add `levitate-recipe` dependency to Cargo.toml
- Create `recipes/lib/rocky.rhai` - shared Rocky Linux module
- Create `recipes/00-rocky-iso.rhai` - download ISO recipe
- Add `ensure_source_iso()` to main.rs
- Update `.gitignore` for ISO files

## Status
- [x] Read existing engine.rs code
- [x] Add missing helpers to recipe crate (already implemented)
- [x] Fix executor lifecycle to check is_installed() first
- [x] Test recipe crate builds
- [x] Create recipes/ directory structure
- [x] Create rocky.rhai module
- [x] Create rocky-iso.rhai recipe
- [x] Add .gitignore for ISO files

## Implementation Notes

### Executor Lifecycle Fix (recipe/src/engine.rs)
Changed `execute()` to follow proper package lifecycle:
1. `is_installed()` - Check if already done, skip if true
2. `acquire()` - Get source materials
3. `build()` - Only called if defined in recipe (optional)
4. `install()` - Copy to PREFIX

Key changes:
- Added `has_action()` helper to check if action exists in AST
- Added `get_recipe_name()` helper to extract recipe name
- Renamed `call_phase()` to `call_action()` with stricter error handling
- Now logs "already installed, skipping" when `is_installed()` returns true

### Files Created
- `leviso/recipes/lib/rocky.rhai` - Module with `find_rpm()` and `install_rpms()`
- `leviso/recipes/00-rocky-iso.rhai` - ISO download recipe with proper actions
- `leviso/.gitignore` - Ignores ISO files, build outputs
