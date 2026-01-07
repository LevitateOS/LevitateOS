# Phase 2: Design

**Parent**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Draft

## Proposed Solution

### High-Level Flow

```
User calls clone(flags, stack, parent_tid, tls, child_tid)
                    │
                    ▼
           ┌───────────────────┐
           │   sys_clone       │
           │ (kernel/syscall/  │
           │  process.rs)      │
           └────────┬──────────┘
                    │
       ┌────────────┴─────────────┐
       ▼                          ▼
  CLONE_VM set?             Fork-style?
  (thread case)             (not supported yet)
       │                          │
       ▼                          ▼
  create_thread()            return ENOSYS
       │
       ▼
  ┌─────────────────────────────────┐
  │ 1. Allocate kernel stack        │
  │ 2. Create new TCB               │
  │    - Copy ttbr0 from parent     │
  │    - New PID/TID                │
  │    - Set user_sp from arg       │
  │ 3. Set up Context               │
  │    - SP = kernel_stack_top      │
  │    - LR = thread_entry_wrapper  │
  │    - TPIDR_EL0 from tls arg     │
  │ 4. Handle TID address flags     │
  │ 5. Add to scheduler             │
  └─────────────────────────────────┘
                    │
       ┌────────────┴─────────────┐
       ▼                          ▼
  Return child TID         Child starts:
  to parent                thread_entry_wrapper
                                  │
                                  ▼
                           enter_user_mode()
                           at provided stack
```

### API Design

```rust
// Kernel syscall handler
pub fn sys_clone(
    flags: u64,
    child_stack: usize,  // User stack for child
    parent_tid: usize,   // Write parent TID here if CLONE_PARENT_SETTID
    tls: usize,          // TLS base if CLONE_SETTLS
    child_tid: usize,    // Address for CHILD_SETTID/CLEARTID
) -> i64
```

### Minimum Viable Flags

For `std::thread` support, minimum flags needed:
- `CLONE_VM` — Share address space (mandatory for threads)
- `CLONE_THREAD` — Share thread group
- `CLONE_SETTLS` — Set TPIDR_EL0 for child
- `CLONE_CHILD_CLEARTID` — Clear tid and wake futex on exit

Other flags (CLONE_FS, CLONE_FILES, CLONE_SIGHAND) can be stubs initially.

## Data Model Changes

### New Function: `create_thread`

> **Location**: `kernel/src/task/thread.rs` (new file)

```rust
// in task/mod.rs or task/thread.rs
pub fn create_thread(
    parent: &Arc<TaskControlBlock>,
    child_stack: usize,
    tls: usize,
    clear_child_tid: usize,
) -> Result<Arc<TaskControlBlock>, ThreadError>
```

### Thread Entry Wrapper

Need a new entry point for threads (different from process entry):

```rust
fn thread_entry_wrapper() -> ! {
    let task = current_task();
    unsafe {
        crate::arch::switch_mmu_config(task.ttbr0);
        crate::arch::enter_user_mode_thread(task.user_entry, task.user_sp);
    }
}
```

Actually, we can reuse `user_task_entry_wrapper` since it already:
1. Switches TTBR0
2. Calls enter_user_mode

Threads just need different `user_entry` (return address on child stack) and `user_sp`.

> [!NOTE]
> **TEAM_229 Review Finding**: The existing `enter_user_mode` at `arch/aarch64/task.rs:38`
> already clears x0 to 0 (`mov x0, xzr`), so no new function is needed. The child
> inherently returns 0 when entering user mode.

## Behavioral Decisions

### Q1: What is the child's entry point?
**Answer**: In Linux clone for threads, the child returns to the caller (returns 0).
The child stack should be set up so that when we `eret` to user mode:
- PC = return address placed on child_stack by caller
- SP = child_stack

For simplicity, we set `user_entry = 0` and `user_sp = child_stack`. The caller
(pthread library) sets up the child stack with the return address at the top.

**Decision**: Set `user_entry` to be read from child's stack (top of stack).

### Q2: How do we handle the clone return in child?
**Return 0 to child, child TID to parent.**

This requires:
- Parent: sys_clone returns normally with child TID
- Child: When scheduled, enters user mode with x0 = 0

The existing `enter_user_mode` clears all registers including x0, so this is already satisfied.

### Q3: Shared fd_table for threads?
**Decision for MVP**: Don't share fd_table yet. Each thread gets its own.
This is technically incorrect for CLONE_FILES, but simplifies implementation.

**TODO**: Future work to share fd_table when CLONE_FILES is set.

### Q4: Thread exit behavior?
When a thread exits:
1. If `clear_child_tid != 0`:
   - Write 0 to `*clear_child_tid`
   - Wake 1 waiter on `futex(clear_child_tid)`
2. Free thread resources (kernel stack)
3. Don't exit process unless last thread

**Decision for MVP**: All threads share process exit behavior.
When any thread calls `exit`, it terminates the whole process.
Proper thread exit tracking requires thread group management (future work).

---

## Implementation Steps

### Step 1: Create thread infrastructure
1. Add `create_thread()` function in `task/mod.rs`
2. Add thread-specific Context initialization

### Step 2: Implement sys_clone core logic
1. Parse flags and validate
2. Call `create_thread()` for thread case
3. Handle TID address features

### Step 3: Update thread exit
1. Modify `task_exit` to check `clear_child_tid`
2. Clear tid and wake futex

### Step 4: Integration test
1. Create userspace clone test
2. Verify parent/child behavior

---

## Open Questions

> **Note**: These questions should be answered before implementation.

### Q5: Stack setup for clone
Linux pthread_create sets up the child stack with:
```
[stack_top]
  return_address   <- child returns here after clone returns 0
  thread_func ptr
  arg ptr
```

Should we require the caller to set this up, or should kernel set entry point?

**Proposed Answer**: Caller sets up stack. Kernel just sets SP and enters user mode.
The return from `clone()` in child will pop return address from stack.

### Q6: What happens if parent dies before child?
**Proposed Answer**: For MVP, don't handle orphan threads. The address space
remains valid as long as any thread using it exists.

**TODO**: Proper reference counting on address space for full implementation.

---

## Verification Plan

### Unit Tests
None — threading requires full kernel integration.

### Integration Test
Create `userspace/clone_test/` that:
1. Allocates a stack (using mmap)
2. Calls clone with CLONE_VM | CLONE_THREAD
3. Child writes to shared memory
4. Parent waits on futex for clear_child_tid
5. Verifies child ran

### Manual Verification
1. Boot kernel in QEMU
2. Run clone_test from shell
3. Observe both threads run
4. Verify process joins correctly
