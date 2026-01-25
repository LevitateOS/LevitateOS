# TEAM_039: Recipe Package Manager Bug Fixes

## Objective
Implement fixes from the Recipe PM audit document covering safety, correctness, and robustness improvements.

## Status: COMPLETED

## Changes Implemented

### Phase 1: Safety (Critical)
- [x] **File locking**: Added `fs2` crate for file locking. Recipe execution now acquires an exclusive lock via `acquire_recipe_lock()` before any operations. Lock is automatically released via RAII `RecipeLock` guard. Both `execute()` and `remove()` functions are protected.
- [x] **Atomic state updates**: `set_var()` in `recipe_state.rs` now writes to a temp file first, then atomically renames. This prevents partial writes leaving recipe files in corrupt state.

### Phase 2: Correctness (Important)
- [x] **Semantic version comparison**: Added `semver` crate. Version comparison in `upgrade()` now uses `semver::Version::parse()` with fallback to string comparison for non-semver versions. `version_is_up_to_date()` helper function handles the comparison.
- [x] **Dependency validation**: `topological_sort()` now calls `validate_dependencies()` before sorting. Returns clear error message listing all packages with missing dependencies.

### Phase 3: Robustness (Nice to have)
- [x] **Comment handling**: Added `strip_inline_comments()` helper in `recipe_state.rs` that strips `//` line comments and `/* */` block comments from values before parsing. Handles comments inside quoted strings correctly.
- [x] **Path traversal validation**: Added `validate_path_within_prefix()` function in `install.rs`. All install functions (`install_to_dir`, `install_man`, `rpm_install`) now validate that destination paths stay within PREFIX using canonicalization.

### Phase 4: Polish
- [x] **Configurable HTTP timeout**: Changed `HTTP_TIMEOUT_SECS` constant to `get_http_timeout()` function that reads from `RECIPE_HTTP_TIMEOUT` env var with clamping (5-300 seconds). Cached via `OnceLock` for performance.
- [x] **Context cleanup guard**: Added `ContextGuard` RAII struct in `context.rs` that ensures context cleanup even if recipe execution panics. Used in `execute()` function.

## Files Modified
- `recipe/Cargo.toml` - Added `fs2 = "0.4"` and `semver = "1"` dependencies
- `recipe/src/engine/lifecycle.rs` - File locking, context guard, semantic version comparison
- `recipe/src/engine/recipe_state.rs` - Atomic writes, comment stripping
- `recipe/src/engine/deps.rs` - Dependency validation
- `recipe/src/engine/phases/install.rs` - Path traversal validation
- `recipe/src/engine/util/http.rs` - Configurable timeout
- `recipe/src/engine/context.rs` - ContextGuard RAII struct

## Tests
All 240 tests pass:
- 155 unit tests
- 17 CLI tests
- 32 e2e tests
- 19 integration tests
- 17 regression tests

One test was updated to reflect new expected behavior:
- `test_missing_dependency_in_chain` - Now expects error when dependency is missing (was silently ignoring)

## Dependencies Added
- `fs2 = "0.4"` - Cross-platform file locking
- `semver = "1"` - Semantic version parsing and comparison
