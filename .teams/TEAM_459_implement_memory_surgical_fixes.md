# TEAM_459: Implement Memory Surgical Fixes

## Objective

Implement the three surgical fixes recommended by TEAM_458 review to prevent future memory management bugs like TEAM_455 and TEAM_456.

## Fixes Implemented

1. **Warning Comment**: Added prominent warning to `map_user_page()` about VMA tracking requirement
2. **Debug Assertion**: Added `verify_ttbr0_consistency()` at syscall entry (debug builds only)
3. **Documentation**: Added GOTCHA #37 (ttbr0/CR3 sync) and GOTCHA #38 (VMA tracking)

## Progress Log

### Session 1 (2026-01-12)
- Started implementation based on TEAM_458 review
- Decided against making `map_user_page()` private due to circular dependency issues (mm crate can't depend on sched crate where TCB lives)
- Added warning comment to `map_user_page()` in `mm/src/user/mapping.rs`
- Added `verify_ttbr0_consistency()` function in `syscall/src/lib.rs`
- Added GOTCHA #37 and #38 to `docs/GOTCHAS.md`
- Fixed aarch64 build issues:
  - Added cfg attributes to fork.rs debug logging (x86_64-specific register names)
  - Fixed ttbr0 AtomicUsize loading in main.rs signal handler
- Verified both x86_64 and aarch64 builds pass

## Files Modified

| File | Change |
|------|--------|
| `mm/src/user/mapping.rs` | Added warning comment to `map_user_page()` |
| `syscall/src/lib.rs` | Added `verify_ttbr0_consistency()` debug assertion |
| `docs/GOTCHAS.md` | Added GOTCHA #37 and #38 |
| `sched/src/fork.rs` | Fixed aarch64 build (cfg attributes on debug logs) |
| `levitate/src/main.rs` | Fixed ttbr0 AtomicUsize loading |

## Key Decisions

1. **Warning comment vs API restriction**: Making `map_user_page()` private would require the mm crate to depend on sched crate (for TCB access), creating circular dependencies. A warning comment is simpler and achieves the same goal of alerting developers.

2. **Debug-only assertion**: The ttbr0 consistency check only runs in debug builds to avoid runtime overhead in release.

3. **Two GOTCHA entries**: Created separate entries for the two different bug patterns:
   - GOTCHA #37: Forgetting to update task.ttbr0 after CR3 switch
   - GOTCHA #38: Forgetting to track VMAs when mapping pages

## Handoff Notes

The surgical fixes are complete and both architectures build. Future teams should:
- Read GOTCHA #37 and #38 before working on memory management
- Use debug builds to catch ttbr0 desync issues early
- Check the warning comment on `map_user_page()` before using it directly
