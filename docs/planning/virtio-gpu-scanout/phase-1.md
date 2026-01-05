# Phase 1: Discovery and Safeguards

## Refactor Summary
Refactor the VirtIO GPU scanout configuration to fix the blank QEMU display issue. Rendering is verified to work via QMP, but the host-side presentation fails on some configurations.

## Success Criteria
- QEMU graphical window shows boot messages and interactive prompt.
- `cargo xtask gpu-dump` still produces correct images.
- All behavior tests pass.

## Behavioral Contracts
- VirtIO MMIO interface (0x0A00_0000 range).
- GPU Resolution: Queryable via `gpu::get_resolution()`.
- Display logic: Uses `embedded-graphics` `DrawTarget`.

## Golden/Regression Tests
- `tests/golden_boot.txt`: Should remain consistent (matches boot log).
- `xtask gpu-dump`: Must produce a valid screenshot.

## Constraints
- No changes to `virtio-drivers` crate (use internal implementation if needed).
- Maintain 10Hz flush cadence to keep display active.

## Steps
1. **Step 1 – Verify Current State**
   - Run `cargo xtask run` and `cargo xtask gpu-dump`.
   - Confirm "Display output is not active" in window.
2. **Step 2 – Research SET_SCANOUT Params**
   - Research if `width`/`height` must be power-of-two or specific alignment.
3. **Step 3 – Identify Missing Commands**
   - Check if `VIRTIO_GPU_CMD_RESOURCE_SET_SCANOUT` (1.2) is actually what's needed for some hosts.
