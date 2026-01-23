# TEAM_093: distro-spec SSOT Integration

**Status**: PHASE 2 COMPLETE - AWAITING LEVISO REFACTOR COMPLETION
**Started**: 2026-01-23

## Objective

Make distro-spec the actual Single Source of Truth (SSOT) with compile-time enforcement. Currently leviso declares distro-spec as dependency but imports nothing.

## BLOCKED

**Another agent (TEAM_092) is actively refactoring leviso.** Do NOT modify leviso code until that work is complete.

---

## Phase 1: Audit Results (COMPLETE)

### Constants Found in leviso That Should Come From distro-spec

| File | Line | Current Value | Proposed Constant | Notes |
|------|------|---------------|-------------------|-------|
| `iso.rs` | 13 | `"LEVITATEOS"` | `ISO_LABEL` | Volume label for boot device detection |
| `iso.rs` | 33 | `"filesystem.squashfs"` | `SQUASHFS_NAME` | Already exists in distro-spec! |
| `iso.rs` | 143 | `"live/filesystem.squashfs"` | Use `SQUASHFS_NAME` | Path construction |
| `squashfs/pack.rs` | 32 | `"gzip"` | `SQUASHFS_COMPRESSION` | Compression algorithm |
| `squashfs/pack.rs` | 33 | `"1M"` | `SQUASHFS_BLOCK_SIZE` | Block size for mksquashfs |
| `initramfs/mod.rs` | 58-70 | `BOOT_MODULES` array | `BOOT_MODULES` | Kernel modules for initramfs |

### Constants That Already Exist in distro-spec (Unused!)

- `SQUASHFS_NAME` = `"filesystem.squashfs"` (in `paths.rs`)
- `SQUASHFS_CDROM_PATH` = `"/media/cdrom/live/filesystem.squashfs"` (in `paths.rs`)

### Constants NOT Worth Moving (Build-Time Details)

- `BUSYBOX_COMMANDS` - Only used by initramfs builder
- `SUPPLEMENTARY_RPMS` - Build-time RPM list
- `DEFAULT_BUSYBOX_URL` - Download URL (env-overridable)

---

## Phase 2: Constants Added to distro-spec (COMPLETE)

Added to `distro-spec/src/levitate/paths.rs`:

```rust
// ISO constants
pub const ISO_LABEL: &str = "LEVITATEOS";

// Squashfs build constants
pub const SQUASHFS_COMPRESSION: &str = "gzip";
pub const SQUASHFS_BLOCK_SIZE: &str = "1M";
```

Added to `distro-spec/src/levitate/boot.rs`:

```rust
/// Kernel modules required in the initramfs for boot.
pub const BOOT_MODULES: &[&str] = &[
    "kernel/drivers/cdrom/cdrom.ko.xz",
    "kernel/drivers/scsi/sr_mod.ko.xz",
    "kernel/drivers/scsi/virtio_scsi.ko.xz",
    "kernel/fs/isofs/isofs.ko.xz",
    "kernel/drivers/block/virtio_blk.ko.xz",
    "kernel/drivers/block/loop.ko.xz",
    "kernel/fs/squashfs/squashfs.ko.xz",
    "kernel/fs/overlayfs/overlay.ko.xz",
];
```

Updated `distro-spec/src/levitate/mod.rs` to re-export:

```rust
pub use boot::{..., BOOT_MODULES};
pub use paths::{..., ISO_LABEL, SQUASHFS_COMPRESSION, SQUASHFS_BLOCK_SIZE};
```

---

## Phase 3: leviso Import Updates (BLOCKED - DO NOT IMPLEMENT YET)

After TEAM_092 completes, update these files:

### iso.rs
```rust
use distro_spec::levitate::{ISO_LABEL, SQUASHFS_NAME};

fn iso_label() -> String {
    env::var("ISO_LABEL").unwrap_or_else(|_| ISO_LABEL.to_string())
}
```

### squashfs/pack.rs
```rust
use distro_spec::levitate::{SQUASHFS_COMPRESSION, SQUASHFS_BLOCK_SIZE};

.args(["-comp", SQUASHFS_COMPRESSION])
.args(["-b", SQUASHFS_BLOCK_SIZE])
```

### initramfs/mod.rs
```rust
use distro_spec::levitate::BOOT_MODULES;
// Remove local BOOT_MODULES constant
```

---

## Phase 4 & 5: Deferred

Verification and documentation to be done after implementation.

---

## Notes

- 2026-01-23: Audit complete. Discovered leviso already depends on distro-spec but imports nothing.
- 2026-01-23: BLOCKED - TEAM_092 is actively refactoring leviso. Cannot modify.
- 2026-01-23: Phase 2 complete - Added ISO_LABEL, SQUASHFS_COMPRESSION, SQUASHFS_BLOCK_SIZE, BOOT_MODULES to distro-spec. Verified build passes.
