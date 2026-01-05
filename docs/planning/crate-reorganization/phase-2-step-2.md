# Phase 2, Step 2: Move VirtIO HAL Implementation

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 1 complete  
**Status:** Planned  
**Estimated Effort:** 1-2 UoW

---

## Objective

Move VirtIO HAL implementation from `levitate-hal/src/virtio.rs` to `levitate-virtio/src/hal_impl.rs` to establish correct layer boundaries.

---

## Current State

### levitate-hal/src/virtio.rs

Contains:
- `VirtioHal` trait impl for `virtio-drivers` crate
- `LevitateVirtioHal` struct
- `StaticMmioTransport` type alias
- DMA allocation/deallocation
- Physical/virtual address translation

### Why This Is Wrong

HAL should only contain CPU/platform-specific code:
- MMU
- GIC
- Timer
- UART
- Exceptions

VirtIO HAL impl is **driver-layer code** that should live in the transport layer.

---

## Target State

### levitate-virtio/src/hal_impl.rs

Will contain:
- `LevitateVirtioHal` struct
- VirtioHal trait implementation
- DMA helpers

### levitate-hal

Will:
- Export only DMA primitives (alloc_page, virt_to_phys)
- Not depend on virtio-drivers crate
- Not export any VirtIO types

---

## Tasks

### Task 1: Create hal_impl.rs in levitate-virtio

```rust
// levitate-virtio/src/hal_impl.rs
use crate::VirtioHal;

pub struct LevitateVirtioHal;

impl VirtioHal for LevitateVirtioHal {
    // ... implementation using levitate-hal primitives
}
```

### Task 2: Move Implementation

Move from `levitate-hal/src/virtio.rs`:
- `LevitateVirtioHal` struct
- All trait method implementations
- `StaticMmioTransport` type alias

### Task 3: Update levitate-virtio Cargo.toml

Add dependency:
```toml
levitate-hal = { path = "../levitate-hal" }
```

### Task 4: Update levitate-virtio lib.rs

```rust
mod hal_impl;
pub use hal_impl::{LevitateVirtioHal, StaticMmioTransport};
```

### Task 5: Update levitate-hal

1. Delete `src/virtio.rs`
2. Remove from `lib.rs`: `pub mod virtio;`
3. Remove from Cargo.toml: `virtio-drivers` dependency
4. Keep DMA primitives exported for hal_impl to use

### Task 6: Update All Imports

Find and replace:
```rust
// Old
use levitate_hal::{LevitateVirtioHal, StaticMmioTransport};

// New  
use levitate_virtio::{LevitateVirtioHal, StaticMmioTransport};
```

Affected files:
- kernel/src/gpu.rs
- kernel/src/virtio.rs
- Any other files using these types

### Task 7: Verify Build

```bash
cargo build --release
cargo xtask test
```

---

## Exit Criteria

- [ ] levitate-hal no longer has virtio.rs
- [ ] levitate-hal no longer depends on virtio-drivers
- [ ] levitate-virtio exports LevitateVirtioHal
- [ ] All imports updated
- [ ] Build passes
- [ ] Tests pass
