# TEAM_048: Leviso Learning & Testing

## Objective
Learn the leviso codebase, roadmap, and docs data to understand the canonical state of the system.

## Status
- [x] Study leviso structure
- [x] Study roadmap
- [x] Study docs data
- [x] Run QEMU in GUI mode for user testing
- [x] Added efivarfs mount to init
- [x] Added gzip, timedatectl to initramfs
- [ ] Add systemd as init (IN PROGRESS)

## Current Bug (systemd integration)

**Error:**
```
/init: error while loading shared libraries: libseccomp.so.2: cannot open shared object file: No such file or directory
```

**Cause:** systemd binary requires `libseccomp.so.2` for seccomp sandboxing, but we didn't copy it to the initramfs.

**Fix needed:** Add libseccomp to the library copying in `setup_systemd()` or ensure ldd catches it from the rootfs.

## Understanding

### What leviso IS (canonical)
A minimal bootable Linux ISO builder. Current capabilities:

**Commands:**
- `leviso build` - Full build (download + extract + initramfs + iso)
- `leviso download` - Download Rocky 10 Minimal ISO
- `leviso extract` - Extract squashfs rootfs from Rocky ISO
- `leviso initramfs` - Build minimal initramfs with bash + coreutils
- `leviso iso` - Create bootable hybrid BIOS/UEFI ISO
- `leviso test [--gui] [--bios]` - Test in QEMU

**What's in the ISO (Phase 1 + 2 complete):**
- Bash shell as init
- Coreutils: ls, cat, cp, mv, rm, mkdir, chmod, chown, ln, etc.
- Disk utilities: lsblk, blkid, fdisk, parted, wipefs, mkfs.ext4, mkfs.fat
- System tools: mount, umount, hostname, chroot, hwclock, date, loadkeys
- Keymaps for loadkeys

**What's NOT yet in the ISO:**
- User management (passwd, useradd, groupadd)
- Text editors (nano, vi)
- Networking (ip, ping, dhcpcd, nmcli)
- Package manager (recipe)
- Bootloader tools (bootctl)
- Systemd (systemctl, timedatectl, locale-gen)

### Docs vs Reality Gap
The installation docs show a full flow including:
- NetworkManager/nmcli
- recipe bootstrap
- nano for editing
- systemctl enable
- bootctl install

These tools don't exist yet in the live ISO. The roadmap shows the planned path.

## Notes
- Rocky is ONLY for userspace binaries - kernel should eventually be vanilla
- Rocky 10 is NON-NEGOTIABLE
- UEFI boot works by default (uses OVMF if available)
