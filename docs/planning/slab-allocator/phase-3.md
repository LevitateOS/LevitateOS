# Phase 3: Implementation - Slab Allocator

**Team:** TEAM_050 (Planning Only)
**Status:** Ready for Implementation
**Date:** 2026-01-04
**Depends On:** Phase 2 (Design)

---

## For Future Teams

This document provides a **step-by-step implementation plan** for the Slab Allocator.
Each step is designed to be completable in a single session with clear verification criteria.

**Prerequisites:**
- Read `phase-1.md` (Discovery) for context
- Read `phase-2.md` (Design) for data structures and behavioral contracts

---

## Implementation Order

```
Step 1: SlabList       → Step 2: SlabPage       → Step 3: SlabCache
    ↓                       ↓                        ↓
  (list.rs)              (page.rs)               (cache.rs)
                                                     ↓
                                              Step 4: SlabAllocator
                                                     ↓
                                                  (mod.rs)
                                                     ↓
                                              Step 5: Integration
                                                     ↓
                                              Step 6: Tests
```

---

## Step 1: SlabList (Intrusive Linked List)

**File:** `levitate-hal/src/allocator/slab/list.rs`

**Goal:** Implement an intrusive doubly-linked list for SlabPage management.

### 1.1 Create Module Structure

```bash
mkdir -p levitate-hal/src/allocator/slab
touch levitate-hal/src/allocator/slab/list.rs
```

### 1.2 Implement SlabList

```rust
// TEAM_XXX: Slab Allocator - Intrusive List
// See docs/planning/slab-allocator/phase-2.md for design

use core::ptr::NonNull;

/// Intrusive list node trait.
/// Types stored in SlabList must implement this.
pub trait ListNode {
    fn next(&self) -> Option<NonNull<Self>>;
    fn prev(&self) -> Option<NonNull<Self>>;
    fn set_next(&mut self, next: Option<NonNull<Self>>);
    fn set_prev(&mut self, prev: Option<NonNull<Self>>);
}

/// Intrusive doubly-linked list.
pub struct SlabList<T: ListNode> {
    head: Option<NonNull<T>>,
    count: usize,
}

impl<T: ListNode> SlabList<T> {
    pub const fn new() -> Self { /* ... */ }
    pub fn push_front(&mut self, node: &mut T) { /* ... */ }
    pub fn remove(&mut self, node: &mut T) { /* ... */ }
    pub fn pop_front(&mut self) -> Option<NonNull<T>> { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
    pub fn len(&self) -> usize { /* ... */ }
}
```

### 1.3 Verification

```rust
#[cfg(test)]
mod tests {
    // T1: Test push_front adds to head
    // T2: Test remove from middle
    // T3: Test pop_front returns head
    // T4: Test empty list behavior
}
```

**Done when:** All list operations work correctly in isolation.

---

## Step 2: SlabPage (Page Structure)

**File:** `levitate-hal/src/allocator/slab/page.rs`

**Goal:** Implement the 4KB slab page with metadata and bitfield.

### 2.1 Implement SlabPage

```rust
// TEAM_XXX: Slab Allocator - Page Structure
// Layout: [data: 4032 bytes][metadata: 64 bytes]

use core::sync::atomic::{AtomicU64, Ordering};
use core::ptr::NonNull;
use super::list::ListNode;

pub const PAGE_SIZE: usize = 4096;
pub const META_SIZE: usize = 64;
pub const DATA_SIZE: usize = PAGE_SIZE - META_SIZE;

/// Metadata stored at end of each slab page.
#[repr(C)]
pub struct SlabPageMeta {
    pub bitfield: AtomicU64,              // 8B - allocation bitmap
    pub size_class: u8,                   // 1B - class index (0-5)
    pub allocated_count: u8,              // 1B - objects allocated
    _pad: [u8; 6],                         // 6B - alignment
    pub next: Option<NonNull<SlabPage>>,  // 8B - list ptr
    pub prev: Option<NonNull<SlabPage>>,  // 8B - list ptr
    pub phys_addr: usize,                 // 8B - for freeing
    _reserved: [u8; 24],                  // 24B - future use
}

/// A 4KB slab page.
#[repr(C)]
pub struct SlabPage {
    data: [u8; DATA_SIZE],
    pub meta: SlabPageMeta,
}

impl SlabPage {
    /// Initialize metadata for a new slab page.
    pub unsafe fn init(page_ptr: *mut u8, size_class: u8, phys_addr: usize) { /* ... */ }
    
    /// Allocate one object, return offset within page.
    pub fn alloc_object(&mut self, object_size: usize) -> Option<usize> { /* ... */ }
    
    /// Free object at given offset.
    pub fn free_object(&mut self, offset: usize, object_size: usize) { /* ... */ }
    
    /// Check if page is full.
    pub fn is_full(&self, objects_per_page: usize) -> bool { /* ... */ }
    
    /// Check if page is empty.
    pub fn is_empty(&self) -> bool { /* ... */ }
}

impl ListNode for SlabPage { /* ... */ }
```

### 2.2 Bitfield Operations

```rust
impl SlabPage {
    /// Find first free slot (first zero bit).
    fn find_free_slot(&self) -> Option<usize> {
        let bits = self.meta.bitfield.load(Ordering::Relaxed);
        if bits == u64::MAX {
            return None; // All allocated
        }
        Some(bits.trailing_ones() as usize)
    }
    
    /// Mark slot as allocated.
    fn set_allocated(&self, index: usize) {
        self.meta.bitfield.fetch_or(1 << index, Ordering::Relaxed);
    }
    
    /// Mark slot as free.
    fn set_free(&self, index: usize) {
        self.meta.bitfield.fetch_and(!(1 << index), Ordering::Relaxed);
    }
}
```

### 2.3 Verification

```rust
#[cfg(test)]
mod tests {
    // T1: Test init sets metadata correctly
    // T2: Test alloc_object returns sequential offsets
    // T3: Test free_object clears bit
    // T4: Test is_full after max allocations
    // T5: Test is_empty after freeing all
}
```

**Done when:** Can allocate/free objects within a single page.

---

## Step 3: SlabCache (Per-Size-Class)

**File:** `levitate-hal/src/allocator/slab/cache.rs`

**Goal:** Implement per-size-class allocator with three lists.

### 3.1 Size Class Constants

```rust
// TEAM_XXX: Slab Allocator - Cache
// See phase-2.md behavioral contracts [S1]-[S6]

pub struct SizeClass {
    pub object_size: usize,
    pub objects_per_page: usize,
}

pub const SIZE_CLASSES: [SizeClass; 6] = [
    SizeClass { object_size: 64,   objects_per_page: 63 },
    SizeClass { object_size: 128,  objects_per_page: 31 },
    SizeClass { object_size: 256,  objects_per_page: 15 },
    SizeClass { object_size: 512,  objects_per_page: 7 },
    SizeClass { object_size: 1024, objects_per_page: 3 },
    SizeClass { object_size: 2048, objects_per_page: 1 },
];
```

### 3.2 Implement SlabCache

```rust
pub struct SlabCache {
    class_index: usize,
    partial: SlabList<SlabPage>,
    full: SlabList<SlabPage>,
    empty: SlabList<SlabPage>,
}

impl SlabCache {
    pub const fn new(class_index: usize) -> Self { /* ... */ }
    
    /// [S1][S2][S3] Allocate object from this cache.
    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        // 1. Try partial list
        // 2. If empty, try empty list (promote to partial)
        // 3. If still empty, call grow()
        // 4. Allocate from page, update lists if needed
    }
    
    /// [S4][S5][S6] Free object back to this cache.
    pub unsafe fn free(&mut self, ptr: NonNull<u8>) {
        // 1. Compute page from ptr (mask lower 12 bits)
        // 2. Free object in page
        // 3. Update lists based on new state
    }
    
    /// Request new page from BuddyAllocator.
    fn grow(&mut self) -> Option<NonNull<SlabPage>> {
        // Call FRAME_ALLOCATOR.0.lock().alloc(0)
        // Initialize page metadata
        // Add to partial list
    }
}
```

### 3.3 Verification

```rust
#[cfg(test)]
mod tests {
    // T1: Test alloc from empty cache triggers grow
    // T2: Test alloc moves page partial→full
    // T3: Test free moves page full→partial
    // T4: Test free moves page partial→empty
    // T5: Test reuse of empty page
}
```

**Done when:** Single-size-class allocation/deallocation works.

---

## Step 4: SlabAllocator (Top-Level)

**File:** `levitate-hal/src/allocator/slab/mod.rs`

**Goal:** Implement top-level allocator routing to appropriate cache.

### 4.1 Module Exports

```rust
// TEAM_XXX: Slab Allocator - Top Level

mod list;
mod page;
mod cache;

pub use page::{SlabPage, PAGE_SIZE, DATA_SIZE};
pub use cache::{SlabCache, SIZE_CLASSES};

use levitate_utils::Spinlock;
use core::alloc::Layout;
use core::ptr::NonNull;
```

### 4.2 Implement SlabAllocator

```rust
pub struct SlabAllocator {
    caches: [SlabCache; 6],
}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            caches: [
                SlabCache::new(0),
                SlabCache::new(1),
                SlabCache::new(2),
                SlabCache::new(3),
                SlabCache::new(4),
                SlabCache::new(5),
            ],
        }
    }
    
    pub fn alloc(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let class = Self::size_to_class(layout.size())?;
        self.caches[class].alloc()
    }
    
    pub unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        if let Some(class) = Self::size_to_class(layout.size()) {
            self.caches[class].free(ptr);
        }
    }
    
    fn size_to_class(size: usize) -> Option<usize> {
        match size {
            0 => None,
            1..=64 => Some(0),
            65..=128 => Some(1),
            129..=256 => Some(2),
            257..=512 => Some(3),
            513..=1024 => Some(4),
            1025..=2048 => Some(5),
            _ => None,
        }
    }
}

/// Global slab allocator instance.
pub static SLAB_ALLOCATOR: Spinlock<SlabAllocator> = Spinlock::new(SlabAllocator::new());
```

### 4.3 Update Module Exports

In `levitate-hal/src/allocator/mod.rs`:

```rust
pub mod buddy;
pub mod page;
pub mod slab;  // TEAM_XXX: Add slab module

pub use buddy::BuddyAllocator;
pub use page::Page;
pub use slab::SLAB_ALLOCATOR;  // TEAM_XXX: Export global
```

**Done when:** Can allocate/free through `SLAB_ALLOCATOR`.

---

## Step 5: Integration

**Goal:** Wire SlabCache::grow() to BuddyAllocator.

### 5.1 Import FRAME_ALLOCATOR

In `cache.rs`:

```rust
use crate::mmu;  // For phys_to_virt

fn grow(&mut self) -> Option<NonNull<SlabPage>> {
    // Get physical page from buddy allocator
    let phys_addr = crate::allocator::FRAME_ALLOCATOR.0.lock().alloc(0)?;
    
    // Convert to virtual address
    let virt_addr = mmu::phys_to_virt(phys_addr);
    let page_ptr = virt_addr as *mut SlabPage;
    
    // Initialize metadata
    unsafe {
        SlabPage::init(
            page_ptr as *mut u8,
            self.class_index as u8,
            phys_addr,
        );
    }
    
    // Add to partial list
    let page = unsafe { &mut *page_ptr };
    self.partial.push_front(page);
    
    NonNull::new(page_ptr)
}
```

**Done when:** Slab pages are backed by real physical memory.

---

## Step 6: Tests

**File:** `levitate-hal/src/allocator/slab/tests.rs` (or inline)

### 6.1 Unit Tests

| ID | Test | Contract |
|----|------|----------|
| T1 | `test_alloc_64b` | [S1] Basic allocation |
| T2 | `test_alloc_all_classes` | All 6 size classes |
| T3 | `test_fill_page` | [S1] Partial→Full transition |
| T4 | `test_free_to_partial` | [S6] Full→Partial |
| T5 | `test_free_to_empty` | [S5] Partial→Empty |
| T6 | `test_reuse_slot` | Free slot reused |
| T7 | `test_grow` | [S3] Growth on empty |
| T8 | `test_size_routing` | Correct class selection |

### 6.2 Run Tests

```bash
# Host tests (with std feature)
cargo test -p levitate-hal --features std

# Kernel integration (boot test)
./run.sh  # Verify boot succeeds
```

---

## Gotchas for Future Teams

### G1: Page Alignment

SlabPage must be 4KB-aligned. BuddyAllocator order-0 guarantees this.

### G2: Metadata at End

Metadata is at `page_addr + 4032`, NOT at start. This preserves object alignment.

### G3: Bitfield Atomics

Use `AtomicU64` with `Ordering::Relaxed` for bitfield ops. The global Spinlock provides synchronization.

### G4: Virtual vs Physical Addresses

- BuddyAllocator returns **physical** addresses
- Slab operations use **virtual** addresses
- Use `mmu::phys_to_virt()` and `mmu::virt_to_phys()`

### G5: Don't Free to Buddy

Per design decision, empty slabs stay cached. Do NOT return them to BuddyAllocator.

---

## Verification Checklist

Before marking implementation complete:

- [ ] `cargo build -p levitate-hal` succeeds
- [ ] `cargo test -p levitate-hal --features std` passes
- [ ] `./run.sh` boots without panic
- [ ] Manual test: allocate/free objects of each size class
- [ ] Update behavior-inventory.md with new tests

---

## Files to Create

```
levitate-hal/src/allocator/slab/
├── mod.rs      # SlabAllocator, exports, SLAB_ALLOCATOR
├── list.rs     # SlabList<T>
├── page.rs     # SlabPage, SlabPageMeta
└── cache.rs    # SlabCache, SIZE_CLASSES
```

---

## Summary

| Step | File | LOC Estimate | Dependencies |
|------|------|--------------|--------------|
| 1 | list.rs | ~80 | None |
| 2 | page.rs | ~120 | list.rs |
| 3 | cache.rs | ~150 | page.rs, list.rs |
| 4 | mod.rs | ~60 | cache.rs |
| 5 | Integration | ~20 | BuddyAllocator, MMU |
| 6 | Tests | ~100 | All above |

**Total:** ~530 lines of code

---

## Next Phase

→ **Phase 4: Integration & Testing** — Comprehensive testing, kernel integration, benchmarks
