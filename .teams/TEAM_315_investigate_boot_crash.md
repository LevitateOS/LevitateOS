# TEAM_315: Investigate Boot Crash After GDT/IDT/ELF Changes

## Bug Report

**Symptom:** Kernel crashes immediately on boot (triple fault)

**Output:** `SMGPCLEHXMRjka` followed by crash

## Root Cause

**The bug was NOT caused by GDT/IDT/ELF changes.** It was a pre-existing regression introduced between v0.1.0-alpha and HEAD.

### Technical Details

The diagnostic inline asm blocks added in commits after v0.1.0-alpha did not declare proper clobber lists:

```rust
unsafe {
    core::arch::asm!("mov al, 'T'", "out dx, al"); // No clobbers declared!
}
let switch_cr3 = !is_limine_boot;  // Value corrupted by asm above
los_hal::arch::init_with_options(switch_cr3);  // Receives wrong value
```

This caused:
1. `switch_cr3` boolean to be corrupted (showed `F` in kernel, but HAL received `1`/true)
2. HAL ran MMU init code meant for multiboot path on Limine boot
3. Page fault during MMU init with IDT not yet loaded
4. Triple fault → crash

## Fix

Complete reset of `kernel/` and `crates/hal/` directories to v0.1.0-alpha:
```bash
git checkout v.0.1.0-alpha -- kernel/ crates/hal/
cargo clean && cargo xtask run default --arch x86_64
```

## Verification

Boot now succeeds: `[SUCCESS] LevitateOS System Ready`

## Lessons Learned

1. **Always declare clobbers in inline asm** - even for "simple" serial output
2. **Test boot after adding any asm** - register corruption is subtle
3. **Golden logs can mask real issues** - the original golden log was stale
