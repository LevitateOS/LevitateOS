# TEAM 002: Recipe Executor Implementation

## Status: Complete

## Goal
Implement `executor.rs` for the `levitate-recipe` crate to actually run parsed recipes.

## Design Decisions
1. **Shell-based execution** - Use `sh -c` for commands, shell out to `curl`, `tar`, `git`
2. **Minimal dependencies** - Use std::process::Command only (no new deps)
3. **Context struct** - prefix, build_dir, arch, nproc, dry_run, verbose

## Implementation Phases
- [x] Phase 1: Core executor scaffold (ExecuteError, Context, Executor, run_cmd)
- [x] Phase 2: Acquire action (Source, Binary, Git)
- [x] Phase 3: Build action (Skip, Extract, Steps)
- [x] Phase 4: Install action (ToBin, ToLib, ToConfig, ToMan, ToShare, Link)
- [x] Phase 5: Start/Stop/Remove actions
- [ ] Phase 6: Docker test harness (deferred - not in original scope)

## Files Modified
- `crates/recipe/src/executor.rs` - New file (650+ lines)
- `crates/recipe/src/lib.rs` - Added module export

## Exported Types
- `Context` - Execution context (prefix, build_dir, arch, nproc, dry_run, verbose)
- `Executor` - Main executor with `execute()`, `acquire()`, `build()`, `install()`, etc.
- `ExecuteError` - Error enum for execution failures

## Key Features
- Variable expansion: `$PREFIX`, `$NPROC`, `$ARCH`, `$BUILD_DIR`
- Shell quoting for safe command construction
- Dry-run mode for testing without side effects
- Verbose mode for debugging
- Checksum verification (sha256sum)
- Archive extraction (tar-gz, tar-xz, tar-bz2, zip)
- Git clone with tag/branch/commit support

## Tests Added
- `test_expand_vars` - Variable substitution
- `test_url_filename` - URL filename extraction
- `test_shell_quote` - Shell quoting safety
- `test_context_default` - Default context values
- `test_context_builder` - Builder pattern

## Progress Log
- Created team file
- Read existing recipe types
- Implemented full executor
- Fixed format string brace escaping bug
- All 14 tests passing
