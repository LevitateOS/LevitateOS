# Syscall Abstractions Refactor Plan

**TEAM_411** | Created: 2026-01-10

---

## Overview

This refactor introduces common abstractions to reduce boilerplate across syscall implementations while **maintaining full Linux ABI compatibility**.

## Key Constraint

> **All changes are internal to the kernel. The syscall interface visible to userspace remains identical.**

---

## Abstractions to Implement

| Priority | Abstraction | Purpose | LOC Reduction |
|----------|-------------|---------|---------------|
| 1 | `impl From<VfsError> for i64` | Consistent error mapping | ~50 match arms |
| 2 | `UserSlice<T>` | Safe user-space buffer access | ~100 lines |
| 3 | `get_fd()` / `get_vfs_file()` | Reduce fd lookup boilerplate | ~75 lines |
| 4 | `resolve_at_path()` | Proper dirfd support (fix stub) | ~30 lines + new feature |
| 5 | `SyscallContext` (optional) | Ergonomic wrapper | Convenience |

---

## Phase Structure

| Phase | File | Purpose |
|-------|------|---------|
| 1 | `phase-1.md` | Discovery: Map current patterns, lock in tests, document Linux ABI |
| 2 | `phase-2.md` | Extraction: Implement new abstractions (no migration yet) |
| 3 | `phase-3.md` | Migration: Update syscalls to use new abstractions |
| 4 | `phase-4.md` | Cleanup: Remove dead code, tighten visibility |
| 5 | `phase-5.md` | Hardening: Final verification, documentation, handoff |

---

## Linux ABI Preservation

### Immutable (Must Not Change)
- Syscall numbers (x86_64 and aarch64)
- Syscall argument order and types
- Return value semantics
- errno values
- Struct layouts (`stat`, `statx`, `dirent64`, `iovec`, `timespec`)

### Verification
- Eyra behavior tests must pass with **unchanged golden output**
- Manual boot tests must work identically
- Both architectures must build and run

---

## Quick Start

1. Read `phase-1.md` for context and constraints
2. Implement abstractions per `phase-2.md`
3. Migrate syscalls per `phase-3.md` call site inventory
4. Clean up per `phase-4.md`
5. Verify and hand off per `phase-5.md`

---

## Success Criteria

- [ ] ~30% reduction in syscall boilerplate
- [ ] Consistent VfsError → errno mapping
- [ ] Proper dirfd support (not stubbed)
- [ ] Zero behavioral changes visible to userspace
- [ ] All tests pass unchanged
