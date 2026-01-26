# TEAM_122: UKI Implementation for Live ISO

## Status: COMPLETE

## Summary

Replaced GRUB with systemd-boot and UKIs (Unified Kernel Images) for the live ISO boot process.

## Why UKI?

- **Single signed artifact**: kernel + initramfs + cmdline in one PE binary
- **Secure Boot ready**: Single file to sign and verify
- **Auto-detection**: systemd-boot auto-detects UKIs in /EFI/Linux/
- **Simpler**: No grub.cfg complexity
- **Modern**: Used by Fedora, Arch, and systemd-first distros

## Boot Architecture Change

**Before (GRUB):**
```
UEFI → BOOTX64.EFI → grubx64.efi → grub.cfg → vmlinuz + initramfs-live.img
```

**After (UKI):**
```
UEFI → BOOTX64.EFI (systemd-boot) → /EFI/Linux/*.efi (UKIs)
```

## Files Changed

### New Files

| File | Purpose |
|------|---------|
| `distro-spec/src/shared/uki.rs` | UKI constants (paths, filenames) |
| `distro-spec/src/levitate/uki.rs` | UKI boot entries definition |
| `leviso/src/artifact/uki.rs` | UKI builder using `ukify` |

### Modified Files

| File | Change |
|------|--------|
| `distro-spec/src/shared/mod.rs` | Export uki module |
| `distro-spec/src/shared/iso.rs` | Increase EFIBOOT_SIZE_MB from 16 to 200 |
| `distro-spec/src/levitate/mod.rs` | Export uki module and constants |
| `leviso/src/artifact/mod.rs` | Export uki module |
| `leviso/src/artifact/iso.rs` | Replace GRUB setup with UKI |
| `leviso/src/preflight/host_tools.rs` | Add ukify and systemd-boot checks |

## Host Requirements

```bash
# Install required tools (Fedora/Rocky)
sudo dnf install systemd-ukify systemd-boot

# Verify
ukify --version
ls /usr/lib/systemd/boot/efi/systemd-bootx64.efi
```

## UKI Boot Entries

The ISO includes three UKIs:

1. `levitateos-live.efi` - Normal boot
2. `levitateos-emergency.efi` - Emergency shell
3. `levitateos-debug.efi` - Debug mode

## Verification

```bash
# Rebuild ISO with UKI support
cd leviso
cargo run -- iso

# Verify UKIs were created
ls -la output/iso-root/EFI/Linux/

# Boot test
cargo run -- run
```

## Design Decisions

1. **No GRUB fallback**: Clean break - systemd-boot + UKI only
2. **No objcopy fallback**: ukify is required - no manual UKI creation
3. **200MB EFI boot image**: Each UKI is ~50MB (kernel + initramfs)
4. **Host tools check**: Preflight checks fail fast if ukify or systemd-boot missing

## Testing

- `cargo check --package distro-spec` - PASS
- `cargo check --package leviso` - PASS
- `cargo test --package distro-spec --package leviso` - PASS
- `cargo build --workspace` - PASS

## Terminology Update

Updated user-facing messages from "squashfs" to "rootfs (EROFS)" terminology:

| File | Change |
|------|--------|
| `leviso/src/commands/build.rs` | "Building EROFS rootfs image..." |
| `leviso/src/commands/show.rs` | "Rootfs (EROFS):" in status display |
| `leviso/src/component/builder.rs` | "Building complete system for rootfs (EROFS)..." |
| `leviso/src/clean.rs` | "Removing EROFS rootfs..." |
| `leviso/src/main.rs` | Updated CLI help text to mention EROFS |

CLI subcommand names (`leviso build squashfs`, `leviso clean squashfs`) kept for backward compatibility.

## Installed System UKI Support

Added pre-built UKIs for installed systems (daily driver boot):

### New Files
| File | Change |
|------|--------|
| `distro-spec/src/shared/uki.rs` | Added `UKI_INSTALLED_FILENAME`, `UKI_INSTALLED_RECOVERY_FILENAME` |
| `distro-spec/src/levitate/paths.rs` | Added `UKI_INSTALLED_ISO_DIR`, `UKI_INSTALLED_ISO_PATH`, etc. |
| `distro-spec/src/levitate/uki.rs` | Added `UKI_INSTALLED_ENTRIES` array |
| `leviso/src/artifact/uki.rs` | Added `build_installed_ukis()` function |
| `leviso/src/artifact/iso.rs` | Build installed UKIs during ISO creation |

### ISO Contents
```
ISO/
├── EFI/Linux/                    # Live boot UKIs
│   ├── levitateos-live.efi
│   ├── levitateos-emergency.efi
│   └── levitateos-debug.efi
└── boot/uki/                     # Installed system UKIs
    ├── levitateos.efi            # Normal boot (root=LABEL=root rw)
    └── levitateos-recovery.efi   # Recovery mode (single user)
```

### Installation Usage
During installation, users copy the pre-built UKI to their system:
```bash
# After recstrap extracts the system
mount /dev/sdX1 /mnt/boot          # Mount ESP
mkdir -p /mnt/boot/EFI/Linux
cp /media/cdrom/boot/uki/levitateos.efi /mnt/boot/EFI/Linux/
bootctl install --esp-path=/mnt/boot
```

systemd-boot auto-discovers UKIs in `/boot/EFI/Linux/` - no boot entry file needed.

### Cmdline
Installed UKIs use `root=LABEL=root rw` - user must label their root partition `root`.
Can be edited at boot time via systemd-boot if different label/UUID needed.

## Related

- TEAM_121: EROFS migration (prerequisite - complete)
- Future: Secure Boot signing support
