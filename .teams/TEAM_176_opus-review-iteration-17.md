# TEAM_176: Opus Review — After Iteration 17

**Date**: 2026-02-04
**Status**: COMPLETE
**Type**: Review (Opus reviewing haiku iterations 13-17)

## Scope Reviewed

- AcornOS: 3 commits (ssh.rs sshd_config fix, live.rs test instrumentation, definitions.rs EROFS size)
- IuppiterOS: 3 commits (rootfs build system + 2 doc fixes — d4983fa, f5b0bad, e6279c2)
- distro-spec: 2 commits (cargo fmt, iuppiter skeleton)
- distro-builder: 0 commits in review window

## Bugs Found and Fixed

### Critical: IuppiterOS imports from `distro_spec::acorn` (5 files)

Every IuppiterOS artifact builder and utility file imported constants from the *wrong distro spec module*:
- `qemu.rs` — wrong ISO_FILENAME, QEMU_MEMORY_GB, QEMU_DISK_GB
- `iso.rs` — wrong ISO_LABEL, ISO_FILENAME, OS_NAME, ROOTFS paths
- `initramfs.rs` — wrong BOOT_MODULES (missing SAS/SCSI), wrong ISO_LABEL
- `rebuild.rs` — wrong INITRAMFS_LIVE_OUTPUT, ISO_FILENAME, ROOTFS_NAME
- `keys.rs` — wrong ALPINE_KEYS source (functionally identical, but wrong dependency)

This would cause IuppiterOS to build with AcornOS's ISO label ("ACORNOS" vs "IUPPITER"), AcornOS's boot modules (no SAS/SCSI), and AcornOS's filenames.

### Critical: IuppiterOS definitions.rs is AcornOS with wrong branding

The entire definitions.rs was copied from AcornOS with only hostname/hosts changed:
- OS_RELEASE, MOTD, ISSUE all said "AcornOS"
- test_branding_content() asserted "AcornOS" — **the test was passing but validating the wrong distro**
- fstab, inittab, network interfaces comments all said "AcornOS"

### Critical: IuppiterOS includes excluded packages

Per PRD, IuppiterOS excludes WiFi, LUKS/LVM, Btrfs. But definitions.rs included:
- `cryptsetup`, `lvm`, `mkfs.btrfs`, `sgdisk` in ADDITIONAL_SBINS
- `iwd` in OPENRC_SCRIPTS
- WiFi directory `var/lib/iwd` in NETWORK component
- WiFi firmware copy in FIRMWARE component

### Minor: SSH key comment and URLs

- ssh.rs: `root@acornos` → `root@iuppiter`
- live.rs: `distro_spec::acorn` import → `distro_spec::iuppiter`
- live.rs: hardcoded `acorn/docs` URL → `iuppiter/docs`
- builder.rs: print messages said "AcornOS"

## AcornOS Review

AcornOS commits were clean. The sshd_config fix (line-by-line matching) was already reviewed in the previous opus iteration. The live.rs test instrumentation and EROFS size optimization were well-implemented.

## Files Modified

IuppiterOS (10 files):
- `src/artifact/initramfs.rs` — fix import
- `src/artifact/iso.rs` — fix import + doc
- `src/artifact/rootfs.rs` — fix doc
- `src/component/builder.rs` — fix doc + print
- `src/component/custom/live.rs` — fix import + URL
- `src/component/custom/ssh.rs` — fix key comment
- `src/component/definitions.rs` — fix branding, packages, firmware, test
- `src/keys.rs` — fix import
- `src/qemu.rs` — fix import + doc
- `src/rebuild.rs` — fix import + doc

## Key Decisions

- Removed cryptsetup/lvm/mkfs.btrfs/sgdisk from IuppiterOS ADDITIONAL_SBINS — these violate the appliance design
- Changed FIRMWARE component from CopyWifiFirmware to just creating the directory — IuppiterOS is wired-only
- Did NOT fix remaining cosmetic AcornOS references in qemu.rs user-facing strings (low priority, will be caught in next pass)

## Remaining AcornOS References in IuppiterOS

There are still ~40 cosmetic "acorn" references in IuppiterOS files (qemu.rs error messages, iso.rs strings, etc.). These are user-facing strings that won't affect functionality since the distro_spec imports are now correct. They should be cleaned up in a future iteration but are low priority.
