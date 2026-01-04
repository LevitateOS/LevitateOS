# Phase 4: Implementation and Tests

**Team:** TEAM_077  
**Parent:** `plan.md`  
**Prerequisite:** `phase-3.md`  
**Status:** READY FOR IMPLEMENTATION

---

## Overview

This phase contains all implementation work broken into steps and UoWs.
Total: **8 steps, ~11 UoWs**

---

## Step 1: Define Device Virtual Addresses

**File:** `levitate-hal/src/mmu.rs`  
**UoW:** 1  
**Complexity:** Low

### Task
Add constants for device virtual addresses:

```rust
// Device virtual addresses (mapped via TTBR1)
pub const DEVICE_VIRT_BASE: usize = KERNEL_VIRT_START;
pub const UART_VA: usize = DEVICE_VIRT_BASE + 0x0900_0000;
pub const VIRTIO_MMIO_VA: usize = DEVICE_VIRT_BASE + 0x0A00_0000;
pub const GIC_DIST_VA: usize = DEVICE_VIRT_BASE + 0x0800_0000;
pub const GIC_CPU_VA: usize = DEVICE_VIRT_BASE + 0x0801_0000;
```

### Acceptance Criteria
- [ ] Constants defined in `mmu.rs`
- [ ] Constants are `pub` for use by drivers
- [ ] Build succeeds

---

## Step 2: Update phys_to_virt for Devices

**File:** `levitate-hal/src/mmu.rs`  
**UoW:** 1  
**Complexity:** Low

### Task
Change `phys_to_virt()` to map ALL physical addresses to high VA:

```rust
pub fn phys_to_virt(pa: usize) -> usize {
    pa + KERNEL_VIRT_START
}
```

### Acceptance Criteria
- [ ] `phys_to_virt()` always adds `KERNEL_VIRT_START`
- [ ] Build succeeds

---

## Step 3: Map Device Regions in TTBR1 During Boot

**File:** `levitate-hal/src/mmu.rs`  
**UoW:** 2 (mapping + testing)  
**Complexity:** Medium

### Task
In `reinit()`, add 2MB block mappings for device regions:

```rust
// Map UART region (0x0900_0000 -> high VA)
map_block(l1, UART_VA, 0x0900_0000, PageFlags::DEVICE_BLOCK)?;

// Map VirtIO region (0x0A00_0000 -> high VA)
map_block(l1, VIRTIO_MMIO_VA, 0x0A00_0000, PageFlags::DEVICE_BLOCK)?;

// Map GIC region (0x0800_0000 -> high VA)
map_block(l1, GIC_DIST_VA, 0x0800_0000, PageFlags::DEVICE_BLOCK)?;
```

### Notes
- Use `PageFlags::DEVICE_BLOCK` for device memory attributes
- Align to 2MB boundaries for block mappings
- Order: Map devices before jumping to high kernel

### Acceptance Criteria
- [ ] Device regions mapped in `reinit()`
- [ ] Correct memory attributes (device, non-cacheable)
- [ ] Build succeeds

---

## Step 4: Update Console Driver

**File:** `levitate-hal/src/console.rs`  
**UoW:** 1  
**Complexity:** Low

### Task
Change UART address to use high VA:

```rust
pub const UART0_BASE: usize = mmu::UART_VA;  // Was 0x0900_0000
```

### Notes
- Import `mmu` module if needed
- May need to update any other hardcoded UART addresses

### Acceptance Criteria
- [ ] `UART0_BASE` uses `mmu::UART_VA`
- [ ] Console output works during boot
- [ ] Build succeeds

---

## Step 5: Update VirtIO Drivers

**Files:**
- `levitate-hal/src/virtio/mod.rs`
- `levitate-hal/src/virtio/blk.rs`
- `levitate-hal/src/virtio/gpu.rs`
- `levitate-hal/src/virtio/net.rs`
- `levitate-hal/src/virtio/input.rs`

**UoW:** 2  
**Complexity:** Medium

### Task
Update all VirtIO device base addresses to use high VA via `phys_to_virt()`.

### Notes
- Check each driver for hardcoded MMIO addresses
- Use `mmu::phys_to_virt(VIRTIO_BASE_PA)` pattern
- May need to audit descriptor ring addresses

### Acceptance Criteria
- [ ] All VirtIO drivers use high VA for MMIO
- [ ] VirtIO block read works
- [ ] VirtIO GPU works (if tested)
- [ ] Build succeeds

---

## Step 6: Update GIC Driver

**File:** `levitate-hal/src/gic.rs`  
**UoW:** 1  
**Complexity:** Low

### Task
Change GIC addresses to use high VA:

```rust
pub const GICD_BASE: usize = mmu::GIC_DIST_VA;
pub const GICC_BASE: usize = mmu::GIC_CPU_VA;
```

### Acceptance Criteria
- [ ] GIC uses high VA addresses
- [ ] Interrupts work (timer, keyboard)
- [ ] Build succeeds

---

## Step 7: Remove Identity Mapping from TTBR0 (Optional)

**File:** `levitate-hal/src/mmu.rs`  
**UoW:** 1  
**Complexity:** Low

### Task
Remove low-address identity mappings from boot page tables.
Only TTBR1 should have kernel/device mappings.

### Notes
- This may be deferred if it causes boot issues
- Focus on making devices work via TTBR1 first

### Acceptance Criteria
- [ ] Identity mappings removed (or documented as deferred)
- [ ] Kernel boots successfully
- [ ] Build succeeds

---

## Step 8: Test All Device Paths

**UoW:** 2  
**Complexity:** Medium

### Tests to Run

| Test | Command | Expected |
|------|---------|----------|
| Golden boot | `cargo xtask test` | PASS |
| Userspace | `cargo xtask run` | "Hello from userspace!" |
| VirtIO block | Disk read during boot | Success |
| Timer | Preemption demo | Switches between tasks |
| Keyboard | `--features multitask-demo` | Responds to input |

### Acceptance Criteria
- [ ] `cargo xtask test` passes
- [ ] Userspace process runs and prints output
- [ ] VirtIO devices work
- [ ] Interrupts work (timer, keyboard)

---

## Summary

| Step | Description | UoWs | Files |
|------|-------------|------|-------|
| 1 | Define device VA constants | 1 | mmu.rs |
| 2 | Update phys_to_virt | 1 | mmu.rs |
| 3 | Map devices in TTBR1 | 2 | mmu.rs |
| 4 | Update console driver | 1 | console.rs |
| 5 | Update VirtIO drivers | 2 | virtio/*.rs |
| 6 | Update GIC driver | 1 | gic.rs |
| 7 | Remove identity mapping | 1 | mmu.rs |
| 8 | Test all paths | 2 | - |
| **Total** | | **11** | |

**Phase 4 Ready.** Proceed to implementation or Phase 5 planning.
