# TEAM_189: Opus Review (after Iteration 26)

**Date:** 2026-02-04
**Status:** Complete
**Type:** Review of haiku iterations 23-26

## Scope Reviewed

AcornOS commits: `70ecebc` (UKI building)
IuppiterOS commits: `b36c78e` (UKI building), `0ee84a8` (QEMU runner), `4800f91` (serial flag)

Files reviewed:
- AcornOS: `src/artifact/uki.rs`, `src/artifact/iso.rs`, `src/artifact/mod.rs`
- IuppiterOS: `src/artifact/uki.rs`, `src/artifact/iso.rs`, `src/artifact/mod.rs`, `src/qemu.rs`, `src/main.rs`

## Verification Results

- `cargo check --workspace`: Clean (only pre-existing leviso warnings)
- `cargo test -p acornos`: 34 tests pass
- `cargo test -p iuppiteros`: 22 tests pass
- `cargo test -p distro-builder`: 60 tests pass
- `cargo test -p distro-spec`: 73 tests pass

## Bugs Found and Fixed (2 commits)

### 1. Hardcoded ISO_LABEL in UKI cmdlines (AcornOS + IuppiterOS)

Both UKI builders hardcoded the ISO label in `root=LABEL=ACORNOS` / `root=LABEL=IUPPITER` format strings instead of using the `ISO_LABEL` constant from distro-spec. If the label were ever changed in distro-spec, the UKI boot cmdline would silently go out of sync and booting would fail.

**Fix:** Import `ISO_LABEL` from distro-spec and use `format!("root=LABEL={}", ISO_LABEL, ...)`. Updated tests to also use the constant.

- AcornOS commit: `92f7cb1` fix(acorn): use ISO_LABEL constant instead of hardcoded label in UKI cmdlines
- IuppiterOS commit: `5be0db3` fix(iuppiter): use ISO_LABEL constant in UKI cmdlines, add -display none for serial mode

### 2. Missing `-display none` in QEMU serial_only mode (IuppiterOS)

The QEMU builder's `serial_only` mode configured `-serial stdio` but did not disable the graphical display. Without `-display none`, QEMU still tries to open a GTK/SDL window, making the "headless" mode not truly headless.

Compare with `test_iso()` which correctly uses `-nographic` for headless operation.

**Fix:** Added `-display none` when `serial_only` is true. Also cleaned up `&format!("stdio")` → `"stdio"` (unnecessary format macro for a literal).

### 3. Minor: `&format!("stdio")` → `"stdio"` (IuppiterOS qemu.rs)

Unnecessary `format!()` macro wrapping a string literal. No functional impact but misleading.

## Code Quality Observations (no action needed)

- AcornOS UKI implementation is clean and well-structured. Correctly imports from `distro_spec::acorn` and uses proper constants for console settings.
- IuppiterOS UKI is properly differentiated: serial-only cmdline (no VGA_CONSOLE), correct distro-spec imports from `distro_spec::iuppiter`.
- IuppiterOS GRUB config in iso.rs correctly uses `iso_label()` variable (not hardcoded), so that path was fine.
- IuppiterOS `--serial` flag defaults to false (GUI default), which seems counterintuitive for a headless appliance, but matches the PRD spec (task 5.9 defines the interface as `cargo run -- run --serial`).
- No remaining AcornOS references in IuppiterOS src/ (verified with grep -ri).

## No Blocked Tasks

All PRD tasks through 6.5 (AcornOS) and 5.9 (IuppiterOS) are correctly marked [x]. No tasks marked BLOCKED. Next unchecked tasks are 6.6-6.11 (IuppiterOS boot verification).

## Files Modified

- `AcornOS/src/artifact/uki.rs` — use ISO_LABEL constant
- `IuppiterOS/src/artifact/uki.rs` — use ISO_LABEL constant
- `IuppiterOS/src/qemu.rs` — add -display none for serial_only, clean up format!
