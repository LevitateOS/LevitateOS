# LevitateOS Installer Requirements

## Overview

A Rust-based installer inspired by archinstall, but with **FunctionGemma LLM** for natural language understanding. Users describe what they want, the LLM translates to actions.

## Design Philosophy

1. **Conversational** - User types natural language, not menu selections
2. **Transparent** - Always show what will happen before doing it
3. **Recoverable** - Mistakes can be undone where possible
4. **Minimal** - Only install what's needed

---

## Installation Stages

### 1. Disk Configuration

**What it does:**
- List available disks and partitions
- Create partition tables (GPT preferred)
- Create partitions (EFI, swap, root, home)
- Format filesystems (ext4, btrfs, xfs)
- Setup encryption (LUKS) if requested

**User interaction examples:**
```
> use the whole 500gb nvme drive
> dual boot with windows on sda
> encrypted root with separate home partition
> just use the defaults
```

**LLM output (structured):**
```json
{
  "action": "partition",
  "disk": "/dev/nvme0n1",
  "scheme": [
    {"mount": "/boot/efi", "size": "512M", "fs": "vfat"},
    {"mount": "swap", "size": "8G", "fs": "swap"},
    {"mount": "/", "size": "rest", "fs": "ext4"}
  ]
}
```

### 2. System Installation

**What it does:**
- Mount target partitions
- Copy live system to target (rsync or unsquash)
- Generate /etc/fstab
- Install bootloader (GRUB for BIOS+UEFI compat)

**User interaction:**
```
> install the system
> (mostly automatic, shows progress)
```

### 3. System Configuration

**What it does:**
- Set hostname
- Set timezone
- Set locale/language
- Set keyboard layout

**User interaction examples:**
```
> hostname is my-laptop
> timezone new york
> us english keyboard
```

### 4. User Setup

**What it does:**
- Create user account(s)
- Set passwords
- Configure sudo access

**User interaction examples:**
```
> create user vince with sudo
> password is hunter2
```

### 5. Bootloader

**What it does:**
- Install GRUB to EFI partition or MBR
- Generate grub.cfg
- Set as default boot entry

**User interaction:**
```
> install bootloader
> (automatic based on disk config)
```

### 6. Finalize

**What it does:**
- Unmount partitions
- Offer to reboot

---

## FunctionGemma Integration

### Model Requirements
- Base: FunctionGemma 2B (fits in ~4GB RAM)
- LoRA adapter: Trained on installation commands
- Inference: CPU-only (live ISO environment)

### Input/Output Contract

**Input:** Natural language user request
**Output:** JSON action object

### Action Types

```rust
enum InstallerAction {
    ListDisks,
    Partition { disk: String, scheme: Vec<Partition> },
    Format { partition: String, filesystem: String },
    Mount { partition: String, mountpoint: String },
    CopySystem { source: String, target: String },
    SetHostname { name: String },
    SetTimezone { zone: String },
    CreateUser { name: String, sudo: bool },
    SetPassword { user: String, password: String },
    InstallBootloader { target: String },
    Reboot,
    Help,
    Clarify { question: String },  // LLM needs more info
}
```

### Conversation Flow

```
1. User types request
2. LLM parses → InstallerAction
3. If Clarify → ask user, goto 1
4. Show plan to user
5. User confirms (y/n)
6. Execute action
7. Show result
8. Loop
```

---

## Technical Requirements

### Rust Crates Needed
- `serde` / `serde_json` - Action serialization
- `nix` - Low-level system calls (mount, chroot)
- `gpt` / `mbr` - Partition table manipulation
- `block-utils` - Disk enumeration
- `indicatif` - Progress bars
- `rustyline` - Line editing / history

### FunctionGemma Integration
- Load GGUF model via `llama.cpp` bindings or `candle`
- Apply LoRA adapter
- Tokenize input, generate, parse output

### Privilege Handling
- Run as root (installer context)
- Or use capability-based access for specific operations

---

## Non-Goals (for v1.0)

- Network configuration (use NetworkManager post-install)
- Package selection (minimal base only)
- Desktop environment choice (headless focus)
- RAID/LVM (future)
- Secure boot signing (future)

---

## Open Questions

1. **How to handle LLM failures?** Fall back to explicit prompts?
2. **Offline model loading** - Bundle model in ISO? Separate download?
3. **Disk operations library** - Use `parted` CLI or pure Rust?
4. **Copy method** - rsync vs unsquashfs vs cp?

---

## Example Full Session

```
LevitateOS Installer v1.0
Type what you want to do. I'll help you install.

> what disks do I have

Found:
  /dev/sda - 500GB SSD (Samsung 870 EVO)
  /dev/sdb - 1TB HDD (WD Blue)

> install on the ssd, whole disk, encrypted

Plan:
  Disk: /dev/sda (500GB)
  - /dev/sda1: 512MB EFI (vfat)
  - /dev/sda2: 499.5GB LUKS → ext4 root

  WARNING: This will ERASE all data on /dev/sda

Proceed? [y/n] > y

Creating partition table... done
Creating EFI partition... done
Creating LUKS container...
  Enter encryption passphrase: ********
  Confirm: ********
Formatting root... done
Mounting... done
Copying system... [████████████████████] 100%
Generating fstab... done

> timezone los angeles

Set timezone to America/Los_Angeles

> create user vince with sudo

Created user 'vince' with sudo access
Set password for vince: ********

> install bootloader

Installing GRUB to /dev/sda... done
Generating grub.cfg... done

> done

Unmounting partitions... done

Installation complete! Remove the USB and reboot.
Reboot now? [y/n] > y
```
