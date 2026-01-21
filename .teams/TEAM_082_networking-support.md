# TEAM_082: Networking Support for Live ISO

**STATUS: COMPLETE ✅**

## Goal
Add full networking support (NetworkManager + WiFi + Ethernet) to the live ISO so users can connect to networks and download packages.

## Scope
- Full NetworkManager with nmcli and wpa_supplicant
- Common WiFi firmware (Intel, Atheros, Realtek, Broadcom, Cypress, MediaTek)
- Auto-start on boot via systemd

## Files Changed
| File | Action |
|------|--------|
| `src/initramfs/network.rs` | CREATE - Main networking module |
| `src/initramfs/mod.rs` | MODIFY - Add `mod network`, call `setup_network()` |
| `src/config.rs` | MODIFY - Add virtio_net to default modules |

## Components Added to Initramfs
- NetworkManager (daemon + nmcli)
- wpa_supplicant (WiFi auth)
- ip (iproute2)
- NetworkManager plugins and configs
- D-Bus policies for NetworkManager
- Common WiFi firmware (~100-150 MB)

## Boot Behavior
1. systemd starts
2. D-Bus socket activates
3. NetworkManager.service starts (WantedBy=multi-user.target)
4. NetworkManager auto-configures Ethernet via DHCP
5. WiFi available via `nmcli device wifi`

## Verification
- `systemctl status NetworkManager` shows active
- `nmcli device` shows network interfaces
- `nmcli connection up <ethernet>` connects (if DHCP available)

---

# Rocky Linux ISO Binary Sources

## Quick Reference: Where to Find Binaries

| Looking for... | Location |
|----------------|----------|
| **Installable RPM packages** | `iso-contents/BaseOS/Packages/{a-z}/` (946 RPMs) |
| **Extra applications** | `iso-contents/AppStream/Packages/{a-z}/` (4504 RPMs) |
| **Pre-extracted binaries** (limited) | `rootfs/usr/bin/`, `rootfs/usr/sbin/` |
| **Kernel** | `iso-contents/images/pxeboot/vmlinuz` |
| **Installer initramfs** | `iso-contents/images/pxeboot/initrd.img` |
| **UEFI bootloader** | `iso-contents/EFI/BOOT/BOOTX64.EFI` |
| **GRUB** | `iso-contents/EFI/BOOT/grubx64.efi` |

## Overview

The Rocky 10.1 DVD ISO (9.3 GB) contains binaries in **multiple locations**:

| Source | Size | Contents | Notes |
|--------|------|----------|-------|
| `BaseOS/Packages/` | 1.4 GB | 946 RPMs | Core system packages |
| `AppStream/Packages/` | 6.4 GB | 4504 RPMs | Applications and extras |
| `images/install.img` | 773 MB | Anaconda installer rootfs | **LIMITED SUBSET** |
| `images/pxeboot/` | 171 MB | vmlinuz + initrd.img | Kernel + installer initramfs |
| `EFI/BOOT/` | 6 MB | UEFI bootloader | BOOTX64.EFI, grubx64.efi |

## Directory Structure

```
Rocky-10.1-x86_64-dvd1.iso
├── BaseOS/
│   ├── Packages/          # 946 RPMs organized a-z
│   │   ├── a/             # RPMs starting with 'a'
│   │   ├── b/             # ...
│   │   └── z/
│   └── repodata/          # DNF/YUM metadata
│
├── AppStream/
│   ├── Packages/          # 4504 RPMs organized a-z
│   └── repodata/
│
├── images/
│   ├── install.img        # 773 MB squashfs → Anaconda installer
│   ├── efiboot.img        # 8.7 MB - EFI boot partition image
│   ├── eltorito.img       # 37 KB - BIOS boot image
│   └── pxeboot/
│       ├── vmlinuz        # 16 MB - Linux kernel
│       └── initrd.img     # 155 MB - Installer initramfs
│
├── EFI/BOOT/
│   ├── BOOTX64.EFI        # shim (Secure Boot)
│   ├── grubx64.efi        # GRUB2 EFI
│   └── grub.cfg
│
└── boot/grub2/            # BIOS boot (grub.cfg, i386-pc modules)
```

## The install.img Problem

**Current implementation** extracts binaries from `images/install.img`:

```
install.img (squashfs) → LiveOS/rootfs.img (ext4) → Anaconda installer environment
```

This rootfs is the **Anaconda installer**, NOT a complete Rocky system. It contains only utilities needed for installation:

| Package | Full RPM | install.img |
|---------|----------|-------------|
| procps-ng | free, ps, top, vmstat, uptime, pgrep, pidof, pmap, w, watch | **Only: ps, top, pidof** |
| coreutils | 100+ utilities | Most included |
| util-linux | mount, fdisk, lsblk, etc. | Most included |

**Missing utilities** the installer doesn't need but users expect:
- `free` - memory info
- `vmstat` - virtual memory stats
- `uptime` - system uptime
- `w` - who is logged in
- `watch` - execute program periodically

## All Binary Sources (Detailed)

### 1. BaseOS/Packages/ (946 RPMs, 1.4 GB)
**Path:** `downloads/iso-contents/BaseOS/Packages/{a-z}/*.rpm`

Core system packages. Organized alphabetically in subdirectories.

```
BaseOS/Packages/b/bash-5.2.26-5.el10.x86_64.rpm
BaseOS/Packages/c/coreutils-9.5-6.el10.x86_64.rpm
BaseOS/Packages/s/systemd-256.10-3.el10.x86_64.rpm
BaseOS/Packages/p/procps-ng-4.0.4-8.el10.x86_64.rpm      # free, vmstat, uptime, etc.
BaseOS/Packages/u/util-linux-2.40.2-5.el10.x86_64.rpm
BaseOS/Packages/n/NetworkManager-1.48.10-2.el10.x86_64.rpm
BaseOS/Packages/w/wpa_supplicant-2.11-4.el10.x86_64.rpm
BaseOS/Packages/l/linux-firmware-20240909-2.el10.noarch.rpm
```

**To extract an RPM:**
```bash
rpm2cpio package.rpm | cpio -idmv
```

### 2. AppStream/Packages/ (4504 RPMs, 6.4 GB)
**Path:** `downloads/iso-contents/AppStream/Packages/{a-z}/*.rpm`

Applications, dev tools, extras. Same a-z organization.

```
AppStream/Packages/g/gcc-14.2.1-7.el10.x86_64.rpm
AppStream/Packages/p/python3-3.12.7-1.el10.x86_64.rpm
AppStream/Packages/n/nano-8.3-1.el10.x86_64.rpm
AppStream/Packages/v/vim-enhanced-9.1.705-1.el10.x86_64.rpm
```

### 3. images/install.img → rootfs/ (773 MB → 1.9 GB extracted)
**Path:** `downloads/rootfs/usr/bin/`, `downloads/rootfs/usr/sbin/`

⚠️ **WARNING: This is the Anaconda INSTALLER, not a complete system!**

Contains only utilities needed for installation. Many common tools are missing:
- ❌ `free` (not needed by installer)
- ❌ `vmstat`, `uptime`, `w`, `watch`
- ✅ `ps`, `top`, `pidof` (installer uses these)
- ✅ Most coreutils
- ✅ Most util-linux tools

**Current build extracts from here, then supplements with RPMs.**

### 4. images/pxeboot/ (171 MB)
**Path:** `downloads/iso-contents/images/pxeboot/`

```
vmlinuz      # 16 MB - Linux kernel
initrd.img   # 155 MB - Installer initramfs (compressed)
```

### 5. EFI/BOOT/ (6 MB)
**Path:** `downloads/iso-contents/EFI/BOOT/`

```
BOOTX64.EFI   # 959 KB - shim (Secure Boot)
grubx64.efi   # 4.0 MB - GRUB2 EFI bootloader
mmx64.efi     # 847 KB - MOK manager
grub.cfg      # GRUB config
```

### 6. boot/grub2/ (BIOS boot)
**Path:** `downloads/iso-contents/boot/grub2/`

```
grub.cfg      # GRUB config for BIOS
i386-pc/      # GRUB modules for BIOS boot
```

### 7. Other Files
**Path:** `downloads/iso-contents/`

```
images/efiboot.img    # 8.7 MB - EFI System Partition image
images/eltorito.img   # 37 KB - BIOS boot image
.treeinfo             # Repo metadata
media.repo            # DNF repo config
RPM-GPG-KEY-Rocky-10  # Package signing key
```

## Solution: Extract from RPMs, not install.img

To get `free` and other missing utilities:

### Option 1: Add specific utilities from RPMs
Extract only the needed binaries from specific RPMs:
```bash
rpm2cpio BaseOS/Packages/p/procps-ng-*.rpm | cpio -idmv ./usr/bin/free
```

### Option 2: Use RPM staging area (already exists)
The build already has `src/rootfs/builder/mod.rs` which extracts RPMs to a staging area. Add procps-ng to the list of RPMs to extract.

### Option 3: Merge RPM contents into rootfs
During `leviso extract`, also extract essential RPMs and merge them with the install.img rootfs.

## Recommended Fix

Modify `src/initramfs/mod.rs` to add `free` to COREUTILS (it will be found if we extract procps-ng RPM), OR modify the rootfs builder to extract procps-ng-*.rpm alongside the install.img extraction.

The cleanest solution is to have a "supplementary RPM" list that gets extracted and merged, since install.img will always be missing utilities that regular users expect.
