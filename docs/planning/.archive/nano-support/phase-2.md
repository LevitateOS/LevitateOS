# Phase 2: File-Backed mmap Implementation

## Objective

Implement `mmap()` support for file-backed mappings (MAP_PRIVATE with fd != -1) to enable the musl dynamic linker to load shared libraries.

## Current State

The kernel's mmap implementation (`crates/kernel/syscall/src/mm.rs`) only supports `MAP_ANONYMOUS`:

```rust
// Lines 193-204
if flags & MAP_ANONYMOUS == 0 {
    log::warn!("[MMAP] Only MAP_ANONYMOUS supported, got flags=0x{:x}", flags);
    return Err(EINVAL);
}
if fd != -1 || offset != 0 {
    log::warn!("[MMAP] File-backed mappings not supported");
    return Err(EINVAL);
}
```

## What musl's Dynamic Linker Needs

From `musl/ldso/dynlink.c`:

```c
// Map library segments with correct permissions
void *base = mmap(0, ph->p_memsz + (ph->p_vaddr & PAGE_SIZE-1),
                  PROT_READ|PROT_WRITE,  // Initially writable for relocations
                  MAP_PRIVATE,           // NOT shared with other processes
                  fd,                    // Library file descriptor
                  ph->p_offset);         // File offset (page-aligned)

// Later: mprotect to set correct permissions
mprotect(base, len, PROT_READ|PROT_EXEC);  // For .text
```

## Design Decisions

### 1. Eager Copy vs. True Copy-on-Write

**Decision: Eager Copy (MVP)**

| Approach | Complexity | Memory Use | When to Use |
|----------|------------|------------|-------------|
| Eager copy | Low (~50 lines) | Higher (copies everything) | MVP - just make it work |
| True CoW | High (~300+ lines) | Efficient | Future optimization for fork() |

**Rationale:** Eager copy is correct and simpler. Dynamic linking doesn't rely on CoW behavior - it just needs a private copy. True CoW is only beneficial for fork() where parent/child share pages until write.

### 2. Offset Handling

File offsets must be page-aligned (Linux requirement). Non-page-aligned offsets return EINVAL.

### 3. Partial Page at End

If `offset + len` extends past file size:
- Read available bytes
- Zero-fill remainder
- This is standard behavior for mmap

### 4. VMA Tracking

File-backed mappings get a new flag: `VmaFlags::FILE_BACKED`. This helps with:
- msync() (future)
- /proc/self/maps debugging

## Implementation Plan

### Step 1: Add VMA Flags

```rust
// In los_mm/src/vma.rs
bitflags! {
    pub struct VmaFlags: u32 {
        const READ = 0x1;
        const WRITE = 0x2;
        const EXEC = 0x4;
        const FILE_BACKED = 0x10;  // NEW: Indicates file-backed mapping
    }
}
```

### Step 2: Helper Function - Read File Data

```rust
// In syscall/src/mm.rs

/// Read file data for mmap.
/// Returns vector of bytes, zero-padded to page alignment.
fn read_file_for_mmap(fd: i32, offset: usize, len: usize) -> Result<Vec<u8>, u32> {
    use los_sched::current_task;

    let task = current_task();
    let fds = task.fds.lock();
    let file = fds.get(fd as usize).ok_or(EBADF)?.clone();
    drop(fds);  // Release lock early

    // Allocate buffer (page-aligned size)
    let aligned_len = page_align_up(len);
    let mut buf = vec![0u8; aligned_len];

    // Read from file at offset
    let bytes_read = file.read_at(offset, &mut buf[..len])
        .map_err(|_| EIO)?;

    // Zero-fill if we got less than requested (end of file)
    // buf is already zero-initialized, so no action needed

    Ok(buf)
}
```

### Step 3: Modify sys_mmap

```rust
pub fn sys_mmap(
    addr: usize,
    len: usize,
    prot: u32,
    flags: u32,
    fd: i32,
    offset: usize,
) -> SyscallResult {
    if len == 0 {
        return Err(EINVAL);
    }

    // NEW: Check if this is a file-backed mapping
    let is_anonymous = flags & MAP_ANONYMOUS != 0;

    if !is_anonymous {
        // File-backed mapping
        if fd < 0 {
            return Err(EBADF);
        }
        // Offset must be page-aligned
        if !is_page_aligned(offset) {
            return Err(EINVAL);
        }
        return sys_mmap_file(addr, len, prot, flags, fd, offset);
    }

    // Anonymous mapping (existing code)
    if fd != -1 || offset != 0 {
        return Err(EINVAL);
    }
    // ... rest of existing anonymous mmap code ...
}
```

### Step 4: File-Backed mmap Implementation

```rust
/// Handle file-backed mmap (MAP_PRIVATE with fd).
fn sys_mmap_file(
    addr: usize,
    len: usize,
    prot: u32,
    flags: u32,
    fd: i32,
    offset: usize,
) -> SyscallResult {
    log::trace!(
        "[MMAP] File-backed: fd={} len=0x{:x} offset=0x{:x} prot=0x{:x}",
        fd, len, offset, prot
    );

    // Only MAP_PRIVATE supported (not MAP_SHARED)
    const MAP_PRIVATE: u32 = 0x2;
    if flags & MAP_PRIVATE == 0 {
        log::warn!("[MMAP] Only MAP_PRIVATE supported for file mappings");
        return Err(EINVAL);
    }

    let task = los_sched::current_task();
    let ttbr0 = task.ttbr0.load(Ordering::Acquire);

    // 1. Read file data into buffer
    let file_data = read_file_for_mmap(fd, offset, len)?;

    // 2. Find free region in user address space
    let pages_needed = file_data.len() / PAGE_SIZE;
    let base_addr = if addr != 0 && flags & MAP_FIXED != 0 {
        if !is_page_aligned(addr) {
            return Err(EINVAL);
        }
        addr
    } else {
        find_free_mmap_region(ttbr0, file_data.len()).ok_or(ENOMEM)?
    };

    // 3. Create RAII guard for cleanup on failure
    let mut guard = MmapGuard::new(ttbr0);
    let page_flags = prot_to_page_flags(prot);

    // 4. Allocate pages and copy data
    for i in 0..pages_needed {
        let va = base_addr + i * PAGE_SIZE;
        let data_offset = i * PAGE_SIZE;

        // Allocate physical page
        let phys = FRAME_ALLOCATOR.alloc_page().ok_or(ENOMEM)?;

        // Copy file data to page
        let page_ptr = mmu::phys_to_virt(phys) as *mut u8;
        unsafe {
            let src = file_data[data_offset..data_offset + PAGE_SIZE].as_ptr();
            core::ptr::copy_nonoverlapping(src, page_ptr, PAGE_SIZE);
        }

        // Map into user address space
        if unsafe { mm_user::map_user_page(ttbr0, va, phys, page_flags) }.is_err() {
            FRAME_ALLOCATOR.free_page(phys);
            return Err(ENOMEM);
        }

        guard.track(va, phys);
    }

    // 5. Success - commit the guard
    guard.commit();

    // 6. Record VMA with FILE_BACKED flag
    {
        use los_mm::vma::Vma;
        let mut vma_flags = prot_to_vma_flags(prot);
        vma_flags |= VmaFlags::FILE_BACKED;
        let vma = Vma::new(base_addr, base_addr + file_data.len(), vma_flags);
        let mut vmas = task.vmas.lock();
        let _ = vmas.insert(vma);
    }

    log::trace!(
        "[MMAP] File-backed: mapped {} pages at 0x{:x}",
        pages_needed, base_addr
    );

    Ok(base_addr as i64)
}
```

### Step 5: File read_at Method

The VFS needs to support positional reads. Check if this exists:

```rust
// In VFS File trait
fn read_at(&self, offset: usize, buf: &mut [u8]) -> VfsResult<usize>;
```

If not present, fall back to seek + read (less efficient but works).

## Testing Plan

### Unit Tests

```rust
#[test]
fn test_mmap_file_read() {
    // Create temp file with known content
    // mmap it
    // Verify can read content through mapping
}

#[test]
fn test_mmap_file_private_cow() {
    // mmap file as MAP_PRIVATE
    // Write to mapping
    // Verify original file unchanged
}

#[test]
fn test_mmap_file_offset() {
    // Create file with multiple pages of content
    // mmap at offset PAGE_SIZE
    // Verify correct content visible
}
```

### Integration Tests

```bash
# Test dynamic hello world
cargo xtask vm exec "/bin/hello_dynamic"
# Expected: "Hello, World!" (once file mmap works)

# Test ldd
cargo xtask vm exec "/bin/ldd /bin/nano"
# Expected: List of shared library dependencies
```

## Complexity Estimate

| Component | Lines | Difficulty |
|-----------|-------|------------|
| `read_file_for_mmap()` | 30 | Easy |
| `sys_mmap_file()` | 80 | Medium |
| VMA flags update | 5 | Easy |
| Error handling | 20 | Easy |
| **Total** | ~135 | Medium |

## Risks and Mitigations

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| VFS read_at missing | Medium | Add read_at or use seek+read |
| Memory exhaustion (large files) | Low | Add sanity limit (e.g., 256MB max) |
| Incorrect page permissions | Medium | Verify with test binaries |
| Alignment edge cases | Low | Comprehensive tests |

## Acceptance Criteria

- [ ] `mmap(fd, MAP_PRIVATE, offset)` returns valid address
- [ ] Mapped memory contains correct file data
- [ ] Writes to mapping don't affect original file
- [ ] Page-unaligned offsets return EINVAL
- [ ] `munmap` correctly frees pages
- [ ] `/bin/hello_dynamic` prints "Hello, World!"
- [ ] nano launches (may fail later for other reasons)

## Dependencies

| Dependency | Status |
|------------|--------|
| VFS file operations | Complete |
| Frame allocator | Complete |
| Page mapping | Complete |
| PT_INTERP support | Complete (TEAM_470) |

## Files to Modify

| File | Changes |
|------|---------|
| `syscall/src/mm.rs` | Add `sys_mmap_file()`, modify `sys_mmap()` |
| `los_mm/src/vma.rs` | Add `FILE_BACKED` flag (optional) |
| `los_vfs` | Add `read_at()` if missing |

## Future Enhancements (Out of Scope)

- True copy-on-write (for fork() efficiency)
- MAP_SHARED support (shared memory between processes)
- msync() for writing back to files
- mmap huge pages support
