# Phase 1: Understanding and Scoping

## Bug Summary

BusyBox init starts but immediately triggers system shutdown instead of reading /etc/inittab and spawning ash shell.

**Severity:** Critical - system unusable without working init
**Impact:** Cannot boot into userspace shell

## Reproduction

1. Build with `cargo xtask build all --arch x86_64`
2. Run with `./run-term.sh`
3. System boots, shows "LevitateOS System Ready", then immediately shuts down

## Root Cause

**CONFIRMED:** devtmpfs doesn't create `/dev/console`.

BusyBox init behavior:
1. Opens `/dev/console` for stdin/stdout/stderr (file descriptors 0, 1, 2)
2. If open fails → calls `reboot()` syscall to shutdown
3. Never reaches inittab parsing

Evidence:
- Boot log shows no "LevitateOS (BusyBox) starting..." (first inittab sysinit action)
- `[SHUTDOWN] Initiating graceful shutdown...` appears right after `arch_prctl` (TLS setup)
- devtmpfs only creates: null, zero, full, urandom (no console)

## Fix Strategy

Add `/dev/console` character device to devtmpfs that connects to the kernel's serial/GPU console.

## Files to Modify

1. `crates/kernel/fs/devtmpfs/src/lib.rs` - Add console device creation
2. `crates/kernel/fs/devtmpfs/src/devices/mod.rs` - Add console module
3. `crates/kernel/fs/devtmpfs/src/devices/console.rs` - New file for console device

## Estimated Effort

1 session - straightforward device addition
