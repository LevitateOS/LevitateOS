# CLONE_FILES: File Descriptor Table Sharing

**Status**: ✅ IMPLEMENTED (TEAM_443)  
**Priority**: HIGH - Blocks brush shell (Tokio multi-threaded runtime)  
**Discovered by**: TEAM_442  
**Implemented by**: TEAM_443 (2026-01-12)  
**Related**: Brush crash investigation, clone syscall

---

## Problem Statement

When `clone()` is called with `CLONE_FILES` flag (0x400), the child thread should **share** the parent's file descriptor table. Currently, LevitateOS creates a **new** fd table for the child instead.

### Impact

This breaks Tokio's multi-threaded async runtime:

1. Parent creates epoll fd, eventfd, socketpair before `clone()`
2. `clone()` creates child with `CLONE_FILES` (expects shared fd table)
3. Child gets a NEW empty fd table (only stdin/stdout/stderr)
4. Child tries to use epoll/eventfd/socketpair fds → they don't exist
5. Tokio runtime fails, brush shell crashes

### Evidence

From syscall trace (TEAM_442):
```
nr=291 (epoll_create1) → result=3
nr=290 (eventfd2) → result=4
nr=53  (socketpair) → result=0 (creates fds 6,7)
nr=56  (clone) flags=0x13d0f00 → includes CLONE_FILES (0x400)
```

The child thread should have access to fds 3, 4, 6, 7 but currently gets a fresh table.

---

## Current Implementation

### File: `crates/kernel/sched/src/fd_table.rs`

```rust
/// TEAM_168: Thread-safe wrapper for FdTable.
pub type SharedFdTable = IrqSafeLock<FdTable>;

/// TEAM_168: Create a new shared fd table.
pub fn new_shared_fd_table() -> SharedFdTable {
    IrqSafeLock::new(FdTable::new())
}
```

**Problem**: `SharedFdTable` is `IrqSafeLock<FdTable>`, not `Arc<IrqSafeLock<FdTable>>`. This means each task owns its fd table directly - it cannot be shared.

### File: `crates/kernel/sched/src/thread.rs` (line ~129)

```rust
// TEAM_230: For MVP, threads get their own fd table
// TODO(TEAM_230): Share fd_table when CLONE_FILES is set
fd_table: fd_table::new_shared_fd_table(),
```

---

## Proposed Solution

### Step 1: Change SharedFdTable to use Arc

**File**: `crates/kernel/sched/src/fd_table.rs`

```rust
// BEFORE:
pub type SharedFdTable = IrqSafeLock<FdTable>;

// AFTER:
pub type SharedFdTable = Arc<IrqSafeLock<FdTable>>;

pub fn new_shared_fd_table() -> SharedFdTable {
    Arc::new(IrqSafeLock::new(FdTable::new()))
}
```

### Step 2: Update TaskControlBlock

**File**: `crates/kernel/sched/src/lib.rs`

The `fd_table` field type changes from `IrqSafeLock<FdTable>` to `Arc<IrqSafeLock<FdTable>>`. This is a transparent change for most code since `.lock()` still works.

### Step 3: Share fd_table in clone_thread when CLONE_FILES is set

**File**: `crates/kernel/sched/src/thread.rs`

```rust
// In create_thread():
let fd_table = if flags & CLONE_FILES != 0 {
    // Share parent's fd table
    current_task().fd_table.clone()  // Arc::clone, cheap
} else {
    // Create new fd table (fork semantics - copy fds)
    fd_table::new_shared_fd_table()
};
```

### Step 4: Pass flags to create_thread

Currently `create_thread` doesn't receive clone flags. Update signature:

```rust
pub fn create_thread(
    parent_ttbr0: usize,
    child_stack: usize,
    child_tls: usize,
    clear_child_tid: usize,
    clone_flags: u32,  // ADD THIS
    tf: &SyscallFrame,
) -> Result<Arc<TaskControlBlock>, ThreadError>
```

Update caller in `clone_thread()` to pass flags.

---

## Files to Modify

| File | Change |
|------|--------|
| `crates/kernel/sched/src/fd_table.rs` | Change `SharedFdTable` to `Arc<IrqSafeLock<FdTable>>` |
| `crates/kernel/sched/src/lib.rs` | Update `TaskControlBlock.fd_table` type |
| `crates/kernel/sched/src/thread.rs` | Add flags param, share fd_table when CLONE_FILES |
| `crates/kernel/sched/src/user.rs` | Update `UserTask` if it uses SharedFdTable |
| `crates/kernel/sched/src/process.rs` | Update process spawn if it uses SharedFdTable |

---

## Testing

### Manual Test

1. Run `cargo xtask run --term`
2. Brush shell should start without crashing
3. Child thread should be able to use epoll/eventfd

### Verification Points

- [ ] `cargo xtask build kernel` compiles for x86_64
- [ ] `cargo xtask build kernel --arch aarch64` compiles for aarch64
- [ ] `cargo test --workspace` passes
- [ ] `cargo xtask test behavior` passes
- [ ] Brush shell starts and shows prompt

---

## Implementation Notes

### Why Arc?

`Arc` (Atomic Reference Counting) allows multiple owners of the same data:
- Parent and child both hold `Arc<IrqSafeLock<FdTable>>`
- They share the same underlying `FdTable`
- When both exit, refcount drops to 0 and fd table is freed

### CLONE_FILES Flag

From Linux `include/uapi/linux/sched.h`:
```c
#define CLONE_FILES  0x00000400  /* set if open files shared between processes */
```

### What About Fork?

Fork (`clone()` without `CLONE_VM`) should **copy** the fd table, not share it. The current implementation is actually correct for fork - it creates a new fd table. The issue is only with threads (CLONE_VM | CLONE_FILES).

---

## Related Issues

- **CLONE_SIGHAND**: Signal handlers should also be shared (similar pattern)
- **CLONE_FS**: Filesystem info (cwd, umask) should be shared
- These can be addressed with the same Arc pattern

---

## References

- `cargo xtask syscall fetch clone` - Full clone specification
- `.teams/TEAM_442_investigate_brush_post_clone_crash.md` - Investigation log
- `docs/specs/syscalls/clone.md` - Clone syscall reference
