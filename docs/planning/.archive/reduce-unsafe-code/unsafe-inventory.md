# Unsafe Code Inventory

**TEAM_131** | Generated for review and refactoring decisions

**Last Updated:** TEAM_135 (2026-01-06)

**Total unsafe usages:** ~130 (down from 171 baseline)

---

## Progress Tracking

| Team | Work Done | Unsafe Δ |
|------|-----------|----------|
| TEAM_131 | Initial inventory | Baseline: 171 |
| TEAM_132 | Barriers + DAIF migration to aarch64-cpu | -15 |
| TEAM_133 | ESR/ELR/VBAR migration | -3 |
| TEAM_135 | Buddy allocator → IntrusiveList | -3 |
| TEAM_135 | Slab allocator → IntrusiveList, removed SlabList | -9 |

### Available Abstractions

| Pattern | Crate | Status |
|---------|-------|--------|
| Memory barriers (dsb, isb, dmb) | `aarch64-cpu` | ✅ In use |
| Standard sysregs (DAIF, ESR, ELR, VBAR, SCTLR) | `aarch64-cpu` | ✅ In use |
| Timer registers (CNT*) | `aarch64-cpu` | ✅ Fully migrated (TEAM_132) |
| GICv3 sysregs (ICC_*) | N/A | ❌ Not in aarch64-cpu — must use raw asm |
| TLB instructions (tlbi) | N/A | ❌ Not in aarch64-cpu — must use raw asm |
| Volatile MMIO | `safe-mmio` | ⏳ Available, not migrated (see notes below) |
| Intrusive linked lists | `IntrusiveList` (internal) | ✅ In use (buddy, slab) |

---

## TEAM_135: Volatile MMIO Migration Notes

### Why `safe-mmio`?

The `safe-mmio` crate (Google-maintained) is preferred over raw `read_volatile`/`write_volatile` because:

1. **Avoids creating references to MMIO space** — Creating `&T` to MMIO can cause UB
2. **Works around aarch64 virtualization bug** — Uses inline asm internally
3. **Distinguishes read types** — Pure reads vs side-effect reads
4. **Already in Cargo.lock** — No new dependency needed

### Migration Scope

| File | Volatile Count | Priority | Notes |
|------|----------------|----------|-------|
| `levitate-virtio/src/queue.rs` | 8 | HIGH | DMA descriptor access |
| `levitate-virtio/src/transport.rs` | 3 | HIGH | MMIO transport |
| `levitate-hal/src/gic.rs` | ~15 | MEDIUM | GIC MMIO registers |
| `levitate-hal/src/uart_pl011.rs` | 4 | LOW | UART registers |

### Migration Pattern

```rust
// Before
unsafe { core::ptr::write_volatile(&mut self.avail_idx, new_idx) }

// After (with safe-mmio)
use safe_mmio::{UniqueMmioPointer, fields::ReadWrite};
self.avail_idx.write(new_idx)  // No unsafe!
```

### Gotchas

1. **Requires struct redesign** — MMIO structs need `#[repr(C)]` with `safe-mmio` field types
2. **Pointer provenance** — Must create `UniqueMmioPointer` from raw address carefully
3. **Not a drop-in replacement** — Requires rethinking how MMIO regions are accessed

### Recommendation

Start with `levitate-virtio` (smaller scope, cleaner boundaries) before tackling GIC.

---

## Summary by File

| File | Count | Primary Pattern |
|------|-------|-----------------|
| `levitate-hal/src/gic.rs` | 26 | asm!, volatile MMIO |
| `levitate-hal/src/mmu.rs` | 14 | asm!, page table ops |
| `kernel/src/main.rs` | 12 | init, ptr ops |
| `levitate-hal/src/allocator/slab/list.rs` | 0 | DELETED - TEAM_135 migrated to IntrusiveList |
| `levitate-hal/src/allocator/slab/page.rs` | 6 | slab allocator |
| `kernel/src/task/user_mm.rs` | 6 | user page tables |
| `levitate-virtio/src/queue.rs` | 9 | DMA volatile ops |
| `levitate-hal/src/timer.rs` | 0 | ✅ DONE - TEAM_132 migrated to aarch64-cpu |
| `kernel/src/loader/elf.rs` | 6 | ELF loading |
| `levitate-hal/src/allocator/buddy.rs` | 3 | buddy allocator (TEAM_135: reduced via IntrusiveList) |
| `levitate-hal/src/allocator/slab/cache.rs` | 5 | slab cache |
| `kernel/src/syscall.rs` | 4 | user slice creation |
| `levitate-hal/src/uart_pl011.rs` | 4 | UART MMIO |
| `levitate-hal/src/interrupts.rs` | 4 | asm! DAIF |
| `levitate-virtio/src/transport.rs` | 3 | volatile MMIO |
| Other files | ~20 | misc |

---

## Detailed Inventory

### Legend

| Decision | Meaning |
|----------|---------|
| ✅ VALID | Unsafe is necessary and properly documented |
| 🔄 REFACTOR | Can be wrapped in safe abstraction |
| ⚠️ REVIEW | Needs closer review for correctness |
| 🗑️ REMOVE | Can be eliminated entirely |

---

## kernel/src/main.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 288 | `BOOT_DTB_ADDR` static read | Global static | ✅ VALID | TEAM_135: Set once at boot, read-only after |
| 301 | `read_volatile(addr as *const u32)` | Volatile read | 🔄 REFACTOR | → `Volatile<u32>` |
| 368 | Heap init `ALLOCATOR.lock().init()` | Init | ✅ VALID | Boot-time only |
| 384 | `static mut ROOT_PT` access | Static mut | ✅ VALID | TEAM_135: Boot-time only, single-threaded context |
| 495 | `mmu::enable_mmu()` call | MMU | ✅ VALID | Inherently unsafe |
| 511 | `from_raw_parts(ptr, 1MB)` | Slice creation | 🔄 REFACTOR | Needs bounds validation |
| 534 | `task::set_current_task()` | Task setup | ✅ VALID | Boot-time init |
| 594 | `mmu::switch_ttbr0()` | MMU | ✅ VALID | Inherently unsafe |
| 616 | `from_raw_parts(initrd_va, size)` | Slice creation | 🔄 REFACTOR | Needs bounds validation |
| 726 | `interrupts::enable()` | Interrupts | ✅ VALID | Inherently unsafe |
| 736 | `interrupts::enable()` | Interrupts | ✅ VALID | Inherently unsafe |

---

## kernel/src/exceptions.rs

**Status:** ✅ MOSTLY DONE (TEAM_132, TEAM_133)

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 251 | `asm!("mrs {}, esr_el1")` | Sysreg read | ✅ DONE | TEAM_133: Migrated to `ESR_EL1::get()` |
| 263 | `asm!("mrs {}, elr_el1")` | Sysreg read | ✅ DONE | TEAM_133: Migrated to `ELR_EL1::get()` |
| 288 | `asm!("wfi")` | Wait for interrupt | ✅ DONE | TEAM_132: Migrated to `aarch64_cpu::asm::wfi()` |
| 340 | `asm!("msr vbar_el1, {}")` | Sysreg write | ✅ DONE | TEAM_133: Migrated to `VBAR_EL1::set()` |

---

## kernel/src/syscall.rs

**Status:** 🚨 SECURITY CRITICAL — See `docs/planning/user-pointer-validation/`

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 217 | `from_raw_parts_mut(buf, max_read)` | User slice | 🚨 SECURITY | TEAM_135: Needs page table walk validation |
| 290 | `from_raw_parts(buf, len)` | User slice | 🚨 SECURITY | TEAM_135: Needs page table walk validation |
| 347 | `from_raw_parts(path_ptr, path_len)` | User slice | 🚨 SECURITY | TEAM_135: Needs page table walk validation |
| 402 | `from_raw_parts(path_ptr, path_len)` | User slice | 🚨 SECURITY | TEAM_135: Needs page table walk validation |

**Vulnerability:** Current validation only checks address range, not:
- Whether memory is actually mapped
- Whether user has read/write permission
- Whether entire buffer range is valid

**Design doc:** `docs/planning/user-pointer-validation/phase-1.md`

---

## kernel/src/memory/mod.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 54-56 | Linker symbol access | Extern static | ✅ VALID | Standard pattern |
| 153 | `from_raw_parts_mut(mem_map_va, pages)` | Slice creation | ✅ VALID | TEAM_135: Boot-time only, VA from kernel mapping |
| 162 | `allocator.init()` | Init | ✅ VALID | Boot-time only |
| 215 | `allocator.add_range()` | Init | ✅ VALID | Boot-time only |

---

## kernel/src/task/*.rs

| File | Line | Code Pattern | Category | Decision | Notes |
|------|------|--------------|----------|----------|-------|
| mod.rs | 77 | `asm!("wfi")` | Wait | 🔄 REFACTOR | → `barrier::wfi()` |
| mod.rs | 112 | `CURRENT_TASK.store()` | Global state | ✅ VALID | Task switch |
| mod.rs | 268 | Context switch | Asm | ✅ VALID | Inherently unsafe |
| user.rs | 45 | `asm!("eret")` | User return | ✅ VALID | Inherently unsafe |
| user_mm.rs | 41,74,105 | Page table cast | Ptr cast | 🔄 REFACTOR | Type-safe wrapper |
| user_mm.rs | 149-199 | Page mapping | MMU ops | ✅ VALID | Inherently unsafe |
| process.rs | 52 | Stack setup | Ptr ops | ✅ VALID | Process creation |

---

## kernel/src/loader/elf.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 275 | `&*(ptr as *const Phdr)` | Ptr cast | ⚠️ REVIEW | Alignment check needed |
| 325 | Page table cast | Ptr cast | 🔄 REFACTOR | Type-safe wrapper |
| 343 | `core::ptr::write()` | Write | ✅ VALID | ELF segment copy |
| 349 | `core::ptr::write_bytes()` | Memset | ✅ VALID | BSS zero |
| 380 | Page table cast | Ptr cast | 🔄 REFACTOR | Type-safe wrapper |
| 395 | Stack mapping | MMU ops | ✅ VALID | Inherently unsafe |

---

## levitate-hal/src/gic.rs

**Status:** ⚠️ PARTIAL — ICC_* registers NOT in aarch64-cpu, barriers done

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 74 | `asm!("mrs {}, ICC_SRE_EL1")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 81 | `asm!("msr ICC_SRE_EL1, {}")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 88 | `asm!("mrs {}, ICC_IAR1_EL1")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 95 | `asm!("msr ICC_EOIR1_EL1, {}")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 101 | `asm!("msr ICC_PMR_EL1, {}")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 107 | `asm!("msr ICC_IGRPEN1_EL1, {}")` | Sysreg | ✅ VALID | ICC_* not in aarch64-cpu — must use raw asm |
| 113 | `asm!("isb")` | Barrier | ✅ DONE | TEAM_132: Migrated to `barrier::isb(SY)` |
| 193-607 | `read/write_volatile` | MMIO | 🔄 REFACTOR | → `safe-mmio` (see notes above) |
| 229 | `unsafe impl Sync for Gic` | Trait impl | ✅ VALID | Protected by global lock |
| 311 | `ACTIVE_GIC_PTR.load()` deref | Global ptr | ✅ VALID | Set once at boot |
| 347-400 | `asm!("dmb sy")` | Barrier | ✅ DONE | TEAM_132: Now uses aarch64-cpu barriers |

---

## levitate-hal/src/timer.rs

**Status:** ✅ FULLY MIGRATED (TEAM_132)

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 79 | `asm!("mrs {}, id_aa64mmfr1_el1")` | Sysreg | ✅ DONE | TEAM_132: Now uses `ID_AA64MMFR1_EL1.get()` |
| 97-141 | Timer register access | Sysreg | ✅ DONE | TEAM_132: All CNT* registers via aarch64-cpu |

---

## levitate-hal/src/mmu.rs

**Status:** ⚠️ PARTIAL — Barriers migrated, TLBI/sysregs not in aarch64-cpu

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 29 | `PAGE_ALLOCATOR_PTR` write | Global | ✅ VALID | Boot-time init |
| 383 | `asm!("tlbi vmalle1")` | TLB flush | ✅ VALID | TLBI not in aarch64-cpu — must use raw asm |
| 397 | `asm!("tlbi vae1, {}")` | TLB flush | ✅ VALID | TLBI not in aarch64-cpu — must use raw asm |
| 447-522 | MMU enable/disable/switch | Sysreg | ✅ VALID | Inherently unsafe (sequence-critical) |
| 556 | `set_page_allocator()` | Init | ✅ VALID | Boot-time init |
| 642-752 | Page table walks | Ptr cast | ✅ VALID | Inherently unsafe (page table manipulation) |
| 1238-1254 | Test code | Test | ✅ VALID | Test-only |
| (dsb/isb) | Barriers | Barrier | ✅ DONE | TEAM_132: Migrated to aarch64-cpu |

---

## levitate-hal/src/interrupts.rs

**Status:** ✅ MOSTLY DONE (TEAM_132) — 2 remaining are special instructions

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 9 | `asm!("mrs {}, daif")` | Sysreg | ✅ DONE | TEAM_132: Now uses `DAIF.get()` |
| 11 | `asm!("msr daifset, #2")` | Sysreg | ✅ VALID | Immediate-only instruction, not in aarch64-cpu |
| 20 | `asm!("msr daifclr, #2")` | Sysreg | ✅ VALID | Immediate-only instruction, not in aarch64-cpu |
| 68 | `asm!("msr daif, {}")` | Sysreg | ✅ DONE | TEAM_132: Now uses `DAIF.set()` |

---

## levitate-hal/src/uart_pl011.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 63 | `read_volatile(&self.0)` | MMIO | 🔄 REFACTOR | → `Volatile<u32>` |
| 66 | `write_volatile(&mut self.0, val)` | MMIO | 🔄 REFACTOR | → `Volatile<u32>` |
| 99 | `&*(base as *const Registers)` | Ptr cast | 🔄 REFACTOR | Type-safe MMIO region |
| 103 | `&mut *(base as *mut Registers)` | Ptr cast | 🔄 REFACTOR | Type-safe MMIO region |

---

## levitate-hal/src/allocator/buddy.rs

**Status:** ✅ MIGRATED (TEAM_135) — Now uses IntrusiveList

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 27-28 | `unsafe impl Send/Sync` | Trait impl | ✅ VALID | Protected by Spinlock |
| 79 | `page_ptr.as_mut()` | NonNull deref | ✅ DONE | TEAM_135: Migrated to IntrusiveList |
| 152 | `ptr.add(index)` | Ptr arithmetic | ✅ VALID | Bounds checked, required for mem_map access |
| 175-190 | Linked list ops | NonNull deref | ✅ DONE | TEAM_135: Migrated to IntrusiveList |
| 209 | Test init | Test | ✅ VALID | Test-only |

---

## levitate-hal/src/allocator/slab/*.rs

**Status:** ✅ MIGRATED (TEAM_135) — Now uses shared IntrusiveList

| File | Line | Code Pattern | Category | Decision | Notes |
|------|------|--------------|----------|----------|-------|
| list.rs | N/A | DELETED | N/A | ✅ DONE | TEAM_135: File removed, using shared IntrusiveList |
| cache.rs | 97-183 | Ptr casts | Slab ops | ✅ VALID | Required for slab page access |
| cache.rs | 235-236 | `unsafe impl Send/Sync` | Trait impl | ✅ VALID | Protected by lock |
| page.rs | 73-282 | Bitmap/ptr ops | Slab page | ✅ VALID | Required for bitmap operations |
| mod.rs | 72 | Cache access | Global | ✅ VALID | Protected by lock |

---

## levitate-virtio/src/queue.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 182 | `(*desc_ptr).next` | Ptr deref | 🔄 REFACTOR | → safe accessor |
| 186-228 | `write_volatile` | DMA | 🔄 REFACTOR | → `Volatile<T>` |
| 244 | `write_volatile(avail_ring)` | DMA | 🔄 REFACTOR | → `Volatile<T>` |
| 254 | `write_volatile(avail_idx)` | DMA | 🔄 REFACTOR | → `Volatile<T>` |
| 263 | `asm!("dsb sy")` | Barrier | 🔄 REFACTOR | → `barrier::dsb_sy()` |
| 278 | `read_volatile(used_idx)` | DMA | 🔄 REFACTOR | → `Volatile<T>` |
| 296 | `read_volatile(used_ring)` | DMA | 🔄 REFACTOR | → `Volatile<T>` |

---

## levitate-virtio/src/transport.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 181 | `read_volatile(base + offset)` | MMIO | 🔄 REFACTOR | → `Volatile<T>` |
| 187 | `write_volatile(base + offset)` | MMIO | 🔄 REFACTOR | → `Volatile<T>` |
| 264 | `read_volatile(config + offset)` | MMIO | 🔄 REFACTOR | → `Volatile<T>` |

---

## levitate-virtio/src/hal_impl.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 18 | `unsafe impl VirtioHal` | Trait impl | ✅ VALID | Required by trait |
| 21 | `alloc_zeroed(layout)` | Alloc | ✅ VALID | DMA alloc |
| 33 | `dealloc(vaddr, layout)` | Dealloc | ✅ VALID | DMA dealloc |

---

## levitate-hal/src/virtio.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 28 | `unsafe impl Hal` | Trait impl | ✅ VALID | Required by trait |
| 37 | `alloc_zeroed(layout)` | Alloc | ✅ VALID | DMA alloc |
| 51 | `dealloc(vaddr, layout)` | Dealloc | ✅ VALID | DMA dealloc |

---

## levitate-gpu/src/lib.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 41-42 | `unsafe impl Send/Sync` | Trait impl | ✅ VALID | TEAM_135: GPU is behind Spinlock in kernel, single-writer |
| 91 | `from_raw_parts_mut(ptr, fb_size)` | Slice creation | 🔄 REFACTOR | Type-safe FB wrapper |

---

## unsafe impl Trait Summary

| File | Line | Trait | Decision | Notes |
|------|------|-------|----------|-------|
| buddy.rs | 27-28 | Send/Sync for BuddyAllocator | ✅ VALID | Spinlock protected |
| page.rs | 42-43 | Send/Sync for Page | ✅ VALID | Simple data struct |
| cache.rs | 235-236 | Send/Sync for SlabCache | ✅ VALID | Lock protected |
| memory.rs | 13-14 | Send/Sync for FrameAllocator | ✅ VALID | Lock protected |
| gic.rs | 229 | Sync for Gic | ✅ VALID | TEAM_135: GIC accessed via global lock pattern |
| hal_impl.rs | 18 | VirtioHal | ✅ VALID | Required by crate |
| virtio.rs | 28 | Hal | ✅ VALID | Required by crate |
| lib.rs | 41-42 | Send/Sync for Gpu | ✅ VALID | TEAM_135: GPU behind Spinlock |

---

## Decision Summary

**Updated by TEAM_135 (2026-01-06)**

| Decision | Count | Action Required |
|----------|-------|------------------|
| ✅ VALID/DONE | ~95 | None - properly justified or already migrated |
| 🔄 REFACTOR | ~30 | Volatile MMIO → safe-mmio (see notes above) |
| 🚨 SECURITY | 4 | User pointer validation (CRITICAL - see design doc) |
| 🗑️ REMOVE | 0 | N/A |

---

## Refactoring Priority

### 🚨 CRITICAL (Security)

1. **User pointer validation** — 4 blocks — See `docs/planning/user-pointer-validation/`

### High Priority (reduces most unsafe)

1. **`sysreg!` macro** — ✅ DONE (TEAM_132/133 migrated to aarch64-cpu)
2. **`Volatile<T>` wrapper** — ~25 blocks → `safe-mmio` (see MMIO notes)
3. **`barrier` module** — ✅ DONE (TEAM_132 migrated to aarch64-cpu)

### Medium Priority

4. **Intrusive list** — ✅ DONE (TEAM_135 migrated buddy/slab)
5. **Page table typed wrapper** — ~8 blocks → Future work

### Low Priority (needs design work)

6. **MMIO region types** — architectural change (depends on safe-mmio migration)
