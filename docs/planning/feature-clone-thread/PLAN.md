# Feature: Full sys_clone Thread Creation

**SSOT**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Planning  
**Priority**: P1 — Required for `std::thread`

## Purpose
Implement full Linux-compatible `sys_clone` syscall to support creating threads
that share address space with the parent. This is required for Rust `std::thread::spawn`.

## Phase Summary

| Phase | Name | Status |
|-------|------|--------|
| 1 | Discovery | In Progress |
| 2 | Design | Not Started |
| 3 | Implementation | Not Started |
| 4 | Testing | Not Started |
| 5 | Polish | Not Started |

## Key Files

### Kernel
- `kernel/src/task/mod.rs` — TaskControlBlock, switch_to
- `kernel/src/task/user.rs` — UserTask, Pid
- `kernel/src/task/process.rs` — spawn_from_elf
- `kernel/src/task/scheduler.rs` — SCHEDULER, add_task
- `kernel/src/syscall/process.rs` — sys_clone (stub)
- `kernel/src/arch/aarch64/task.rs` — Context, TPIDR_EL0

### Userspace
- `userspace/libsyscall/src/lib.rs` — clone wrapper

## Dependencies
- Existing futex implementation (TEAM_208)
- Existing TPIDR_EL0 context switch (TEAM_217)
- Existing mmap implementation (TEAM_228)
