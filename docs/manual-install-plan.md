# Manual Installation Plan

> WARNING: This document is historical planning notes and does not match the current installer.
>
> Current installs are **A/B immutable by default** (`EFI + system-a + system-b + var`). Slot B is the inactive target for atomic updates + rollback. **Mutable mode** (where it exists) is an explicit opt-in for daredevils, and is unsafe if you let an LLM author recipes without review.
>
> For real installation instructions, follow the docs site pages under `docs/content/src/content/01-getting-started/`.

## Overview

- **No dnf/pacman/apt** - LevitateOS uses `recipe` for package management
- **ISO contains `recipe` binary + prebuilt base packages**
- **`recipe bootstrap`** installs base system to target (like Arch's `pacstrap`)

## Installation Flow

```
Boot ISO → Partition → Mount → recipe bootstrap /mnt → Chroot → Configure → Reboot
```

## What `recipe bootstrap /mnt` Does

Like Arch's `pacstrap`, but using the recipe system:

1. **Creates filesystem structure**
   ```
   /mnt/{bin,boot,dev,etc,home,lib,lib64,mnt,opt,proc,root,run,sbin,srv,sys,tmp,usr,var}
   ```

2. **Installs base recipes to /mnt** (using `--prefix /mnt`)
   - `base` - filesystem hierarchy, coreutils, bash, util-linux
   - `linux` - kernel + modules
   - `linux-firmware` - firmware blobs
   - `systemd` - init system + systemd-boot
   - `networkmanager` - networking
   - `recipe` - so the new system can install more packages

3. **Copies recipes to new root**
   ```
   /mnt/usr/share/recipe/recipes/*.recipe
   ```

4. **Initializes recipe database**
   ```
   /mnt/var/lib/recipe/installed
   ```

## Manual Installation Steps

```bash
# 1. Partition disk
parted /dev/sda --script \
  mklabel gpt \
  mkpart "EFI" fat32 1MiB 513MiB \
  set 1 esp on \
  mkpart "root" ext4 513MiB 100%

# 2. Format partitions
mkfs.fat -F32 -n EFI /dev/sda1
mkfs.ext4 -L root /dev/sda2

# 3. Mount target
mount /dev/sda2 /mnt
mkdir -p /mnt/boot
mount /dev/sda1 /mnt/boot

# 4. Install base system
recipe bootstrap /mnt

# 5. Generate fstab
cat > /mnt/etc/fstab << EOF
UUID=$(blkid -s UUID -o value /dev/sda2)  /      ext4  defaults  0 1
UUID=$(blkid -s UUID -o value /dev/sda1)  /boot  vfat  defaults  0 2
EOF

# 6. Chroot
mount --bind /dev /mnt/dev
mount --bind /proc /mnt/proc
mount --bind /sys /mnt/sys
mount --bind /sys/firmware/efi/efivars /mnt/sys/firmware/efi/efivars
chroot /mnt /bin/bash

# 7. Configure system
ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime
hwclock --systohc
echo "LANG=en_US.UTF-8" > /etc/locale.conf
echo "levitate" > /etc/hostname

cat > /etc/hosts << 'EOF'
127.0.0.1   localhost
::1         localhost
127.0.1.1   levitate.localdomain levitate
EOF

# 8. Set passwords and create user
passwd
useradd -m -G wheel -s /bin/bash yourname
passwd yourname
echo "%wheel ALL=(ALL:ALL) ALL" > /etc/sudoers.d/wheel

# 9. Install bootloader
bootctl install

cat > /boot/loader/loader.conf << 'EOF'
default levitate.conf
timeout 3
editor no
EOF

cat > /boot/loader/entries/levitate.conf << EOF
title   LevitateOS
linux   /vmlinuz-linux
initrd  /initramfs-linux.img
options root=UUID=$(blkid -s UUID -o value /dev/sda2) rw quiet
EOF

# 10. Enable services
systemctl enable NetworkManager

# 11. Exit and reboot
exit
umount -R /mnt
reboot
```

## Comparison

| Arch | LevitateOS |
|------|------------|
| `pacstrap /mnt base linux` | `recipe bootstrap /mnt` |
| `pacman -S firefox` | `recipe install firefox` |
| Downloads from repos | Builds from source recipes |
| `/var/lib/pacman/` | `/var/lib/recipe/installed` |

## TODO

- [ ] Implement `recipe bootstrap` subcommand
- [ ] Define `base` meta-recipe (or package list)
- [ ] Ensure `recipe` binary is on live ISO
- [ ] Ensure all base recipes are on live ISO
