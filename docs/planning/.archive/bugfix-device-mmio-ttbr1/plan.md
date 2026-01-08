# Bugfix Plan: Device MMIO via TTBR1

**Team:** TEAM_076 (investigation), TEAM_077 (plan structuring)  
**Date:** 2026-01-04  
**Status:** READY FOR IMPLEMENTATION

---

## Phase Structure

| Phase | File | Status |
|-------|------|--------|
| 1 | `phase-1.md` | ✅ Complete (Understanding and Scoping) |
| 2 | `phase-2.md` | ✅ Complete (Root Cause Analysis) |
| 3 | `phase-3.md` | ✅ Complete (Fix Design and Validation Plan) |
| 4 | `phase-4.md` | 🔲 Ready (Implementation and Tests - 8 steps, 11 UoWs) |
| 5 | `phase-5.md` | 🔲 Ready (Cleanup, Regression Protection, and Handoff) |

**Next Action:** Implement Phase 4 steps.

---

## 1. Problem Statement

### Root Cause
Device MMIO (UART at `0x0900_0000`, VirtIO at `0x0A00_0000`, etc.) uses identity mapping via TTBR0.
When TTBR0 is switched to a user page table for userspace execution, device access causes a translation fault.

### Impact
- Kernel cannot print after switching to user page table
- Syscall handlers cannot access console
- Userspace execution is completely broken

### Evidence
```
[TASK] Before switch_ttbr0(0x48011000)
<hang>
```

The hang occurs because `println!` after `switch_ttbr0()` accesses unmapped UART address.

## 2. Solution: Map Devices via TTBR1

### Design
Change device mapping from identity (TTBR0) to high virtual address (TTBR1).

**New device VA layout:**
```
0xFFFF_8000_0000_0000  Kernel code/data (existing)
0xFFFF_8000_0900_0000  UART PL011 (NEW - remapped from 0x0900_0000)
0xFFFF_8000_0A00_0000  VirtIO MMIO base (NEW)
0xFFFF_8000_0800_0000  GIC (NEW)
```

### Benefits
- Devices accessible regardless of TTBR0 state
- Clean separation: TTBR0 = user, TTBR1 = kernel + devices
- Follows Redox/Theseus patterns

## 3. Implementation Steps

### Step 1: Define Device Virtual Addresses (1 UoW)
**File:** `levitate-hal/src/mmu.rs`

Add constants:
```rust
pub const DEVICE_VIRT_BASE: usize = KERNEL_VIRT_START;
pub const UART_VA: usize = DEVICE_VIRT_BASE + 0x0900_0000;
pub const VIRTIO_MMIO_VA: usize = DEVICE_VIRT_BASE + 0x0A00_0000;
pub const GIC_DIST_VA: usize = DEVICE_VIRT_BASE + 0x0800_0000;
pub const GIC_CPU_VA: usize = DEVICE_VIRT_BASE + 0x0801_0000;
```

### Step 2: Update phys_to_virt for Devices (1 UoW)
**File:** `levitate-hal/src/mmu.rs`

Change:
```rust
pub fn phys_to_virt(pa: usize) -> usize {
    // ALL physical addresses get high VA mapping
    pa + KERNEL_VIRT_START
}
```

Or keep conditional but map devices to high VA:
```rust
if pa < 0x4000_0000 {
    pa + KERNEL_VIRT_START  // Devices also use high VA
} else {
    pa + KERNEL_VIRT_START  // Kernel code/data
}
```

### Step 3: Map Device Regions in TTBR1 During Boot (2 UoW)
**File:** `levitate-hal/src/mmu.rs` - `reinit()` function

Add 2MB block mappings for device regions:
```rust
// Map UART region (0x0900_0000 -> high VA)
map_block(l1, UART_VA, 0x0900_0000, PageFlags::DEVICE_BLOCK)?;

// Map VirtIO region (0x0A00_0000 -> high VA)  
map_block(l1, VIRTIO_MMIO_VA, 0x0A00_0000, PageFlags::DEVICE_BLOCK)?;

// Map GIC region (0x0800_0000 -> high VA)
map_block(l1, GIC_DIST_VA, 0x0800_0000, PageFlags::DEVICE_BLOCK)?;
```

### Step 4: Update Console Driver (1 UoW)
**File:** `levitate-hal/src/console.rs`

Change UART address:
```rust
pub const UART0_BASE: usize = mmu::UART_VA;  // Was 0x0900_0000
```

### Step 5: Update VirtIO Drivers (2 UoW)
**Files:** 
- `levitate-hal/src/virtio/mod.rs`
- `levitate-hal/src/virtio/gpu.rs`
- `levitate-hal/src/virtio/blk.rs`
- etc.

Update all device base addresses to use high VA.

### Step 6: Update GIC Driver (1 UoW)
**File:** `levitate-hal/src/gic.rs`

Change GIC addresses:
```rust
pub const GICD_BASE: usize = mmu::GIC_DIST_VA;
pub const GICC_BASE: usize = mmu::GIC_CPU_VA;
```

### Step 7: Remove Identity Mapping from TTBR0 (1 UoW)
**File:** `levitate-hal/src/mmu.rs`

Remove the low-address identity mappings from boot page tables.
Only TTBR1 should have kernel/device mappings.

### Step 8: Test All Device Paths (2 UoW)
- Boot to console output
- VirtIO block read
- VirtIO network
- Timer interrupts
- Userspace execution

## 4. Verification Checklist

- [ ] Kernel boots and prints to console
- [ ] `cargo xtask test` passes
- [ ] Userspace process runs and prints "Hello from userspace!"
- [ ] VirtIO devices work (block, net, gpu)
- [ ] Interrupts work (timer, keyboard)

## 5. Risk Assessment

**Risk:** Breaking boot if device mappings are wrong
**Mitigation:** Keep old mappings until new ones verified, then remove

**Risk:** Missing a device region
**Mitigation:** Comprehensive device audit before implementation

## 6. Total Estimate

~11 UoW (Units of Work)

## 7. Handoff Notes

This plan is ready for implementation by any team. Key references:
- `@/home/vince/Projects/LevitateOS/.teams/TEAM_076_investigate_userspace_hang.md` - Investigation details
- `@/home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs` - Current MMU implementation
- `@/home/vince/Projects/LevitateOS/kernel/src/task/process.rs:75` - CONFIRMED breadcrumb
