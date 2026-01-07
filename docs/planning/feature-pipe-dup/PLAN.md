# Feature: Process Orchestration (pipe2, dup)

**SSOT**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Planning  
**Priority**: P2 — Required for `std::process::Command`

## Purpose
Implement `sys_pipe2`, `sys_dup`, and `sys_dup3` syscalls to enable process I/O redirection 
and piping. This is Phase 6 of the std-support plan.

## Phase Summary

| Phase | Name | Status |
|-------|------|--------|
| 1 | Discovery | Complete |
| 2 | Design | Complete |
| 3 | Implementation | Complete |
| 4 | Testing | Not Started |
| 5 | Polish | Not Started |

## Key Files

### Kernel
- `kernel/src/syscall/fs/mod.rs` — Syscall dispatch
- `kernel/src/fs/pipe.rs` — (NEW) Pipe implementation
- `kernel/src/fs/vfs/fd.rs` — File descriptor operations

### Userspace  
- `userspace/libsyscall/src/lib.rs` — Syscall wrappers

## Dependencies
- Existing VFS and file descriptor table
- Futex for blocking operations (exists)
- mmap (exists, for pipe buffer if needed)

## References
- Linux pipe2(2), dup(2), dup2(2), dup3(2) man pages
- `std-support/phase-6.md` for detailed UoW breakdown
