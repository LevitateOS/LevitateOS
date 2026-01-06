# Phase 1: Discovery and Safeguards - Syscall Refactor

## Refactor Summary
`kernel/src/syscall.rs` is approximately 850 lines long and handles all system calls for the kernel. This is becoming difficult to maintain and violates **Rule 7 (Modular Refactoring)** regarding file size and responsibility separation.

## Success Criteria
- `syscall.rs` is replaced by a `syscall/` directory.
- Syscall handlers are grouped logically (Process, FS, Memory, etc.).
- No change in observable syscall behavior (Legacy ABI preserved).
- Build succeeds with zero warnings and all tests pass.

## Behavioral Contracts
- **ABI**: Legacy ABI (x8=NR, x0-x5=Args) remains the same.
- **Syscall Numbers**: 0=Read, 1=Write, 2=Exit, 3=GetPid, 4=Sbrk, 5=Spawn, 6=Exec, 7=Yield, 8=Shutdown, 9=Openat, 10=Close, 11=Fstat.

## Golden/Regression Tests
- `tests/golden_boot.txt` (must match after boot).
- Keyboard input test (interactive shell).
- Task spawning and execution.

## Current Architecture Notes
- `syscall.rs` depends on `los_hal`, `crate::task`, `crate::fs`, `crate::input`, `crate::gpu`, etc.
- It is tightly coupled with `task::process` and `task::user_mm`.

## Proposed Logical Splits
- `mod.rs`: Dispatcher, error codes (`errno`), common helper functions.
- `process.rs`: `sys_exit`, `sys_getpid`, `sys_yield`, `sys_spawn`, `sys_exec`.
- `fs.rs`: `sys_read`, `sys_write`, `sys_openat`, `sys_close`, `sys_fstat`.
- `mm.rs`: `sys_sbrk`.
- `sys.rs`: `sys_shutdown`.

## Steps
1. **Step 1 – Map current imports and helpers**.
2. **Step 2 – Create the new directory structure**.
3. **Step 3 – Move code incrementally**.
