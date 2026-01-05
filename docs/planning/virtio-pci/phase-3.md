# Phase 3: Implementation - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113

## 1. Implementation Overview
We will implement PCI support in layers, starting from the kernel's ability to access ECAM, then the scanning logic, and finally the driver integration.

## 2. Dependencies
- `virtio-drivers` with `pci` feature enabled.
- `bitflags` (already present).

## 3. Steps

### Step 1: Enable PCI Feature
- Update `kernel/Cargo.toml` dependencies:
  ```toml
  virtio-drivers = { version = "0.12", features = ["pci"] }
  ```

### Step 2: Implement PCI Subsystem (`kernel/src/pci.rs`)
- Define `ECAM_BASE` constant: `0x40_1000_0000`.
- Create `PciConfigRegion` struct.
- Implement `virtio_drivers::transport::pci::PciConfiguration` trait for it.
  - This requires implementing `read/write` for 8/16/32/64 bits to config space.
  - Formula: `addr = base + ((bus << 20) | (device << 15) | (func << 12) | offset)`.
- Implement `find_device(vendor_id, device_id) -> Option<PciTransport>`.
  - Iterate Bus 0-0 (Virt machine puts everything on Bus 0 usually, but can check 0-255).
  - Iterate Device 0-31.
  - Iterate Function 0-7.
  - Read Vendor/Device ID.
  - If match `0x1AF4` / `0x1050`, Create `PciTransport::new(...)`.

### Step 3: Map ECAM in MMU (`levitate-hal/src/mmu.rs`)
- Add `ECAM_VA` constant (somewhere in kernel high half).
- In `init()`, call `map_block_2mb` or `map_page` to map the 256MB ECAM region as `PageFlags::DEVICE` (Uncached).
- **Critical:** This ensures the CPU access to config space is correct (Device memory).

### Step 4: Refactor GPU Driver (`levitate-drivers-gpu`)
- Modify `VirtioGpu` to take `T: Transport`.
- Update `device.rs`:
  - `pub struct VirtioGpu<H: VirtioHal, T: Transport> { transport: T, ... }`
  - Update `new()` to accept `T`.

### Step 5: Wire Up (`kernel/src/main.rs`)
- In `kmain`, initializing GPU:
  - Initialize PCI subsystem (map ECAM).
  - Call `pci::find_device(0x1AF4, 0x1050)`.
  - If found, create `VirtioGpu::new(transport)`.
  - Initialize as before.
