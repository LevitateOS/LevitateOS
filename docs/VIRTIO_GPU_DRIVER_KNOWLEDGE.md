# VirtIO GPU Driver Architecture & Knowledge Base

**TEAM_099: Knowledge Documentation for Future Teams**

## 1. Overview
The VirtIO GPU subsystem has been refactored from a legacy `virtio-drivers` wrapper into a custom, async-ready crate-based architecture. This provides full protocol visibility and removes reliance on external "black box" drivers.

## 2. Crate Structure
*   **`levitate-virtio`**: General VirtIO transport. Implements `VirtQueue` (split ring) and `MmioTransport`.
*   **`levitate-virtio-gpu`**: GPU-specific protocol and driver.
    *   `protocol/`: Structs matching VirtIO 1.1 Spec Section 5.7.
    *   `driver.rs`: State machine for GPU initialization sequence.
    *   `device.rs`: The integrated `VirtioGpu<H>` type that kernel code uses.

## 3. The Wiring Pattern
To use the new driver, the kernel implements the `VirtioHal` trait defined in `levitate-virtio`.

**Pattern for static GPU access:**
```rust
// kernel/src/gpu.rs
pub type GpuState = VirtioGpu<LevitateVirtioHal>;
pub static GPU: IrqSafeLock<Option<GpuState>> = IrqSafeLock::new(None);
```

**Pattern for Drawing (embedded-graphics):**
The new `VirtioGpu` driver implements `DrawTarget`. You no longer need the `Display` wrapper.
```rust
if let Some(gpu) = gpu_guard.as_mut() {
    Text::new("Hello", Point::new(10, 30), style).draw(gpu).ok();
    gpu.flush().ok();
}
```

## 4. Verification Techniques
*   **Build Check**: Always build for the target architecture to catch assembly and register issues:
    `cargo build -p levitate-kernel --release --target aarch64-unknown-none --features verbose`
*   **Dead Code Detection**: Use `cargo build` and look for `dead_code` or `unused` warnings. TEAM_099 has cleaned up 25+ abandoned scaffolding items.

## 5. Known Gotchas & Cleanup Notes
*   **Diverging Functions (`!`)**: Functions using `asm!` with `options(noreturn)` must diverge. Rust occasionally requires an explicit `loop {}` or removal of trailing semicolons to satisfy the `!` return type.
*   **Self-Referential Imports**: Avoid `use crate::gpu::GPU` inside `kernel/src/gpu.rs` as it causes definition conflicts. Use `crate::gpu::GPU` in other modules.
*   **Abandoned Features**: Scaffolding for Userspace (Phase 8) exists in `kernel/src/task/user.rs` and `user_mm.rs`. It is currently marked `#[allow(dead_code)]` to keep the build clean while preserving planned work.
*   **Redundant Constants**: Do not duplicate layout constants (like `STACK_TOP`) across `user.rs` and `user_mm.rs`. Maintain a single source of truth in `user_mm.rs`.

## 6. Future Work
*   **Cursor Queue**: `CONTROLQ` (0) is implemented. `CURSORQ` (1) is defined but not yet used.
*   **Async Integration**: The driver supports `PendingCommand` and `Waker`, but the current `send_command` implementation uses busy-waiting. Future teams should wire this up to the interrupt system.
