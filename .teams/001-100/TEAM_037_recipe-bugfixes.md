# TEAM 037: Recipe Package Manager Bug Fixes

## Task
Implement bug fixes identified in the recipe package manager audit.

## Status
COMPLETE

## Bugs Fixed

### Critical (4)
1. **Variable name substring matching** (`recipe_state.rs:17-24`)
   - Added word boundary check after variable name
   - `get_var("installed")` no longer matches `installed_files`

2. **No HTTP timeout** (`http.rs`)
   - Added 30-second timeout to all HTTP requests
   - Prevents indefinite hangs on network issues

3. **Array parser escape bug** (`recipe_state.rs:189-213`)
   - Fixed escape handling: `\\` -> `\`, `\"` -> `"`
   - Unknown escapes now preserve backslash (e.g., `\d` -> `\d`)

4. **Path traversal in package names** (`recipe.rs:198-248`)
   - Added `validate_package_name()` function
   - Package names must be alphanumeric with `_` and `-` only
   - Prevents attacks like `../../../etc/passwd`

### High Priority (4)
5. **Partial removal marked complete** (`lifecycle.rs:257-277`)
   - Now fails if any file deletion fails
   - State only updated after ALL files successfully removed

6. **Network errors silently ignored in update** (`lifecycle.rs:364-367`)
   - Now returns proper error instead of `Ok(None)`
   - Caller can distinguish between "no updates" and "check failed"

7. **rpm_install symlinks lost** (`install.rs:151`)
   - Changed `is_file()` to `is_file() || is_symlink()`
   - Important symlinks like `/bin/sh` now tracked

8. **Dead code removed** (`context.rs`)
   - Removed unused `init_context()` function
   - Removed unused `get_recipe_path()` function
   - Added `#[allow(dead_code)]` to `recipe_path` field (kept for API)

### Medium Priority (1)
9. **GitHub rate limiting** (`http.rs:26-40, 52-66`)
   - Detect HTTP 403 and report rate limit exceeded
   - Detect HTTP 404 and report repository not found

## Files Modified
- `recipe/src/engine/recipe_state.rs` - Substring fix, escape fix, new tests
- `recipe/src/engine/util/http.rs` - Timeouts, rate limiting
- `recipe/src/bin/recipe.rs` - Package name validation
- `recipe/src/engine/lifecycle.rs` - Partial removal fix, update error fix
- `recipe/src/engine/phases/install.rs` - Symlink tracking
- `recipe/src/engine/context.rs` - Dead code removal

## Tests Added
- `test_var_substring_no_match` - Verifies variable matching doesn't have substring bug
- `test_array_escape_sequences` - Verifies `\\`, `\"`, `\t` etc. are handled
- `test_array_unknown_escape_preserved` - Verifies unknown escapes preserve backslash

## Comprehensive Test Suite Added

After bug fixes, added **99 new tests** covering edge cases:

### recipe_state.rs (46 tests total)
- Empty/whitespace files
- Variable parsing edge cases (no spaces, tabs, indentation)
- Unicode and emoji in strings
- Array edge cases (empty, trailing comma, escapes, paths with spaces)
- Invalid types (boolean, integer, array syntax)
- Roundtrip tests for all types
- Similar variable name handling

### recipe.rs CLI (17 tests)
- Package name validation (valid names, empty, special chars, dots)
- Path traversal attack prevention
- Recipe resolution (simple, hyphen, subdir, explicit path)
- Find installed/upgradable recipes

### lifecycle.rs (21 tests)
- has_action, get_recipe_name, call_action
- Remove: not installed, no files, deletes files, partial failure preserves state
- Update: no checker, returns unit, returns version, check fails
- Upgrade: not installed, already up to date
- cleanup_empty_dirs edge cases

### install.rs (15 tests)
- install_to_dir: no context, no files, copies, glob patterns, permissions
- install_bin, install_lib, install_man (sections 1, 5, default)
- Edge cases: nested dirs, overwrite, preserve content

### http.rs (11 tests + 4 ignored network tests)
- parse_version: all prefix types, empty, suffixes, multiple prefixes
- http_get: invalid URL, nonexistent domain
- Network tests (run with --ignored): real GitHub API calls

## Verification
```bash
cargo test  # 112 tests (95 lib + 17 bin), 4 ignored network tests
cargo build --release  # Builds without warnings
```

## Bug Found During Testing
- `parse_version` had incorrect prefix stripping order (fixed)
