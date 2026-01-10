# TEAM_410: Implement VFS Inode Truncate

**Date**: 2026-01-10
**Status**: ✅ COMPLETED
**Task**: Implement VFS inode truncate support for sys_truncate and sys_ftruncate

## Objective

Implement proper file truncation support in the VFS layer so that `truncate` and `ftruncate` syscalls actually truncate files instead of being stubs.

## Findings

The VFS infrastructure was **already in place**:
- `InodeOps::truncate()` trait method exists in `vfs/ops.rs:113-116`
- `Inode::truncate()` delegation exists in `vfs/inode.rs:228-231`
- `TmpfsFileOps` already implements truncate in `tmpfs/file_ops.rs:97-143`
- Initramfs is read-only (CPIO archive) - truncate returns `NotSupported`

The only missing piece was wiring up the syscalls to call the VFS layer.

## Changes Made

### 1. `crates/kernel/src/syscall/fs/fd.rs`
- **sys_truncate**: Now looks up file by path via VFS dcache, calls `inode.truncate()`
- **sys_ftruncate**: Now gets file from fd table, calls `file.inode.truncate()`
- Both properly validate arguments and handle errors

### 2. `crates/kernel/src/syscall/mod.rs`
- Added missing errno constants: `EISDIR`, `ENOSPC`, `EROFS`, `EFBIG`

## Error Handling

| VFS Error | Errno | Meaning |
|-----------|-------|---------|
| NotSupported | EROFS | Read-only filesystem (initramfs) |
| NoSpace | ENOSPC | No space left on device |
| FileTooLarge | EFBIG | File size limit exceeded |
| Other | EIO | General I/O error |

## Test Results

- ✅ Kernel builds for aarch64
- ✅ Kernel builds for x86_64
- Truncate works on tmpfs files (writable)
- Truncate returns EROFS on initramfs files (read-only)

## Handoff Notes

Future work:
- Implement truncate for ext4 filesystem when disk writes are needed
- The current implementation is complete for the in-memory filesystems

---
