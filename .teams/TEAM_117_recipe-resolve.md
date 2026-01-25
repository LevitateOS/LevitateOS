# TEAM_117: Unify Acquisition with Recipe Resolve

## Goal
Add `resolve` capability to recipe so it can replace leviso-deps as the universal acquisition layer.

## Status: COMPLETE (Phases 1-5)

## What Was Done

### Phase 1 - New Helpers (COMPLETE)
- `helpers/git.rs` - git_clone, git_clone_depth for cloning repos to BUILD_DIR
- `helpers/torrent.rs` - torrent download via aria2c, download_with_resume
- `helpers/mod.rs` - registered all new functions

### Phase 2 - Resolve Lifecycle (COMPLETE)
- `core/lifecycle.rs` - added `resolve()` function
- `lib.rs` - added `RecipeEngine::resolve()` method

### Phase 3 - CLI Command (COMPLETE)
- `bin/recipe.rs` - added `recipe resolve <name>` command with `--format` option

### Phase 4 - Dependency Recipes (COMPLETE)
- `leviso/deps/linux.rhai` - Linux kernel source resolution
- `leviso/deps/rocky.rhai` - Rocky Linux ISO resolution
- `leviso/deps/recstrap.rhai` - recstrap tool resolution
- `leviso/deps/recfstab.rhai` - recfstab tool resolution
- `leviso/deps/recchroot.rhai` - recchroot tool resolution

### Phase 5 - Leviso Integration (COMPLETE)
- `leviso/src/resolve.rs` - helper to call recipe resolve from leviso
- `leviso/Cargo.toml` - added `which` dependency
- `leviso/src/lib.rs` - exported resolve module

### Phase 6 - Remove leviso-deps (NOT DONE)
- Intentionally deferred - leviso-deps is kept as the primary implementation
- `leviso::resolve` is available as an alternative for future migration

## Usage

### Recipe CLI
```bash
# Resolve linux dependency
cd leviso && recipe resolve linux -r deps

# JSON output
recipe resolve linux -r deps --format json
```

### From Rust (leviso)
```rust
use leviso::resolve::resolve_dep;

let linux_path = resolve_dep("linux")?;
let rocky_path = resolve_dep("rocky")?;
```

## Test Results
- 225 tests pass in levitate-recipe
- All leviso tests pass

## Future Work
- Migrate leviso preflight checks to use recipe resolve
- Remove leviso-deps once recipe resolve is proven reliable
- Add more dependency recipes as needed

## Implementation Log

- 2026-01-25: Completed Phases 1-5
- 2026-01-25: Bug Fixes & Hardening
  - **Security Fixes (Priority 1)**
    - Added URL scheme validation to git.rs (https/http/ssh/git@ only)
    - Added URL scheme validation to torrent.rs (http/https/magnet only)
    - Fixed JSON output injection vulnerability in recipe.rs (now uses serde_json)
    - Added filename sanitization in torrent.rs (handles magnet links, query strings, path traversal)
    - Added path validation to resolve() (canonicalizes returned paths)

  - **Resource Leaks & Error Handling (Priority 2)**
    - Added RAII ProgressGuard for progress bars (auto-cleanup on any exit path)
    - Capture stderr from git/aria2c for better error messages
    - Added recipe lock to resolve() function to prevent concurrent resolution

  - **Edge Case Handling (Priority 3)**
    - Fixed URL fragment/query string handling in extract_repo_name()
    - Added depth parameter validation (1-1000000) in git_clone_depth()
    - Added validation that cloned repo is complete before skipping (git rev-parse)
    - Fixed .unwrap() panics on non-UTF8 paths (now returns error)

  - **Rhai Recipe Fixes (Priority 4)**
    - Added ROCKY_VERSION env override to rocky.rhai
    - Added LINUX_REF env override to linux.rhai
    - Added fallback HTTP download if torrent fails in rocky.rhai

  - **Tests Added**
    - URL validation tests for git.rs and torrent.rs
    - SSH URL parsing test
    - Fragment/query string handling tests
    - Depth validation tests
    - Filename sanitization tests (magnet, empty, dot, dotdot)
