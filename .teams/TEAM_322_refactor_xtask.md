# TEAM_322 — Xtask Refactor

**Registered:** 2026-01-09  
**Objective:** Refactor the `xtask` crate to eliminate code duplication, establish clear module boundaries, and improve maintainability.

## Scope

- `xtask/src/run.rs` (925 lines → split and consolidate)
- `xtask/src/main.rs` (CLI dispatch consolidation)
- `xtask/src/image.rs` (move `Screenshot` to appropriate module)
- Create new `xtask/src/qemu/` module hierarchy

## Key Pain Points Identified

1. **Massive code duplication in `run.rs`**
   - 7 QEMU run functions: `run_qemu`, `run_qemu_gdb`, `run_qemu_vnc`, `run_qemu_test`, `run_qemu_term`, `verify_gpu`
   - Repeated arch-specific device selection (`virtio-keyboard-pci` vs `virtio-keyboard-device`)
   - Same kernel/ISO logic copy-pasted across functions

2. **No builder pattern for QEMU args**
   - Each function manually constructs `vec![]` with 30+ arguments
   - Version skew risk when adding new devices

3. **Misplaced commands**
   - `Screenshot` lives in `image.rs` but depends on `qmp.rs`
   - Should be in a `qemu` or `debug` module

4. **Inconsistent file sizes**
   - `run.rs`: 925 lines (violates Rule 7: <500 ideal)
   - Other modules: reasonably sized

## Progress Log

- [x] Registered team (TEAM_322)
- [x] Analyzed codebase structure
- [x] Phase 1: Discovery and Safeguards
- [x] Phase 2: Structural Extraction
- [x] Phase 3: Migration
- [x] Phase 4: Cleanup
- [x] Phase 5: Hardening

## Results

| Metric | Before | After |
|--------|--------|-------|
| `run.rs` lines | 925 | 483 |
| Module structure | Flat (7 files) | Hierarchical (6 dirs) |
| Code duplication | High | Eliminated |
