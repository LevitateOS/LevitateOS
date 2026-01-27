# TEAM_130: recstrap bugfixes

## Status: COMPLETE

## Summary

Fixed bugs and gaps in `tools/recstrap/src/main.rs` identified during code review.

## Bugs Fixed

### 1. CRITICAL: Unconditional `unsquashfs` check blocked EROFS extraction
- Line 785-796 checked for `unsquashfs` before we knew rootfs type
- EROFS users without squashfs-tools could not extract
- **Fix:** Moved tool check after rootfs type detection, now only checks for required tools

### 2. BUG: `cp -a source/.` path construction could fail
- Used `mount_point.join(".")` which relied on shell glob behavior
- **Fix:** Now uses `cp -aT` flag (treats destination as normal file)

### 3. BUG: Wrong comment (line 1060)
- Said "rsync" but code used "cp"
- **Fix:** Updated comment

### 4. BUG: No EROFS kernel module check
- Mount failed with cryptic error if module not loaded
- **Fix:** Added `erofs_supported()` and `ensure_erofs_module()` functions
- Checks /proc/filesystems and tries `modprobe erofs` if needed

### 5. GAP: Missing magic byte validation
- Previously trusted file extension only
- **Fix:** Added `validate_rootfs_magic()` function
  - EROFS: validates `0xe0f5e1e2` at offset 1024
  - Squashfs: validates `hsqs` at offset 0
- New error code E016 for invalid format

### 6. GAP: No cleanup on interrupt (Ctrl+C)
- EROFS mount/tempdir left behind on SIGINT
- **Fix:** Added `MountGuard` RAII struct with `Drop` impl for cleanup

### 7. GAP: No progress indication
- Multi-GB extraction showed nothing
- **Fix:** Added status messages during EROFS extraction phases

## New Error Codes

| Code | Exit | Description |
|------|------|-------------|
| E016 | 16 | Invalid rootfs format (bad magic bytes) |
| E017 | 17 | EROFS filesystem not supported by kernel |

## Files Modified

- `tools/recstrap/src/main.rs` - Main fixes
- `tools/recstrap/CLAUDE.md` - Updated documentation

## Testing

```
cargo test: 43 unit tests + 20 integration tests = 63 tests passing
cargo clippy: No warnings
```

## Line Count

Before: 1507 lines
After: ~1740 lines (added validation, cleanup guard, tests)
