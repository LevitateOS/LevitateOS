# Phase 2: Design - Slab Allocator

**Team:** TEAM_050
**Status:** In Progress
**Date:** 2026-01-04
**Depends On:** Phase 1 (Discovery)

---

## Design Overview

A SLUB-style slab allocator optimized for Pixel 6 (Google Tensor GS101) stability.

```
┌─────────────────────────────────────────────────────────────┐
│                      SlabAllocator                          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐     ┌─────────┐       │
│  │Cache[0] │ │Cache[1] │ │Cache[2] │ ... │Cache[5] │       │
│  │  64B    │ │  128B   │ │  256B   │     │  2048B  │       │
│  └────┬────┘ └────┬────┘ └────┬────┘     └────┬────┘       │
│       │           │           │               │             │
│       ▼           ▼           ▼               ▼             │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐     ┌─────────┐       │
│  │SlabList │ │SlabList │ │SlabList │     │SlabList │       │
│  │partial  │ │partial  │ │partial  │     │partial  │       │
│  └────┬────┘ └────┬────┘ └────┬────┘     └────┬────┘       │
│       │           │           │               │             │
│       ▼           ▼           ▼               ▼             │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐     ┌─────────┐       │
│  │SlabPage │ │SlabPage │ │SlabPage │     │SlabPage │       │
│  │ 4KB     │ │ 4KB     │ │ 4KB     │     │ 4KB     │       │
│  └─────────┘ └─────────┘ └─────────┘     └─────────┘       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │ BuddyAllocator  │
                    │ (backing pages) │
                    └─────────────────┘
```

---

## Data Structures

### 1. SlabPage (4KB)

A single physical page subdivided into fixed-size objects.

```rust
/// A 4KB page used as a slab for fixed-size allocations.
/// 
/// Memory Layout:
/// ┌────────────────────────────────────────┐ 0x000
/// │           Object Storage               │
/// │         (4032 bytes usable)            │
/// ├────────────────────────────────────────┤ 0xFC0
/// │              Metadata                  │
/// │            (64 bytes)                  │
/// └────────────────────────────────────────┘ 0x1000
///
/// Metadata is at the END to preserve object alignment at page start.
#[repr(C)]
pub struct SlabPage {
    /// Object storage area (page_start to page_start + DATA_SIZE)
    data: [u8; Self::DATA_SIZE],
    
    /// Metadata section (64 bytes, cache-line aligned)
    meta: SlabPageMeta,
}

#[repr(C)]
pub struct SlabPageMeta {
    /// Bitfield tracking allocation status (up to 64 objects)
    /// Bit set = allocated, bit clear = free
    bitfield: AtomicU64,           // 8 bytes
    
    /// Size class index (0-5)
    size_class: u8,                // 1 byte
    
    /// Number of allocated objects in this page
    allocated_count: u8,           // 1 byte
    
    /// Padding for alignment
    _pad: [u8; 6],                 // 6 bytes
    
    /// Intrusive list pointers (for SlabList)
    next: Option<NonNull<SlabPage>>,  // 8 bytes
    prev: Option<NonNull<SlabPage>>,  // 8 bytes
    
    /// Physical address of this page (for freeing back to Buddy)
    phys_addr: usize,              // 8 bytes
    
    /// Reserved for future use
    _reserved: [u8; 24],           // 24 bytes
}
// Total: 64 bytes

impl SlabPage {
    pub const SIZE: usize = 4096;
    pub const META_SIZE: usize = 64;
    pub const DATA_SIZE: usize = Self::SIZE - Self::META_SIZE; // 4032 bytes
}
```

### 2. SlabList

Intrusive doubly-linked list of SlabPages.

```rust
/// A list of SlabPages with the same allocation state.
pub struct SlabList {
    head: Option<NonNull<SlabPage>>,
    count: usize,
}

impl SlabList {
    pub const fn new() -> Self {
        Self { head: None, count: 0 }
    }
    
    /// Insert page at front of list. O(1).
    pub fn push_front(&mut self, page: &mut SlabPage);
    
    /// Remove specific page from list. O(1).
    pub fn remove(&mut self, page: &mut SlabPage);
    
    /// Pop page from front. O(1).
    pub fn pop_front(&mut self) -> Option<NonNull<SlabPage>>;
    
    /// Check if list is empty.
    pub fn is_empty(&self) -> bool;
}
```

### 3. SlabCache

Per-size-class allocator managing partial/full/empty pages.

```rust
/// Size class configuration
pub struct SizeClass {
    pub object_size: usize,
    pub objects_per_page: usize,
}

pub const SIZE_CLASSES: [SizeClass; 6] = [
    SizeClass { object_size: 64,   objects_per_page: 63 },  // 4032/64 = 63
    SizeClass { object_size: 128,  objects_per_page: 31 },  // 4032/128 = 31
    SizeClass { object_size: 256,  objects_per_page: 15 },  // 4032/256 = 15
    SizeClass { object_size: 512,  objects_per_page: 7 },   // 4032/512 = 7
    SizeClass { object_size: 1024, objects_per_page: 3 },   // 4032/1024 = 3
    SizeClass { object_size: 2048, objects_per_page: 1 },   // 4032/2048 = 1
];

/// Manages all slabs for a single size class.
pub struct SlabCache {
    /// Size class index (0-5)
    class_index: usize,
    
    /// Pages with some free objects (allocation target)
    partial: SlabList,
    
    /// Pages with all objects allocated (skip during alloc)
    full: SlabList,
    
    /// Pages with no objects allocated (can return to Buddy)
    empty: SlabList,
    
    /// Statistics
    total_allocs: usize,
    total_frees: usize,
}

impl SlabCache {
    /// Allocate an object from this cache.
    /// Returns virtual address of allocated object, or None if OOM.
    /// 
    /// # Behavior
    /// 1. Try partial list first
    /// 2. If empty, try to reclaim from empty list
    /// 3. If still empty, request new page from BuddyAllocator
    /// 4. Update page state (partial → full if needed)
    pub fn alloc(&mut self) -> Option<NonNull<u8>>;
    
    /// Free an object back to this cache.
    /// 
    /// # Safety
    /// Caller must ensure ptr was allocated from this cache.
    /// 
    /// # Behavior
    /// 1. Compute page from pointer (mask lower 12 bits)
    /// 2. Compute object index within page
    /// 3. Clear bit in bitfield
    /// 4. Update page state (full → partial, partial → empty)
    pub unsafe fn free(&mut self, ptr: NonNull<u8>);
    
    /// Request a new backing page from BuddyAllocator.
    fn grow(&mut self) -> Option<NonNull<SlabPage>>;
}
```

### 4. SlabAllocator

Top-level allocator routing requests to appropriate SlabCache.

```rust
/// Global slab allocator managing all size classes.
/// 
/// Thread-safety: Protected by Spinlock (single global lock).
pub struct SlabAllocator {
    caches: [SlabCache; 6],
}

impl SlabAllocator {
    pub const fn new() -> Self;
    
    /// Allocate memory for given layout.
    /// 
    /// # Behavior
    /// - Size ≤ 64: use cache[0]
    /// - Size ≤ 128: use cache[1]
    /// - ...
    /// - Size > 2048: return None (use BuddyAllocator directly)
    pub fn alloc(&mut self, layout: Layout) -> Option<NonNull<u8>>;
    
    /// Free memory.
    /// 
    /// # Safety
    /// Caller must provide correct layout that was used for allocation.
    pub unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout);
    
    /// Get cache index for given size.
    fn size_to_class(size: usize) -> Option<usize> {
        match size {
            0..=64 => Some(0),
            65..=128 => Some(1),
            129..=256 => Some(2),
            257..=512 => Some(3),
            513..=1024 => Some(4),
            1025..=2048 => Some(5),
            _ => None,
        }
    }
}

/// Global instance wrapped in Spinlock
pub static SLAB_ALLOCATOR: Spinlock<SlabAllocator> = Spinlock::new(SlabAllocator::new());
```

---

## Behavioral Contracts

### [S1] Allocation from Partial Slab

**Precondition:** `partial` list is non-empty.

**Behavior:**
1. Get head page from `partial` list
2. Find first clear bit in `bitfield` (trailing zeros)
3. Set bit, increment `allocated_count`
4. If page is now full (`allocated_count == objects_per_page`):
   - Remove from `partial`, add to `full`
5. Return `page_addr + (bit_index * object_size)`

**Postcondition:** One object allocated, page state updated correctly.

### [S2] Allocation from Empty Slab

**Precondition:** `partial` is empty, `empty` is non-empty.

**Behavior:**
1. Pop page from `empty` list
2. Add to `partial` list
3. Proceed as [S1]

**Postcondition:** Empty page promoted to partial, object allocated.

### [S3] Allocation Requiring Growth

**Precondition:** `partial` and `empty` are both empty.

**Behavior:**
1. Call `BuddyAllocator.alloc(0)` for one 4KB page
2. If OOM, return `None`
3. Initialize `SlabPageMeta` at end of page
4. Add to `partial` list
5. Proceed as [S1]

**Postcondition:** New page created, object allocated.

### [S4] Deallocation to Partial Slab

**Precondition:** Page has `allocated_count > 1` after free.

**Behavior:**
1. Compute `page_addr = ptr & !0xFFF`
2. Compute `index = (ptr - page_addr) / object_size`
3. Clear bit at `index` in `bitfield`
4. Decrement `allocated_count`
5. If page was in `full` list, move to `partial`

**Postcondition:** Object freed, page remains in `partial`.

### [S5] Deallocation Making Page Empty

**Precondition:** Page has `allocated_count == 1` before free.

**Behavior:**
1. Free object as in [S4]
2. `allocated_count` becomes 0
3. Move page from `partial` to `empty`
4. **Do NOT return to BuddyAllocator** (stability decision)

**Postcondition:** Page in `empty` list, cached for future allocations.

### [S6] Deallocation from Full Slab

**Precondition:** Page is in `full` list.

**Behavior:**
1. Free object as in [S4]
2. Page is no longer full
3. Move from `full` to `partial`

**Postcondition:** Page promoted to `partial`, available for allocation.

---

## Edge Cases & Error Handling

| Scenario | Behavior |
|----------|----------|
| Alloc size = 0 | Return `None` (invalid) |
| Alloc size > 2048 | Return `None` (use Buddy directly) |
| Alloc alignment > object_size | Return `None` (unsupported) |
| BuddyAllocator OOM | Return `None`, do not panic |
| Double free | **Undefined behavior** (debug: assert bit is set) |
| Free wrong size class | **Undefined behavior** (debug: assert class matches) |
| Free null pointer | Early return, no-op |

---

## Module Structure

```
levitate-hal/src/allocator/
├── mod.rs          # Re-exports
├── buddy.rs        # Existing BuddyAllocator
├── page.rs         # Existing Page descriptor
└── slab/
    ├── mod.rs      # SlabAllocator, SLAB_ALLOCATOR global
    ├── cache.rs    # SlabCache implementation
    ├── page.rs     # SlabPage, SlabPageMeta
    └── list.rs     # SlabList intrusive list
```

---

## API Summary

```rust
// Primary API (explicit, type-safe)
use levitate_hal::allocator::slab::SLAB_ALLOCATOR;

// Allocate 128 bytes
let ptr = SLAB_ALLOCATOR.lock().alloc(Layout::from_size_align(128, 8).unwrap());

// Free
unsafe { SLAB_ALLOCATOR.lock().dealloc(ptr, Layout::from_size_align(128, 8).unwrap()); }
```

---

## Testing Strategy

### Unit Tests (Phase 3)

| ID | Test | Behavior Verified |
|----|------|-------------------|
| T1 | `test_alloc_single` | [S1] Basic allocation |
| T2 | `test_alloc_fill_page` | [S1] Fill page, verify full transition |
| T3 | `test_free_to_partial` | [S4] Free from full page |
| T4 | `test_free_to_empty` | [S5] Free last object |
| T5 | `test_alloc_after_free` | Reuse freed slot |
| T6 | `test_grow` | [S3] Allocation triggers growth |
| T7 | `test_size_classes` | All 6 classes work |
| T8 | `test_alignment` | Objects aligned to size |

### Integration Tests (Phase 4)

| ID | Test | Behavior Verified |
|----|------|-------------------|
| I1 | `test_buddy_integration` | SlabAllocator uses BuddyAllocator |
| I2 | `test_concurrent_alloc` | Spinlock correctness |
| I3 | `test_oom_recovery` | Graceful OOM handling |

---

## Phase 2 Checklist

- [x] Define `SlabPage` memory layout
- [x] Define `SlabPageMeta` structure
- [x] Define `SlabList` intrusive list
- [x] Define `SlabCache` per-class allocator
- [x] Define `SlabAllocator` top-level API
- [x] Document behavioral contracts [S1]-[S6]
- [x] Document edge cases
- [x] Define module structure
- [x] Define testing strategy

---

## Next Steps

→ **Phase 3: Implementation** — Implement data structures and unit tests
