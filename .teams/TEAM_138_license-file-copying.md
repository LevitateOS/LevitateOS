# TEAM_138: License File Copying for Redistributed Binaries

## Status: Complete

## Problem

LevitateOS selectively copies binaries + library dependencies from Rocky Linux, but NOT the license files. Legal compliance requires including licenses for all redistributed software.

**Constraint:** Keep the selective binary copy model - don't switch to copying whole packages.

## Solution

1. Create static mappings (binary → package, library → package) in `distro-spec`
2. Create a `LicenseTracker` that accumulates package names during the build
3. Modify binary/library copy functions to register used packages
4. After build completes, copy `/usr/share/licenses/<pkg>/` for each used package

## Files Created

- `distro-spec/src/shared/licenses.rs` - Static mappings (~300 binaries, ~100 libraries)
- `leviso/src/build/licenses.rs` - LicenseTracker struct and license copying logic

## Files Modified

- `distro-spec/src/shared/mod.rs` - Add `pub mod licenses;`
- `leviso/src/build/mod.rs` - Add `pub mod licenses;`
- `leviso/src/build/libdeps.rs` - Add tracker parameter to copy functions
- `leviso/src/component/executor.rs` - Pass tracker through op execution, add tracking for:
  - `Op::Bin` / `Op::Bins` - registers binaries and their libraries
  - `Op::Bash` - registers bash
  - `Op::SystemdBinaries` - registers systemd
  - `Op::SudoLibs` - registers sudo
  - `Op::UdevHelpers` - registers systemd (udev helpers are part of systemd)
  - `Op::CopyTree` - registers packages for known directories (usr/lib64/security → pam)
- `leviso/src/component/builder.rs` - Create tracker, pass to phases, copy licenses at end
- `leviso/tests/unit_tests.rs` - Updated all tests to pass tracker parameter

## Changes Made

1. **Op::SudoLibs** - Registers sudo package
2. **Op::UdevHelpers** - Registers systemd package
3. **Op::SystemdBinaries** - Registers systemd package
4. **Custom operations** - Register packages for non-binary content:
   - `CopyWifiFirmware` / `CopyAllFirmware` → linux-firmware, microcode_ctl
   - `CopyModules` → kernel
   - `CopyTimezoneData` → tzdata
   - `CopyKeymaps` → kbd
5. **Missing udev helpers** - Added ata_id, scsi_id, cdrom_id, v4l_id, dmi_memory_id, mtd_probe to BINARY_TO_PACKAGE

## API

- `LicenseTracker::register_binary(name)` - Register via BINARY_TO_PACKAGE mapping
- `LicenseTracker::register_library(name)` - Register via LIB_TO_PACKAGE mapping
- `LicenseTracker::register_package(name)` - Register directly (for firmware, kernel, data files)
- `LicenseTracker::copy_licenses(source, staging)` - Copy all tracked license directories

## Design Notes

- PAM tracked via `unix_chkpwd` binary (in AUTH_SBIN → mapped to pam package)
- D-Bus tracked via `dbus-broker` binary
- Systemd tracked via `systemctl`, `systemd`, etc. binaries
- LevitateOS-owned tools don't need Rocky Linux license tracking

## Verification

Added license verification to `testing/fsdbg`:

- `testing/fsdbg/src/checklist/mod.rs` - Added `License` to `CheckCategory` enum
- `testing/fsdbg/src/checklist/rootfs.rs` - Added license verification (section 19)
  - Checks for `usr/share/licenses/` directory existence
  - Verifies critical packages have license directories:
    - Core: glibc, bash, coreutils, systemd, util-linux
    - Auth: pam, shadow-utils
    - Network: NetworkManager, iproute, openssh-clients
    - Filesystem: e2fsprogs, btrfs-progs, dosfstools
    - Compression: gzip, xz, tar
    - Editors: vim-minimal
    - Kernel: kernel, linux-firmware
    - Data: tzdata, kbd
  - Reports total license count

Usage:
```bash
fsdbg verify rootfs.img --type rootfs
```

## Testing Status: READY

**2026-01-28:** Build issues fixed in TEAM_139. ISO builds successfully.

Build output shows license tracking is working:
- `Copied licenses for 116 packages`
- 37 packages reported without license dirs (some are subpackages, kernel modules, or don't ship licenses)

**To test license verification:**
1. Build ISO: `cargo run -- build`
2. Run `fsdbg verify <rootfs.img> --type rootfs`
3. Check for License category results
