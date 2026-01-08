# Phase 2, Step 4: Extract Block Driver

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 2 complete (VirtIO HAL moved)  
**Status:** Planned  
**Estimated Effort:** 2-3 UoW

---

## Objective

Extract VirtIO Block driver from `kernel/src/block.rs` into new `levitate-drivers-blk` crate.

---

## Current State

### kernel/src/block.rs

Contains:
- VirtIOBlk wrapper
- Block device initialization
- Read/write operations
- Static device storage

### Dependencies

Currently uses:
- `virtio-drivers::device::blk::VirtIOBlk`
- `levitate-hal::virtio::VirtioHal`
- `levitate-hal::virtio::StaticMmioTransport`

---

## Target State

### New Crate: levitate-drivers-blk/

```
levitate-drivers-blk/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs
```

### Public API

```rust
pub struct BlockDevice { ... }

impl BlockDevice {
    pub fn new(transport: MmioTransport) -> Result<Self, BlockError>;
    pub fn read_block(&mut self, block: u64, buf: &mut [u8]) -> Result<(), BlockError>;
    pub fn write_block(&mut self, block: u64, buf: &[u8]) -> Result<(), BlockError>;
    pub fn capacity(&self) -> u64;
}

pub enum BlockError {
    InitFailed,
    ReadFailed,
    WriteFailed,
}
```

---

## Tasks

### Task 1: Create Crate Structure

```bash
mkdir -p levitate-drivers-blk/src
```

### Task 2: Create Cargo.toml

```toml
[package]
name = "levitate-drivers-blk"
version = "0.1.0"
edition = "2024"
description = "VirtIO Block driver for LevitateOS"

[dependencies]
levitate-virtio = { path = "../levitate-virtio" }
virtio-drivers = "0.12"  # Temporary, will remove when using levitate-virtio queue

[lints]
workspace = true
```

### Task 3: Create lib.rs

Move and refactor code from kernel/src/block.rs:
- Extract BlockDevice struct
- Define clean public API
- Keep internal implementation private

### Task 4: Update Workspace

Add to workspace Cargo.toml:
```toml
"levitate-drivers-blk",
```

### Task 5: Create README.md

Document the crate purpose and API.

### Task 6: Update Kernel

Update kernel/Cargo.toml:
```toml
levitate-drivers-blk = { path = "../levitate-drivers-blk" }
```

Update kernel/src/block.rs to re-export or wrap:
```rust
pub use levitate_drivers_blk::{BlockDevice, BlockError};
```

### Task 7: Verify Build

```bash
cargo build --release
cargo xtask test
```

---

## Exit Criteria

- [ ] New crate created with proper structure
- [ ] Block driver logic moved to crate
- [ ] Clean public API defined
- [ ] Kernel uses new crate
- [ ] Build passes
- [ ] Block device operations work
- [ ] Tests pass
