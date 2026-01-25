# TEAM_076: Install Tests Boot Issues

**Status:** BLOCKED - Post-reboot verification fails
**Date:** 2026-01-22
**Context:** Fixing install-tests to use recstrap and tiny initramfs

---

## Executive Summary

The installation phase (steps 1-18) now passes completely. The system installs correctly. However, the installed system fails to boot during the verification phase. The issue is in the UEFI boot configuration - systemd-boot shows a menu but the kernel doesn't load.

---

## Problem Timeline

### Phase 1: Wrong Initramfs (FIXED)

**Symptom:** Boot timeout after 120 seconds
**Cause:** `install-tests/src/main.rs` used `initramfs.cpio.gz` (175MB standalone rootfs) instead of `initramfs-tiny.cpio.gz` (775KB, mounts squashfs from ISO)

**Fix:**
```rust
// OLD (wrong)
let initramfs_path = leviso_dir.join("output/initramfs.cpio.gz");

// NEW (correct)
let initramfs_path = leviso_dir.join("output/initramfs-tiny.cpio.gz");
```

---

### Phase 2: Missing Kernel Modules (FIXED)

**Symptom:** Various "device not found" errors during boot

**Cause:** The tiny initramfs didn't load necessary kernel modules. Rocky kernel has these as modules, not built-in.

**Missing modules identified:**
1. `cdrom.ko.xz` - CDROM driver
2. `sr_mod.ko.xz` - SCSI CDROM driver
3. `virtio_scsi.ko.xz` - QEMU virtio-scsi controller
4. `isofs.ko.xz` - ISO 9660 filesystem
5. `virtio_blk.ko.xz` - QEMU virtio block device (/dev/vda)
6. `loop.ko.xz` - Loop device for squashfs
7. `squashfs.ko.xz` - Squashfs filesystem
8. `overlay.ko.xz` - Overlay filesystem

**Fix in `leviso/src/initramfs/mod.rs`:**
```rust
const BOOT_MODULES: &[&str] = &[
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

**Fix in `leviso/profile/init_tiny`:**
```bash
for mod in cdrom virtio_scsi sr_mod isofs virtio_blk loop squashfs overlay; do
    # ... load each module
done
```

---

### Phase 3: FAT32 Ownership Error (FIXED)

**Symptom:** `unsquashfs` failed with "Operation not permitted" on `/mnt/boot/vmlinuz`

**Cause:** EFI partition (FAT32) was mounted at `/mnt/boot`. FAT32 doesn't support Unix ownership. When recstrap extracted files to `/mnt/boot/`, chown failed.

**Fix:** Changed mount layout:
- OLD: `/mnt/boot` = FAT32 EFI partition
- NEW: `/mnt/boot` = part of ext4 root, `/mnt/boot/efi` = FAT32 EFI partition

**Files modified:**
- `install-tests/src/steps/phase2_disk.rs` - Mount EFI at `/mnt/boot/efi`
- `install-tests/src/steps/phase5_boot.rs` - Use `--esp-path=/boot/efi`

---

### Phase 4: dracut Missing Dependencies (FIXED)

**Symptom:** dracut failed with various missing binary errors

**Errors encountered (in order):**
1. `sha512hmac` - FIPS module dependency
2. `/etc/rdma/mlx4.conf` - RDMA module dependency
3. `switch_root` - Base module dependency
4. `timeout` - Shutdown module dependency
5. `systemd-sysusers`, `systemd-journald` - Dependency chain failures

**Fix - Two parts:**

1. **Added missing binaries to squashfs** (`leviso/src/build/binary_lists.rs`):
```rust
// Added to SBIN
"switch_root",

// Added to BIN (coreutils)
"timeout", "sleep", "true", "false", "test", "[",
```

2. **Omit problematic dracut modules** (`install-tests/src/steps/phase5_boot.rs`):
```rust
"dracut --force --no-hostonly --omit 'fips bluetooth crypt nfs rdma systemd-sysusers systemd-journald systemd-initrd dracut-systemd' /boot/initramfs.img {}"
```

---

### Phase 5: Missing systemd-boot EFI Files (FIXED)

**Symptom:** Step 17 was silently skipped with "SKIPPED: /usr/lib/systemd/boot/efi not in tarball"

**Cause:** The squashfs didn't include systemd-boot EFI binaries. These are in a separate RPM package (`systemd-boot-unsigned`) on the Rocky ISO.

**Fix:** Added `copy_systemd_boot_efi()` function to `leviso/src/squashfs/system.rs`:
- Finds `systemd-boot-unsigned-*.rpm` in ISO contents
- Extracts with `rpm2cpio | cpio`
- Copies EFI files to `/usr/lib/systemd/boot/efi/`

---

### Phase 6: bootctl Install Failure (FIXED)

**Symptom:** `bootctl install --esp-path=/boot/efi --boot-path=/boot` failed with "Directory /boot is not the root of the file system"

**Cause:** bootctl's `--boot-path` option expects `/boot` to be a separate mount point, not part of the root filesystem.

**Fix:** Removed `--boot-path`, added `--no-variables`:
```rust
"bootctl install --esp-path=/boot/efi --no-variables"
```

---

## Current State: BLOCKED

### What Works
All 18 installation steps pass:
```
▶ Step  1: Verify UEFI Boot Mode... PASS (0.5s)
▶ Step  2: Sync System Clock... PASS (8.3s)
▶ Step  3: Identify Target Disk... PASS (0.6s)
▶ Step  4: Partition Disk (GPT)... PASS (9.9s)
▶ Step  5: Format Partitions... PASS (3.1s)
▶ Step  6: Mount Partitions... PASS (4.5s)
▶ Step  7: Mount Installation Media... PASS (0.2s)
▶ Step  8: Extract Base System (recstrap)... PASS (39.8s)
▶ Step  9: Generate fstab... PASS (0.5s)
▶ Step 10: Setup Chroot... PASS (1.0s)
▶ Step 11: Set Timezone... PASS (0.4s)
▶ Step 12: Configure Locale... PASS (0.2s)
▶ Step 13: Set Hostname... PASS (0.3s)
▶ Step 14: Set Root Password... PASS (0.3s)
▶ Step 15: Create User Account... PASS (1.4s)
▶ Step 16: Generate Initramfs... PASS (82.4s)
▶ Step 17: Install Bootloader... PASS (1.3s)
▶ Step 18: Enable Services... PASS (3.4s)
```

### What Fails
The verification phase (booting the installed system) fails:
```
BdsDxe: loading Boot0002 "UEFI Misc Device"
Boot in 3 s...
[QEMU exits]
```

OVMF finds systemd-boot on the disk and shows its menu, but then the boot fails.

---

## Root Cause Analysis: Boot Failure

### Hypothesis 1: Boot Entry Points to Wrong Paths

The boot entry is written to `/mnt/boot/efi/loader/entries/levitateos.conf` with content like:
```
title    LevitateOS
linux    /vmlinuz
initrd   /initramfs.img
options  root=UUID=xxx rw
```

But systemd-boot looks for these files relative to the EFI partition (`/boot/efi`), not the root partition.

**Expected paths on EFI partition (FAT32):**
- `/EFI/systemd/systemd-bootx64.efi` (bootloader) ✓
- `/loader/loader.conf` (config) ✓
- `/loader/entries/levitateos.conf` (boot entry) ✓

**Expected paths for kernel (should be on ext4 `/boot/`):**
- `/boot/vmlinuz` - Kernel
- `/boot/initramfs.img` - Initramfs

The problem: systemd-boot on the EFI partition can't access files on the ext4 root partition directly unless the UEFI firmware can read ext4, or the files are on the EFI partition.

### Hypothesis 2: UEFI Variable Setup

`bootctl install --no-variables` skips setting EFI boot variables. OVMF may not have the correct boot order or entry pointing to systemd-boot.

However, OVMF did find and load systemd-boot, so this is probably not the issue.

### Hypothesis 3: Kernel/Initramfs Location

With our partition layout:
- `/dev/vda1` (FAT32) mounted at `/boot/efi` - Contains bootloader
- `/dev/vda2` (ext4) mounted at `/` - Contains `/boot/vmlinuz` and `/boot/initramfs.img`

systemd-boot needs to find the kernel. Options:
1. Kernel on EFI partition (works but FAT32 has limitations)
2. Kernel on ext4, systemd-boot uses UEFI LoadFile protocol (modern approach)
3. Use XBOOTLDR partition (separate /boot partition)

**Most likely fix:** Copy kernel and initramfs to the EFI partition, or use an XBOOTLDR partition.

---

## Files Modified During This Investigation

### leviso/

| File | Changes |
|------|---------|
| `src/initramfs/mod.rs` | Added BOOT_MODULES (8 modules), renamed from CDROM_MODULES |
| `profile/init_tiny` | Added module loading for all 8 boot modules |
| `src/build/binary_lists.rs` | Added switch_root, timeout, sleep, true, false, test, [ |
| `src/squashfs/system.rs` | Added `copy_systemd_boot_efi()` function |

### install-tests/

| File | Changes |
|------|---------|
| `src/main.rs` | Fixed initramfs path to `initramfs-tiny.cpio.gz`, boot timeout to 60s |
| `src/qemu/console.rs` | Added boot success patterns (login:, LevitateOS Live) |
| `src/steps/phase2_disk.rs` | Changed mount from `/mnt/boot` to `/mnt/boot/efi` |
| `src/steps/phase5_boot.rs` | Updated dracut --omit list, bootctl command, loader paths |

---

## Next Steps to Fix Boot

1. **Option A: Put kernel on EFI partition**
   - Change dracut to output initramfs to `/boot/efi/`
   - Copy kernel to `/boot/efi/`
   - Update boot entry to use paths relative to ESP

2. **Option B: Use XBOOTLDR partition**
   - Create 3 partitions: EFI (512MB), XBOOTLDR (1GB), Root
   - Mount XBOOTLDR at `/boot`
   - systemd-boot auto-discovers XBOOTLDR

3. **Option C: Different bootloader**
   - Use GRUB instead of systemd-boot
   - GRUB can read ext4 and doesn't have these path issues

4. **Option D: Unified Kernel Image (UKI)**
   - Combine kernel + initramfs + cmdline into single EFI binary
   - Place on EFI partition
   - No separate boot entry needed

---

## Test Commands

```bash
# Rebuild everything
cd leviso
cargo run -- build squashfs
cargo run -- build iso

# Run install tests
cd ../install-tests
cargo run -- run

# Run specific phase
cargo run -- run --phase 5
```

---

## Related Files

- `/home/vince/Projects/LevitateOS/distro-spec/src/levitate/boot.rs` - Boot entry spec
- `/home/vince/Projects/LevitateOS/leviso/profile/init_tiny` - Tiny initramfs init script
- `/home/vince/Projects/LevitateOS/recstrap/src/main.rs` - System extractor
