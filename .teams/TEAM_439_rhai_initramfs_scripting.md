# TEAM_439: Rhai Initramfs Scripting

## Objective
Design and implement a Rhai-based scripting system for declaratively defining initramfs contents, replacing the current hardcoded Rust approach with a flexible, programmable configuration.

## Progress Log

### Session 1 (2026-01-11)
- Identified gaps in current xtask build system
- Fixed critical issues: brush not in `build all`, silent failures, inconsistent dependency handling
- Created `apps.rs` abstraction for external apps
- User requested more flexible approach using Rhai scripting language
- Completed all 5 planning phases

## Key Decisions
1. **Rhai over TOML**: Rhai provides conditional logic, loops, and functions that TOML cannot
2. **Single script file**: `initramfs.rhai` at repo root for all initramfs contents
3. **Fail-fast for required entries**: Missing required components fail immediately with clear errors
4. **Script location**: Repo root for visibility (not xtask/)
5. **BUILD_TYPE conditionals**: Single script with conditionals instead of separate test script

## Gotchas Discovered
- Rhai requires `Arc<Mutex<>>` for builder state accessed from callbacks
- Must pin Rhai version (1.23.x) due to potential breaking changes in ahash dependency

## Remaining Work
- [ ] Implementation (see Phase 3)

## Handoff Notes

**Planning complete.** See `docs/planning/rhai-initramfs-scripting/`:
- `phase-1.md` - Discovery: Problem statement, success criteria
- `phase-2.md` - Design: API design, behavioral decisions, open questions (MOST IMPORTANT)
- `phase-3.md` - Implementation: Step-by-step code changes
- `phase-4.md` - Integration: Test strategy, impact analysis
- `phase-5.md` - Polish: Cleanup, documentation

**Key files to create:**
1. Add `rhai = "1.23"` to `xtask/Cargo.toml`
2. Create `xtask/src/build/initramfs.rs` with Rhai engine
3. Create `initramfs.rhai` at repo root
4. Update `commands.rs` to use new system
5. Remove `apps.rs` after migration

**Open questions to resolve before implementation:**
- See Phase 2, "Must Answer Before Phase 3" section
