# Phase 3 — Migration

**Refactor:** Eliminate Type Shims - Make linux-raw-sys the Canonical Source
**Team:** TEAM_420
**Date:** 2026-01-10

---

## Migration Strategy

**"Let the compiler scream. Fix every callsite."**

No shims. No backward compatibility. Break the code, then fix it.

---

## Migration Order

### Step 1: Delete shim modules (cause compiler errors)

```bash
# In syscall/mod.rs:
# 1. Delete entire `pub mod errno { ... }` block (lines ~27-64)
# 2. Delete entire `pub mod fcntl { ... }` block (lines ~66-76)

# In syscall/process/mod.rs:
# 1. Delete all CLONE_* constants (lines ~31-40)

# In syscall/epoll.rs:
# 1. Delete the local EPOLL_CTL_* and EPOLL_CLOEXEC constants
```

### Step 2: Fix errno callsites (~100+ locations)

Every `errno::ENOENT` becomes `-(linux_raw_sys::errno::ENOENT as i64)`.

Files to update:
- `syscall/fs/*.rs` (open, read, write, stat, dir, fd, link, statx)
- `syscall/mm.rs`
- `syscall/process/*.rs` (lifecycle, thread, groups, resources, identity)
- `syscall/epoll.rs`
- `syscall/signal.rs`
- `syscall/sync.rs`
- `syscall/time.rs`
- `syscall/helpers.rs`
- `fs/vfs/error.rs`
- `fs/pipe.rs`

### Step 3: Fix fcntl/AT_* callsites

Every `fcntl::AT_FDCWD` becomes `linux_raw_sys::general::AT_FDCWD`.

Files to update:
- `syscall/fs/open.rs`
- `syscall/fs/stat.rs`
- `syscall/fs/dir.rs`
- `syscall/fs/link.rs`
- `syscall/helpers.rs`

### Step 4: Fix CLONE_* callsites and change signature

In `syscall/process/thread.rs`:
```rust
// Change signature
pub fn sys_clone(flags: u32, ...) {  // was u64
    use linux_raw_sys::general::{CLONE_VM, CLONE_THREAD, ...};
    // flags & CLONE_VM works because both are u32
}
```

In `syscall/mod.rs` dispatcher:
```rust
Some(SyscallNumber::Clone) => process::sys_clone(
    frame.arg0() as u32,  // was u64
    // ...
)
```

### Step 5: Fix EPOLL_* callsites and change signature

In `syscall/epoll.rs`:
```rust
use linux_raw_sys::general::{
    EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD, EPOLL_CLOEXEC,
    EPOLLIN, EPOLLOUT, ...
};

// Change signature
pub fn sys_epoll_ctl(epfd: i32, op: u32, fd: i32, ...) {  // op was i32
```

In `syscall/mod.rs` dispatcher:
```rust
Some(SyscallNumber::EpollCtl) => epoll::sys_epoll_ctl(
    frame.arg0() as i32,
    frame.arg1() as u32,  // was i32
    // ...
)
```

---

## Callsite Inventory

### errno usage (grep for `errno::E`)

| File | Approx Count |
|------|--------------|
| syscall/fs/open.rs | 5 |
| syscall/fs/read.rs | 3 |
| syscall/fs/write.rs | 4 |
| syscall/fs/stat.rs | 2 |
| syscall/fs/dir.rs | 4 |
| syscall/fs/fd.rs | 8 |
| syscall/fs/link.rs | 4 |
| syscall/mm.rs | 10 |
| syscall/process/lifecycle.rs | 5 |
| syscall/process/thread.rs | 3 |
| syscall/process/groups.rs | 2 |
| syscall/process/resources.rs | 3 |
| syscall/epoll.rs | 4 |
| syscall/signal.rs | 2 |
| syscall/sync.rs | 3 |
| syscall/time.rs | 2 |
| syscall/helpers.rs | 5 |
| fs/vfs/error.rs | 27 |
| fs/pipe.rs | 3 |
| **Total** | **~95** |

### fcntl::AT_* usage

| File | Approx Count |
|------|--------------|
| syscall/fs/open.rs | 2 |
| syscall/fs/stat.rs | 1 |
| syscall/fs/dir.rs | 3 |
| syscall/fs/link.rs | 4 |
| syscall/helpers.rs | 2 |
| **Total** | **~12** |

### CLONE_* usage

| File | Count |
|------|-------|
| syscall/process/thread.rs | 6 |
| **Total** | **6** |

### EPOLL_* usage

| File | Count |
|------|-------|
| syscall/epoll.rs | 8 |
| **Total** | **8** |

---

## Rollback Plan

**None needed.** This is a compile-time refactor:
- If it compiles, it works (same constant values)
- If it doesn't compile, fix the errors
- Git provides rollback if needed: `git checkout -- crates/kernel/src/syscall/`

---

## Verification

After migration:
```bash
# Build both architectures
cargo xtask build kernel --arch x86_64
cargo xtask build kernel --arch aarch64

# Grep for shims (should return nothing)
grep -r "as i64\)" crates/kernel/src/syscall/mod.rs
grep -r "as u64\)" crates/kernel/src/syscall/process/mod.rs
```

---

## Exit Criteria for Phase 3

- [ ] All shim modules deleted
- [ ] All callsites fixed with direct linux-raw-sys imports
- [ ] Function signatures updated to match linux-raw-sys types
- [ ] Both architectures compile
- [ ] No type casts in constant definitions
