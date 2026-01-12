# TEAM_453: Bugfix - BusyBox Init Immediately Shuts Down

## Status: FIXED ✅

## Bug Report

**Symptom:** BusyBox init starts successfully but immediately triggers system shutdown instead of starting ash shell.

**Expected:** BusyBox init should read `/etc/inittab`, execute sysinit scripts, and spawn ash shell.

**Actual:** Init process starts, then system immediately shuts down.

## Root Cause Analysis

Used syscall tracing to identify the exact issue:

```
[SYSCALL] PID=1 nr=39 (getpid) -> 1      ✓ PID is correct
[SYSCALL] PID=1 nr=169 (reboot) args=[0xfee1dead, 0x28121969, 0x0]
[SHUTDOWN] Initiating graceful shutdown...
```

**The bug:** BusyBox init calls `reboot(LINUX_REBOOT_CMD_CAD_OFF)` (cmd=0x0) to disable Ctrl+Alt+Del - this is **normal init behavior**, NOT a shutdown request. But our `sys_shutdown` was treating ALL reboot syscalls as shutdown requests, ignoring the `cmd` argument.

## Fix Applied

### 1. Fixed reboot syscall (`crates/kernel/syscall/src/sys.rs`)
- Added Linux reboot command constants (CAD_OFF, CAD_ON, HALT, RESTART, POWER_OFF)
- Changed `sys_shutdown` to check the `cmd` argument (arg2, not arg0)
- CAD_OFF/CAD_ON now return success without shutting down
- Only HALT/RESTART/POWER_OFF actually trigger shutdown

### 2. Fixed syscall dispatch (`crates/kernel/syscall/src/lib.rs`)
- Changed from `frame.arg0()` to `frame.arg2()` for reboot cmd

### 3. Added vfork syscall (`crates/kernel/arch/x86_64/src/lib.rs`)
- Added Vfork syscall number (58 on x86_64)
- Implemented as fork semantics (safer than CLONE_VM without proper parent blocking)

### 4. Added waitpid(-1) support (`crates/kernel/syscall/src/process/lifecycle.rs`)
- BusyBox init uses `waitpid(-1, ...)` to wait for any child
- Added `try_wait_any()` to process_table

### 5. Added /dev/console device (from previous session)
- Created console device in devtmpfs for BusyBox init I/O

## Files Modified

- `crates/kernel/syscall/src/sys.rs` - Fixed reboot command handling
- `crates/kernel/syscall/src/lib.rs` - Fixed arg passing, added vfork handler
- `crates/kernel/arch/x86_64/src/lib.rs` - Added Vfork syscall number
- `crates/kernel/syscall/src/process/lifecycle.rs` - Added waitpid(-1) support
- `crates/kernel/sched/src/process_table.rs` - Added try_wait_any()
- `crates/kernel/fs/devtmpfs/src/devices/console.rs` - New console device
- `crates/kernel/fs/devtmpfs/src/devices/mod.rs` - Console registration
- `crates/kernel/fs/devtmpfs/src/lib.rs` - Console device creation
- `crates/kernel/sched/src/fd_table.rs` - Added new_shared_fd_table_with_stdio()
- `crates/kernel/levitate/src/init.rs` - Use fd table with stdio for init

## Verification

- ✅ Build succeeds
- ✅ All 19 workspace tests pass
- ✅ Behavior test passes - system no longer shuts down immediately
- ✅ Fork working - child PID=2 created from init
- ✅ Golden logs auto-updated (SILVER mode)

## Remaining Work

BusyBox init now runs and can fork child processes. For full ash shell support:
- Child process needs to successfully exec `/bin/ash`
- Console I/O needs keyboard input (currently read returns EOF)
- May need additional syscalls for full shell functionality
