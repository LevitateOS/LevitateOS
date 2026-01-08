# Phase 1: Discovery

**Parent**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Complete

## Feature Summary
Implement full Linux-compatible `sys_clone` to create threads sharing address space.
This is P1 priority — required for `std::thread::spawn` to work.

## Success Criteria
1. Clone syscall creates new thread sharing `ttbr0` (address space)
2. Child starts execution at provided stack with correct TLS
3. Parent receives child TID, child receives 0
4. CLONE_CHILD_CLEARTID wakes futex on exit
5. std::thread::spawn basic usage works

## Current State Analysis

### What Works Today
- Process creation via `spawn_from_elf` and `spawn_from_elf_with_args`
- Each process gets its own page table (`ttbr0`)
- Context switch saves/restores TPIDR_EL0 (TEAM_217)
- Futex wait/wake implemented (TEAM_208)
- `sys_clone` stub exists (returns ENOSYS)
- `sys_set_tid_address` implemented (TEAM_228)
- `clear_child_tid` field in TCB (TEAM_228)

### What's Missing
1. Thread creation (clone in same address space)
2. Setting up child context to run at provided stack
3. Handling CLONE_SETTLS to set child's TPIDR_EL0
4. Handling CLONE_PARENT_SETTID, CLONE_CHILD_SETTID
5. Thread exit clearing tid address and waking futex

## Codebase Reconnaissance

### Key Structures

| Structure | Location | Role |
|-----------|----------|------|
| `TaskControlBlock` | `task/mod.rs` | Main TCB with state, context, ttbr0 |
| `Context` | `arch/aarch64/task.rs` | CPU state (x19-x29, sp, lr, tpidr_el0) |
| `UserTask` | `task/user.rs` | User process creation helper |
| `Pid` | `task/user.rs` | PID generation |
| `Scheduler` | `task/scheduler.rs` | Ready queue, task scheduling |

### Key Functions

| Function | Location | Role |
|----------|----------|------|
| `UserTask::new` | `task/user.rs` | Create UserTask with kernel stack |
| `spawn_from_elf_with_args` | `task/process.rs` | Full process spawn |
| `switch_to` | `task/mod.rs` | Context switch to new task |
| `SCHEDULER.add_task` | `task/scheduler.rs` | Add task to ready queue |
| `cpu_switch_to` | asm | Assembly context switch |

### Extension Points
1. **For thread creation**: Similar to `spawn_from_elf` but:
   - Reuse parent's `ttbr0` (share address space)
   - Use provided stack instead of allocating new one
   - No ELF loading needed

2. **For context setup**: Similar to `Context::new` but:
   - Set custom entry point (child function)
   - Set custom stack pointer (provided by clone)
   - Set TPIDR_EL0 (if CLONE_SETTLS)

## Constraints
- Must maintain Linux ABI compatibility
- Clone flags determine sharing behavior
- Thread exit must clear tid and wake futex (for join)
- No userspace stack allocation in kernel — caller provides

## Affected Tests
- Manual testing in QEMU with clone test program
- Future: std::thread::spawn once full std support complete
