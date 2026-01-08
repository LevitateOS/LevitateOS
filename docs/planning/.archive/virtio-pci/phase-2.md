# Phase 2: Design - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113 (created), TEAM_114 (revised)

## 1. Proposed Solution (Revised by TEAM_114)
Migrate GPU from MMIO to PCI transport using `virtio-drivers` APIs directly.

**Key Components:**
1. **ECAM Mapping:** Map PCI config space at `0x4010000000` as Device memory
2. **PCI Subsystem:** Use `virtio_drivers::transport::pci::bus` module:
   - `MmioCam` for ECAM access
   - `PciRoot` for bus enumeration
   - BAR allocation for device memory
3. **GPU Driver:** Use `virtio_drivers::device::gpu::VirtIOGpu<HalImpl, PciTransport>`

## 2. API Design (Revised by TEAM_114)

### New Module: `kernel/src/pci.rs`
```rust
use virtio_drivers::transport::pci::bus::{MmioCam, PciRoot, Cam, Command};
use virtio_drivers::transport::pci::PciTransport;

/// ECAM base address for QEMU virt machine (Highmem)
pub const ECAM_BASE_PA: usize = 0x4010_0000_0000;
pub const ECAM_SIZE: usize = 256 * 1024 * 1024; // 256MB for 256 buses

/// PCI memory region for BAR allocation (from DTB ranges)
pub const PCI_MEM32_BASE: u32 = 0x1000_0000;
pub const PCI_MEM32_SIZE: u32 = 0x2eff_0000;

/// Initialize PCI subsystem and find VirtIO GPU
pub fn init_gpu() -> Option<VirtIOGpu<HalImpl, PciTransport>>;
```

### Using virtio-drivers API correctly:
```rust
// 1. Create MmioCam for ECAM access
let cam = unsafe { MmioCam::new(ecam_va as *mut u8, Cam::Ecam) };

// 2. Create PciRoot for enumeration
let mut pci_root = PciRoot::new(cam);

// 3. Enumerate bus 0, find GPU
for (device_function, info) in pci_root.enumerate_bus(0) {
    if virtio_device_type(&info) == Some(DeviceType::GPU) {
        // 4. Allocate BARs
        allocate_bars(&mut pci_root, device_function, &mut allocator);
        
        // 5. Enable device
        pci_root.set_command(device_function, 
            Command::MEMORY_SPACE | Command::BUS_MASTER);
        
        // 6. Create PciTransport
        let transport = PciTransport::new::<HalImpl, _>(&mut pci_root, device_function)?;
        
        // 7. Create GPU driver
        return Some(VirtIOGpu::new(transport)?);
    }
}
```

## 3. Data Model Changes
- **ECAM Address:** `0x4010_0000_0000` (Highmem ECAM for QEMU virt)
- **PCI Memory:** `0x1000_0000` - `0x3eff_ffff` (32-bit MMIO for BARs)
- **MMU Mapping:**
  - Map ECAM region with `PageFlags::DEVICE`
  - BAR regions will be mapped dynamically

## 4. Behavioral Decisions
- **Polling Mode:** GPU uses polling, no IRQ setup needed initially
- **Error Handling:** Log and continue if GPU not found
- **BAR Allocation:** Simple bump allocator for 32-bit memory region

## 5. Resolved Questions (TEAM_114)
- **Q1 (BAR mapping):** Must allocate BARs ourselves using PciRoot methods
- **Q2 (ECAM base):** `0x4010_0000_0000` confirmed for Highmem

## 6. Implementation Steps (Revised)
1. Add ECAM constants to `levitate-hal/src/mmu.rs`
2. Map ECAM region in MMU initialization
3. Create `kernel/src/pci.rs` with proper virtio-drivers API usage
4. Update `kernel/src/gpu.rs` to use PCI GPU
5. Update QEMU flags in xtask and shell scripts
