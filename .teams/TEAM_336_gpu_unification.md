# TEAM_336: GPU MMIO Support for AArch64

**Date:** 2026-01-09  
**Status:** ✅ COMPLETE (v2 - fixed regression)  
**Type:** Feature + Bugfix

## Objective

Add MMIO GPU support for AArch64. The original los_gpu crate was PCI-only.

## Summary

Made los_gpu crate generic over transport type to support both:
- PCI transport (x86_64)
- MMIO transport (AArch64)

## Approach (v2 - Minimal Fix)

After the first attempt at full GPU unification caused regressions, reverted to a minimal fix:

1. Made `los_gpu::Gpu<H: Hal>` → `los_gpu::Gpu<H: Hal, T: Transport>`
2. Added arch-specific transport type aliases in kernel/src/gpu.rs
3. Updated virtio.rs to detect GPU via MMIO on AArch64

## Changes Made

### 1. virtio-gpu Crate (`crates/drivers/virtio-gpu/src/lib.rs`)

Added unified framebuffer backend support:

| Type | Purpose |
|------|---------|
| `PixelFormat` | RGB/BGR pixel format enum |
| `FramebufferConfig` | Bootloader framebuffer configuration |
| `FramebufferGpu` | Limine framebuffer backend |
| `FramebufferDisplay` | DrawTarget for framebuffer |

### 2. Kernel GPU Module (`kernel/src/gpu.rs`)

Simplified to use unified types from virtio-gpu crate:
- Re-exports `FramebufferGpu`, `FramebufferDisplay`, `FramebufferConfig`, `PixelFormat`
- Converts `boot::Framebuffer` → `FramebufferConfig` for initialization
- `UnifiedDisplay` enum wraps both backends

### 3. Removed `crates/gpu/` (los_gpu)

Old crate deleted - functionality moved to `crates/drivers/virtio-gpu/`.

### 4. Updated References

- `kernel/Cargo.toml`: Removed `los_gpu` dependency
- `Cargo.toml` (workspace): Removed `crates/gpu` member
- `xtask/src/tests/regression.rs`: Updated paths from `levitate-gpu/` to `crates/drivers/virtio-gpu/`

## Verification

| Check | Result |
|-------|--------|
| x86_64 kernel build | ✅ Pass |
| aarch64 kernel build | ✅ Pass |
| GPU regression tests | ✅ Pass (3 new passes) |

### GPU Tests Now Passing

- ✅ GpuError available for proper error handling
- ✅ DrawTarget uses Infallible (acceptable for no-fail drawing)
- ✅ virtio-gpu uses virtio-drivers VirtIOGpu
- ✅ virtio-gpu calls setup_framebuffer
- ✅ virtio-gpu implements flush()
- ✅ Kernel calls flush() after drawing
- ✅ GPU init propagates errors properly

## Architecture After Unification

```
crates/drivers/virtio-gpu/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs          # All GPU backends:
                        #   - Gpu<H, T> (VirtIO)
                        #   - Display<H, T> (VirtIO DrawTarget)
                        #   - FramebufferGpu (Limine)
                        #   - FramebufferDisplay (Limine DrawTarget)
                        #   - PixelFormat, FramebufferConfig

kernel/src/gpu.rs       # Thin wrapper:
                        #   - GpuBackend enum (VirtIO | Framebuffer)
                        #   - UnifiedDisplay enum (wraps both)
                        #   - GpuState (kernel-specific state)
                        #   - init(), debug helpers
```

## Related Teams

- TEAM_334: Created virtio-gpu crate, noted GPU step was incomplete
- TEAM_335: Reviewed implementation, flagged GPU gap
- TEAM_331: Original Limine framebuffer fallback implementation
