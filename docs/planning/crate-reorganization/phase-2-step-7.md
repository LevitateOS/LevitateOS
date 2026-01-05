# Phase 2, Step 7: Create Filesystem Crate

**Parent:** [phase-2.md](./phase-2.md)  
**Depends:** Step 4 complete (Block driver extracted)  
**Status:** Planned  
**Estimated Effort:** 3 UoW

---

## Objective

Create `levitate-fs` crate to wrap filesystem abstractions (FAT32, ext4).

---

## Current State

### kernel/src/fs/

Contains:
- FAT32 support via `embedded-sdmmc`
- ext4 support via `ext4-view`
- Various filesystem operations

### Direct Dependencies in kernel/Cargo.toml

```toml
embedded-sdmmc = { version = "0.9", default-features = false }
ext4-view = { version = "0.9", default-features = false }
```

---

## Target State

### New Crate: levitate-fs/

```
levitate-fs/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── traits.rs      # Filesystem trait definitions
    ├── fat32.rs       # FAT32 wrapper
    └── ext4.rs        # ext4 wrapper
```

### Public API

```rust
pub trait Filesystem {
    fn read_file(&mut self, path: &str) -> Result<Vec<u8>, FsError>;
    fn list_dir(&mut self, path: &str) -> Result<Vec<DirEntry>, FsError>;
    fn exists(&self, path: &str) -> bool;
}

pub struct Fat32Fs<B: BlockDevice> { ... }
pub struct Ext4Fs<B: BlockDevice> { ... }

impl<B: BlockDevice> Filesystem for Fat32Fs<B> { ... }
impl<B: BlockDevice> Filesystem for Ext4Fs<B> { ... }
```

---

## Tasks

### Task 1: Create Crate Structure

```bash
mkdir -p levitate-fs/src
```

### Task 2: Create Cargo.toml

```toml
[package]
name = "levitate-fs"
version = "0.1.0"
edition = "2024"
description = "Filesystem abstractions for LevitateOS"

[dependencies]
levitate-drivers-blk = { path = "../levitate-drivers-blk" }
embedded-sdmmc = { version = "0.9", default-features = false }
ext4-view = { version = "0.9", default-features = false }

[lints]
workspace = true
```

### Task 3: Define Filesystem Trait

```rust
// src/traits.rs
pub trait Filesystem {
    type Error;
    fn read_file(&mut self, path: &str) -> Result<alloc::vec::Vec<u8>, Self::Error>;
    fn list_dir(&mut self, path: &str) -> Result<alloc::vec::Vec<DirEntry>, Self::Error>;
}
```

### Task 4: Wrap FAT32

Move FAT32-related code from kernel/src/fs/ to levitate-fs/src/fat32.rs

### Task 5: Wrap ext4

Move ext4-related code from kernel/src/fs/ to levitate-fs/src/ext4.rs

### Task 6: Update Workspace

### Task 7: Update Kernel

Remove direct deps on embedded-sdmmc, ext4-view:
```toml
[dependencies]
levitate-fs = { path = "../levitate-fs" }
# Remove: embedded-sdmmc, ext4-view
```

### Task 8: Verify Build

---

## Exit Criteria

- [ ] New crate created
- [ ] Filesystem trait defined
- [ ] FAT32 wrapped
- [ ] ext4 wrapped
- [ ] Kernel uses levitate-fs
- [ ] Kernel no longer has direct embedded-sdmmc/ext4-view deps
- [ ] Build passes
- [ ] Initramfs still loads
- [ ] Tests pass
