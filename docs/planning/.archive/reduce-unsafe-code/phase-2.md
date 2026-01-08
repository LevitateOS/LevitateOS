# Phase 2 — Root Cause Analysis

**TEAM_131** | Reduce Unsafe Code via Safe Abstractions

## Hypotheses List

The unsafe code exists because:

1. **H1: No volatile wrapper abstraction** — MMIO/DMA requires volatile access, currently done with raw `read_volatile`/`write_volatile`
2. **H2: No system register abstraction** — ARM system registers require inline asm
3. **H3: No safe slice-from-address abstraction** — Creating slices from physical addresses requires unsafe
4. **H4: No intrusive linked list library** — Buddy allocator uses raw NonNull manipulation

**Confidence:** HIGH for all — these are known missing abstractions.

## Key Code Areas

### Pattern 1: Volatile I/O (12+ occurrences)

**Files:**
- `levitate-virtio/src/queue.rs` — 11 volatile operations for DMA descriptors
- `kernel/src/main.rs` — DTB magic read
- `levitate-hal/src/gic.rs` — GIC register access

**Current Pattern:**
```rust
unsafe { core::ptr::write_volatile(&mut self.field, value) }
unsafe { core::ptr::read_volatile(&self.field) }
```

**Abstraction Opportunity:** `Volatile<T>` wrapper type

---

### Pattern 2: System Register Access (30+ occurrences)

**Files:**
- `levitate-hal/src/gic.rs` — ICC_* system registers
- `levitate-hal/src/timer.rs` — CNT* timer registers
- `levitate-hal/src/mmu.rs` — TTBR, SCTLR, TLB operations
- `levitate-hal/src/interrupts.rs` — DAIF manipulation
- `kernel/src/exceptions.rs` — ESR_EL1, ELR_EL1, VBAR_EL1

**Current Pattern:**
```rust
unsafe { core::arch::asm!("mrs {}, register_name", out(reg) val) }
unsafe { core::arch::asm!("msr register_name, {}", in(reg) val) }
```

**Abstraction Opportunity:** `SysReg` trait + register-specific types

---

### Pattern 3: Raw Slice Creation (7 occurrences)

**Files:**
- `kernel/src/main.rs` — DTB slice, initrd slice
- `kernel/src/memory/mod.rs` — mem_map slice
- `kernel/src/syscall.rs` — user buffer slices (4x)

**Current Pattern:**
```rust
unsafe { core::slice::from_raw_parts(ptr as *const u8, len) }
```

**Abstraction Opportunity:** Validated slice creation with bounds checking

---

### Pattern 4: Intrusive Linked Lists (4+ occurrences)

**Files:**
- `levitate-hal/src/allocator/buddy.rs` — Page free lists
- `levitate-hal/src/allocator/slab/list.rs` — Slab free lists

**Current Pattern:**
```rust
unsafe { page_ptr.as_mut() }
unsafe { next_ptr.as_mut().prev = ... }
```

**Abstraction Opportunity:** Safe intrusive list library

---

## Investigation Strategy

1. **Start with volatile wrapper** — Highest impact, well-understood pattern
2. **Then system registers** — Architecture-specific but high reuse
3. **Then intrusive lists** — Complex but isolated to allocators
4. **Last, slice creation** — Requires careful validation design

## Root Cause Summary

The unsafe code is not bugs — it's **missing safe abstractions** for hardware access patterns that are inherently unsafe at the primitive level but can be safely encapsulated.

**Solution:** Create zero-cost abstraction libraries that:
- Encapsulate the unsafe primitives
- Provide safe APIs with documented invariants
- Allow compile-time verification where possible
