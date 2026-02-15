# LevitateOS

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)

**Customization from the package level.**

A Linux distribution where you maintain your own packages. Write simple Rhai recipes, build from source, with local AI assistance. Atomic A/B updates are the default; mutable mode is an explicit opt-in for daredevils (unsafe if you let an LLM author recipes without review).

## The Idea

While other Linux distributions rely on centralized repositories, LevitateOS takes a different approach. You write your own packages called "recipes": declaring how to acquire, build, and install each piece of software.

Too slow? A locally running LLM (SmolLM3) generates, customizes, and maintains your recipes.

By default, LevitateOS is **A/B immutable**: recipes compose changes into the inactive slot, you reboot to trial-boot it once, then commit (or roll back by rebooting). Mutable mode exists, but it is explicitly unsafe.

**Manual install. Full control. AI-assisted packaging.**

## Variants

| | LevitateOS | AcornOS | IuppiterOS |
|---|---|---|---|
| **Purpose** | Daily-driver desktop | Daily-driver desktop | Refurbishment appliance (disk tooling) |
| **Stack** | glibc / systemd / GNU | musl / OpenRC / busybox | musl / OpenRC / busybox |
| **Base** | Rocky Linux RPMs | Alpine Linux APKs | Alpine Linux APKs |
| **Best for** | Maximum compatibility | Smaller, simpler base | Headless-first systems + optional kiosk UI |

All three share the same recipe system and build tooling. Variant specs live in `distro-spec/src/{levitate,acorn,iuppiter}/`.

## Recipe System

Instead of `apt` or `dnf`, LevitateOS uses `recipe`. A recipe is a `.rhai` script that describes how to acquire, build, and install a package:

```rhai
// recipes/ripgrep.rhai
let ctx = #{
    name: "ripgrep",
    version: "14.1.0",
    repo: "BurntSushi/ripgrep",
};

fn acquire(ctx) {
    let tarball = github_download_release(ctx.repo, ctx.version, "*.tar.gz");
    extract(tarball, BUILD_DIR);
}

fn build(ctx) {
    shell_in(BUILD_DIR + "/rg", "cargo build --release");
}

fn install(ctx) {
    let bin = BUILD_DIR + "/rg/target/release/rg";
    shell("install -Dm755 " + bin + " $OUT/usr/bin/rg");
}
```

- Pin exact versions - your system stays reproducible
- Read the build logic in seconds - it's just Rhai script
- Add patches directly - your recipes, your rules
- Full audit trail of what's installed and how

## Local Recipe Assistant

```bash
$ recipe llm "create a recipe for htop"

Drafting recipe for htop...
Recipe saved. Review and run: recipe install htop
```

The assistant runs entirely local - your machine, your data. You review and execute.

## Quick Start

```bash
cd leviso
cargo run -- build    # Downloads ~2GB, builds ISO
cargo run -- run      # Boots in QEMU
```

Requirements: Rust 1.75+, 50GB disk, QEMU with OVMF.

## Installation

Default install is A/B immutable: `EFI + system-a + system-b + var`. Slot B is the inactive slot for atomic updates; persistent state (including home directories) lives under `/var` (`/var/home`).

From the live ISO (like Arch Linux):

```bash
# Partition and format (UEFI/GPT)
DISK=/dev/nvme0n1  # change me

sgdisk -Z "$DISK"
sgdisk \\
  -n 1:0:+1G  -t 1:ef00 -c 1:EFI \\
  -n 2:0:+64G -t 2:8300 -c 2:system-a \\
  -n 3:0:+64G -t 3:8300 -c 3:system-b \\
  -n 4:0:0    -t 4:8300 -c 4:var \\
  "$DISK"

mkfs.fat -F32 -n EFI ${DISK}p1
mkfs.ext4 -L system-a ${DISK}p2
mkfs.ext4 -L system-b ${DISK}p3
mkfs.ext4 -L var ${DISK}p4

# Mount slot A + EFI + /var (leave system-b unmounted)
mount ${DISK}p2 /mnt
mount --mkdir ${DISK}p1 /mnt/boot
mount --mkdir ${DISK}p4 /mnt/var

# Extract and configure
recstrap /mnt
recfstab /mnt >> /mnt/etc/fstab
recchroot /mnt

# Inside chroot
passwd
useradd -m -G wheel myuser
passwd myuser

mkdir -p /boot/EFI/Linux
cp /media/cdrom/boot/uki/levitateos-system-a.efi /boot/EFI/Linux/
cp /media/cdrom/boot/uki/levitateos-system-b.efi /boot/EFI/Linux/

bootctl install

mkdir -p /boot/loader/entries
cat > /boot/loader/entries/levitate-a.conf << 'EOF'
title   LevitateOS (Slot A)
efi     /EFI/Linux/levitateos-system-a.efi
EOF
cat > /boot/loader/entries/levitate-b.conf << 'EOF'
title   LevitateOS (Slot B)
efi     /EFI/Linux/levitateos-system-b.efi
EOF
cat > /boot/loader/loader.conf << 'EOF'
default levitate-a.conf
timeout 3
editor no
EOF

exit

umount -R /mnt
reboot
```

## Project Structure

```
leviso/           # ISO builder
tools/
  recipe/         # Package manager (Rhai-based)
  recstrap/       # System extractor (like pacstrap)
  recfstab/       # Fstab generator (like genfstab)
  recchroot/      # Chroot helper (like arch-chroot)
distro-spec/      # Shared specifications
testing/          # E2E test suites
AcornOS/          # Alpine-based variant
IuppiterOS/       # Alpine-based appliance variant
```

## Hardware Requirements

| Resource | Minimum | LLM-Ready |
|----------|---------|-----------|
| CPU | x86-64-v3 (Haswell+) | 8+ cores |
| RAM | 16 GB | 32-64 GB |
| Storage | 512 GB NVMe | 1-2 TB NVMe |
| GPU | Integrated | RTX 3060+ 12GB |
| Boot | UEFI required | — |

## Status

E2E tested, boots on real hardware.

| Working | In Progress |
|---------|-------------|
| ISO boots (UEFI + BIOS) | Desktop environment recipes |
| Live environment with root shell | Binary package cache |
| recstrap/recfstab/recchroot | Extended hardware testing |
| recipe install runs Rhai scripts | AI installer integration |

## Standing on Giants

- [Arch Linux](https://archlinux.org) - Blueprint for user-controlled distributions
- [Rocky Linux](https://rockylinux.org) - Stable RPM packages we extract
- [Alpine Linux](https://alpinelinux.org) - Proving musl works in production
- [Nix](https://nixos.org) - Pioneering reproducible builds
- [Gentoo](https://gentoo.org) - Compile-time customization philosophy
- [Universal Blue](https://universal-blue.org) - Image-based atomicity
- [Hugging Face](https://huggingface.co) - SmolLM3 for local AI
- [Rust](https://rust-lang.org) - Memory-safe systems programming

## License

MIT OR Apache-2.0

## Links

- [Website](https://levitateos.org)
- [Documentation](docs/)
- [Supply Chain](SUPPLY_CHAIN.md)
