# TEAM_181: Opus Review after Iteration 20

**Date**: 2026-02-04
**Status**: Complete
**Type**: Code review + bug fixes

## What Was Reviewed

Last 3 haiku iterations (18-20):
- AcornOS: EROFS size reduction, sshd_config fix, test instrumentation
- IuppiterOS: test instrumentation, inittab serial primary, welcome script fix

## Bugs Found and Fixed

### Critical: IuppiterOS live overlay inittab overrides definitions.rs

Haiku's iteration 18 fixed the BASE_INITTAB in `definitions.rs` to be serial-only.
But `iso.rs::create_live_overlay()` writes its own `/etc/inittab` to the live overlay
directory, which takes precedence over the EROFS base (overlay > base). The iso.rs
inittab still had tty1-tty6 VGA terminals from AcornOS.

**Impact**: IuppiterOS live boot would spawn 6 VGA gettys on a headless appliance.
**Fix**: Replaced iso.rs inittab with serial-only version matching definitions.rs.

### Pervasive AcornOS copy-paste bugs (40+ occurrences, 14 files)

Despite 3 previous opus reviews catching copy-paste bugs, many files were never
audited: iso.rs, qemu.rs, initramfs.rs, rebuild.rs, cache.rs, component/mod.rs,
component/context.rs, component/custom/filesystem.rs, component/custom/modules.rs,
artifact/mod.rs, artifact/rootfs.rs, profile files.

Functional impact: wrong CLI command suggestions, wrong branding in live ISO,
wrong /etc/issue content, wrong test file references.

### live-docs.sh was entirely AcornOS-specific

The script referenced acorn-docs, had AcornOS ASCII art, mentioned WiFi (iwctl),
and checked for tty1 (which IuppiterOS doesn't have). Rewrote for headless
serial console with refurbishment tool quick reference.

## Files Modified

IuppiterOS submodule (14 files):
- `src/artifact/iso.rs` - Live overlay inittab, branding, error messages
- `src/qemu.rs` - Error messages, smoke test UI, comments
- `src/artifact/initramfs.rs` - Doc comments, error messages
- `src/artifact/mod.rs` - Doc comment
- `src/artifact/rootfs.rs` - Doc comment
- `src/cache.rs` - Doc comment
- `src/rebuild.rs` - Doc comments
- `src/component/mod.rs` - Doc comments, test assertion
- `src/component/context.rs` - Doc comment
- `src/component/custom/filesystem.rs` - Doc comments
- `src/component/custom/modules.rs` - Error message
- `profile/init_tiny.template` - Boot messages
- `profile/live-overlay/etc/profile.d/live-docs.sh` - Full rewrite
- `profile/etc/login.defs` - Header comment

## Key Decisions

1. Fixed ALL AcornOS references found via grep, not just the ones from last 3 iterations
2. Rewrote live-docs.sh instead of just changing branding (was fundamentally wrong for headless)
3. Did NOT fix .rhai recipe files (they reference AcornOS downloads symlink by design)

## Verification

- `cargo check --workspace`: Clean
- `cargo test -p iuppiteros`: 18 tests pass
- `cargo test -p acornos`: 31 tests pass
