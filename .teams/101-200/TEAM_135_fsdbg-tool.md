# TEAM_135: fsdbg - Filesystem Debugging Tool

## Mission
Implement `fsdbg`, a CLI tool for inspecting and verifying LevitateOS filesystem artifacts (CPIO, EROFS, ISO) without extraction or mounting.

## Status: COMPLETE

## Problem Statement
During debugging, we need to:
1. Extract/inspect initramfs without fakeroot/permission issues
2. List archive contents without clunky cpio commands
3. Verify symlinks resolve correctly
4. Check library dependencies
5. Inspect ISO/EROFS without root mounting
6. Validate expected vs actual contents

## Implementation Phases

### Phase 1: Core Infrastructure
- [x] Create `tools/fsdbg/Cargo.toml`
- [x] Create error handling (`src/error.rs`)
- [x] Create CLI skeleton (`src/main.rs`)
- [x] Add to workspace

### Phase 2: Archive Reading
- [x] CPIO reader (`src/cpio.rs`)
- [x] EROFS inspection (`src/erofs.rs`)
- [x] ISO inspection (`src/iso.rs`)

### Phase 3: Verification Checklists
- [x] Checklist trait (`src/checklist/mod.rs`)
- [x] Install initramfs checklist
- [x] Live initramfs checklist
- [x] Rootfs checklist

### Phase 4: Commands
- [x] `inspect` command
- [x] `verify` command
- [x] `check-symlinks` command
- [x] `diff` command
- [ ] `check-libs` command (future enhancement)

## Files Created
- `tools/fsdbg/Cargo.toml`
- `tools/fsdbg/src/main.rs`
- `tools/fsdbg/src/lib.rs`
- `tools/fsdbg/src/error.rs`
- `tools/fsdbg/src/cpio.rs`
- `tools/fsdbg/src/erofs.rs`
- `tools/fsdbg/src/iso.rs`
- `tools/fsdbg/src/checklist/mod.rs`
- `tools/fsdbg/src/checklist/install_initramfs.rs`
- `tools/fsdbg/src/checklist/live_initramfs.rs`
- `tools/fsdbg/src/checklist/rootfs.rs`

## Verification
```bash
cargo build -p fsdbg
cargo run -p fsdbg -- inspect leviso/output/initramfs-installed.img
cargo run -p fsdbg -- verify leviso/output/initramfs-installed.img --type install-initramfs
cargo run -p fsdbg -- check-symlinks leviso/output/initramfs-installed.img
```

## Integration Points

### Leviso Build Integration
After building initramfs, `leviso/src/artifact/initramfs.rs` automatically runs fsdbg verification:
- `build_tiny_initramfs()` → verifies with LiveInitramfs checklist
- `build_install_initramfs()` → verifies with InstallInitramfs checklist
- Build FAILS if critical items are missing

### Install-Tests Preflight
Both serial and QMP test runners call `require_preflight()` BEFORE starting QEMU:
- `testing/install-tests/src/bin/serial.rs`
- `testing/install-tests/src/bin/qmp.rs`
- Catches broken artifacts WITHOUT waiting for QEMU boot

## Notes
- `-.slice` and `system.slice` are NOT required as files - they're built-in systemd units
  compiled directly into the systemd binary. The checklist was updated to reflect this.
