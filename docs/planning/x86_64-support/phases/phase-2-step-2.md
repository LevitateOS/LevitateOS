# Phase 2 — Step 2: HAL Trait Extraction

## Parent
Phase 2: Design — x86_64 Support

## Goal
Extract architecture-independent traits for MMU and Interrupts to decouple the kernel from AArch64 hardware specifics.

## Tasks
1. **Define Traits in `los_hal`**:
    - Create `crates/hal/src/traits.rs`.
    - Define `MmuInterface` trait as sketched in `phase-2.md`.
    - Define `InterruptController` trait as sketched in `phase-2.md`.
2. **Refactor `crates/hal/src/mmu.rs`**:
    - Implement `MmuInterface` for the existing AArch64 MMU logic.
    - Move AArch64-specific constants and flags into an `arch` submodule or keep them conditionally compiled but separate from the trait definition.
3. **Refactor `crates/hal/src/gic/mod.rs`**:
    - Implement `InterruptController` for the GIC.
4. **Update Kernel usage**:
    - Update `kernel/src/init.rs` and `kernel/src/input.rs` to use the generic traits instead of direct `gic` or `mmu` calls where possible.

## Expected Outputs
- `los_hal` provides a clean trait interface for core hardware services.
- Kernel code is less dependent on `aarch64-cpu` and GIC-specific structures.
