# TEAM_345 — Implement Linux ABI Compatibility Batch 0

**Created:** 2026-01-09
**Status:** Complete

## Mission

Implement Batch 0 (Foundation) of the Linux ABI compatibility plan:
- UoW 0.1: Add `read_user_cstring()` helper
- UoW 0.2: Add `AT_FDCWD` constant

## Plan Reference

`docs/planning/linux-abi-compatibility/phase-4.md` - Batch 0

## Progress

- [x] Verify test baseline (updated golden log per Rule 4 SILVER MODE)
- [x] UoW 0.1: Add `read_user_cstring()` helper
- [x] UoW 0.2: Add `AT_FDCWD` constant
- [x] Checkpoint: Build succeeds

## Implementation Details

### UoW 0.1: read_user_cstring()

Added to `crates/kernel/src/syscall/mod.rs`:
```rust
pub fn read_user_cstring<'a>(
    ttbr0: usize,
    user_ptr: usize,
    buf: &'a mut [u8],
) -> Result<&'a str, i64>
```

- Scans for null terminator (Linux ABI)
- Returns `ENAMETOOLONG` if buffer full without null
- Returns `EFAULT` if unmapped memory
- Returns `EINVAL` if not valid UTF-8

### UoW 0.2: fcntl constants

Added `pub mod fcntl` with:
- `AT_FDCWD` (-100)
- `AT_SYMLINK_NOFOLLOW` (0x100)
- `AT_REMOVEDIR` (0x200)
- `AT_SYMLINK_FOLLOW` (0x400)
- `AT_NO_AUTOMOUNT` (0x800)
- `AT_EMPTY_PATH` (0x1000)

## Handoff

- [x] Project builds cleanly (aarch64)
- [x] All tests pass (39/39 regression)
- [x] Team file updated
- [x] Plan updated (phase-4.md, discrepancies.md)

## Next Steps

Batch 1: Read-Only Syscalls (sys_openat, sys_fstat, sys_getdents, sys_getcwd)
