# TEAM_118: Implement recipe resolve (leviso-deps replacement)

## Status: COURSE CORRECTED

## Goal
Replace `leviso-deps` with `recipe resolve` subcommand as the dependency resolver for leviso's ISO build process.

## Key Insight (CRITICAL)
**Recipe is a package-agnostic executor.** Package details belong in `.rhai` files (living data), NOT in Rust code.

Wrong approach (reverted):
- Hardcoding Rocky/Linux/tools details in `core/resolve/*.rs`
- Creating `BuildDependencies` struct with package-specific methods

Correct approach (to be implemented):
- Create `recipes/build/rocky-iso.rhai` with `fn resolve()` containing URLs, checksums
- Create `recipes/build/linux-kernel.rhai` with `fn resolve()` for git clone
- leviso calls `engine.resolve("recipes/build/rocky-iso.rhai")`
- Recipe Rust code stays package-agnostic

## Completed
- [x] Add agnostic helpers (disk space, GitHub releases, stall detection, checksum progress)

## Reverted
- [x] `core/resolve/` module with hardcoded package details
- [x] `BuildDependencies` struct exports
- [x] `BuildDeps` CLI subcommand

## Still TODO
- [ ] Create `recipes/build/rocky-iso.rhai` (Rocky 10.1, living data)
- [ ] Create `recipes/build/linux-kernel.rhai`
- [ ] Create `recipes/build/recstrap.rhai`, etc.
- [ ] Update leviso to call `engine.resolve()` on these recipes

## Progress Log

### 2026-01-25: Course correction
- Initially implemented hardcoded resolvers in Rust (WRONG)
- User caught the mistake: "Recipe is an executor, not a resolver with hardcoded logic"
- Reverted package-specific code, kept agnostic helpers
- Committed: agnostic download helpers (disk.rs, GitHub releases, etc.)
