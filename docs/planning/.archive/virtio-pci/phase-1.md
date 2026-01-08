# Phase 1: Discovery - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113 (created), TEAM_114 (reviewed & revised)
**Goal:** Fix AArch64 GPU display cache coherency by switching from MMIO to PCI.

## 1. Feature Summary
The current `virtio-gpu` driver uses MMIO transport. On AArch64 QEMU `virt` machine, the MMIO regions for the Framebuffer are mapped by the Guest as Normal-Cacheable, but the Host (QEMU) does not snoop the caches for these regions properly when using MMIO transport, leading to a "Display output is not active" error or a black screen.
Switching to standard `virtio-pci` solves this because the PCI controller (ECAM) infrastructure in AArch64 ensures that BAR memory regions are treated with the correct attributes (Device/Uncached) or that the platform ensures coherency.

## 2. Success Criteria
- [ ] Kernel boots with `virtio-gpu-pci` device enabled in QEMU.
- [ ] Kernel enumerates PCI bus and finds the GPU.
- [ ] `virtio-drivers` initializes the GPU via PCI transport.
- [ ] `cargo xtask run-vnc` shows a visible terminal or test pattern (no "Display output is not active").

## 3. Current State Analysis (Updated by TEAM_114)
- **Transport:** `levitate-virtio` uses MMIO. `virtio-drivers` includes PCI support by default.
- **GPU Driver:** `levitate-drivers-gpu` archived. Using `virtio_drivers::device::gpu::VirtIOGpu` instead.
- **Boot Flow:** `kernel/src/virtio.rs` scans MMIO regions. No PCI scanning yet.
- **QEMU:** Currently `-device virtio-gpu-device` (MMIO). Need `-device virtio-gpu-pci`.

## 4. Codebase Reconnaissance (Updated by TEAM_114)
- **Dependencies:**
    - `kernel/Cargo.toml`: `virtio-drivers = "0.12"` (PCI included by default)
- **New Modules Needed:**
    - `kernel/src/pci.rs`: PCI subsystem using `virtio_drivers::transport::pci`
- **Modifications Needed:**
    - `kernel/src/gpu.rs`: Integrate `VirtIOGpu<HalImpl, PciTransport>`
    - `levitate-hal/src/mmu.rs`: Add ECAM mapping constants
    - `xtask/src/main.rs`: Change QEMU flags to use PCI
    - `run.sh`, `run-vnc.sh`: Update device flags

## 5. Constraints
- **Address Space:** AArch64 `virt` machine places ECAM at `0x4010000000` (Highmem PCIE). We must ensure our MMU maps this region if it's not already identity mapped, or use the lower alias if available.
- **Interrupts:** PCI uses INTx/MSI. We might need to support Legacy INTx mapping to GICv3 interrupts if MSI is too complex for now. The DTB will tell us the IRQ mapping.
