# Phase 2, Step 3: Rename GPU Driver Crate

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 1 complete (VirtQueue fixed)  
**Status:** Planned  
**Estimated Effort:** 1 UoW

---

## Objective

Rename `levitate-virtio-gpu` → `levitate-drivers-gpu` to follow naming convention.

---

## Current State

```
levitate-virtio-gpu/
├── Cargo.toml          # name = "levitate-virtio-gpu"
├── src/
│   ├── lib.rs
│   ├── device.rs
│   ├── driver.rs
│   ├── command.rs
│   └── protocol/
```

---

## Target State

```
levitate-drivers-gpu/
├── Cargo.toml          # name = "levitate-drivers-gpu"
├── src/
│   └── ... (same structure)
```

---

## Tasks

### Task 1: Rename Directory

```bash
mv levitate-virtio-gpu levitate-drivers-gpu
```

### Task 2: Update Cargo.toml

```toml
[package]
name = "levitate-drivers-gpu"  # Changed
version = "0.1.0"
edition = "2024"
description = "VirtIO GPU driver for LevitateOS"
```

### Task 3: Update Workspace Cargo.toml

```toml
members = [
    "kernel",
    "levitate-hal",
    "levitate-utils",
    "levitate-gpu",           # Will be deleted in Phase 4
    "levitate-terminal",
    "levitate-virtio",
    "levitate-drivers-gpu",   # Changed from levitate-virtio-gpu
    "xtask",
]
```

### Task 4: Update kernel/Cargo.toml

```toml
[dependencies]
levitate-drivers-gpu = { path = "../levitate-drivers-gpu" }  # Changed
```

### Task 5: Update Imports

Find and replace in all files:
```rust
// Old
use levitate_virtio_gpu::...

// New
use levitate_drivers_gpu::...
```

### Task 6: Update README

Update `levitate-drivers-gpu/README.md` with new name.

### Task 7: Verify Build

```bash
cargo build --release
cargo xtask test
```

---

## Exit Criteria

- [ ] Directory renamed
- [ ] Cargo.toml name updated
- [ ] Workspace updated
- [ ] All imports updated
- [ ] Build passes
- [ ] Tests pass
