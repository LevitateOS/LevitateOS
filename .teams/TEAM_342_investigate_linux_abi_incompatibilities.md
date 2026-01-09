# TEAM_342 — Investigate Linux ABI Incompatibilities

**Created:** 2026-01-09
**Status:** Complete

## Mission

Investigate what is NOT compatible with Linux ABI and:
- Fix easy things immediately
- Document bigger things for later planning

## Fixes Applied

### 1. Errno Module Consolidation ✅

**Problem:** Two overlapping errno modules (`errno` and `errno_file`) with duplicate constants.

**Fix:** Consolidated into single `errno` module with all constants. Deprecated `errno_file` for backward compatibility.

**Files modified:**
- `crates/kernel/src/syscall/mod.rs` - Consolidated errno module
- `crates/kernel/src/syscall/fs/dir.rs` - Use errno module
- `crates/kernel/src/syscall/fs/fd.rs` - Use errno module  
- `crates/kernel/src/syscall/fs/link.rs` - Use errno module
- `crates/kernel/src/syscall/fs/mount.rs` - Use errno module
- `crates/kernel/src/syscall/fs/open.rs` - Use errno module
- `crates/kernel/src/syscall/process.rs` - Use errno module

### 2. Magic Number Replacement ✅

**Problem:** Magic error numbers like `-34`, `-17`, `-39`, `-18` scattered in code.

**Fix:** Replaced with named constants:
- `-34` → `errno::ERANGE`
- `-17` → `errno::EEXIST`
- `-39` → `errno::ENOTEMPTY`
- `-18` → `errno::EXDEV`

### 3. Added Missing Errno Constants ✅

Added to `errno` module:
- `ERANGE` (-34) - Result too large
- `ENAMETOOLONG` (-36) - File name too long
- `ENOTEMPTY` (-39) - Directory not empty
- `EXDEV` (-18) - Cross-device link
- `EACCES` (-13) - Permission denied

## Remaining Incompatibilities (Need Planning)

### HIGH Priority - Syscall Signature Changes

| Syscall | Issue | Linux Signature | Current Signature |
|---------|-------|-----------------|-------------------|
| `openat` | Missing dirfd, uses length-counted path | `(dirfd, pathname, flags, mode)` | `(path, path_len, flags)` |
| `mkdirat` | Uses length-counted path | `(dirfd, pathname, mode)` | `(dfd, path, path_len, mode)` |
| `unlinkat` | Uses length-counted path | `(dirfd, pathname, flags)` | `(dfd, path, path_len, flags)` |
| `renameat` | Uses length-counted paths | `(olddirfd, oldpath, newdirfd, newpath)` | `(..., old_path_len, ..., new_path_len)` |
| `symlinkat` | Uses length-counted paths | `(target, newdirfd, linkpath)` | `(target, target_len, linkdirfd, linkpath, linkpath_len)` |
| `readlinkat` | Uses length-counted path | `(dirfd, pathname, buf, bufsiz)` | `(dirfd, path, path_len, buf, bufsiz)` |
| `linkat` | Uses length-counted paths | `(olddirfd, oldpath, newdirfd, newpath, flags)` | `(..., oldpath_len, ..., newpath_len, flags)` |
| `utimensat` | Uses length-counted path | `(dirfd, pathname, times, flags)` | `(dirfd, path, path_len, times, flags)` |
| `mount` | Uses length-counted paths | `(source, target, fstype, flags, data)` | `(src, src_len, target, target_len, flags)` |
| `umount` | Uses length-counted path | `(target, flags)` | `(target, target_len)` |

### MEDIUM Priority - Return Value Differences

| Syscall | Issue |
|---------|-------|
| `getcwd` | Returns path length on success, Linux returns pointer |

### MEDIUM Priority - Architecture Issues

| Issue | Location | Details |
|-------|----------|--------|
| `__NR_pause` hardcoded | `sysno.rs:61` | Fixed to 34 (x86_64), aarch64 Linux has no pause syscall |

### LOW Priority - Struct Verification Needed

| Struct | Status | Notes |
|--------|--------|-------|
| `Stat` | ⚠️ Needs verification | x86_64 layout may differ from Linux |
| `Termios` | ⚠️ Needs verification | Custom definitions |
| `Timespec` | ⚠️ Needs verification | May have alignment issues |
| `Dirent64` | ✅ Looks correct | Matches Linux layout |

## Recommended Next Steps

See existing plan at `docs/planning/linux-abi-compatibility/` which already covers:
1. **Batch 0:** Add `read_user_cstring()` helper, `AT_FDCWD` constant
2. **Batch 1-3:** Migrate syscalls to null-terminated strings
3. **Batch 4:** Fix `__NR_pause` arch issue
4. **Batch 5:** Struct verification

## Handoff

- [x] Project builds cleanly (both aarch64 and x86_64)
- [x] All unit tests pass (25/25 HAL, 20/20 others)
- [x] All regression tests pass (39/39)
- [x] Team file updated
- [x] Remaining TODOs documented above

**Note:** Initial test failure was traced to pre-existing Cargo.toml changes from TEAM_343, not this work. After reverting that unrelated change, all tests pass.
