# Phase 3: Migration and Scanout Re-configuration

## Goal
Migrate `GpuState` to use the new driver and implement the periodic `SET_SCANOUT` re-configuration.

## Migration Strategy
- Switch `GpuState` to use the internal `GpuDriver`.
- Verify that basic rendering and flushing still work.
- Add "re-sync" logic to `GpuState::flush`.

## Steps
1. **Step 1 – Port Initialization**
   - Update `GpuState::new` to initialize the internal driver.
2. **Step 2 – Implement Flush Keep-Alive**
   - In `GpuState::flush`, call `SET_SCANOUT` every N flushes (e.g., every 100 flushes @ 10Hz = every 10 seconds).
   - This "pings" the host to ensure the scanout-resource link is active.
3. **Step 3 – Opt-in to VIRTIO_GPU_F_EDID (Optional)**
   - If available, read EDID to ensure resolution matches what QEMU expects.
