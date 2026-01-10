# Phase 1 — Discovery and Safeguards

**Refactor:** Eliminate Type Shims - Make linux-raw-sys the Canonical Source
**Team:** TEAM_420
**Date:** 2026-01-10

---

## Problem Statement

The TEAM_419 migration to linux-raw-sys introduced **type shims** instead of properly adapting the codebase. This is backwards:

**WRONG (Current State):**
```rust
// We cast linux-raw-sys types to match OUR types
pub const CLONE_VM: u64 = linux_raw_sys::general::CLONE_VM as u64;
pub const EPOLL_CTL_ADD: i32 = linux_raw_sys::general::EPOLL_CTL_ADD as i32;
pub const EPERM: i64 = -(raw::EPERM as i64);
```

**RIGHT (Target State):**
```rust
// We use linux-raw-sys types DIRECTLY
// Our functions accept linux-raw-sys types, not shims
pub use linux_raw_sys::general::CLONE_VM;  // This is u32, and we accept u32
```

The library IS the canonical source. If there's a type mismatch, **WE change**, not the library.

---

## Pain Points

1. **Shim modules everywhere**: `errno`, `fcntl`, `process/mod.rs` all have const wrappers
2. **Hidden type conversions**: Casts like `as u64`, `as i32` hide the real types
3. **Fragility**: If linux-raw-sys changes types, our casts may silently break
4. **Complexity**: Extra indirection for no benefit
5. **Violation of SSOT**: The shim IS the source, not linux-raw-sys

---

## Root Cause Analysis

The core issue is that **syscall function signatures expect specific types**:

| Syscall Arg | Current Type | linux-raw-sys Type |
|-------------|--------------|-------------------|
| `clone flags` | `u64` | `u32` |
| `errno return` | `i64` | `u16` (positive) |
| `EPOLL_CTL_*` | `i32` | `u32` |

Instead of changing the **signatures** to match linux-raw-sys, we created **shims**.

---

## Success Criteria

### Before (WRONG)
```rust
// process/mod.rs - SHIM
pub const CLONE_VM: u64 = linux_raw_sys::general::CLONE_VM as u64;

// thread.rs - uses OUR type
pub fn sys_clone(flags: u64, ...) {
    if flags & CLONE_VM != 0 { ... }
}
```

### After (CORRECT)
```rust
// thread.rs - uses linux-raw-sys type DIRECTLY
pub fn sys_clone(flags: u32, ...) {
    use linux_raw_sys::general::CLONE_VM;
    if flags & CLONE_VM != 0 { ... }
}
```

---

## Shim Inventory

### 1. errno module (`syscall/mod.rs`)
- 34 constants with `-(raw::* as i64)` pattern
- **Issue**: Syscall returns are i64, errno in linux-raw-sys is u16
- **Solution**: Create a helper `fn neg_errno(e: u16) -> i64` used at return sites, OR use i64 returns and negate at callsite

### 2. Clone flags (`process/mod.rs`)
- 9 constants with `as u64` casts
- **Issue**: `sys_clone` takes `flags: u64`
- **Solution**: Change `sys_clone` signature to take `flags: u32`

### 3. Epoll constants (`epoll.rs`)
- 4 constants with `as i32` casts
- **Issue**: `sys_epoll_ctl` takes `op: i32`
- **Solution**: Change `sys_epoll_ctl` to take `op: u32`

### 4. fcntl constants (`syscall/mod.rs`)
- Currently re-exports directly (no shims) - OK
- AT_FDCWD is already i32 in linux-raw-sys - OK

---

## Behavioral Contracts

1. **Syscall ABI unchanged**: Arguments passed from userspace are the same
2. **Return values unchanged**: Negative errno for errors
3. **Constant values identical**: Just using canonical types

---

## Constraints

1. **Syscall frame types**: `frame.arg0()` returns `usize` - casts at dispatch are OK
2. **Architecture differences**: May need `#[cfg]` for type differences
3. **Errno negation**: Linux-raw-sys errno are positive; returns must be negative

---

## Open Questions

1. Should errno helper be a function or macro?
   - Function: `return neg_errno(ENOENT)`
   - Macro: `return errno!(ENOENT)`
   - Direct: `return -(ENOENT as i64)` (most explicit)

2. For clone flags, is u32 sufficient for the ABI?
   - Check: Does any architecture use >32 bits for clone flags?
   - Answer: No, clone flags fit in 32 bits on all Linux archs

---

## Phase 1 Steps

### Step 1 — Audit all shims
- [ ] List every `as i32`, `as u32`, `as i64`, `as u64` cast related to linux-raw-sys
- [ ] Categorize by type of fix needed

### Step 2 — Lock in baseline tests
- [ ] `cargo xtask build kernel --arch x86_64`
- [ ] `cargo xtask build kernel --arch aarch64`
- [ ] `cargo xtask test` (if applicable)

### Step 3 — Document type mappings
- [ ] Create mapping table: syscall arg -> linux-raw-sys type

---

## Exit Criteria for Phase 1

- [ ] Complete shim inventory
- [ ] Baseline tests pass
- [ ] Type mapping documented
- [ ] Decision on errno handling approach
