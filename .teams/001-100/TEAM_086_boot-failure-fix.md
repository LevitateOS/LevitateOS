# TEAM_086: Fix Post-Reboot Boot Failure

## Status: Complete

## Problem

Installation steps 1-18 pass, but post-reboot verification fails. systemd-boot shows its menu but the kernel doesn't load.

**Root cause:** Two issues:
1. systemd-boot can ONLY read files from FAT-formatted partitions (ESP). Our kernel was on ext4 because ESP was mounted at `/boot/efi`.
2. `bootctl install` in chroot doesn't detect ESP without explicit `--esp-path`

## Solution

1. Mount ESP at `/mnt/boot` instead of `/mnt/boot/efi` (matching distro-spec)
2. Remove `/boot` from squashfs FHS structure (it's a mount point, not a directory)
3. Remove kernel from squashfs (it's on ISO for live boot)
4. Copy kernel from ISO to ESP during installation
5. Add `--esp-path=/boot` to bootctl command (chroot mount detection doesn't work)

## Files Modified

| File | Change |
|------|--------|
| `install-tests/Cargo.toml` | Add `[[bin]]` for boot-test |
| `install-tests/src/bin/boot-test.rs` | NEW: Isolated boot hypothesis test |
| `leviso/src/squashfs/system.rs` | Remove copy_kernel() call and function |
| `leviso/src/build/filesystem.rs` | Remove `/boot` from FHS structure |
| `install-tests/src/steps/phase2_disk.rs` | Mount ESP at /mnt/boot |
| `install-tests/src/steps/phase5_boot.rs` | Copy kernel from ISO, add --esp-path=/boot |
| `install-tests/src/main.rs` | Reduce post-reboot wait |

## Key Fixes

### 1. ESP Mount Point
```diff
- mkdir -p /mnt/boot/efi && mount /dev/vda1 /mnt/boot/efi
+ mkdir -p /mnt/boot && mount /dev/vda1 /mnt/boot
```

### 2. Remove /boot from squashfs
```diff
- "boot",  // in FHS structure
+ // NOTE: /boot is NOT created - it's the ESP mount point
```

### 3. Kernel copy step added
```rust
console.exec("cp /media/cdrom/boot/vmlinuz /mnt/boot/vmlinuz", ...)?;
```

### 4. bootctl with explicit ESP path
```diff
- bootctl install --no-variables
+ bootctl install --esp-path=/boot --no-variables
```

## References

- [Arch Wiki - systemd-boot](https://wiki.archlinux.org/title/Systemd-boot)
- distro-spec: `ESP_MOUNT_POINT = "/boot"`
