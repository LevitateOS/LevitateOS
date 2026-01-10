# TEAM_413: Implement Syscall Abstractions

**Created**: 2026-01-10
**Status**: Complete
**Plan**: `docs/planning/syscall-abstractions/`
**Review**: `.teams/TEAM_412_review_syscall_abstractions_plan.md`

---

## Objective

Implement the syscall helper abstractions designed in TEAM_411 plan, incorporating feedback from TEAM_412 review.

## Abstractions Implemented

| Priority | Abstraction | Status |
|----------|-------------|--------|
| 1 | `impl From<VfsError> for i64` | Complete |
| 2 | `UserPtr<T>` / `UserSlice<T>` | Complete |
| 3 | `get_fd()` / `get_vfs_file()` / `is_valid_fd()` | Complete |
| 4 | `write_struct_to_user<T>` / `read_struct_from_user<T>` | Complete |
| 5 | `resolve_at_path()` | Complete (with TODO for full dirfd support) |
| 6 | `SyscallResultExt` trait | Complete |

## Progress Log

- 2026-01-10: Team registered, baseline verified (build passes)
- 2026-01-10: Phase 2 complete - all abstractions implemented in helpers.rs
- 2026-01-10: Phase 3 complete - migrated stat.rs, time.rs, fd.rs
- 2026-01-10: Phase 4 complete - removed unused import from sync.rs
- 2026-01-10: Both architectures build successfully

## Files Created/Modified

- `crates/kernel/src/syscall/helpers.rs` (NEW - 427 lines)
- `crates/kernel/src/syscall/mod.rs` (exports added)
- `crates/kernel/src/fs/vfs/error.rs` (From impl added)
- `crates/kernel/src/syscall/fs/stat.rs` (migrated to use helpers)
- `crates/kernel/src/syscall/time.rs` (migrated to use helpers)
- `crates/kernel/src/syscall/fs/fd.rs` (migrated to use helpers)
- `crates/kernel/src/syscall/sync.rs` (removed unused import)

## Syscalls Migrated

- `sys_fstat` - uses `get_fd()`, `write_struct_to_user()`
- `sys_fstatat` - uses `resolve_at_path()`, `write_struct_to_user()`
- `sys_clock_getres` - uses `write_struct_to_user()`
- `sys_clock_gettime` - uses `write_struct_to_user()`
- `sys_gettimeofday` - uses `write_struct_to_user()`
- `sys_fcntl` - uses `is_valid_fd()`
- `sys_isatty` - uses `get_fd()`
- `sys_ioctl` - uses `get_fd()`
- `sys_lseek` - uses `get_fd()`
- `sys_ftruncate` - uses `get_fd()`
- `sys_pread64` - uses `get_fd()`
- `sys_pwrite64` - uses `get_fd()`
- `sys_fchmod` - uses `is_valid_fd()`
- `sys_fchown` - uses `is_valid_fd()`

## Known Limitations

- `resolve_at_path()` only supports AT_FDCWD for relative paths
- Full dirfd support requires storing path in FdEntry (future work)

## Verification

- [x] Baseline build passes
- [x] All abstractions implemented
- [x] Phase 3 migration complete
- [x] Phase 4 cleanup complete
- [x] aarch64 build passes
- [x] x86_64 build passes
