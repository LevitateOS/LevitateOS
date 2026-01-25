# TEAM_064: Stage3 Tarball Builder

## Status: COMPLETE

## Goal
Build a minimal stage3 rootfs tarball for LevitateOS installation.

## Key Details
- **Kernel**: NOT included (installed separately in Phase 5)
- **Init**: systemd
- **Package manager**: recipe (included)
- **Tarball size**: ~32 MB (compressed with xz)
- **Differences from live (initramfs)**:
  - Disk-based, persistent root (not RAM)
  - Full /etc configuration
  - Real PAM authentication (not permissive autologin)
  - systemd-networkd for networking

## Usage
```bash
cd stage3
cargo run -- build --source ../leviso/downloads/rootfs --output ./output
cargo run -- verify ./output/levitateos-stage3.tar.xz
cargo run -- list ./output/levitateos-stage3.tar.xz
```

## Files Created
- `stage3/src/binary.rs` - Binary/library copying utilities
- `stage3/src/context.rs` - Build context
- `stage3/src/builder.rs` - Main builder implementation
- `stage3/src/rootfs/mod.rs` - Module exports
- `stage3/src/rootfs/filesystem.rs` - FHS structure
- `stage3/src/rootfs/binaries.rs` - Binary lists (coreutils, sbin, systemd)
- `stage3/src/rootfs/systemd.rs` - Installed system units
- `stage3/src/rootfs/etc.rs` - /etc files (passwd, shadow, fstab, etc.)
- `stage3/src/rootfs/pam.rs` - Real PAM config
- `stage3/src/rootfs/recipe.rs` - Recipe integration

## Tarball Contents
- **~66 coreutils binaries** (ls, cat, cp, mv, rm, etc.)
- **~38 sbin utilities** (mount, fsck, useradd, etc.)
- **systemd** with 60+ unit files for installed system
- **PAM** with full authentication stack
- **recipe** package manager
- **/etc** configuration (passwd, shadow, fstab, hostname, etc.)
- **timezone data** (UTC, America, Europe, Asia, Etc)
- **locale-archive**
- **udev rules**, **tmpfiles.d**, **sysctl.d**

## Issues Fixed
1. UTC timezone is a file, not a directory - fixed copy_timezone_data
2. FHS creation tried to create UTC as directory - removed from dir list

## Known Warnings (Normal)
- Some binaries not found in Rocky minimal (file, printf, diff, etc.) - these are optional
- "libsystemd-shared not found" from ldd - library is copied from rootfs anyway
