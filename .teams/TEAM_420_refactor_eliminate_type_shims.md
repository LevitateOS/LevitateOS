# TEAM_420: Eliminate Type Shims - Make linux-raw-sys Canonical

**Status:** PLANNED
**Date:** 2026-01-10
**Type:** Refactor

## Summary

TEAM_419 introduced `linux-raw-sys` but created type shims instead of properly adapting the codebase. This refactor eliminates ALL shims and makes linux-raw-sys the true canonical source.

## Problem

Current state has shim modules:
```rust
// WRONG: Shim with type cast
pub mod errno {
    pub const EPERM: i64 = -(linux_raw_sys::errno::EPERM as i64);
}
```

Target state uses library directly:
```rust
// CORRECT: Direct use, adapt callsite
return -(linux_raw_sys::errno::EPERM as i64);
```

## Scope

### Shims to Delete
- `syscall/mod.rs::errno` module (34 constants)
- `syscall/mod.rs::fcntl` module (6 constants)
- `syscall/process/mod.rs` CLONE_* constants (9 constants)
- `syscall/epoll.rs` EPOLL_* constants (4 constants)

### Signatures to Change
- `sys_clone(flags: u64)` -> `sys_clone(flags: u32)`
- `sys_epoll_ctl(op: i32)` -> `sys_epoll_ctl(op: u32)`

### Callsites to Update
- ~95 errno usages
- ~12 fcntl usages
- ~6 CLONE_* usages
- ~8 EPOLL_* usages

## Plan Location

`docs/planning/refactor-eliminate-type-shims/`

## Principle

**The library IS the canonical source. If types don't match, WE change.**

No shims. No wrappers. No re-exports with casts.
