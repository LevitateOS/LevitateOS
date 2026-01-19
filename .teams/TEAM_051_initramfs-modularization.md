# TEAM 051: Initramfs Modularization

## Task
Refactor the 916-line `initramfs.rs` into focused modules while fixing 22 identified bugs.

## Status: Complete

## Key Decisions
- Keep public API unchanged: `initramfs::build_initramfs()`
- Module structure in `initramfs/` directory
- Fix bugs during extraction (not separate pass)

## Module Structure
```
leviso/src/initramfs/
├── mod.rs              # Public API + orchestration
├── context.rs          # BuildContext struct
├── rootfs.rs           # Rootfs detection/validation
├── binary.rs           # Binary & library copying
├── filesystem.rs       # FHS structure creation
├── systemd.rs          # Systemd setup
├── pam.rs              # PAM authentication
├── dbus.rs             # D-Bus setup
├── chrony.rs           # NTP daemon setup
└── users.rs            # User/group management
```

## Bug Fixes
1. Silent library failures (line 355) - Log warnings instead of `let _ =`
2. `.unwrap()` panics (lines 30, 32, 43, 305) - Use `.with_context()`
3. Symlink overwrite (line 159) - Check existence before creating
4. Hardcoded UIDs (lines 834, 837, 844, 847) - Read from rootfs passwd/group
5. Missing host tool check - Add early validation
6. Incomplete ldd parsing (lines 240-262) - Handle "not found" lines

## Progress
- [x] Create team file
- [x] Create module directory structure
- [x] Extract context.rs
- [x] Extract binary.rs (with bug fixes)
- [x] Extract users.rs (with UID fix)
- [x] Extract filesystem.rs (with symlink fix)
- [x] Extract rootfs.rs
- [x] Extract pam.rs
- [x] Extract chrony.rs
- [x] Extract dbus.rs
- [x] Extract systemd.rs
- [x] Clean up mod.rs
- [x] Test build
- [x] Test functional

## Files Modified
- DELETE: `leviso/src/initramfs.rs`
- CREATE: `leviso/src/initramfs/mod.rs`
- CREATE: `leviso/src/initramfs/context.rs`
- CREATE: `leviso/src/initramfs/rootfs.rs`
- CREATE: `leviso/src/initramfs/binary.rs`
- CREATE: `leviso/src/initramfs/filesystem.rs`
- CREATE: `leviso/src/initramfs/systemd.rs`
- CREATE: `leviso/src/initramfs/pam.rs`
- CREATE: `leviso/src/initramfs/dbus.rs`
- CREATE: `leviso/src/initramfs/chrony.rs`
- CREATE: `leviso/src/initramfs/users.rs`
