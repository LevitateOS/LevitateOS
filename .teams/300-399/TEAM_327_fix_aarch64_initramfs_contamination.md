# TEAM_327: Fix aarch64 Initramfs Cross-Architecture Contamination

**Date:** 2026-01-09  
**Status:** COMPLETE  
**Bug Report:** `.teams/TEAM_325_aarch64_init_investigation.md`

---

## Summary

Fixed the root cause of aarch64 failing to spawn init with `Elf(WrongArchitecture)` error.

---

## Root Cause Analysis

### Symptom
```
[BOOT] ERROR: Failed to spawn init: Elf(WrongArchitecture)
[BOOT] SPEC-4: Initrd required but not found.
```

### Investigation Path

1. **Initial Hypothesis:** Userspace built for wrong architecture
   - **Result:** RULED OUT - `file` command confirmed aarch64 init is ARM64

2. **Second Hypothesis:** Build system not passing arch correctly
   - **Result:** RULED OUT - `build_userspace()` correctly maps arch to target

3. **Root Cause Found:** Single `initramfs.cpio` shared across architectures
   - When `screenshot_levitate.rs` runs:
     ```rust
     build::build_all("aarch64")?;  // Creates initramfs.cpio with aarch64 binaries
     build::build_all("x86_64")?;   // OVERWRITES initramfs.cpio with x86_64 binaries!
     
     run_arch("aarch64");  // Uses x86_64 initramfs.cpio → WrongArchitecture!
     run_arch("x86_64");
     ```

---

## Fix Applied

Changed from single `initramfs.cpio` to arch-specific files:
- `initramfs_aarch64.cpio` - Contains ARM64 binaries
- `initramfs_x86_64.cpio` - Contains x86-64 binaries

### Files Modified

| File | Change |
|------|--------|
| `xtask/src/build/commands.rs` | `create_initramfs()` creates `initramfs_{arch}.cpio` |
| `xtask/src/qemu/builder.rs` | `QemuBuilder::new()` selects arch-specific initramfs |
| `xtask/src/vm/exec.rs` | Uses `initramfs_aarch64.cpio` for aarch64 |
| `xtask/src/support/clean.rs` | Cleans both arch-specific files |
| `xtask/src/tests/behavior.rs` | Uses `initramfs_{arch}.cpio` |
| `xtask/src/tests/keyboard_input.rs` | Uses `initramfs_aarch64.cpio` |
| `xtask/src/tests/serial_input.rs` | Uses `initramfs_aarch64.cpio` |
| `xtask/src/tests/shutdown.rs` | Uses `initramfs_aarch64.cpio` |

---

## Verification

```bash
# Both initramfs files now exist with correct architectures:
$ file /tmp/verify_aarch64/init
init: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked

$ file /tmp/verify_x86_64/init  
init: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked
```

---

## Handoff Checklist

- [x] Root cause identified and documented
- [x] Fix implemented (arch-specific initramfs files)
- [x] All xtask code compiles
- [x] Both arch initramfs files verified
- [ ] Full `cargo xtask test levitate` pending user verification

---

## Notes

The ISO build (`build_iso`) copies the arch-specific initramfs to a fixed name (`initramfs.cpio`) inside the ISO, which is correct - the bootloader expects that name.
