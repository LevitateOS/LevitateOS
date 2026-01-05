# Phase 2: Design - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113

## 1. Proposed Solution
Migrate the GPU driver from `MmioTransport` to `PciTransport`.
This involves three main components:
1.  **Memory Mapping:** Map the PCI ECAM (Enhanced Configuration Access Mechanism) region in the kernel's address space. memory attributes: `Device-nGnRE` (Uncached).
2.  **PCI Enumeration:** Implement a simple scanner to walk the PCI bus (Bus 0, Devices 0-31, Functions 0-7) via ECAM, looking for the VirtIO GPU device (Vendor 0x1AF4, Device 0x1050).
3.  **Driver Initialization:** Instantiate `VirtioGpu` using the `PciTransport` provided by `virtio-drivers`.

## 2. API Design
### New Module: `kernel/src/pci.rs`
```rust
pub struct PciConfigRegion {
    base: usize, // Virtual address of ECAM base
}

impl PciConfigRegion {
    pub fn new(base: usize) -> Self;
    pub fn read(&self, bus: u8, dev: u8, func: u8, offset: u16) -> u32;
    pub fn write(&self, bus: u8, dev: u8, func: u8, offset: u16, value: u32);
}

// Simple scanner
pub fn find_device(vendor: u16, device: u16) -> Option<PciTransport>;
```

### Refactored `levitate-drivers-gpu`
We need to change `VirtioGpu` to accept a generic Transport or specifically `PciTransport`.
Given we might want to keep MMIO valid (e.g. for non-AArch64?), making it generic is best.
`pub struct VirtioGpu<H: VirtioHal, T: Transport>`

## 3. Data Model Changes
- **ECAM Address:** For QEMU `virt` machine (AArch64), Highmem ECAM is at `0x4010000000`. Size is 256MB (covering Bus 0-255).
- **MMU Mapping:**
  - Need to add `ECAM_VA` constant.
  - Map `0x4010000000` -> `ECAM_VA` with `PageFlags::DEVICE` attributes (Uncached).

## 4. Behavioral Decisions
- **Error Handling:** If PCI device not found, fail gracefully (log warning) but don't panic.
- **Interrupts:** PCI Modern interrupts can be MSI-X or INTx. `PciTransport` handles the configuration, but we need to supply the mapped BARs. The interrupt number (INTx) comes from the configuration space. We'll need to route that to the GIC.
    - *Simplification:* Run in polling mode initially? `VirtioGpu` currently polls `has_used()` in `send_command`, so interrupts are not strictly required for the *synchronous* flush loop we use. We can defer IRQ setup.

## 5. Open Questions
- **Q1:** Does `virtio-drivers` PciTransport handle BAR mapping?
    - *Answer:* No, `PciTransport` usually expects you to provide the header and handle BAR mapping yourself, or it reads BARs and expects you to provide a function to map them.
    - *Refinement:* I need to check `virtio_drivers::transport::pci::PciTransport` signature. It typically takes a `PciRoot` structure.

- **Q2:** What is the exact ECAM base for `virt`?
    - *Answer:* `0x40_1000_0000` (Highmem) or `0x3f00_0000` (Lowmem). QEMU `virt` usually exposes Highmem ECAM. I will assume Highmem.

## 6. Implementation Steps
1.  **Refactor GPU Driver:** Make `VirtioGpu` generic over `T: Transport`.
2.  **Enable Feature:** Add `pci` to `virtio-drivers` keys.
3.  **Implement PCI Scan:** Add `kernel/src/pci.rs`.
4.  **Update Boot:** In `kernel/src/main.rs`, init PCI instead of MMIO scan.
