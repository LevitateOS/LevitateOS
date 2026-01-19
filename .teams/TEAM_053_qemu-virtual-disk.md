# TEAM 053: QEMU Virtual Disk Support

## Task
Add virtual disk support to QEMU for testing disk utilities.

## Status: Complete

## What was done
1. 8GB disk attached by default (Rule #2: don't put required features behind flags)
2. Added kernel module support to initramfs (virtio_blk.ko.xz)
3. Init script loads virtio_blk module at boot
4. Added `insmod` and `xz` binaries to initramfs

## Usage
```bash
cargo run -- run              # 8GB disk by default
cargo run -- run --no-disk    # No disk
cargo run -- run --disk-size 16G  # Different size
cargo run -- test             # Also has 8GB disk now
```

## Key insight
Rocky's kernel has disk drivers as MODULES, not built-in. Had to:
- Copy `virtio_blk.ko.xz` to initramfs
- Add `insmod` and `xz` to decompress and load it
- Load module in init script before systemd starts

## Files Modified
- `leviso/src/main.rs` - Disk by default, --no-disk flag
- `leviso/src/qemu.rs` - Virtio disk attachment
- `leviso/src/initramfs/mod.rs` - Added insmod, xz
- `leviso/src/initramfs/modules.rs` - NEW: Copy kernel modules
- `leviso/profile/init` - Load virtio_blk at boot
