# Phase 2: Root Cause Analysis

**Team:** TEAM_077  
**Parent:** `plan.md`  
**Prerequisite:** `phase-1.md`  
**Status:** COMPLETE (inherited from TEAM_076 investigation)

---

## 1. Hypotheses List

| ID | Hypothesis | Confidence | Evidence | Result |
|----|------------|------------|----------|--------|
| H1 | Page table entry doesn't allow EL0 execution | HIGH | Check USER_CODE flags | ❌ Eliminated |
| H2 | Entry point mismatch - code at wrong VA | MEDIUM | Compare ELF entry vs mapped VA | ❌ Eliminated |
| H3 | TTBR0 switch incomplete - stale TLB | LOW | Check TLB flush | ❌ Eliminated |
| H4 | Silent page fault - no output from handler | HIGH | Check exception handler | ✅ **CONFIRMED** |
| H5 | User code stuck in loop before syscall | LOW | Check _start code | ❌ Eliminated |

---

## 2. Key Code Areas

### Primary (Root Cause Location)
- `levitate-hal/src/mmu.rs` - `phys_to_virt()` returns identity mapping for devices
- `levitate-hal/src/console.rs` - `UART0_BASE = 0x0900_0000` (identity mapped)

### Secondary (Affected by Fix)
- `levitate-hal/src/gic.rs` - GIC addresses
- `levitate-hal/src/virtio/*.rs` - VirtIO MMIO addresses

### Data Flow
```
switch_ttbr0(user_table)
    ↓
println!() called
    ↓
UART write to VA 0x0900_0000
    ↓
TTBR0 lookup (user table) → no mapping
    ↓
Translation Fault
    ↓
Exception handler tries to print → same fault
    ↓
Hang (recursive fault)
```

---

## 3. Investigation Strategy

### Test 1: Debug prints around switch_ttbr0
**Output:**
```
[TASK] Before switch_ttbr0(0x48011000)
<hang>
```
**Conclusion:** Hang occurs DURING `switch_ttbr0()`, not after ERET.

### Test 2: Analyze UART address
- UART PA: `0x0900_0000`
- `phys_to_virt(0x0900_0000)` returns `0x0900_0000` (identity)
- Identity mappings use TTBR0 (VA < 0x8000_0000_0000)
- **After switch_ttbr0(), UART is unmapped**

---

## 4. Root Cause (CONFIRMED)

### Summary
Device MMIO (UART at `0x0900_0000`) uses identity mapping via TTBR0.
When TTBR0 is switched to user page table, device access faults.

### Causal Chain
1. `switch_ttbr0(user_page_table)` is called
2. TTBR0 now points to user L0 table (no device mappings)
3. Next `println!` tries to write to UART at VA `0x0900_0000`
4. VA lookup via TTBR0 fails → Translation Fault
5. Exception handler tries to print → same fault → hang

### Invariant Violated
**Kernel devices must be accessible regardless of TTBR0 state.**

Devices should be mapped via TTBR1 (high VA), not TTBR0 (low VA).

---

## 5. Steps (Completed)

| Step | Description | Status |
|------|-------------|--------|
| 1 | Map execution path | ✅ Done by TEAM_076 |
| 2 | Narrow down faulty region | ✅ Done by TEAM_076 |
| 3 | Validate/eliminate hypotheses | ✅ Done by TEAM_076 |

**Phase 2 Complete.** Proceed to Phase 3.
