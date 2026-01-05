# Phase 2: Structural Extraction

## Target Design
- New internal module `levitate_gpu::driver` that implements the VirtIO GPU protocol.
- Public API in `levitate_gpu` remains consistent: `GpuState`, `Display`.

## Extraction Strategy
- Copy core GPU logic from `virtio_drivers` to `levitate-gpu/src/driver.rs`.
- Adapt to use `StaticMmioTransport` and `VirtioHal` directly.
- Ensure all commands (`RESOURCE_CREATE_2D`, `RESOURCE_ATTACH_BACKING`, `SET_SCANOUT`, `TRANSFER_TO_HOST_2D`, `RESOURCE_FLUSH`) are implemented.

## Steps
1. **Step 1 – Create `driver.rs`**
   - Implement basic VirtIO command/response structures.
2. **Step 2 – Implement Command Dispatch**
   - Port `request` and `cursor_request` logic.
3. **Step 3 – Integrate with `GpuState`**
   - Replace `VirtIOGpu` field with the new internal driver.
