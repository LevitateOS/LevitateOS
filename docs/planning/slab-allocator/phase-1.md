# Phase 1: Discovery - Slab Allocator

## Feature Summary
**Feature:** Slab Allocator for LevitateOS
**Problem:** The current Buddy Allocator is efficient for large page-aligned allocations but inefficient and prone to fragmentation for small, fixed-size kernel objects (e.g., `Task`, `FileHandle`, `Inode`).
**Goal:** Implement a Slab Allocator to provide fast, fragmentation-free allocation for specific object types by caching initialized objects.

## Success Criteria
- [ ] `SlabAllocator` struct implemented.
- [ ] Can create caches for specific object sizes/types.
- [ ] Benchmarks show faster allocation/deallocation compared to generic heap for hot objects.
- [ ] Integration with `GlobalAlloc` or specific kernel subsystems.

## Current State Analysis
- **Current Allocator:** `BuddyAllocator` (Page Granularity) + `LinkedHeap` (Global Heap).
- **Gap:** No dedicated mechanism for frequent fixed-size allocations. `LinkedHeap` generic allocator is slower and causes fragmentation.
- **Dependencies:** Relies on `BuddyAllocator` to provide backing pages (slabs).

## Codebase Reconnaissance
- **Modules:** `kernel/src/memory/`, `levitate-hal/src/allocator/`.
- **Inspiration:**
  - **Theseus**: `kernel/slabmalloc`, `kernel/slabmalloc_safe` (Primary Reference).
  - Redox: `src/allocator` (Seems minimal, likely generic linked list).
  - Tock: (None found yet).

## Constraints
- **Start:** Must use `BuddyAllocator` for backing memory.
- **Safety:** Must handle concurrency (SMP safe) or be per-CPU.
- **no_std:** Must adhere to core library limitations.

## Phase 1 Plan (Discovery)

### Step 1: Analyze Reference Implementations
- [ ] Search `.external-kernels` for slab/pool patterns.
- [ ] Identify how they handle:
  - Cache creation
  - Slab growth/shrinking
  - Partial vs Full slab tracking

### Step 2: Define Requirements
- [ ] List initial object types needing slabs (e.g., Process Control Blocks, Sockets).
- [ ] Interface design: `Slab::new(size)`, `Slab::alloc()`, `Slab::free()`.
