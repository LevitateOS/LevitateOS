# Phase 3: Implementation - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113 (created), TEAM_114 (revised & implementing)

## 1. Implementation Overview (Revised by TEAM_114)
Implement PCI support using `virtio-drivers` built-in PCI module.
Use `virtio_drivers::device::gpu::VirtIOGpu` directly (custom driver archived).

## 2. Dependencies
- `virtio-drivers = "0.12"` (PCI included by default, no feature flag needed)
- `bitflags` (already present)

## 3. Steps (Revised by TEAM_114)

### Step 1: Add ECAM Constants (`levitate-hal/src/mmu.rs`)
```rust
/// PCI ECAM base physical address (QEMU virt Highmem)
pub const ECAM_PA: usize = 0x4010_0000_0000;
/// PCI ECAM virtual address (high half)
pub const ECAM_VA: usize = KERNEL_VIRT_START + ECAM_PA;
/// ECAM size (256MB for 256 buses)
pub const ECAM_SIZE: usize = 256 * 1024 * 1024;
```

### Step 2: Map ECAM in Boot (`kernel/src/main.rs`)
- Before GPU init, map ECAM region as Device memory
- Use existing `map_range` with `PageFlags::DEVICE_BLOCK`

### Step 3: Create PCI Subsystem (`kernel/src/pci.rs`)
```rust
use virtio_drivers::transport::pci::bus::{
    BarInfo, Cam, Command, DeviceFunction, MmioCam, PciRoot,
};
use virtio_drivers::transport::pci::{virtio_device_type, PciTransport};
use virtio_drivers::transport::{DeviceType, Transport};

/// Simple BAR allocator for 32-bit PCI memory
struct PciMemoryAllocator {
    next: u32,
    end: u32,
}

/// Initialize PCI and find VirtIO GPU
pub fn init_gpu() -> Option<PciTransport> {
    // 1. Create MmioCam for ECAM
    // 2. Create PciRoot
    // 3. Enumerate bus 0
    // 4. Find GPU, allocate BARs
    // 5. Return PciTransport
}
```

### Step 4: Update GPU Module (`kernel/src/gpu.rs`)
- Use `virtio_drivers::device::gpu::VirtIOGpu<HalImpl, PciTransport>`
- Remove stub code, integrate real driver

### Step 5: Update QEMU Flags
- `xtask/src/main.rs`: Change `virtio-gpu-device` to `virtio-gpu-pci`
- `run.sh`, `run-vnc.sh`: Same change

### Step 6: Cleanup (Rule 6)
- Remove any dead MMIO GPU code paths
- Update documentation
