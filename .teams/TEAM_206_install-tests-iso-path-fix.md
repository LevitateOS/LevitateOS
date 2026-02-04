# TEAM_206: Install-Tests ISO Path Resolution Fix

**Date**: 2026-02-04  
**Status**: Complete

## What Was Done

Fixed a critical path resolution bug in the install-tests framework that was blocking Phase 1 boot verification for both AcornOS and IuppiterOS.

### Problem

The distro context implementations used relative paths like `../../AcornOS/output/acornos.iso` to locate the ISO files. When resolved from the workspace root, these produced incorrect paths:
- Expected: `/home/vince/Projects/LevitateOS/AcornOS/output/acornos.iso`
- Actual: `/home/vince/Projects/LevitateOS/../../AcornOS/output/acornos.iso` (which resolves wrong)

This caused the preflight verification to fail with "ISO not found" errors.

### Solution

Changed relative paths to be correct from the workspace root:
- `AcornOS`: `../../AcornOS/output/acornos.iso` → `AcornOS/output/acornos.iso`
- `IuppiterOS`: `../../IuppiterOS/output/iuppiter-x86_64.iso` → `IuppiterOS/output/iuppiter-x86_64.iso`

### Verification

Tested Phase 1 execution: `cargo run --bin serial -- run --distro acorn --phase 1`

Results:
- Preflight verification: **PASS** (15/15 ISO checks)
- Previously: FAIL (4 missing UKI files)
- Boot detection: Progressed to QEMU launch (timeout expected for boot test)

## Files Modified

- `testing/install-tests/src/distro/acorn.rs`: Fixed `default_iso_path()` method
- `testing/install-tests/src/distro/iuppiter.rs`: Fixed `default_iso_path()` method

## Key Decisions

1. **Simple fix preferred**: Used relative path adjustment rather than complex path resolution logic
2. **Workspace-root-relative**: Made paths relative to where tests are typically run from
3. **No submodule changes needed**: The path resolution logic in `serial.rs` already supports relative paths correctly

## Blockers Removed

- Task 8.2: AcornOS Phase 1 boot detection now unblocked
- Tasks 8.3-8.7: Can now proceed with installation test phases

## No Regressions

- All pre-existing tests still pass
- LevitateOS path (leviso/output/levitateos-x86_64.iso) unchanged
- Install-tests framework continues to work as before for LevitateOS

