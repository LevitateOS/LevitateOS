# TEAM_473: Low-Hanging Fruit Syscalls

## Objective

Implement commonly-needed syscalls that are simple to add, improving compatibility with existing Linux programs.

## Progress Log

### Session 1 (2026-01-13)

**COMPLETED** - All planned syscalls implemented.

#### Trivial Wrappers (x86_64 only - aarch64 uses *at variants)
- `pipe` (22) - Wrapper for `pipe2(pipefd, 0)`
- `readlink` (89) - Wrapper for `readlinkat(AT_FDCWD, ...)`
- `creat` (85) - Wrapper for `openat(AT_FDCWD, ..., O_CREAT|O_WRONLY|O_TRUNC, mode)`

#### No-Op Stubs (Both architectures)
- `sync` - Sync all filesystems (no-op, tmpfs is memory-backed)
- `fsync` - Sync file to disk (no-op)
- `fdatasync` - Sync file data (no-op)
- `msync` - Sync mmap region (no-op)
- `fadvise64` - File access hints (advisory, always returns success)

#### Simple Implementations (Both architectures)
- `sysinfo` - System information (memory, uptime)
- `statfs` - Filesystem statistics by path
- `fstatfs` - Filesystem statistics by fd

## Key Decisions

- No-op stubs for sync operations are acceptable since tmpfs is memory-backed
- `fadvise64` returns success since it's advisory-only
- Fixed syscall 22 mapping: was incorrectly mapped to Pipe2, now correctly Pipe

## Gotchas Discovered

1. **Syscall 22 conflict**: TEAM_404 had mapped syscall 22 to Pipe2 as a compatibility hack, but Pipe2 is actually syscall 293. Fixed by removing the incorrect mapping.

## Files Modified

| File | Changes |
|------|---------|
| `crates/kernel/arch/x86_64/src/lib.rs` | Added Pipe, Msync, Fsync, Fdatasync, Creat, Readlink, Sysinfo, Statfs, Fstatfs, Sync, Fadvise64; fixed syscall 22 mapping |
| `crates/kernel/arch/aarch64/src/lib.rs` | Added Sync, Fsync, Fdatasync, Sysinfo, Statfs, Fstatfs, Fadvise64, Msync |
| `crates/kernel/syscall/src/lib.rs` | Added dispatch entries for all new syscalls |
| `crates/kernel/syscall/src/sys.rs` | Added sys_sysinfo, sys_statfs, sys_fstatfs |
| `crates/kernel/syscall/src/fs/fd.rs` | Added sys_pipe, sys_sync, sys_fsync, sys_fdatasync, sys_msync, sys_fadvise64 |
| `crates/kernel/syscall/src/fs/mod.rs` | Export new syscalls |

## Syscall Numbers

### x86_64
| Syscall | Number |
|---------|--------|
| pipe | 22 |
| msync | 26 |
| fsync | 74 |
| fdatasync | 75 |
| creat | 85 |
| readlink | 89 |
| sysinfo | 99 |
| statfs | 137 |
| fstatfs | 138 |
| sync | 162 |
| fadvise64 | 221 |

### aarch64
| Syscall | Number |
|---------|--------|
| statfs | 43 |
| fstatfs | 44 |
| sync | 81 |
| fsync | 82 |
| fdatasync | 83 |
| sysinfo | 179 |
| fadvise64 | 223 |
| msync | 227 |

## Verification

- Both x86_64 and aarch64 builds succeed
- Behavior tests pass (Stage 4 reached, no crashes)

## Remaining Work

None - all planned syscalls implemented. Future additions could include:
- [ ] `prctl` - Process control (PR_SET_NAME, etc.)
- [ ] `memfd_create` - Anonymous memory file
- [ ] `flock` - File locking
- [ ] Real memory stats in `sysinfo` (currently hardcoded)

## Handoff Notes

All low-hanging fruit syscalls are now implemented. The key patterns used:
- Trivial wrappers delegate to existing `*at` syscalls
- No-op stubs for sync operations since tmpfs is memory-backed
- `sysinfo` and `statfs` return reasonable default values (could be improved with real stats later)
