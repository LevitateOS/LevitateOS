# TEAM_205: Opus Review — Post-Iteration 36

**Date**: 2026-02-04
**Status**: Complete
**Type**: Review + Unblock

## Scope Reviewed

Last 3 haiku iterations (34-36) covering:
- **AcornOS** (3 commits): UKI building, ISO_LABEL constant fix, APK --usermode removal + db init
- **IuppiterOS** (3 commits): UKI duplicate console fix, udev I/O scheduler rule, operator user creation
- **distro-spec** (1 commit): sdparm removal from REFURBISHMENT_PACKAGES

## Verification Results

- `cargo check --workspace`: Clean (only pre-existing leviso warnings)
- `cargo test -p acornos --lib`: 34 tests pass
- `cargo test -p iuppiteros --lib`: 22 tests pass
- `cargo test -p distro-builder`: 60+ tests pass
- `cargo test -p distro-spec`: 73+ tests pass

## Bugs Found

**None.** The last 3 haiku iterations produced clean, well-scoped code:

1. AcornOS UKI builder correctly uses ISO_LABEL constant, handles extra_cmdline properly
2. IuppiterOS UKI duplicate console fix correctly removes SERIAL_CONSOLE from base cmdline
3. IuppiterOS operator user creation (users.rs) uses correct line-by-line matching
4. IuppiterOS udev rule is clean and correctly targets rotational block devices
5. distro-spec sdparm removal is correct (package unavailable in Alpine v3.23)

## BLOCKED Task Unblocked

**Task 8.2 (install-tests Phase 1 boot detection for AcornOS)** was BLOCKED because:
1. `testing/fsdbg/src/checklist/iso.rs` hardcoded LevitateOS constants (Volume ID, UKI filenames)
2. `testing/install-tests/src/preflight.rs` didn't pass distro context to fsdbg
3. `testing/install-tests/src/bin/serial.rs` called `require_preflight()` without distro context

### Fix Applied (2 commits across fsdbg + install-tests):

1. **fsdbg iso.rs**: Added `verify_distro(reader, distro_id)` function with `DistroIsoConstants` struct. Loads distro-specific constants (volume ID, UKI filenames, boot files, directories) based on distro_id. Original `verify()` preserved for backward compatibility.

2. **install-tests preflight.rs**: Added `run_preflight_for_distro()`, `require_preflight_for_distro()`, and `run_preflight_with_iso_distro()` that thread distro_id through to fsdbg.

3. **install-tests serial.rs**: Changed `require_preflight(iso_dir)` to `require_preflight_for_distro(iso_dir, ctx.id())`.

### Impact

This unblocks `cargo run --bin serial -- run --distro acorn --phase 1` from failing at preflight. The actual Phase 1 boot detection still depends on TEAM_154 (Console I/O buffering) being fixed, but the preflight barrier is now removed for all three distros.

## Files Modified

- `testing/fsdbg/src/checklist/iso.rs` — distro-aware verify_distro() function
- `testing/install-tests/src/preflight.rs` — distro-aware preflight functions
- `testing/install-tests/src/lib.rs` — export new functions
- `testing/install-tests/src/bin/serial.rs` — use distro-aware require_preflight

## Key Decisions

- Used additive approach: new functions alongside existing ones, no breaking changes
- AcornOS/IuppiterOS skip installed initramfs and installed UKI checks (not yet built)
- Kept loader.conf check out of non-LevitateOS checklists (AcornOS/IuppiterOS use GRUB, not systemd-boot exclusively)
