# TEAM_356: Fix Noisy Syscall Trace Logging

**Status:** ✅ RESOLVED  
**Bug:** Golden log flooded with `[SYSCALL] set_return: rax` messages  
**Date:** 2025-01-09

## Symptom

The `tests/golden_boot_x86_64.txt` file contained 725 lines with hundreds of interleaved syscall trace messages like:
```
[INIT] Shell spawned as PID [SYSCALL] set_return: rax <- 0x1c
2[SYSCALL] set_return: rax <- 0x1
```

Output was unreadable due to race conditions between shell output and trace logging.

## Root Cause

`crates/kernel/src/arch/x86_64/mod.rs:466` contained:
```rust
log::trace!("[SYSCALL] set_return: rax <- 0x{:x}", value);
```

This `log::trace!` fired for **every syscall return**. Since behavior tests use `--features verbose` (which sets log level to `Trace`), every single syscall return was logged, flooding output.

## Fix

Removed the trace! call - it was too verbose even for debug builds (hundreds of calls per second).

## Verification

| Metric | Before | After |
|--------|--------|-------|
| Golden log lines | 725 | 132 |
| Behavior test | ✅ Pass | ✅ Pass |

## Files Changed

- `crates/kernel/src/arch/x86_64/mod.rs` - Removed trace! in `set_return()`

## Handoff Checklist

- [x] Project builds cleanly
- [x] Behavior tests pass
- [x] Golden logs clean and readable
- [x] Team file created
