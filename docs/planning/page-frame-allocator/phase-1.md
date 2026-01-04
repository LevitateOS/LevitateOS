# Phase 1: Discovery — Page Frame Allocator Integration

**Status:** [x] Complete
**Owner:** TEAM_054
**Target:** Integrate Buddy Allocator with MMU for on-demand page table allocation

---

## 1. Feature Summary

### Problem Statement
The MMU needs to allocate new page tables dynamically at runtime when:
1. Mapping new virtual address ranges
2. Splitting block mappings into finer-grained 4KB pages
3. Supporting demand paging (future)

Currently, the infrastructure exists but lacks verification and behavioral tests.

### Who Benefits
- Kernel: Can map memory beyond the boot-time static page table pool
- Future features: Demand paging, CoW, user-space virtual memory

### Success Criteria
1. `cargo xtask test behavior` passes (kernel boots correctly)
2. Unit tests verify dynamic allocation behavior
3. Integration verified: MMU requests pages → Buddy Allocator provides them
4. All relevant behaviors added to `behavior-inventory.md`

---

## 2. Current State Analysis

### How It Works Today

The integration is already partially complete:

1. **`PageAllocator` trait** (`levitate-hal/src/mmu.rs:14-19`):
   ```rust
   pub trait PageAllocator: Send + Sync {
       fn alloc_page(&self) -> Option<usize>;
       fn free_page(&self, pa: usize);
   }
   ```

2. **`FrameAllocator` implements trait** (`kernel/src/memory/mod.rs:14-21`):
   ```rust
   impl PageAllocator for FrameAllocator {
       fn alloc_page(&self) -> Option<usize> {
           self.0.lock().alloc(0)  // Order 0 = 1 page = 4KB
       }
       fn free_page(&self, pa: usize) {
           self.0.lock().free(pa, 0)
       }
   }
   ```

3. **Registration** (`kernel/src/memory/mod.rs:164`):
   ```rust
   mmu::set_page_allocator(&FRAME_ALLOCATOR);
   ```

4. **Usage** (`levitate-hal/src/mmu.rs:513-523`):
   ```rust
   let new_table = if let Some(allocator) = unsafe { PAGE_ALLOCATOR_PTR } {
       allocator.alloc_page().map(|pa| {
           let va = phys_to_virt(pa);
           let pt = unsafe { &mut *(va as *mut PageTable) };
           pt.zero();
           pt
       })
   } else {
       alloc_page_table()  // Static pool fallback
   }
   ```

### What Workarounds Exist
- Static pool (`PT_POOL`) of 16 page tables for boot-time allocation
- Falls back to static pool if `PAGE_ALLOCATOR_PTR` is not set

### Current Gaps
1. **No unit tests** for dynamic allocation path
2. **No verification** that the path is actually taken at runtime
3. **No `unmap` functionality** — pages are never freed
4. **No behavior traceability** — missing from `behavior-inventory.md`

---

## 3. Codebase Reconnaissance

### Files Involved

| File | Role |
|------|------|
| `levitate-hal/src/mmu.rs` | `PageAllocator` trait, `set_page_allocator()`, `get_or_create_table()` |
| `levitate-hal/src/memory.rs` | HAL `FrameAllocator` wrapper (used by slab) |
| `kernel/src/memory/mod.rs` | Kernel `FrameAllocator`, calls `set_page_allocator()` |
| `levitate-hal/src/allocator/buddy.rs` | `BuddyAllocator::alloc(0)` returns 4KB frames |

### Public APIs

| API | Location | Purpose |
|-----|----------|---------|
| `PageAllocator` trait | mmu.rs:14 | Interface for page allocation |
| `set_page_allocator()` | mmu.rs:25 | Register allocator with MMU |
| `alloc_page()` / `free_page()` | trait methods | Allocate/free single 4KB page |
| `get_or_create_table()` | mmu.rs:498 | Internal — calls allocator |

### Tests and Snapshots That May Be Impacted

| Test | Location | Impact |
|------|----------|--------|
| `cargo xtask test behavior` | Integration | Must still pass |
| MMU unit tests | `mmu.rs` tests module | Need new dynamic allocation tests |
| Buddy unit tests | `buddy.rs` tests | Already comprehensive |

---

## 4. Constraints

### Performance
- Page table allocation happens during mapping operations
- Allocation should not block for long periods
- Buddy allocator uses a Spinlock (acceptable for kernel-only use)

### Compatibility
- Must not break existing boot sequence
- Static pool fallback must remain for early boot

### Memory
- Each page table consumes 4KB
- A full page table hierarchy can use ~2GB for 256TB address space
- In practice, kernel uses sparse mappings (few tables needed)

---

## 5. Next Steps

1. **Phase 2 — Design**: Define verification approach and any missing APIs
2. **Question**: Is `unmap_page()` in scope for this feature?
3. **Question**: Should we add logging/metrics for dynamic allocation?

---

## 6. References

- Buddy Allocator plan: `docs/planning/buddy-allocator/phase-3.md`
- MMU behaviors: `docs/testing/behavior-inventory.md` (Group 4)
- ARM Architecture Reference Manual: Page table format
