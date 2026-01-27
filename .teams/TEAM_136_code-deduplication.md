# TEAM_136: Code Deduplication

## Status: COMPLETE

## Objective
Eliminate code duplication across the codebase by consolidating ELF utilities, unifying `copy_dir_recursive`, and migrating hardcoded constants to `distro-spec`.

## Phases

### Phase 1: ELF Utility Consolidation - DONE
- [x] Add `leviso-elf` dependency to `tools/recinit`
- [x] Replace duplicated functions in `tools/recinit/src/elf.rs` with re-exports (~275 LOC removed)
- [x] Verify build and tests pass

### Phase 2: copy_dir_recursive Unification - DONE
- [x] Add `copy_dir_recursive_overwrite` variant to `leviso-elf` (handles semantic difference)
- [x] Add `leviso-elf` dependency to `distro-builder`
- [x] Replace local impl in `distro-builder/src/artifact/filesystem.rs` with wrapper
- [x] NOT APPLICABLE: `leviso/src/component/custom/modules.rs` - This is NOT a duplicate, it's a specialized version that counts `.ko` files with a different function signature
- [x] Verify build and tests pass

### Phase 3: Constants Migration - DONE
- [x] Migrate `BOOT_DEVICE_PROBE_ORDER` in `tools/recinit/src/tiny.rs` to use `distro_spec::shared::BOOT_DEVICE_PROBE_ORDER`
- [x] Migrate ISO checklist constants in `testing/fsdbg/src/checklist/iso.rs` to use `distro_spec::shared::*` and `distro_spec::levitate::ISO_LABEL`
- [x] NOT MIGRATED: `tools/recstrap/src/constants.rs` - These are runtime search paths (mount points), different concept from ISO paths
- [x] Verify build and tests pass

## Summary of Changes

### Files Modified

1. **`tools/recinit/Cargo.toml`** - Added `leviso-elf` and `distro-spec` dependencies
2. **`tools/recinit/src/elf.rs`** - Replaced 275 LOC with 10-line re-exports from `leviso-elf`
3. **`tools/recinit/src/tiny.rs`** - `default_boot_devices()` now uses `distro_spec::shared::BOOT_DEVICE_PROBE_ORDER`
4. **`leviso-elf/src/copy.rs`** - Added `copy_dir_recursive_overwrite()` variant and tests
5. **`leviso-elf/src/lib.rs`** - Exported new `copy_dir_recursive_overwrite` function
6. **`distro-builder/Cargo.toml`** - Added `leviso-elf` dependency
7. **`distro-builder/src/artifact/filesystem.rs`** - `copy_dir_recursive()` now wraps `leviso_elf::copy_dir_recursive_overwrite()`
8. **`testing/fsdbg/Cargo.toml`** - Added `distro-spec` dependency
9. **`testing/fsdbg/src/checklist/iso.rs`** - ISO paths now derived from `distro-spec` constants

### Impact

- **~300 lines of duplicated code removed**
- **Improved consistency** - Tools now share constants from single source of truth
- **Better maintainability** - Changing a path/constant in distro-spec updates all consumers
- **Discovered inconsistency** - fsdbg expected `initramfs.img`, distro-spec has `initramfs-live.img` (now aligned)

## Progress Log

### 2026-01-27
- Created team file
- Phase 1 COMPLETE: Replaced 275 LOC in recinit/src/elf.rs with re-exports from leviso-elf
- Phase 2 COMPLETE: Added `copy_dir_recursive_overwrite` to leviso-elf, distro-builder now uses it
  - Note: The two variants have different semantics (skip vs overwrite existing symlinks)
  - Added tests to verify both behaviors work correctly
- Phase 3 COMPLETE: Migrated constants in recinit and fsdbg to use distro-spec
  - Not all constants migrated - some are runtime-specific (recstrap search paths)
- All tests passing across the workspace

### 2026-01-27 (continued)
- Fixed PROTECTED_PATHS duplication between recstrap and recchroot
  - Added `PROTECTED_PATHS` constant and `is_protected_path()` function to `distro-spec/src/shared/paths.rs`
  - Removed duplicate constant from `tools/recstrap/src/constants.rs`
  - Removed duplicate constant from `tools/recchroot/src/main.rs`
  - Removed duplicate `is_protected_path()` function from both tools
  - Both tools now use `distro_spec::shared::is_protected_path()`

- Fixed is_root() duplication between recstrap and recchroot
  - Added `libc` dependency to `distro-spec`
  - Created `distro-spec/src/shared/system.rs` with `is_root()` function
  - Removed duplicate from `tools/recstrap/src/helpers.rs`
  - Removed duplicate from `tools/recstrap/tests/integration.rs`
  - Updated `tools/recchroot/src/main.rs` to use shared function (was using `nix::unistd::geteuid().is_root()`)
  - All 93 tests passing (distro-spec: 7, recchroot: 24, recstrap: 63)
