# Phase 4 — Cleanup

**Refactor:** Eliminate Type Shims - Make linux-raw-sys the Canonical Source
**Team:** TEAM_420
**Date:** 2026-01-10

---

## Dead Code Removal

### Shim modules to delete completely

| Location | What to Delete |
|----------|---------------|
| `syscall/mod.rs` | `pub mod errno { ... }` (entire block) |
| `syscall/mod.rs` | `pub mod fcntl { ... }` (entire block) |
| `syscall/process/mod.rs` | All `CLONE_*` constants |
| `syscall/epoll.rs` | `EPOLL_CTL_ADD`, `EPOLL_CTL_DEL`, `EPOLL_CTL_MOD`, `EPOLL_CLOEXEC` const defs |
| `syscall/process/resources.rs` | `RLIM_INFINITY` local const (use linux-raw-sys or u64::MAX) |

### Unused imports to remove

After fixing callsites, these imports become dead:
- `use super::{CLONE_VM, ...}` in thread.rs
- `use crate::syscall::errno` everywhere
- `use crate::syscall::fcntl` everywhere

---

## Encapsulation Tightening

### Before (leaky)
```rust
// syscall/mod.rs exports errno and fcntl modules
pub mod errno { ... }
pub mod fcntl { ... }
```

### After (tight)
```rust
// No re-exports of linux-raw-sys constants
// Each file imports directly what it needs
```

---

## File Size Check

| File | Before | After (Est.) | Status |
|------|--------|--------------|--------|
| `syscall/mod.rs` | ~400 | ~340 | OK (<500) |
| `syscall/process/mod.rs` | ~41 | ~30 | OK |
| `syscall/epoll.rs` | ~300 | ~290 | OK |

No file size concerns.

---

## Code Style Consistency

### Errno pattern (standardize everywhere)
```rust
// Pattern: -(linux_raw_sys::errno::E* as i64)
return -(linux_raw_sys::errno::ENOENT as i64);

// NOT:
return errno::ENOENT;  // Old shim style
return -2i64;           // Magic number
```

### Import style (standardize)
```rust
// At top of file, group linux-raw-sys imports
use linux_raw_sys::errno::ENOENT;
use linux_raw_sys::general::{AT_FDCWD, CLONE_VM, O_RDONLY};

// NOT scattered imports throughout
```

---

## Verification Grep Commands

```bash
# No shim modules should exist
grep -rn "pub mod errno" crates/kernel/src/syscall/ | grep -v test
grep -rn "pub mod fcntl" crates/kernel/src/syscall/ | grep -v test

# No CLONE_* constants in process/mod.rs
grep -n "pub const CLONE_" crates/kernel/src/syscall/process/mod.rs

# No casts in constant definitions (shim pattern)
grep -rn "= .*as i64\);" crates/kernel/src/syscall/
grep -rn "= .*as u64;" crates/kernel/src/syscall/

# All imports should be from linux_raw_sys
grep -rn "use crate::syscall::errno" crates/kernel/src/
grep -rn "use crate::syscall::fcntl" crates/kernel/src/
```

---

## Exit Criteria for Phase 4

- [ ] All shim modules deleted
- [ ] No `pub const X = linux_raw_sys::*::X as T;` patterns remain
- [ ] No `use crate::syscall::errno` imports remain
- [ ] No `use crate::syscall::fcntl` imports remain
- [ ] All files under 500 lines (ideal)
- [ ] Grep verification passes
