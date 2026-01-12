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

### Session 2 (2026-01-12)
- Investigated BusyBox ash shell not printing prompt
- Root cause: Shell stuck in job control loop
  - Ash calls `setsid()` to become session leader → pgid changes to its own PID
  - Then checks `TIOCGPGRP` (returns `FOREGROUND_PID=1`) vs `getpgid(0)` (returns 5)
  - Mismatch causes shell to think it's not in foreground → sends SIGTTOU, loops
- Fix 1: Set `FOREGROUND_PID` when init spawns (in `init.rs`)
- Fix 2: Set `FOREGROUND_PID` when process acquires controlling terminal via `TIOCSCTTY`
  - This is the key fix: when shell calls `setsid()` then `TIOCSCTTY`, we now update
    `FOREGROUND_PID` to the shell's pgid
- Also fixed keyboard input routing: characters now fed to `CONSOLE_TTY` from interrupt handler
- Shell prompt now displays and shell accepts input!

## Files Modified (Session 2)

| File | Change |
|------|--------|
| `levitate/src/init.rs` | Set `FOREGROUND_PID` when init spawns |
| `syscall/src/fs/fd.rs` | Implement `TIOCSCTTY` to set foreground pgid |
| `levitate/src/input.rs` | Feed keyboard input to `CONSOLE_TTY` from interrupt handler |

## Key Decisions (Session 2)

1. **TIOCSCTTY sets foreground pgid**: When a process acquires the controlling terminal via
   `TIOCSCTTY`, it becomes the foreground process group. This allows shells that call `setsid()`
   before checking job control to work correctly.

2. **Keyboard input fed directly to TTY**: Characters are fed to `CONSOLE_TTY` from the VirtIO
   input interrupt handler, not just buffered in `KEYBOARD_BUFFER`. This ensures TTY receives
   input even when no process is actively reading.

## Gotchas Discovered

- **Job control and session leaders**: Shells typically call `setsid()` to become session leaders,
  which changes their pgid. The TTY's foreground pgid must be updated when the shell acquires
  the controlling terminal, otherwise the shell thinks it's in the background.

## Handoff Notes

The surgical fixes are complete and both architectures build. BusyBox ash shell now prints
prompt and waits for input. Future teams should:
- Read GOTCHA #37 and #38 before working on memory management
- Use debug builds to catch ttbr0 desync issues early
- Check the warning comment on `map_user_page()` before using it directly
- Understand the TTY/session/pgid relationship when debugging shell issues
