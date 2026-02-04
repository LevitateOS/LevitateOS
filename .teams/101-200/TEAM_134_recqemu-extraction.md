# TEAM_134: Extract Shared QEMU Utilities to recqemu

**Status: COMPLETE**

## Goal
Extract shared QEMU utilities from install-tests into a standalone `recqemu` crate that both leviso and install-tests can use.

## Why
- DRY - `find_ovmf()` is duplicated in both crates
- Single source of truth for QEMU configuration
- leviso can benefit from install-tests' better QemuBuilder

## What Goes in recqemu (shared basics)
- `QemuBuilder` - QEMU command builder (basic version)
- `find_ovmf()` / `find_ovmf_vars()` - OVMF discovery
- `create_disk()` - qcow2 disk creation

## What Stays in install-tests (testing-specific)
- `Console` - serial I/O with auth, exec
- `qmp/` - QMP protocol support
- `patterns.rs` - boot/failure patterns
- `acquire_test_lock()` - test coordination
- Anti-cheat protections

## Implementation Steps
1. [x] Create `tools/recqemu/` crate
2. [x] Extract QemuBuilder, find_ovmf, create_disk
3. [x] Update install-tests to import from recqemu
4. [x] Update leviso to import from recqemu
5. [x] Remove duplicated code

## Files Changed
- NEW: `tools/recqemu/Cargo.toml`
- NEW: `tools/recqemu/src/lib.rs` - QemuBuilder, find_ovmf, create_disk, kvm_available
- NEW: `tools/recqemu/CLAUDE.md`
- MODIFY: `testing/install-tests/Cargo.toml` - added recqemu dependency
- MODIFY: `testing/install-tests/src/qemu/builder.rs` - wraps recqemu, adds anti-cheat
- MODIFY: `leviso/Cargo.toml` - added recqemu dependency
- MODIFY: `leviso/src/qemu.rs` - uses recqemu::QemuBuilder
- MODIFY: `Cargo.toml` - added tools/recqemu to workspace

## Result
- Shared QEMU utilities in `recqemu` (~400 lines)
- `leviso/qemu.rs` now uses recqemu (simplified from ~420 to ~270 lines)
- `install-tests` wraps recqemu and adds testing-specific features (anti-cheat, locking)
- No more duplicated find_ovmf() implementations
- All builds pass
