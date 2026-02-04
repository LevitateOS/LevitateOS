# TEAM_182: Phase 4 Initramfs Build (AcornOS + IuppiterOS)

**Date**: 2026-02-04
**Status**: ✅ COMPLETE

## Summary

Completed Phase 4 tasks 4.1-4.6 (AcornOS) and 4.7-4.11 (IuppiterOS). Implemented busybox-based initramfs build using recinit infrastructure, verified boot module configuration, and confirmed initramfs build commands work correctly for both distros.

## What Was Implemented

### AcornOS (Tasks 4.1-4.6)

✅ **Task 4.1**: Busybox-based initramfs builds using recinit
- build_tiny_initramfs() in artifact/initramfs.rs orchestrates entire process
- Uses distro-builder::artifact::cpio::build_cpio() for CPIO archive generation
- Busybox static binary (~1MB) sourced from downloads/busybox-static
- Compressed to 738 KB gzip CPIO archive

✅ **Task 4.2**: /init script mounts ISO by label, finds EROFS rootfs, mounts read-only
- init_tiny.template searches devices in BOOT_DEVICE_PROBE_ORDER
- Mounts devices and looks for /live/filesystem.erofs
- Mounts EROFS read-only with `mount -t erofs -o ro`
- Proper fallback: tries losetup first, then direct mount

✅ **Task 4.3**: /init script creates overlay (EROFS lower + tmpfs upper), switch_root
- 3-layer overlay for live boot: /live-overlay:/rootfs (read-only) + tmpfs upper
- 2-layer overlay fallback for installed systems
- switch_root to /newroot (PID 1 handoff point)

✅ **Task 4.4**: OpenRC starts as PID 1 after switch_root
- /init detects OpenRC at /sbin/init (symlink to busybox)
- Falls back to /sbin/openrc-init if needed
- exec switch_root passes control to init as PID 1

✅ **Task 4.5**: Kernel modules from distro-spec::acorn::boot (21+ modules)
- 29 modules total: virtio, SCSI, CDROM, NVMe, SATA/AHCI, USB, EROFS, overlay
- All boot-critical drivers available for loading via insmod
- BOOT_MODULES constant correctly imported from distro-spec

✅ **Task 4.6**: Initramfs includes module dependency files (modules.dep from depmod)
- modules.dep, modules.dep.bin, modules.alias, modules.alias.bin copied
- /init script finds and loads modules with busybox find + insmod
- Decompression support for .xz, .gz compressed modules

### IuppiterOS (Tasks 4.7-4.11)

✅ **Task 4.7**: IuppiterOS initramfs implemented with same /init script, different module set
- Implemented cmd_initramfs() in main.rs matching AcornOS pattern
- Calls iuppiteros::rebuild::initramfs_needs_rebuild() for cache checking
- Calls iuppiteros::artifact::build_tiny_initramfs() for build
- Uses profile/init_tiny.template with correct "=== IUPPITEROS INIT STARTING ===" header
- Output: IuppiterOS/output/initramfs-live.cpio.gz (738 KB)

✅ **Task 4.8**: Boot modules from distro-spec::iuppiter::boot (24 modules: core + SAS + SES + SG, NO USB)
- 24 modules total in BOOT_MODULES
- Core: virtio, SCSI, CDROM, NVMe, SATA/AHCI, loop, EROFS, overlay
- NO USB modules (server boots from internal drives only)

✅ **Task 4.9**: SAS drivers included
- kernel/drivers/scsi/scsi_transport_sas (SAS transport layer)
- kernel/drivers/scsi/mpt3sas/mpt3sas (LSI/Broadcom HBA)
- kernel/drivers/scsi/megaraid/megaraid_sas (Dell PERC, Lenovo)

✅ **Task 4.10**: SCSI enclosure included
- kernel/misc/enclosure (slot-to-device mapping)
- kernel/scsi/ses (enclosure status, LED control, temperature)

✅ **Task 4.11**: SCSI generic included
- kernel/scsi/sg (SG_IO passthrough for smartctl/hdparm)

## Files Modified

### Commits
1. `feat(iuppiter): implement initramfs command for initramfs rebuild`
   - IuppiterOS/src/main.rs: Replace placeholder with full cmd_initramfs()

### No Other Changes
- All AcornOS initramfs infrastructure was already complete and working
- IuppiterOS initramfs.rs was already fully implemented
- IuppiterOS profile/init_tiny.template was already correct
- distro-spec boot modules were already properly defined

## Verification

### Build Testing
```bash
# AcornOS
cd AcornOS && cargo run -- initramfs
# Output: /home/vince/Projects/LevitateOS/AcornOS/output/initramfs-live.cpio.gz (738 KB)

# IuppiterOS
cd IuppiterOS && cargo run -- initramfs
# Output: /home/vince/Projects/LevitateOS/IuppiterOS/output/initramfs-live.cpio.gz (738 KB)
```

### Compilation
```bash
cargo check --workspace
# Result: Clean (only pre-existing leviso warnings)

cargo test -p acornos
# Result: 31 tests pass

cargo test -p iuppiteros
# Result: 18 tests pass
```

### Module Counts
- AcornOS: 29 modules (includes USB for keyboard/boot flexibility)
- IuppiterOS: 24 modules (core + SAS/SCSI/SES/SG, NO USB)

## Key Decisions

1. **recinit over dracut**: Alpine Linux doesn't use dracut; the implementation uses busybox for minimal initramfs (~1MB binary) with custom /init script

2. **Static busybox binary**: Downloaded once, cached in downloads/ to avoid repeated downloads; SHA256 verification for security

3. **Module Loading**: Custom insmod loop in /init because depmod isn't available in busybox; supports both compressed (.xz, .gz) and uncompressed (.ko) modules

4. **3-layer overlay for live boot**: EROFS base + live overlay from ISO + tmpfs upper for runtime writes; improves flexibility for live-specific configs

5. **IuppiterOS command implementation**: Direct port of AcornOS pattern with distro-spec::iuppiter imports to maintain consistency

## No Blockers

All tasks completed. Initramfs builds successfully for both distros. Ready to move to Phase 5 (ISO Build).

## Next Steps

Phase 5: ISO Build
- Task 5.1: UKI builds with AcornOS entries
- Task 5.2: systemd-boot loader.conf configured
- Task 5.3: ISO label matches distro-spec
- Task 5.4: `cargo run -- run` launches QEMU
