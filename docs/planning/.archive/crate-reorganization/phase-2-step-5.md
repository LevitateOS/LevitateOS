# Phase 2, Step 5: Extract Net Driver

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 2 complete  
**Status:** Planned  
**Estimated Effort:** 2 UoW

---

## Objective

Extract VirtIO Net driver from `kernel/src/net.rs` into new `levitate-drivers-net` crate.

---

## Current State

### kernel/src/net.rs

Contains:
- VirtIONet wrapper
- MAC address retrieval
- Network initialization
- Send/receive operations

---

## Target State

### New Crate: levitate-drivers-net/

```
levitate-drivers-net/
├── Cargo.toml
├── README.md
└── src/
    └── lib.rs
```

### Public API

```rust
pub struct NetDevice { ... }

impl NetDevice {
    pub fn new(transport: MmioTransport) -> Result<Self, NetError>;
    pub fn mac_address(&self) -> [u8; 6];
    pub fn send(&mut self, buf: &[u8]) -> Result<(), NetError>;
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, NetError>;
}
```

---

## Tasks

### Task 1: Create Crate Structure
### Task 2: Create Cargo.toml
### Task 3: Move net.rs Logic to lib.rs
### Task 4: Update Workspace
### Task 5: Update Kernel
### Task 6: Verify Build

---

## Exit Criteria

- [ ] New crate created
- [ ] Net driver logic moved
- [ ] Clean public API
- [ ] Build passes
- [ ] MAC address shows in boot log
- [ ] Tests pass
