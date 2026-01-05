# Phase 1: Discovery - VirtIO PCI Migration

**Feature:** VirtIO PCI Transport for GPU
**Team:** TEAM_113
**Goal:** Fix AArch64 GPU display cache coherency by switching from MMIO to PCI.

## 1. Feature Summary
The current `virtio-gpu` driver uses MMIO transport. On AArch64 QEMU `virt` machine, the MMIO regions for the Framebuffer are mapped by the Guest as Normal-Cacheable, but the Host (QEMU) does not snoop the caches for these regions properly when using MMIO transport, leading to a "Display output is not active" error or a black screen.
Switching to standard `virtio-pci` solves this because the PCI controller (ECAM) infrastructure in AArch64 ensures that BAR memory regions are treated with the correct attributes (Device/Uncached) or that the platform ensures coherency.

## 2. Success Criteria
- [ ] Kernel boots with `virtio-gpu-pci` device enabled in QEMU.
- [ ] Kernel enumerates PCI bus and finds the GPU.
- [ ] `virtio-drivers` initializes the GPU via PCI transport.
- [ ] `cargo xtask run-vnc` shows a visible terminal or test pattern (no "Display output is not active").

## 3. Current State Analysis
- **Transport:** Mixed. We have `levitate-virtio` using MMIO. `virtio-drivers` crate has a `pci` feature (currently disabled/unused).
- **GPU Driver:** `levitate-drivers-gpu` is transport-agnostic (uses `GpuDriver` logic) but `device.rs` is hardcoded for `MmioTransport`.
- **Boot Flow:** `kernel/src/virtio.rs` scans MMIO regions from Device Tree (FDT). There is no PCI scanning.
- **QEMU:** We run with `-device virtio-gpu-device` (MMIO). We need `-device virtio-gpu-pci`.

## 4. Codebase Reconnaissance
- **Dependencies:**
    - `kernel/Cargo.toml`: Need to enable `pci` feature for `virtio-drivers`.
- **New Modules Needed:**
    - `kernel/src/pci.rs`: To scan ECAM (Enhanced Configuration Access Mechanism).
- **Modifications Needed:**
    - `levitate-drivers-gpu/src/device.rs`: Refactor `VirtioGpu` struct to be generic over Transport, or switch it to PCI.
    - `levitate-virtio/src/lib.rs`: Expose PCI transport types.
    - `xtask/src/main.rs`: Change QEMU flags to use PCI.

## 5. Constraints
- **Address Space:** AArch64 `virt` machine places ECAM at `0x4010000000` (Highmem PCIE). We must ensure our MMU maps this region if it's not already identity mapped, or use the lower alias if available.
- **Interrupts:** PCI uses INTx/MSI. We might need to support Legacy INTx mapping to GICv3 interrupts if MSI is too complex for now. The DTB will tell us the IRQ mapping.
