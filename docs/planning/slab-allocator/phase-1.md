# Phase 1: Discovery - Slab Allocator

**Team:** TEAM_050
**Status:** Complete
**Date:** 2026-01-04

---

## Feature Summary

**Feature:** Slab Allocator for LevitateOS

**Problem:** The current Buddy Allocator is efficient for large page-aligned allocations but inefficient and prone to fragmentation for small, fixed-size kernel objects (e.g., `Task`, `FileHandle`, `Inode`).

**Goal:** Implement a Slab Allocator to provide fast, fragmentation-free allocation for specific object types by caching pre-sized memory blocks.

**Who Benefits:**
- Kernel subsystems allocating many small, same-sized objects (scheduler, VFS, networking)
- Future userspace with frequent syscall-related allocations

---

## Success Criteria

- [ ] `SlabAllocator` (or `ZoneAllocator`) struct implemented
- [ ] Support for multiple size classes (8B to ~8KB)
- [ ] O(1) allocation/deallocation for common cases
- [ ] Integration with `BuddyAllocator` for backing page acquisition
- [ ] Thread-safe (Spinlock or per-CPU design)
- [ ] Unit tests covering allocation, deallocation, slab growth, and edge cases

---

## Current State Analysis

### Existing Allocators

| Allocator | Location | Granularity | Use Case |
|-----------|----------|-------------|----------|
| `BuddyAllocator` | `levitate-hal/src/allocator/buddy.rs` | 4KB pages (order 0-20) | Physical frame allocation |
| `linked_list_allocator` | External crate | Arbitrary | Global heap (`#[global_allocator]`) |

### Gap Analysis

- **No size-class allocator:** All small allocations go through generic heap, causing:
  - Fragmentation over time
  - O(n) free-list search on allocation
  - Cache-unfriendly memory layout
- **BuddyAllocator is page-only:** Cannot efficiently allocate 64-byte `Task` structs

### Integration Point

```
BuddyAllocator (4KB+ pages)
        ↓
   SlabAllocator (fixed-size objects within pages)
        ↓
   Kernel Subsystems (Task, Inode, Socket, etc.)
```

---

## Codebase Reconnaissance

### LevitateOS Memory Subsystem

| File | Purpose |
|------|---------|
| `kernel/src/memory/mod.rs` | `FrameAllocator` wrapper, DTB-based init |
| `levitate-hal/src/allocator/buddy.rs` | Buddy allocator with coalescing |
| `levitate-hal/src/allocator/page.rs` | `Page` descriptor with flags, intrusive list |

**Key APIs:**
- `FRAME_ALLOCATOR.0.lock().alloc(order)` → `Option<usize>` (physical address)
- `FRAME_ALLOCATOR.0.lock().free(pa, order)`

### Theseus slabmalloc (Primary Reference)

**Location:** `.external-kernels/theseus/kernel/slabmalloc/`

**Architecture:**

```
ZoneAllocator
├── small_slabs[0]: SCAllocator<8B>
├── small_slabs[1]: SCAllocator<16B>
├── ...
└── small_slabs[10]: SCAllocator<~8KB>

SCAllocator<Size>
├── empty_slabs: PageList    (no allocations)
├── slabs: PageList          (partial)
└── full_slabs: PageList     (exhausted)

ObjectPage8k (8KB)
├── data[0..SIZE-METADATA]   (object storage)
├── bitfield[8 × u64]        (allocation tracking)
├── prev/next pointers       (intrusive list)
└── metadata (MappedPages, heap_id)
```

**Key Design Decisions:**

1. **Size Classes:** Power-of-two: 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, ~8000
2. **Page Size:** 8KB (two 4KB frames) for better object density
3. **Bitfield Tracking:** 512 bits (8 × u64) track up to 512 objects per page
4. **Three-List Design:** Enables O(1) allocation from partial slabs
5. **Metadata Location:** At END of page (preserves alignment of data area)

**Allocation Flow:**
1. `ZoneAllocator::allocate(layout)` → select `SCAllocator` by size
2. `SCAllocator::allocate()` → try partial slabs, else pop from empty
3. `ObjectPage8k::allocate()` → bitfield first-fit, mark bit
4. If page becomes full → move to `full_slabs`

**Deallocation Flow:**
1. Compute page from pointer (mask lower bits)
2. Clear bit in bitfield
3. If page was full → move to `slabs`
4. If page now empty → move to `empty_slabs`

---

## Constraints

| Constraint | Requirement |
|------------|-------------|
| `no_std` | Core library only, no `std::collections` |
| Concurrency | Must be SMP-safe (Spinlock or per-CPU) |
| Backing Memory | Must use `BuddyAllocator` for page acquisition |
| AArch64 | 4KB base page size, 64-byte cache lines |
| Determinism | Prefer O(1) operations |

---

## Phase 1 Checklist (Discovery)

### Step 1: Analyze Reference Implementations ✅

- [x] Read Theseus `slabmalloc` architecture
- [x] Understand `ZoneAllocator` → `SCAllocator` → `ObjectPage` hierarchy
- [x] Document size classes and bitfield tracking
- [x] Understand three-list (empty/partial/full) design

### Step 2: Analyze Current LevitateOS Memory System ✅

- [x] Read `BuddyAllocator` API and capabilities
- [x] Read `Page` descriptor structure
- [x] Identify integration point (`FRAME_ALLOCATOR`)

### Step 3: Document Requirements ✅

- [x] Define success criteria
- [x] Identify constraints
- [x] List initial object types (deferred to Phase 2)

---

## Design Decisions (Pixel 6 Optimized)

Based on **Google Tensor GS101** hardware constraints for maximum stability:

| Question | Decision | Rationale |
|----------|----------|-----------|
| **Page Size** | **4KB** | Matches ARM64 standard, BuddyAllocator order-0, simpler TLB behavior. 8KB would require order-1 allocations and complicate deallocation. |
| **Size Classes** | **Power-of-two (64B minimum)** | 64-byte cache line on Cortex-X1/A76/A55. Smaller classes waste cache. Classes: 64, 128, 256, 512, 1024, 2048. |
| **Concurrency** | **Single global Spinlock** | Simpler correctness, fewer race conditions. Per-CPU slabs are optimization for later phases. Matches existing `FRAME_ALLOCATOR` pattern. |
| **GlobalAlloc** | **Layer on top** | Don't replace `linked_list_allocator`. Use slab for kernel objects only via explicit `SlabCache<T>` API. Keeps fallback path working. |
| **Slab Shrinking** | **Disabled initially** | Keep empty slabs cached. Reduces allocation churn and avoids BuddyAllocator pressure. Add shrinking in future optimization phase. |
| **Object Constructors** | **Size-based (SLUB-style)** | No pre-initialization. Simpler, fewer invariants. Classic slab constructors add complexity without clear benefit for kernel objects. |

### Pixel 6 Hardware Constraints

```
Google Tensor GS101 (via QEMU emulation):
┌────────────────────────────────────────────────────┐
│ CPU: 2× Cortex-X1 + 2× Cortex-A76 + 4× Cortex-A55 │
│ Cache Line: 64 bytes                               │
│ Page Size: 4KB (standard ARM64)                    │
│ RAM: 8GB LPDDR5                                    │
│ GIC: v3                                            │
└────────────────────────────────────────────────────┘

Implications:
- Minimum object size: 64B (avoid false sharing)
- Page alignment: 4KB (no 8KB pages in hardware)
- Atomic ops: ARMv8.2+ LSE atomics available
- SMP: 8 cores, need lock contention awareness
```

### Recommended Size Classes (6 classes)

| Class | Size | Objects per 4KB page | Use Case |
|-------|------|---------------------|----------|
| 0 | 64B | 63 | Small structs, list nodes |
| 1 | 128B | 31 | Task metadata |
| 2 | 256B | 15 | File handles |
| 3 | 512B | 7 | Inodes |
| 4 | 1024B | 3 | Network buffers |
| 5 | 2048B | 1 | Large objects |

*Note: 4KB page - 64B metadata = 4032B usable. Classes >2KB only fit 1 object.*

### Stability Principles

1. **Simplicity over performance** — Single lock, no per-CPU, no NUMA awareness
2. **Fail-safe defaults** — Keep empty slabs, don't shrink under pressure
3. **Explicit API** — `SlabCache<T>::alloc()` not hidden behind GlobalAlloc
4. **Match existing patterns** — Follow `FRAME_ALLOCATOR` Spinlock design

---

## Next Steps

→ **Phase 2: Design** — Define `SlabCache`, `Slab`, and `SlabAllocator` structs with behavioral contracts
