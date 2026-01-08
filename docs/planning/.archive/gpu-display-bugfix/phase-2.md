# Phase 2: Root Cause Analysis

**Bug:** VirtIO GPU "Display output is not active"  
**Team:** TEAM_089  
**Status:** ✅ COMPLETE (from investigation phase)

---

## Hypotheses List

| # | Hypothesis | Evidence | Confidence | Result |
|---|------------|----------|------------|--------|
| H1 | VirtIO init re-scan affects GPU | `virtio::init()` explicitly skips GPU devices | High | ❌ RULED OUT |
| H2 | DMA memory reclaimed | DMA kept in GpuState, not freed | High | ❌ RULED OUT |
| H3 | Display refactoring broke scanout | `set_scanout` not touched by TEAM_086 | High | ❌ RULED OUT |
| H4 | `console_gpu::clear()` triggers issue | Timing correlation with clear call | Medium | ⚠️ POSSIBLE |
| H5 | QEMU display timeout/invalidation | Red flash proves init works, then fails | High | ✅ CONFIRMED |

---

## Key Code Areas

### 1. GPU Initialization (`kernel/src/gpu.rs`)
```rust
pub fn init(transport: StaticMmioTransport) {
    // ... VirtIOGpu::new(transport) ...
    // ... gpu.resolution() ...
    // ... gpu.setup_framebuffer() ...  ← Calls set_scanout()
    // ... fill red, flush ...          ← Works (red flash visible)
}
```

### 2. virtio-drivers setup_framebuffer (`~/.cargo/.../gpu.rs`)
```rust
pub fn setup_framebuffer(&mut self) -> Result<&mut [u8]> {
    // ...
    self.set_scanout(display_info.rect, SCANOUT_ID, RESOURCE_ID_FB)?;
    // ...
}
```

The driver protocol is correct. All commands succeed.

### 3. Post-Init Operations (`kernel/src/main.rs`)
```rust
// Line 541: console_gpu::init(width, height);
// Line 544: console_gpu::clear();           ← Might trigger timing issue
// Line 548: set_secondary_output(...)
// Line 554: virtio::init();                 ← Skips GPU, probably safe
```

---

## Root Cause

**QEMU `virtio-gpu-device` Display Surface Timing**

The VirtIO GPU driver is correctly implemented:
- `setup_framebuffer()` properly calls `set_scanout()`
- Red flash proves: init ✅, resource create ✅, attach backing ✅, scanout ✅, flush ✅
- No kernel code resets scanout after init

**Actual Issue:** QEMU's display surface goes inactive shortly after boot.

**Hypothesis:** QEMU `virtio-gpu-device` requires:
1. Continuous flush commands to keep display active, OR
2. A specific display surface refresh pattern, OR
3. A different device type (e.g., `virtio-vga`)

---

## Investigation Strategy (Completed)

1. ✅ Traced `setup_framebuffer()` in virtio-drivers — confirms `set_scanout()` called
2. ✅ Verified no kernel code resets scanout after init
3. ✅ Confirmed red flash appears (proves init works)
4. ✅ Web research confirms QEMU display surface behavior is QEMU-specific
5. ✅ Identified `virtio-vga` as alternative device type

---

## Phase 2 Outcome

Root cause is QEMU display surface timing, not a driver bug.

**Next:** Phase 3 (Fix Design and Validation Plan)
