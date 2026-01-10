# Phase 2 — Structural Extraction

**Refactor:** Eliminate Type Shims - Make linux-raw-sys the Canonical Source
**Team:** TEAM_420
**Date:** 2026-01-10

---

## Target Design

### Current Structure (WRONG)
```
syscall/
├── mod.rs
│   ├── mod errno { pub const EPERM: i64 = -(raw::EPERM as i64); ... }  // SHIM
│   └── mod fcntl { pub const AT_FDCWD: i32 = raw::AT_FDCWD; ... }      // SHIM
├── process/
│   └── mod.rs { pub const CLONE_VM: u64 = ... as u64; ... }            // SHIM
└── epoll.rs { pub const EPOLL_CTL_ADD: i32 = ... as i32; ... }         // SHIM
```

### Target Structure (CORRECT)
```
syscall/
├── mod.rs
│   └── (NO errno module - use linux_raw_sys::errno directly)
│   └── (NO fcntl module - use linux_raw_sys::general directly)
├── process/
│   └── mod.rs (NO clone constants - use linux_raw_sys::general directly)
├── thread.rs { use linux_raw_sys::general::CLONE_VM; ... }
└── epoll.rs { use linux_raw_sys::general::{EPOLL_CTL_ADD, ...}; ... }
```

---

## Strategy: Delete Shims, Fix Callsites

### Principle
**NO SHIMS. NO WRAPPERS. NO RE-EXPORTS.**

If we need a constant, we import it directly from linux-raw-sys at the use site.

---

## Module Changes

### 1. Delete `errno` module entirely

**Current:**
```rust
pub mod errno {
    use linux_raw_sys::errno as raw;
    pub const EPERM: i64 = -(raw::EPERM as i64);
    // ... 33 more
}
```

**After:**
```rust
// NO MODULE - delete entirely
```

**Callsite change (EVERY error return):**
```rust
// Before
return errno::ENOENT;

// After
return -(linux_raw_sys::errno::ENOENT as i64);
```

### 2. Delete `fcntl` module entirely

**Current:**
```rust
pub mod fcntl {
    use linux_raw_sys::general as raw;
    pub const AT_FDCWD: i32 = raw::AT_FDCWD;
    // ...
}
```

**After:**
```rust
// NO MODULE - delete entirely
```

**Callsite change:**
```rust
// Before
use crate::syscall::fcntl::AT_FDCWD;

// After
use linux_raw_sys::general::AT_FDCWD;
```

### 3. Delete clone constants from `process/mod.rs`

**Current:**
```rust
pub const CLONE_VM: u64 = linux_raw_sys::general::CLONE_VM as u64;
// ... 8 more
```

**After:**
```rust
// NO CONSTANTS - delete entirely
```

**Callsite change in `thread.rs`:**
```rust
// Before
use super::{CLONE_VM, CLONE_THREAD, ...};
pub fn sys_clone(flags: u64, ...) {
    if flags & CLONE_VM != 0 { ... }
}

// After
use linux_raw_sys::general::{CLONE_VM, CLONE_THREAD, ...};
pub fn sys_clone(flags: u32, ...) {  // NOTE: Changed type!
    if flags & CLONE_VM != 0 { ... }
}
```

### 4. Fix epoll constants in `epoll.rs`

**Current:**
```rust
pub const EPOLL_CTL_ADD: i32 = linux_raw_sys::general::EPOLL_CTL_ADD as i32;
```

**After:**
```rust
use linux_raw_sys::general::{EPOLL_CTL_ADD, EPOLL_CTL_DEL, ...};
// Change function signature to match
pub fn sys_epoll_ctl(epfd: i32, op: u32, fd: i32, ...) {  // op was i32, now u32
```

---

## Function Signature Changes

These functions need signature updates:

| Function | Current Arg | New Arg | Reason |
|----------|-------------|---------|--------|
| `sys_clone` | `flags: u64` | `flags: u32` | CLONE_* are u32 |
| `sys_epoll_ctl` | `op: i32` | `op: u32` | EPOLL_CTL_* are u32 |

The syscall dispatcher will cast `frame.arg0() as u32` instead of `as u64`.

---

## Errno Handling Decision

**Option A: Explicit negation at every return site**
```rust
return -(linux_raw_sys::errno::ENOENT as i64);
```
- Pros: Most explicit, no magic
- Cons: Verbose, repetitive

**Option B: Helper macro**
```rust
macro_rules! neg_errno {
    ($e:ident) => { -(linux_raw_sys::errno::$e as i64) }
}
return neg_errno!(ENOENT);
```
- Pros: Concise, type-safe
- Cons: Adds indirection

**Option C: Helper function**
```rust
#[inline(always)]
pub fn neg_errno(e: u16) -> i64 { -(e as i64) }
use linux_raw_sys::errno::ENOENT;
return neg_errno(ENOENT);
```
- Pros: Clear intent, type-checked
- Cons: Two items to import

**DECISION: Option A** - Most explicit, least magic. The cast is visible.

---

## Coexistence Strategy

**None.** We will:
1. Delete all shim modules
2. Let compiler scream
3. Fix every callsite

No gradual migration. No compatibility layer.

---

## Exit Criteria for Phase 2

- [ ] Target module layout documented
- [ ] All signature changes identified
- [ ] Errno handling approach decided (Option A)
- [ ] No shims in target design
