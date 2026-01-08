# Phase 2: Design

## Proposed Solution
Create `userspace/levbox/src/bin/clone_test.rs` that works as follows:

1. **Allocates Stack**: A `static` or heap-allocated buffer for the child thread's stack.
2. **Defines Shared State**: A `static AtomicU32` or similar to verify memory sharing.
3. **Defines Child TID**: A location for `CLONE_CHILD_CLEARTID`.
4. **Calls clone()**:
   - Flags: `CLONE_VM | CLONE_THREAD | CLONE_CHILD_CLEARTID` (and `CLONE_SIGHAND` per Linux convention, though strictly optional for us now).
   - Stack: Point to top of allocated stack.
   - Child TID: Address of the TID variable.
5. **Parent Behavior**:
   - Prints "Parent waiting".
   - Calls `futex_wait` on the TID variable (waiting for it to become 0).
   - Prints "Parent woke up".
   - Verifies shared state changed by child.
   - Exits.
6. **Child Behavior**:
   - Prints "Child running".
   - Modifies shared state.
   - Exits (kernel handles clearing TID and waking parent).

## API Design
No new APIs. Uses existing `libsyscall`:
- `clone`
- `futex` (wait)
- `exit`
- `println!`

## Behavioral Decisions
- **Stack Size**: 16KB should be plenty for a simple test.
- **Stack Alignment**: Essential for AArch64. Will use `#[repr(align(16))]` struct for stack.
- **TLS**: Pass 0 (NULL) as we don't have full TLS support in userspace yet.
- **Exit**: Child *must* call `exit()` or raw syscall exit. It cannot simply return from the entry function because there's no runtime `start` wrapper to catch it (unless we manually construct one on the stack).

## Open Questions
- **Q1**: Does `libsyscall::clone` wrap the syscall arguments correctly?
  - **A1**: Yes, verified in `libsyscall/src/lib.rs`.
- **Q2**: How does the child exit?
  - **A2**: It must explicitly call `syscall::exit()`. Returning would pop off the stack into undefined territory.

## Design Alternatives
- **Wait loop**: Parent could spin-loop checking the memory change.
  - **Why not?**: We specifically want to verify `CLONE_CHILD_CLEARTID` + futex logic implemented in the kernel.
