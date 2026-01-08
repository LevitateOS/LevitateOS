# Phase 1 Step 2: Research fatfs Crate Requirements

**Phase:** 1 (Discovery)  
**Step:** 2  
**UoW Count:** 1 (this file)

---

## Goal

Document what `fatfs` crate requires to integrate with our block driver.

---

## Tasks

1. Review `fatfs` crate documentation
2. Identify required traits
3. Document feature flags for `no_std`

---

## Expected Output

### fatfs Dependency

```toml
[dependencies]
fatfs = { version = "0.4", default-features = false, features = ["alloc", "lfn"] }
```

### Required Traits

`fatfs::FileSystem::new()` requires a type implementing:

```rust
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
}
```

### Adapter Design

We need a `BlockDeviceIO` struct that:
1. Tracks current byte position
2. Translates byte reads to block reads
3. Handles partial block reads (read full block, copy portion)
4. Buffers writes for partial blocks (read-modify-write)

### Feature Flags

| Feature | Purpose | Need? |
|---------|---------|-------|
| `alloc` | Use heap allocation | ✅ Yes (we have heap) |
| `lfn` | Long filename support | ✅ Yes (nice to have) |
| `unicode` | Unicode case folding | ❌ No (save memory) |

---

## Exit Criteria

- [x] Dependency documented
- [x] Required traits identified
- [x] Adapter design sketched

→ **Next:** Phase 1 Step 3
