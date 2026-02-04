# TEAM_155: IuppiterOS Refurbishment Tools - lsscsi & nvme-cli

**Date**: 2026-02-04
**Status**: COMPLETE
**Iteration**: 28

## What was implemented

Added lsscsi and nvme binary support to IuppiterOS rootfs to complete the disk refurbishment toolchain.

## Changes Made

### IuppiterOS/src/component/definitions.rs
- Added `"lsscsi"` to `ADDITIONAL_BINS` constant (line 134)
  - Provides SCSI device enumeration (maps /dev/sd* to HBA:channel:target:lun)
- Added `"nvme"` to `ADDITIONAL_SBINS` constant (line 167)
  - Provides NVMe management CLI from nvme-cli package

### Verification
- Binaries were already available in downloads/rootfs from Alpine package installation
- Forced rootfs rebuild: `rm output/filesystem.erofs && cargo run -- build rootfs`
- Confirmed presence in staging:
  - `/usr/bin/lsscsi` (77 KB)
  - `/usr/sbin/nvme` (1.3 MB)
- All 22 IuppiterOS tests pass
- EROFS size: 47 MB (expected, includes sg3_utils from task 7.3)

## Task Dependencies

- Phase 7.3 (sg3_utils) already complete
- Phase 7.1-7.2 (smartmontools, hdparm) already complete
- These are purely additive changes; no conflicts with existing code

## Testing

```bash
cd IuppiterOS
cargo test --lib  # All 22 tests pass
cargo run -- build rootfs  # Successful EROFS build
ls -lh output/rootfs-staging/usr/bin/lsscsi output/rootfs-staging/usr/sbin/nvme
```

## Key Decisions

1. **Binary location split**: lsscsi → /usr/bin, nvme → /usr/sbin
   - Follows Alpine convention: sbin for system/admin tools, bin for general utilities
   - nvme is more admin-focused (disk management)

2. **Cache invalidation**: Forced rebuild was necessary
   - EROFS build was cached (hash-based)
   - Removing hash file and rebuilding ensures binaries are included

3. **Reused existing packages**: Both binaries already installed by recipe
   - No changes to packages.rhai needed
   - Just needed to add binary copying to component definitions

## Blockers

None. Task completed cleanly.

## Future Work

Phase 7.5-7.11 for appliance-specific configuration:
- /var/data mount point
- /etc/iuppiter/ config directory
- /opt/iuppiter/ binary directory
- iuppiter-engine service
- Operator user with disk group membership
- udev rules for I/O scheduler

---

**Status**: READY FOR MERGE
