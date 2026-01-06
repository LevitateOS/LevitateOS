# VFS (Virtual Filesystem) Implementation Plan

**Planning Team:** TEAM_200  
**Created:** 2026-01-06  
**Status:** Phases 12-13 Complete, Phase 14 TODO

> **See `STATUS.md` for detailed DONE/DOING/TODO tracking**

---

## Overview

Implement a Linux-style Virtual Filesystem (VFS) layer for LevitateOS to replace the current ad-hoc file handling with a unified, extensible architecture.

## Current Problems

1. **Giant match statements** — Every syscall dispatches by `FdType` variant
2. **No unified abstraction** — Each filesystem is special-cased
3. **No mount infrastructure** — `/tmp` is hardcoded
4. **No inode concept** — Only tmpfs has `ino` field
5. **No dentry cache** — Path lookups repeat work

## Goals

- **Unified file abstraction** — Single `File` struct for all open files
- **Inode operations trait** — Filesystems implement standard operations
- **Mount table** — Dynamic mount/unmount support
- **Dentry cache** — Efficient path-to-inode resolution
- **Clean syscall layer** — Remove per-filesystem dispatch

## Phases

| Phase | Name | Status | Team |
|-------|------|--------|------|
| 12 | VFS Foundation | ✅ Complete | TEAM_201 |
| 13 | Core VFS | ✅ Complete | TEAM_202 |
| 14 | Migration | 🔴 TODO | TBD |

## Phase Files

- `phase-12.md` — VFS Foundation (Prerequisites) ✅
- `phase-13.md` — Core VFS Implementation ✅
- `phase-14.md` — Filesystem Migration 🔴
- `STATUS.md` — Detailed DONE/DOING/TODO tracking
- `reference-analysis.md` — Patterns from Theseus/Redox

## Success Criteria

- [ ] All syscalls use VFS layer (no FdType dispatch)
- [ ] tmpfs implements InodeOps
- [ ] initramfs implements InodeOps
- [ ] mount/umount syscalls work
- [ ] Dentry cache improves lookup performance
- [ ] All existing tests pass

## References

- [Linux VFS](https://www.kernel.org/doc/html/latest/filesystems/vfs.html)
- [Redox Filesystem](https://gitlab.redox-os.org/redox-os/redox/-/tree/master/kernel/src/scheme)
- [xv6 Filesystem](https://pdos.csail.mit.edu/6.828/2012/xv6/book-rev7.pdf) (Chapter 6)

### Local Reference Kernels (`.external-kernels/`)

| Kernel | Key Patterns | Files |
|--------|--------------|-------|
| **Theseus** | Trait-based VFS, Path crate, MemFile | `kernel/fs_node/`, `kernel/path/`, `kernel/memfs/` |
| **Redox** | Scheme abstraction, FileDescription | `src/scheme/mod.rs`, `src/context/file.rs` |

See `reference-analysis.md` for detailed patterns.
