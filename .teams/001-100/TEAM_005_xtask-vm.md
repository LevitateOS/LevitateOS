# TEAM 005: xtask VM Infrastructure

## Objective
Create `cargo xtask vm` tooling for testing LevitateOS in a VM with Wayland/Sway support.

## Status: COMPLETE

## What Was Built

### xtask crate (`crates/xtask/`)
- `cargo xtask vm start` - Start VM with QEMU
- `cargo xtask vm stop` - Stop running VM
- `cargo xtask vm status` - Show VM status
- `cargo xtask vm ssh` - SSH into VM
- `cargo xtask vm send <cmd>` - Run command via SSH
- `cargo xtask vm log [-f]` - View serial log
- `cargo xtask vm setup` - Create disk image

### Features
- **virtio-gpu with virgl** - OpenGL acceleration for Wayland
- **virtio-keyboard/mouse** - Proper input for GUI
- **SSH forwarding** - Port 2222 -> VM:22
- **CDROM boot** - `--cdrom arch` or `--cdrom /path/to/iso`
- **UEFI support** - `--uefi` flag (requires OVMF)
- **GUI/headless modes** - `--gui` for display, default headless

### Files Created
- `crates/xtask/Cargo.toml`
- `crates/xtask/src/main.rs`
- `crates/xtask/src/vm.rs`
- `.cargo/config.toml` (xtask alias)

## Usage

```bash
# First time setup
cargo xtask vm setup

# Download Arch ISO
curl -LO https://mirrors.kernel.org/archlinux/iso/latest/archlinux-x86_64.iso
mv archlinux-*.iso .vm/arch.iso

# Boot installer
cargo xtask vm start --gui --cdrom arch

# After install, boot normally
cargo xtask vm start --gui

# SSH in
cargo xtask vm ssh
```

## Next Steps
- Add `--auto` flag to setup for automated Arch installation
- Add shared folder support (9p/virtiofs)
- Add snapshot support
