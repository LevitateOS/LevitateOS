# TEAM_466: Refactor xtask/src/build/commands.rs

## Objective
Heavy refactor of `commands.rs` (1,372 lines) into focused modules. Extract embedded shell scripts to external files.

## Progress Log
### Session 1 (2026-01-12)
- Analyzed `commands.rs` - found ~700 lines of embedded shell scripts as Rust heredocs
- Identified duplicate CPIO creation code (3 copies)
- Created plan with user approval for module structure
- User decided shell scripts should live in `xtask/initrd_resources/`

## Key Decisions
- **Shell scripts location**: `xtask/initrd_resources/` - keeps them with the build system that uses them
- **Module split pattern**: Follow existing patterns in apps.rs, busybox.rs - small focused modules
- **Backward compatibility**: All public API maintained via re-exports in mod.rs

## Changes Made

### New Files Created
| File | Purpose | Lines |
|------|---------|-------|
| `xtask/initrd_resources/test.sh` | ASH shell test script | ~68 |
| `xtask/initrd_resources/test-core.sh` | Coreutils test suite | ~644 |
| `xtask/src/build/initramfs.rs` | All initramfs creation + CPIO helper | ~340 |
| `xtask/src/build/kernel.rs` | Kernel build function | ~60 |
| `xtask/src/build/userspace.rs` | Userspace build function | ~30 |
| `xtask/src/build/iso.rs` | ISO build + Limine download | ~130 |
| `xtask/src/build/orchestration.rs` | build_all coordination | ~55 |

### Files Modified
| File | Change |
|------|--------|
| `xtask/src/build/commands.rs` | Reduced from 1,372 to ~23 lines (enum only) |
| `xtask/src/build/mod.rs` | Added new module declarations and re-exports |

### Code Improvements
1. **Extracted shell scripts** - Now loaded via `include_str!()` at compile time
2. **Deduplicated CPIO creation** - Single `create_cpio_archive()` helper used by 3 functions
3. **Single responsibility** - Each module handles one concern

## Verification
- [x] `cargo build -p xtask` - compiles with only pre-existing warnings
- [x] `cargo xtask check` - preflight checks pass

## Gotchas Discovered
- Many dead code warnings are pre-existing (apps registry is empty after TEAM_459)
- The `create_initramfs` and `create_test_initramfs` functions are not currently called anywhere (legacy code preserved for potential future use)

## Remaining Work
- None - refactor complete

## Handoff Notes
The refactor maintains full backward compatibility. All functions remain accessible at the same paths:
- `build::build_all(arch)`
- `build::build_kernel_only(arch)`
- `build::build_userspace(arch)`
- `build::create_busybox_initramfs(arch)`
- `build::build_iso(arch)`
- etc.

The shell scripts are now proper `.sh` files that can be edited with syntax highlighting and linting.
