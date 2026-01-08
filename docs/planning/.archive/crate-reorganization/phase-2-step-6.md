# Phase 2, Step 6: Extract Input Driver

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 2 complete  
**Status:** Planned  
**Estimated Effort:** 2 UoW

---

## Objective

Extract VirtIO Input driver from `kernel/src/input.rs` into new `levitate-drivers-input` crate.

---

## Current State

### kernel/src/input.rs

Contains:
- VirtIOInput wrapper
- Keyboard event handling
- Scancode to ASCII mapping
- Input buffer management

---

## Target State

### New Crate: levitate-drivers-input/

```
levitate-drivers-input/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    └── keyboard.rs    # Scancode mapping
```

### Public API

```rust
pub struct InputDevice { ... }

impl InputDevice {
    pub fn new(transport: MmioTransport) -> Result<Self, InputError>;
    pub fn poll(&mut self) -> Option<InputEvent>;
}

pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
    // Mouse events (future)
}
```

---

## Tasks

### Task 1: Create Crate Structure
### Task 2: Create Cargo.toml
### Task 3: Move input.rs Logic
### Task 4: Update Workspace
### Task 5: Update Kernel
### Task 6: Verify Build

---

## Exit Criteria

- [ ] New crate created
- [ ] Input driver logic moved
- [ ] Clean public API
- [ ] Build passes
- [ ] Keyboard input works
- [ ] Tests pass
