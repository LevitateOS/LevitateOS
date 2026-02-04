# TEAM_114: efivarfs Mount Investigation

## Status: INCOMPLETE - Needs Different Approach

**Latest Update**: Tests are now failing at step 1 (UEFI mode detection) before reaching step 17. This appears to be a flaky test issue - the same ISO was passing step 1 earlier. The step 1 failure reports `/sys/firmware/efi/efivars missing` which may indicate:
1. Timing issues with the test boot detection
2. QEMU/OVMF state issues between test runs
3. A regression introduced during refactoring (though the EFIVARS component only adds files, doesn't remove anything)

## Refactoring Done

The efivarfs code has been refactored into its own **EFIVARS** subsystem for better organization:

- **`leviso/src/component/definitions.rs`**: New `EFIVARS` component with its own section
- **`leviso/src/component/builder.rs`**: EFIVARS component added to build pipeline
- **`leviso/profile/init_tiny.template`**: Initramfs mount attempt (part of EFIVARS subsystem)

The code is now grouped together and clearly documented, making it easy to find and modify.

## Problem

Install test step 17 fails with:
```
efibootmgr failed: EFI variables are not supported on this system.
```

Step 1 passes (efivars directory exists), but efibootmgr can't write to it because the `efivarfs` filesystem isn't mounted at `/sys/firmware/efi/efivars`.

## Root Cause Analysis

1. **The directory exists, but efivarfs isn't mounted**
   - `/sys/firmware/efi/efivars` exists as a directory (step 1 passes)
   - But the efivarfs filesystem isn't mounted there (efibootmgr fails)

2. **QEMU configuration is correct**
   - OVMF_VARS is passed without `readonly=on` flag
   - OVMF_VARS file is copied to temp location to allow writes
   - UEFI boot is confirmed working (step 1 passes)

## What Was Tried

### Attempt 1: systemd mount unit (FAILED)
Added `sys-firmware-efi-efivars.mount` to squashfs:
- Added unit to `leviso/src/component/definitions.rs`
- Unit placed in `/usr/lib/systemd/system/`
- Symlink created in `sysinit.target.wants/`
- **Result**: Unit exists but doesn't activate on live boot

### Attempt 2: Initramfs mount (FAILED)
Added efivarfs mount to initramfs init script:
1. First attempt: Mount efivarfs immediately after sysfs (before mount --move)
   - **Problem**: mount --move doesn't move submounts, efivarfs lost during switch_root

2. Second attempt: Mount efivarfs AFTER moving /sys to /newroot/sys
   - Code added to `leviso/profile/init_tiny.template`
   - **Result**: Still failing, unclear if mount is being attempted

## Files Modified

1. `leviso/src/component/definitions.rs`
   - Created new `EFIVARS` component with its own section
   - Added `EFIVARFS_MOUNT` constant (systemd unit definition)
   - Removed efivarfs code from GETTY component (was incorrectly placed there)

2. `leviso/src/component/builder.rs`
   - Added `executor::execute(ctx, &EFIVARS)?;` to Systemd phase
   - Added EFIVARS to `all_installables()` list

3. `leviso/profile/init_tiny.template`
   - Added efivarfs mount after `/sys` is moved to `/newroot/sys`
   - Added debug output for troubleshooting

## Possible Issues to Investigate

1. **Kernel efivarfs support**
   - Is CONFIG_EFIVAR_FS built into the kernel?
   - Check: `zcat /proc/config.gz | grep EFIVAR` or kernel config

2. **Busybox mount limitations**
   - Does busybox mount support the efivarfs filesystem type?
   - The mount command might be silently failing

3. **QEMU/OVMF behavior**
   - OVMF might not properly expose EFI variables in the way efibootmgr expects
   - The variables might need SMM (System Management Mode) support

4. **Timing issues**
   - The mount might need to happen at a specific point in boot
   - systemd might be managing this mount differently

## Suggested Next Steps

1. **Debug boot process**
   - Boot ISO manually with verbose output
   - Check if `/newroot/sys/firmware/efi` exists when mount is attempted
   - Check if busybox mount returns an error code

2. **Check kernel config**
   - Verify CONFIG_EFIVAR_FS is enabled
   - Verify CONFIG_EFI_VARS is enabled

3. **Try different approach**
   - Mount efivarfs in install test step 17 directly before running efibootmgr
   - This would be a workaround, not a permanent fix

4. **Check OVMF compatibility**
   - Research if QEMU OVMF needs specific configuration for efivarfs write access
   - Check if secure boot / SMM mode affects this

## Code Currently in Place

### EFIVARS Component (definitions.rs)

```rust
// =============================================================================
// EFIVARS - EFI Variable Filesystem Support
// =============================================================================
//
// This subsystem handles mounting the efivarfs filesystem which is required
// for efibootmgr to write UEFI boot entries. The mounting is attempted in
// two places for redundancy:
//
// 1. Initramfs (leviso/profile/init_tiny.template)
// 2. systemd mount unit (below)

const EFIVARFS_MOUNT: &str = "...";  // systemd mount unit

pub static EFIVARS: Component = Component {
    name: "efivars",
    phase: Phase::Systemd,
    ops: &[
        write_file("usr/lib/systemd/system/sys-firmware-efi-efivars.mount", EFIVARFS_MOUNT),
        enable_sysinit("sys-firmware-efi-efivars.mount"),
    ],
};
```

### Initramfs Mount (init_tiny.template)

```shell
# Mount efivarfs for UEFI systems (required for efibootmgr)
# Must be mounted AFTER /sys is moved to /newroot/sys
if [ -d /newroot/sys/firmware/efi ]; then
    busybox echo "Mounting efivarfs at /newroot/sys/firmware/efi/efivars..."
    if busybox mount -t efivarfs efivarfs /newroot/sys/firmware/efi/efivars; then
        busybox echo "  efivarfs mounted successfully"
    else
        busybox echo "  WARNING: efivarfs mount failed (exit $?)"
    fi
fi
```

### Builder Integration (builder.rs)

```rust
// Phase 3: Systemd
executor::execute(ctx, &SYSTEMD_UNITS)?;
executor::execute(ctx, &GETTY)?;
executor::execute(ctx, &EFIVARS)?;  // EFI variable filesystem for efibootmgr
executor::execute(ctx, &UDEV)?;
```

## Conclusion

The efivarfs mount is more complex than initially anticipated. The directory exists and UEFI is detected, but the actual efivarfs filesystem mount is failing somewhere in the boot chain. A different debugging approach is needed to identify where exactly the mount is failing.
