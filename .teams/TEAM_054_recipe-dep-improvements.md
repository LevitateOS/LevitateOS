# TEAM_054: Recipe Dependency Management Improvements

## Overview
Implementing comprehensive dependency management improvements for the recipe package manager based on the review in `recipe/RECIPE_DEPENDENCY_MANAGEMENT.md`.

## Phases
1. Source checksums (verify_sha256 enhancement, verify_sha512, verify_blake3, `recipe hash`)
2. Reverse dependency tracking (reverse_deps(), --force on remove)
3. Dry run mode (--dry-run flag)
4. Version constraints (semver, version.rs)
5. Atomic installation (staging + rollback)
6. Orphan detection (installed_as_dep tracking)
7. Query commands (tree, why, impact, --graph)
8. Lock file (lockfile.rs, --locked flag)

## Progress
- [x] Phase 1: Source checksums
- [x] Phase 2: Reverse dependency tracking
- [x] Phase 3: Dry run mode
- [x] Phase 4: Version constraints
- [x] Phase 5: Atomic installation (basic rollback on failure)
- [x] Phase 6: Orphan detection
- [x] Phase 7: Query commands
- [x] Phase 8: Lock file

## Decisions
- Following the implementation order from the review (impact/effort ratio)
- Using semver crate for version parsing
- Lock file format: TOML
- Hash algorithms: sha256, sha512, blake3

## Files Modified
- `recipe/Cargo.toml` - Added serde, sha3, blake3, toml dependencies
- `recipe/src/helpers/acquire.rs` - Added verify_sha512(), verify_blake3(), compute_hashes()
- `recipe/src/helpers/mod.rs` - Registered new verification functions
- `recipe/src/core/deps.rs` - Added reverse_deps(), find_orphans(), version constraint validation
- `recipe/src/core/version.rs` - **NEW** Version constraint parsing with semver
- `recipe/src/core/lockfile.rs` - **NEW** Lock file read/write/validate
- `recipe/src/core/lifecycle.rs` - Added rollback on install failure
- `recipe/src/core/mod.rs` - Exported new modules
- `recipe/src/lib.rs` - Exported lockfile module
- `recipe/src/bin/recipe.rs` - Added many new commands and flags

## New CLI Commands
- `recipe hash <file>` - Compute sha256/sha512/blake3 hashes
- `recipe install --dry-run` - Show what would be installed
- `recipe install --locked` - Verify against lock file
- `recipe remove --force` - Remove even with dependents
- `recipe orphans` - List orphaned packages
- `recipe autoremove --yes` - Remove orphans
- `recipe tree <pkg>` - Show dependency tree
- `recipe why <pkg>` - Show what depends on package
- `recipe impact <pkg>` - Show what would break on removal
- `recipe lock update` - Generate lock file
- `recipe lock show` - Show lock file contents
- `recipe lock verify` - Verify recipes match lock
