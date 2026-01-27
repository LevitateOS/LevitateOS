# TEAM_131: UKI Boot Debug - systemd-stub 258.x Regression

## Status: FIX IMPLEMENTED - AWAITING REBUILD

## Problem

The E2E installation test fails during **LIVE ISO boot** with:

```
[    0.832118] check access for rdinit=/init failed: -2, ignoring
[    1.226206] VFS: Cannot open root device "LABEL=LEVITATEOS" or unknown-block(0,0): error -6
```

The kernel cannot find `/init` in the initramfs. Error `-2` is `ENOENT` (file not found).

## Root Cause

**The initramfs is NOT being passed from systemd-stub to the kernel.**

Evidence:
- Initramfs file contains `/init` (verified)
- UKI has initramfs in `.initrd` section (verified)
- Kernel never prints "Unpacking initramfs..." (initrd_start is NULL)
- Kernel never prints EFI stub initrd loading message

### systemd-stub 258.x Regression

The system uses **systemd 258.3-2.fc43**, which is affected by a known regression:
- GitHub Issue: [systemd/systemd#38104](https://github.com/systemd/systemd/issues/38104)
- Regression commit: `cab9c7b5a42effa8a45611fc6b8556138c869b5f` ("stub: call inner kernel directly")
- Fix: PR #38220

## Changes Made

### 1. Added `efi=debug` to kernel cmdline

**Files changed:**
- `distro-spec/src/shared/iso.rs` - Added `EFI_DEBUG` constant
- `distro-spec/src/shared/mod.rs` - Exported `EFI_DEBUG`
- `distro-spec/src/levitate/mod.rs` - Re-exported `EFI_DEBUG`
- `leviso/src/artifact/uki.rs` - Added `efi=debug` to live and installed UKI cmdlines

This will produce verbose EFI stub output to help diagnose the issue.

### 2. Added initramfs debug tool

**Files added:**
- `testing/install-tests/src/bin/initramfs-debug.rs`
- Updated `testing/install-tests/Cargo.toml`

The `initramfs-debug` binary boots the kernel and initramfs directly via QEMU's
`-kernel`/`-initrd` flags, bypassing UKI/UEFI entirely. This isolates whether
the issue is in the initramfs itself or in the UKI/systemd-stub boot chain.

Usage:
```bash
cd testing/install-tests
cargo run --bin initramfs-debug
```

If this passes but UKI boot fails → problem is systemd-stub/ukify
If this fails → problem is in the initramfs

### 3. Added `build_direct_boot_debug()` to QemuBuilder

**File changed:**
- `testing/install-tests/src/qemu/builder.rs`

New method for debugging that bypasses UEFI entirely. Explicit about being
debug-only and not for production testing.

## Next Steps

1. **Rebuild the ISO** to pick up the `efi=debug` changes:
   ```bash
   cd leviso && cargo run -- build
   ```

2. **Run initramfs debug test** to verify initramfs works independently:
   ```bash
   cd testing/install-tests && cargo run --bin initramfs-debug
   ```

3. **If initramfs works but UKI fails**, options are:
   - Update systemd packages (check if fc43 has the fix)
   - Use GRUB as fallback bootloader
   - Wait for systemd 259 with the fix

## References

- [Kernel patch for rdinit warning](https://patchew.org/linux/20250707091411.1412681-1-lillian@star-ark.net/)
- [systemd-stub regression #38104](https://github.com/systemd/systemd/issues/38104)
- [Arch Wiki: Unified kernel image](https://wiki.archlinux.org/title/Unified_kernel_image)
- Related: TEAM_128 (E2E Test UKI Violation)
