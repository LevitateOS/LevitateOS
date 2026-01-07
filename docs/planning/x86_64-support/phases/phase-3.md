# Phase 3: Implementation — x86_64 Support (Intel NUC)

## Implementation Plan

This phase covers the initial implementation of the x86_64 architecture support, focusing on reaching a bootable state with serial output.

### Step 1: Toolchain and Build Support
- [ ] Add `x86_64-unknown-none` target to `rust-toolchain.toml`.
- [ ] Update `xtask` to support `--arch x86_64`.
- [ ] Add a `q35` QEMU profile to `xtask`.

### Step 2: Early Boot (Assembly)
- [ ] Implement Multiboot2 header in `kernel/src/arch/x86_64/boot.S`.
- [ ] Implement 32-bit to 64-bit transition (Long Mode).
- [ ] Set up early page tables (Identity mapping of first 1GB).
- [ ] Jump to `kernel_main`.

### Step 3: Architecture-Specific Stubs implementation
- [ ] Fill in `kernel/src/arch/x86_64/cpu.rs` with GDT and basic CPU initialization.
- [ ] Implement `kernel/src/arch/x86_64/exceptions.rs` with a basic IDT.
- [ ] Implement `kernel/src/arch/x86_64/task.rs` context switching logic.

### Step 4: HAL Implementation (x86_64)
- [ ] Implement `SerialConsole` (COM1) in `los_hal`.
- [ ] Implement `VgaConsole` (Text Mode) in `los_hal`.
- [ ] Implement `ApicController` for interrupts.
- [ ] Implement `PitTimer` or `ApicTimer`.

### Step 5: MMU & Higher-Half
- [ ] Implement PML4 page table walker and mapper.
- [ ] Transition from early identity mapping to full higher-half kernel.

## Progress Tracking
- [ ] Step 1: Toolchain
- [ ] Step 2: Early Boot
- [ ] Step 3: Arch Stubs
- [ ] Step 4: HAL Backends
- [ ] Step 5: MMU
