# TEAM_031: Rhai Recipe System Rewrite

## Status: COMPLETE

## Objective
Rewrite the recipe system from S-expression data that the executor interprets to Rhai scripts that ARE code. The executor just runs them.

## Problem with Old Design
- S-expression recipes are DATA that the executor INTERPRETS
- Executor has hardcoded logic for each acquire/build/install type
- Adding new capabilities requires modifying executor code

## New Design
- Recipes are CODE (Rhai scripts)
- Executor provides helper functions and calls `acquire()`, `build()`, `install()`
- All logic is in the recipe - infinite extensibility

## Files Changed
- `recipe/Cargo.toml` - Updated deps to use `rhai`
- `recipe/src/lib.rs` - Export engine module
- `recipe/src/engine.rs` - Rhai engine + registered functions
- `recipe/src/bin/recipe.rs` - CLI
- `recipe/examples/lib/rpm.rhai` - RPM helper module
- `recipe/examples/lib/github.rhai` - GitHub release helper module
- `recipe/examples/bash.rhai` - Example RPM-based recipe
- `recipe/examples/ripgrep.rhai` - Example binary download recipe

## Implementation Log

### Step 0: Wipe old code
- Deleted all old S-expression parser/executor code
- Fresh start with minimal Rhai-based design

### Step 1: Add Rhai dependency
- Added `rhai = { version = "1.19", features = ["sync"] }`

### Step 2: Create engine.rs
- RecipeEngine struct with Rhai engine
- Registered all helper functions
- execute() method calls acquire/build/install

### Step 3: Helper function implementations
- download, copy, verify_sha256
- extract, cd, run
- install_bin, install_lib, install_man, rpm_install

### Step 4: CLI
- Simple CLI that takes recipe path and runs it

## Decisions Made
- Using Rhai "sync" feature for thread safety
- Helper functions use Command to shell out
- Module resolver for lib/ imports

## Verification
- `cargo build` - succeeds
- `cargo test` - 2 tests pass
- `cargo run -- run examples/ripgrep.rhai --prefix /tmp/test` - installs rg binary + man page
- `cargo run -- run examples/fd.rhai --prefix /tmp/test` - installs fd binary + man page
