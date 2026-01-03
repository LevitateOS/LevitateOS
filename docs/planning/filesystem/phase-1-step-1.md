# Phase 1 Step 1: Document Block Driver Interface

**Phase:** 1 (Discovery)  
**Step:** 1  
**UoW Count:** 1 (this file)

---

## Goal

Document the exact API exposed by `kernel/src/block.rs` for use by the filesystem layer.

---

## Tasks

1. Read `kernel/src/block.rs`
2. Document public functions and their signatures
3. Note any constraints (buffer size, alignment, error handling)

---

## Expected Output

### Block Driver API

```rust
// kernel/src/block.rs

pub const BLOCK_SIZE: usize = 512;

/// Read one 512-byte block from disk
pub fn read_block(block_id: usize, buf: &mut [u8]);

/// Write one 512-byte block to disk  
pub fn write_block(block_id: usize, buf: &[u8]);
```

### Constraints

- Buffer MUST be exactly 512 bytes (enforced by assert)
- Operations are synchronous (blocking)
- Panics on error (no Result return)
- Single global `BLOCK_DEVICE` protected by `Spinlock`

### Integration Notes

- `fatfs` needs `Read + Write + Seek` traits
- We need an adapter struct that translates byte-level I/O to block-level

---

## Exit Criteria

- [x] API documented
- [x] Constraints noted
- [x] Integration path identified

→ **Next:** Phase 1 Step 2
