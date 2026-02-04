# TEAM_121: EROFS Migration

## Summary
Migrate from squashfs+gzip to EROFS+zstd for the live rootfs image.

## Why EROFS?
- Better read performance (random access vs linear scan)
- zstd compression (better ratio, faster decompression than gzip)
- Used by Fedora 42+, RHEL 10, Android
- Lower memory overhead
- Aligned with modern distro direction

## Files Modified
- `leviso/kconfig` - Add EROFS kernel config
- `distro-spec/src/shared/squashfs.rs` → `rootfs.rs` - Add EROFS constants
- `distro-spec/src/shared/mod.rs` - Update exports
- `distro-spec/src/shared/iso.rs` - Rename SQUASHFS_ISO_PATH to ROOTFS_ISO_PATH
- `distro-spec/src/levitate/paths.rs` - Update re-exports
- `distro-spec/src/levitate/mod.rs` - Update exports
- `distro-spec/src/acorn/paths.rs` - Update re-exports
- `distro-spec/src/acorn/mod.rs` - Update exports
- `leviso/src/artifact/squashfs.rs` → `rootfs.rs` - Use mkfs.erofs
- `leviso/src/artifact/mod.rs` - Update module name
- `leviso/src/artifact/initramfs.rs` - Update imports
- `leviso/src/artifact/iso.rs` - Update imports
- `leviso/profile/init_tiny.template` - Mount as erofs
- `leviso/src/preflight/host_tools.rs` - Add mkfs.erofs check

## Migration Notes
- mkfs.erofs argument order is `OUTPUT SOURCE` (opposite of mksquashfs!)
- Host requires erofs-utils 1.8+ for zstd support
- Kernel requires CONFIG_EROFS_FS_ZIP_ZSTD=y (Linux 6.10+)

## Status
COMPLETE

## Implementation Summary

### Kernel Config (leviso/kconfig)
Added EROFS filesystem support with zstd compression.

### distro-spec Changes
- Renamed `squashfs.rs` → `rootfs.rs` in `shared/`
- Added EROFS constants (EROFS_NAME, EROFS_COMPRESSION, EROFS_CHUNK_SIZE, etc.)
- Added unified ROOTFS_NAME, ROOTFS_CDROM_PATH aliases
- Kept squashfs constants for legacy reference
- Updated exports in levitate/mod.rs and acorn/mod.rs
- Added ROOTFS_ISO_PATH to iso.rs

### leviso Changes
- Renamed `artifact/squashfs.rs` → `artifact/rootfs.rs`
- Updated to use mkfs.erofs with zstd compression
- NOTE: mkfs.erofs argument order is OUTPUT SOURCE (opposite of mksquashfs!)
- Added backward-compatible `build_squashfs()` alias
- Updated initramfs.rs to use ROOTFS_ISO_PATH
- Updated iso.rs to use ROOTFS_NAME and ROOTFS_ISO_PATH
- Updated preflight host_tools.rs to check for mkfs.erofs instead of mksquashfs

### Initramfs Template (leviso/profile/init_tiny.template)
- Changed mount type from squashfs to erofs
- Updated error messages to reference CONFIG_EROFS_FS
- Changed template variable from SQUASHFS_PATH to ROOTFS_PATH
- Mount point /squashfs retained for backward compatibility

### distro-builder Changes
- Renamed `artifact/squashfs.rs` → `artifact/rootfs.rs`
- Updated documentation to mention EROFS support

### AcornOS
NOT MODIFIED - AcornOS still uses squashfs. Separate migration needed if desired.

## Host Requirements
- erofs-utils 1.8+ for zstd compression support
- Linux kernel 6.10+ with CONFIG_EROFS_FS_ZIP_ZSTD=y
