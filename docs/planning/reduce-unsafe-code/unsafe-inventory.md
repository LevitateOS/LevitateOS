# Unsafe Code Inventory

**TEAM_131** | Generated for review and refactoring decisions

**Total unsafe usages:** 171

---

## Summary by File

| File | Count | Primary Pattern |
|------|-------|-----------------|
| `levitate-hal/src/gic.rs` | 26 | asm!, volatile MMIO |
| `levitate-hal/src/mmu.rs` | 14 | asm!, page table ops |
| `kernel/src/main.rs` | 12 | init, ptr ops |
| `levitate-hal/src/allocator/slab/list.rs` | 9 | linked list |
| `levitate-hal/src/allocator/slab/page.rs` | 6 | slab allocator |
| `kernel/src/task/user_mm.rs` | 6 | user page tables |
| `levitate-virtio/src/queue.rs` | 9 | DMA volatile ops |
| `levitate-hal/src/timer.rs` | 6 | asm! timer regs |
| `kernel/src/loader/elf.rs` | 6 | ELF loading |
| `levitate-hal/src/allocator/buddy.rs` | 7 | buddy allocator |
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
| 288 | `BOOT_DTB_ADDR` static read | Global static | ⚠️ REVIEW | Could use AtomicUsize |
| 301 | `read_volatile(addr as *const u32)` | Volatile read | 🔄 REFACTOR | → `Volatile<u32>` |
| 368 | Heap init `ALLOCATOR.lock().init()` | Init | ✅ VALID | Boot-time only |
| 384 | `static mut ROOT_PT` access | Static mut | ⚠️ REVIEW | Consider OnceCell |
| 495 | `mmu::enable_mmu()` call | MMU | ✅ VALID | Inherently unsafe |
| 511 | `from_raw_parts(ptr, 1MB)` | Slice creation | 🔄 REFACTOR | Needs bounds validation |
| 534 | `task::set_current_task()` | Task setup | ✅ VALID | Boot-time init |
| 594 | `mmu::switch_ttbr0()` | MMU | ✅ VALID | Inherently unsafe |
| 616 | `from_raw_parts(initrd_va, size)` | Slice creation | 🔄 REFACTOR | Needs bounds validation |
| 726 | `interrupts::enable()` | Interrupts | ✅ VALID | Inherently unsafe |
| 736 | `interrupts::enable()` | Interrupts | ✅ VALID | Inherently unsafe |

---

## kernel/src/exceptions.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 251 | `asm!("mrs {}, esr_el1")` | Sysreg read | 🔄 REFACTOR | → `ESR_EL1::read()` |
| 263 | `asm!("mrs {}, elr_el1")` | Sysreg read | 🔄 REFACTOR | → `ELR_EL1::read()` |
| 288 | `asm!("wfi")` | Wait for interrupt | 🔄 REFACTOR | → `barrier::wfi()` |
| 340 | `asm!("msr vbar_el1, {}")` | Sysreg write | 🔄 REFACTOR | → `VBAR_EL1::write()` |

---

## kernel/src/syscall.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 217 | `from_raw_parts_mut(buf, max_read)` | User slice | ⚠️ REVIEW | Needs user ptr validation |
| 290 | `from_raw_parts(buf, len)` | User slice | ⚠️ REVIEW | Needs user ptr validation |
| 347 | `from_raw_parts(path_ptr, path_len)` | User slice | ⚠️ REVIEW | Needs user ptr validation |
| 402 | `from_raw_parts(path_ptr, path_len)` | User slice | ⚠️ REVIEW | Needs user ptr validation |

---

## kernel/src/memory/mod.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 54-56 | Linker symbol access | Extern static | ✅ VALID | Standard pattern |
| 153 | `from_raw_parts_mut(mem_map_va, pages)` | Slice creation | ⚠️ REVIEW | Validate alignment |
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

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 74 | `asm!("mrs {}, ICC_SRE_EL1")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 81 | `asm!("msr ICC_SRE_EL1, {}")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 88 | `asm!("mrs {}, ICC_IAR1_EL1")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 95 | `asm!("msr ICC_EOIR1_EL1, {}")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 101 | `asm!("msr ICC_PMR_EL1, {}")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 107 | `asm!("msr ICC_IGRPEN1_EL1, {}")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 113 | `asm!("isb")` | Barrier | 🔄 REFACTOR | → `barrier::isb()` |
| 193-607 | `read/write_volatile` | MMIO | 🔄 REFACTOR | → `Volatile<T>` wrapper |
| 229 | `unsafe impl Sync for Gic` | Trait impl | ⚠️ REVIEW | Verify thread safety |
| 311 | `ACTIVE_GIC_PTR.load()` deref | Global ptr | ⚠️ REVIEW | Could panic if null |
| 347-400 | `asm!("dmb sy")` | Barrier | 🔄 REFACTOR | → `barrier::dmb_sy()` |

---

## levitate-hal/src/timer.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 79 | `asm!("mrs {}, id_aa64mmfr1_el1")` | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |
| 97-141 | Timer register access | Sysreg | 🔄 REFACTOR | → `sysreg!` macro |

---

## levitate-hal/src/mmu.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 29 | `PAGE_ALLOCATOR_PTR` write | Global | ✅ VALID | Boot-time init |
| 383 | `asm!("tlbi vmalle1")` | TLB flush | 🔄 REFACTOR | → `tlb::flush_all()` |
| 397 | `asm!("tlbi vae1, {}")` | TLB flush | 🔄 REFACTOR | → `tlb::flush_page()` |
| 447-522 | MMU enable/disable/switch | Sysreg | ✅ VALID | Inherently unsafe |
| 556 | `set_page_allocator()` | Init | ✅ VALID | Boot-time init |
| 642-752 | Page table walks | Ptr cast | ⚠️ REVIEW | Could use typed wrapper |
| 1238-1254 | Test code | Test | ✅ VALID | Test-only |

---

## levitate-hal/src/interrupts.rs

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 9 | `asm!("mrs {}, daif")` | Sysreg | 🔄 REFACTOR | → `DAIF::read()` |
| 11 | `asm!("msr daifset, #2")` | Sysreg | 🔄 REFACTOR | → `DAIF::set_i()` |
| 20 | `asm!("msr daifclr, #2")` | Sysreg | 🔄 REFACTOR | → `DAIF::clear_i()` |
| 68 | `asm!("msr daif, {}")` | Sysreg | 🔄 REFACTOR | → `DAIF::write()` |

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

| Line | Code Pattern | Category | Decision | Notes |
|------|--------------|----------|----------|-------|
| 27-28 | `unsafe impl Send/Sync` | Trait impl | ✅ VALID | Protected by Spinlock |
| 79 | `page_ptr.as_mut()` | NonNull deref | 🔄 REFACTOR | → intrusive list |
| 152 | `ptr.add(index)` | Ptr arithmetic | ⚠️ REVIEW | Bounds checked above |
| 175-190 | Linked list ops | NonNull deref | 🔄 REFACTOR | → intrusive list |
| 209 | Test init | Test | ✅ VALID | Test-only |

---

## levitate-hal/src/allocator/slab/*.rs

| File | Line | Code Pattern | Category | Decision | Notes |
|------|------|--------------|----------|----------|-------|
| list.rs | 49,71,83,101 | `NonNull::as_mut()` | Linked list | 🔄 REFACTOR | → intrusive list |
| list.rs | 184-249 | Test code | Test | ✅ VALID | Test-only |
| cache.rs | 97-183 | Ptr casts | Slab ops | ⚠️ REVIEW | Complex pointer math |
| cache.rs | 235-236 | `unsafe impl Send/Sync` | Trait impl | ✅ VALID | Protected by lock |
| page.rs | 73-282 | Bitmap/ptr ops | Slab page | ⚠️ REVIEW | Complex, audit needed |
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
| 41-42 | `unsafe impl Send/Sync` | Trait impl | ⚠️ REVIEW | Verify thread safety |
| 91 | `from_raw_parts_mut(ptr, fb_size)` | Slice creation | 🔄 REFACTOR | Type-safe FB wrapper |

---

## unsafe impl Trait Summary

| File | Line | Trait | Decision | Notes |
|------|------|-------|----------|-------|
| buddy.rs | 27-28 | Send/Sync for BuddyAllocator | ✅ VALID | Spinlock protected |
| page.rs | 42-43 | Send/Sync for Page | ✅ VALID | Simple data struct |
| cache.rs | 235-236 | Send/Sync for SlabCache | ✅ VALID | Lock protected |
| memory.rs | 13-14 | Send/Sync for FrameAllocator | ✅ VALID | Lock protected |
| gic.rs | 229 | Sync for Gic | ⚠️ REVIEW | Verify correctness |
| hal_impl.rs | 18 | VirtioHal | ✅ VALID | Required by crate |
| virtio.rs | 28 | Hal | ✅ VALID | Required by crate |
| lib.rs | 41-42 | Send/Sync for Gpu | ⚠️ REVIEW | Verify correctness |

---

## Decision Summary

| Decision | Count | Action Required |
|----------|-------|-----------------|
| ✅ VALID | ~55 | None - properly justified |
| 🔄 REFACTOR | ~70 | Create safe abstractions |
| ⚠️ REVIEW | ~25 | Manual review needed |
| 🗑️ REMOVE | 0 | N/A |

---

## Refactoring Priority

### High Priority (reduces most unsafe)

1. **`sysreg!` macro** — ~30 blocks → 1
2. **`Volatile<T>` wrapper** — ~25 blocks → 2
3. **`barrier` module** — ~10 blocks → 1

### Medium Priority

4. **Intrusive list** — ~10 blocks → 2
5. **Page table typed wrapper** — ~8 blocks → 2

### Low Priority (needs design work)

6. **User pointer validation** — 4 blocks (security critical)
7. **MMIO region types** — architectural change
