# Phase 3: Implementation

**Parent**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Complete (TEAM_230)

## Implementation Overview

### Files to Create/Modify

| File | Change Type | Description |
|------|-------------|-------------|
| `kernel/src/task/mod.rs` | MODIFY | Add `create_thread()` function |
| `kernel/src/task/user.rs` | MODIFY | Add `UserThread` or reuse `UserTask` |
| `kernel/src/syscall/process.rs` | MODIFY | Replace sys_clone stub |
| `kernel/src/task/mod.rs` | MODIFY | Update task_exit for CLEARTID |

### Order of Implementation
1. Step 1: Thread creation infrastructure
2. Step 2: sys_clone core logic
3. Step 3: Thread exit handling

---

## Step 1: Thread Creation Infrastructure

### UoW 1.1: Add `create_thread` function

**File**: `phase-3-step-1-uow-1.md`

**Objective**: Create function to spawn a new thread in same address space.

**Tasks**:
1. Add function to `kernel/src/task/mod.rs`:
```rust
pub fn create_thread(
    parent_ttbr0: usize,
    child_stack: usize,
    child_entry: usize,
    tls: usize,
    clear_child_tid: usize,
) -> Result<Arc<TaskControlBlock>, ThreadError>
```

2. Implementation:
   - Allocate kernel stack (16KB) for new thread
   - Create new PID via `Pid::next()`
   - Create Context with:
     - `sp` = kernel_stack_top
     - `lr` = thread_entry_wrapper
     - `tpidr_el0` = tls
   - Create TCB with:
     - `ttbr0` = parent_ttbr0 (shared!)
     - `user_sp` = child_stack
     - `user_entry` = child_entry (or 0, handled by caller)
     - `clear_child_tid` = provided address

3. Define `ThreadError` enum:
```rust
pub enum ThreadError {
    AllocationFailed,
}
```

**Exit Criteria**: `create_thread()` compiles.

---

### UoW 1.2: Verify thread entry works

**File**: `phase-3-step-1-uow-2.md`

**Objective**: Verify existing entry mechanisms work for threads.

**Tasks**:
1. Verify `user_task_entry_wrapper` in `task/mod.rs` works for threads:
   - Switches TTBR0 ✓ (already shares parent's)
   - Enters user mode at `user_entry`, `user_sp` ✓

2. For clone, child needs to "return 0":
   - Set `user_entry` = return address on child_stack
   - Set x0 = 0 when entering user mode

3. **Verification**: The existing `enter_user_mode` at `arch/aarch64/task.rs:38` already
   clears x0 to 0 (`mov x0, xzr`). **No new function is needed.**

**Exit Criteria**: Confirm `enter_user_mode` clears x0 (already verified by TEAM_229).

---

## Step 2: Implement sys_clone

### UoW 2.1: Replace sys_clone stub

**File**: `phase-3-step-2-uow-1.md`

**Objective**: Implement full sys_clone for thread creation.

**Tasks**:
1. Parse clone flags
2. Validate: require CLONE_VM | CLONE_THREAD for thread case
3. Get parent task's ttbr0
4. Call `create_thread()`
5. Handle flags:
   - `CLONE_SETTLS`: Set child's tpidr_el0 = tls
   - `CLONE_PARENT_SETTID`: Write child TID to *parent_tid
   - `CLONE_CHILD_SETTID`: Write child TID to *child_tid
   - `CLONE_CHILD_CLEARTID`: Store child_tid in TCB
6. Add child to scheduler
7. Return child TID to parent

```rust
pub fn sys_clone(...) -> i64 {
    let parent = current_task();
    
    // For threads, use parent's address space
    let child = create_thread(
        parent.ttbr0,
        child_stack,
        0, // entry: child returns to its stack
        tls,
        if flags & CLONE_CHILD_CLEARTID != 0 { child_tid } else { 0 },
    )?;
    
    let child_pid = child.id.0;
    
    // CLONE_PARENT_SETTID: write child TID to parent's user memory
    if flags & CLONE_PARENT_SETTID != 0 && parent_tid != 0 {
        // Write child_pid to *parent_tid in user space
    }
    
    // CLONE_CHILD_SETTID: write child TID to child's user memory
    if flags & CLONE_CHILD_SETTID != 0 && child_tid != 0 {
        // Write child_pid to *child_tid (shared address space)
    }
    
    SCHEDULER.add_task(child);
    
    child_pid as i64
}
```

**Exit Criteria**: sys_clone creates thread and returns child TID.

---

## Step 3: Thread Exit Handling

### UoW 3.1: Update task_exit for CLEARTID

**File**: `phase-3-step-3-uow-1.md`

**Objective**: Clear tid address and wake futex on thread exit.

**Current Code** (in `task/mod.rs:24-44`):
```rust
pub extern "C" fn task_exit() -> ! {
    let task = current_task();
    task.set_state(TaskState::Exited);
    scheduler::SCHEDULER.schedule();
    // ...wfi loop
}
```

**Tasks**:
1. In `task_exit()` (task/mod.rs), **before** `set_state(Exited)`, add:
```rust
let task = current_task();
let clear_tid = task.clear_child_tid.load(Ordering::Acquire);
if clear_tid != 0 {
    // SAFETY: user_va_to_kernel_ptr verified the address is mapped
    // and belongs to this task's address space (shared via CLONE_VM).
    if let Some(ptr) = mm_user::user_va_to_kernel_ptr(task.ttbr0, clear_tid) {
        unsafe { *(ptr as *mut i32) = 0; }
    }
    // Wake one waiter on futex(clear_tid) for thread join
    sync::futex_wake(clear_tid, 1);
}
```

2. Required imports:
```rust
use crate::memory::user as mm_user;
use crate::sync;
use core::sync::atomic::Ordering;
```

3. Test that joining thread wakes up correctly.

**Exit Criteria**: Thread exit clears tid and wakes waiters.

---

## Deliverables

- [ ] `create_thread()` function in `task/thread.rs`
- [ ] Verification that `enter_user_mode` clears x0 (confirmed)
- [ ] Full `sys_clone` implementation
- [ ] Thread exit with CLEARTID handling (modify `task_exit`)
