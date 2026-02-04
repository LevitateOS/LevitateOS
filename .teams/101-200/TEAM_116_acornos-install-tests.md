# TEAM_116: AcornOS Install Tests

## Status: COMPLETE

## Goal
Add AcornOS support to the existing `testing/install-tests` crate via a `DistroContext` trait that parameterizes distro-specific behavior (OpenRC vs systemd, GRUB vs systemd-boot).

## Approach
In-place parameterization with `--distro acorn` flag. 75% of test code is shared (QEMU, console, disk operations).

## Key Differences

| Aspect | LevitateOS | AcornOS |
|--------|-----------|---------|
| Init system | systemd | OpenRC |
| Service enable | `systemctl enable X` | `rc-update add X runlevel` |
| Service status | `systemctl is-active X` | `rc-service X status` |
| Failed services | `systemctl --failed` | `rc-status --crashed` |
| Target/runlevel | `multi-user.target` | `default` runlevel |
| PID 1 name | `systemd` | `init` (busybox) |
| Bootloader | systemd-boot | systemd-boot (shared!) |
| Chroot shell | `/bin/bash` | `/bin/ash` |

## Files Created/Modified

### New Files
- `testing/install-tests/src/distro/mod.rs` - DistroContext trait
- `testing/install-tests/src/distro/levitate.rs` - LevitateOS context
- `testing/install-tests/src/distro/acorn.rs` - AcornOS context
- `AcornOS/profile/live-overlay/etc/profile.d/00-acorn-test.sh` - test markers

### Modified Files
- `testing/install-tests/src/main.rs` - add --distro CLI flag
- `testing/install-tests/src/qemu/boot.rs` - parameterize boot detection
- `testing/install-tests/src/qemu/patterns.rs` - add AcornOS patterns
- `testing/install-tests/src/steps/mod.rs` - pass distro context
- `testing/install-tests/src/steps/phase5_boot.rs` - use DistroContext
- `testing/install-tests/src/steps/phase6_verify.rs` - use DistroContext

## Progress

- [x] Create DistroContext trait
- [x] Implement LevitateOS context
- [x] Implement AcornOS context
- [x] Parameterize boot detection
- [x] Update phase 5 (bootloader/services)
- [x] Update phase 6 (verification)
- [x] Add CLI flag
- [x] Create AcornOS test instrumentation

## Summary

Implemented AcornOS support in the install-tests crate via a `DistroContext` trait. The trait abstracts:
- Init system differences (systemd vs OpenRC)
- Boot detection patterns
- Service management commands
- PID 1 verification
- User/password configuration

Both distros use systemd-boot for the bootloader, simplifying that component.

### Usage

```bash
# Run LevitateOS tests (default)
cd testing/install-tests
cargo run -- run

# Run AcornOS tests
cargo run -- run --distro acorn --iso ../../AcornOS/output/acornos.iso

# List steps for a specific distro
cargo run -- list --distro acorn
```
