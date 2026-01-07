# Phase 5: I/O (writev/readv) — COMPLETE

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222 (Updated by TEAM_228)  
**Status**: **COMPLETE**  
**Priority**: P1 — Required for `println!`

## Status Update (TEAM_228)
**Fully implemented by TEAM_217.**

### Kernel Implementation
- `sys_writev` — `kernel/src/syscall/fs/write.rs` line 19
- `sys_readv` — `kernel/src/syscall/fs/read.rs` (presumed based on structure)
- Syscall dispatch wiring — `kernel/src/syscall/mod.rs` lines 301-310

### Userspace Implementation  
- `writev(fd, iov)` — `libsyscall/src/lib.rs` lines 111-127
- `readv(fd, iov)` — `libsyscall/src/lib.rs` lines 129-145
- `IoVec` struct — lines 152-157
- `SYS_WRITEV = 66`, `SYS_READV = 65` — lines 148-150

## Deliverables
- [x] Kernel `sys_writev` handler (TEAM_217)
- [x] Kernel `sys_readv` handler (TEAM_217)
- [x] Syscall dispatch wiring (TEAM_217)
- [x] Userspace wrappers (TEAM_217)

No further work required for Phase 5.
