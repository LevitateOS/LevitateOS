# Phase 3: Fix Design and Validation Plan

**Team:** TEAM_077  
**Parent:** `plan.md`  
**Prerequisite:** `phase-2.md`  
**Status:** COMPLETE

---

## 1. Root Cause Summary

**What is wrong:** Device MMIO uses identity mapping via TTBR0.

**Where it lives:** 
- `levitate-hal/src/mmu.rs` - `phys_to_virt()` returns identity for device addresses
- Device drivers use identity-mapped addresses directly

**Why it's wrong:** When TTBR0 is switched for userspace, devices become inaccessible.

---

## 2. Fix Strategy

### Chosen Approach: Map Devices via TTBR1

Change device mapping from identity (TTBR0) to high virtual address (TTBR1).

**New Device VA Layout:**
```
0xFFFF_8000_0000_0000  Kernel code/data (existing)
0xFFFF_8000_0800_0000  GIC (NEW - remapped from 0x0800_0000)
0xFFFF_8000_0900_0000  UART PL011 (NEW - remapped from 0x0900_0000)
0xFFFF_8000_0A00_0000  VirtIO MMIO base (NEW - remapped from 0x0A00_0000)
```

**Formula:** `device_va = KERNEL_VIRT_START + device_pa`

### Benefits
- Devices accessible regardless of TTBR0 state
- Clean separation: TTBR0 = user, TTBR1 = kernel + devices
- Follows Redox/Theseus patterns
- Security: devices not accessible from userspace

### Tradeoffs
- Requires updating all device drivers
- More complex boot mapping

---

## 3. Alternative Options Considered

| Option | Description | Verdict |
|--------|-------------|---------|
| A: TTBR1 mapping | Map devices via high VA | ✅ **CHOSEN** |
| B: User page table | Add devices to every user table | ❌ Security risk |
| C: Remove prints | Don't print after switch | ❌ Doesn't fix syscalls |

---

## 4. Reversal Strategy

**Signals to revert:**
- Kernel fails to boot
- Device access broken after change
- Golden boot test fails unexpectedly

**Reversal steps:**
1. Revert device driver address changes
2. Revert `phys_to_virt()` changes
3. Revert TTBR1 device mappings in `reinit()`
4. Confirm golden boot test passes

---

## 5. Test Strategy

### Regression Tests
| Test | Description | Status |
|------|-------------|--------|
| Golden boot | `cargo xtask test` | Must pass after fix |
| Console output | Kernel prints after switch_ttbr0 | NEW - add |
| VirtIO block | Read from disk | Existing |
| Timer interrupts | Preemption works | Existing |

### New Tests to Add
1. **Userspace boot test** - Verify user process runs and prints output
2. Update golden output to include userspace output

### Edge Cases
- Device access during exception handling
- Device access from interrupt context
- Multiple TTBR0 switches

---

## 6. Impact Analysis

### API Changes
| Symbol | Old | New |
|--------|-----|-----|
| `UART0_BASE` | `0x0900_0000` | `mmu::UART_VA` |
| `GICD_BASE` | `0x0800_0000` | `mmu::GIC_DIST_VA` |
| `GICC_BASE` | `0x0801_0000` | `mmu::GIC_CPU_VA` |
| VirtIO bases | Identity | `mmu::phys_to_virt()` |

### Downstream Impact
- All device drivers must use new VA constants
- `phys_to_virt()` semantics change for device addresses

### Performance
- No performance impact (same number of memory accesses)
- Slightly larger TTBR1 page tables (device mappings added)

---

## 7. Steps

| Step | Description | Status |
|------|-------------|--------|
| 1 | Define fix requirements | ✅ |
| 2 | Propose fix options | ✅ |
| 3 | Choose fix and define tests | ✅ |

**Phase 3 Complete.** Proceed to Phase 4.
