# Investigation Request: aarch64 Fails to Start Shell

**Reported by:** TEAM_325  
**Date:** 2026-01-09  
**Priority:** Medium  
**Status:** Open

---

## Problem Summary

LevitateOS aarch64 boots and **display works**, but fails to start the shell. It drops to a maintenance/failsafe shell instead of the normal user shell.

---

## Evidence

### Screenshot Shows Errors

From `tests/screenshots/levitate_aarch64.png`:

```
[BOOT] ERROR: Failed to spawn init: Elf(WrongArchitecture)
[BOOT] SPEC-4: Initrd required but not found.

[ERROR] Initramfs not found!
Dropping to maintenance shell...

[BOOT] Entering Maintenance Shell (FAILSAFE)
Type 'help' for available commands.
```

### Key Errors

1. **`Failed to spawn init: Elf(WrongArchitecture)`**
   - Init binary is wrong architecture (x86_64 binary on aarch64?)
   
2. **`Initrd required but not found`**
   - Initramfs not being loaded or found

3. **`Initramfs not found!`**
   - Confirms initramfs issue

---

## How to Reproduce

### Quick Test

```bash
# Run the LevitateOS display test
cargo xtask test levitate

# View screenshot
# tests/screenshots/levitate_aarch64.png shows the errors
```

### Manual Test

```bash
# Build aarch64
cargo xtask build all --arch aarch64

# Run
cargo xtask run --arch aarch64

# Observe: Display shows boot errors, drops to maintenance shell
```

---

## Technical Analysis

### Root Cause Hypotheses

1. **Wrong architecture binaries in initramfs**
   - The init binary may be x86_64, not aarch64
   - Check: `file userspace/target/*/release/init`

2. **Initramfs not included in boot**
   - Kernel may not be finding the initramfs
   - Check QEMU `-initrd` argument

3. **Build system architecture mismatch**
   - `cargo xtask build all` may be building userspace for wrong arch
   - Check build targets in `xtask/src/build/commands.rs`

---

## Files to Investigate

```
xtask/src/build/commands.rs          # Build process - is arch passed correctly?
userspace/                           # Userspace binaries - correct target?
kernel/src/init.rs                   # Init loading logic
kernel/src/boot/                     # Boot/initramfs handling
```

---

## Verification Commands

```bash
# Check what architecture the init binary is
file userspace/target/aarch64-unknown-none/release/init

# Check initramfs contents
cpio -t < initramfs.cpio | head

# Verify initramfs is passed to QEMU
# Look for -initrd flag in QEMU command
```

---

## Questions for Investigator

1. Is userspace being built for aarch64 or x86_64?
2. Is the initramfs being passed to QEMU correctly?
3. Why does "WrongArchitecture" ELF error occur?
4. Is there an arch mismatch between kernel and userspace builds?

---

## Acceptance Criteria

- [ ] aarch64 LevitateOS boots to normal shell (not maintenance)
- [ ] `cargo xtask test levitate` shows shell prompt in aarch64 screenshot
- [ ] No "WrongArchitecture" or "Initramfs not found" errors

---

## Related Issues

- See also: `.questions/TEAM_325_x86_64_display_investigation.md` (separate x86_64 issue)

---

## Related Files

- `xtask/src/tests/screenshot_levitate.rs` — Test that captures screenshots
- `tests/screenshots/levitate_aarch64.png` — Shows the errors
