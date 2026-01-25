# TEAM_079: Leviso Audit Priority 1 Implementation

## Objective

Implement Priority 1 items from the leviso audit comparing against archiso:
- P1.0: Remove BIOS/Legacy Support (UEFI-only)
- P1.1: Add CPU Microcode Loading
- P1.2: Add Boot Recovery/Emergency Shell
- P1.3: Add Download Checksum Verification

## Status: COMPLETE

All Priority 1 items have been implemented and verified to compile.

## Changes Made

### P1.0: Remove BIOS/Legacy Support
- `download.rs`: Removed `download_syslinux()` function and `SYSLINUX_URL` constant
- `iso.rs`: Complete rewrite to UEFI-only:
  - Removed syslinux usage and isolinux config
  - Removed BIOS boot paths from xorriso
  - Removed BIOS-only fallback path
  - Added `-partition_offset 16` for better GPT compatibility
  - Fails if EFI files not found (UEFI required)
- `qemu.rs`: Removed `force_bios` parameter, OVMF now required
- `main.rs`: Removed `--bios` flag from Run command

### P1.1: CPU Microcode Loading
- `profile/init`: Added microcode loading section after efivars mount
  - Detects CPU vendor (Intel vs AMD) via /proc/cpuinfo
  - Loads microcode from /lib/firmware/{intel,amd}-ucode if available
  - Graceful fallback if /dev/cpu/microcode not available

### P1.2: Boot Recovery/Emergency Shell
- `iso.rs`: Added "LevitateOS (Emergency Shell)" GRUB menu entry
  - Passes `emergency` kernel cmdline parameter
- `profile/init`: Added emergency mode detection before systemd handoff
  - Checks for `emergency` in /proc/cmdline
  - Drops to bash with helpful commands listed
  - Continues boot to systemd on `exit`

### P1.3: Download Checksum Verification
- `download.rs`: Added `verify_checksum()` function
  - Uses sha256sum to verify downloads
  - Deletes corrupted files on mismatch
- Added SHA256 verification to `download_rocky()` and `download_rocky_dvd()`
- Constant: `ROCKY_DVD_SHA256` with Rocky 10.1 checksum

### Documentation Updates
- `leviso/CLAUDE.md`: Removed `--bios` example and syslinux reference
- `leviso/.gitignore`: Removed syslinux from comment

## Files Modified
- leviso/src/download.rs
- leviso/src/iso.rs
- leviso/src/qemu.rs
- leviso/src/main.rs
- leviso/profile/init
- leviso/CLAUDE.md
- leviso/.gitignore

## Verification

```bash
cd leviso
cargo build  # ✓ Compiles successfully (warnings are pre-existing)
```

### To Test
1. **UEFI Boot**: `cargo run -- run` (should boot UEFI, require OVMF)
2. **Emergency Shell**: In GRUB menu, select "Emergency Shell" option
3. **Checksum**: Delete Rocky ISO and re-download to verify checksum runs

## Notes

- The `--bios` flag no longer exists - running it will show clap's error
- OVMF is now required for QEMU - error message shows install instructions
- Emergency shell provides pre-systemd recovery without panic=30 reboot
- Microcode loading depends on firmware packages being in initramfs
