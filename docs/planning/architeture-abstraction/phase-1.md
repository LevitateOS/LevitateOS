# Phase 1: Discovery and Safeguards - Arch Abstraction

## Refactor Summary
The kernel currently has architecture-specific code (AArch64) tightly coupled with generic kernel logic in `boot.rs`, `exceptions.rs`, `syscall.rs`, and `task/mod.rs`. This refactor will extract this logic into a dedicated `crate::arch` module to enable multi-architecture support.

## Success Criteria
- [ ] Kernel builds for `aarch64` with the new abstraction.
- [ ] Behavior tests pass (Boot, Syscalls, Task Switching).
- [ ] `aarch64` specific code is isolated in `src/arch/aarch64`.
- [ ] Generic kernel code has no direct dependencies on `aarch64-cpu` or AArch64 registers.

## Behavioral Contracts
- **Boot Entry:** Kernel expects to be loaded at physical address base + 0x80000.
- **Syscall ABI:** 
  - Syscall number in `x8`.
  - Arguments in `x0-x5`.
  - Return in `x0`.
- **Exception Table:** Vectors must be correctly set in `VBAR_EL1`.

## Golden/Regression Tests
- `tests/golden_boot.txt`: Must match exactly after refactor.
- `cargo xtask test behavior`: All tests must pass.

## Current Architecture Notes
- `boot.rs`: Contains `global_asm!` block with AArch64 boot sequence and page table setup.
- `exceptions.rs`: Contains AArch64 vector table and exception handlers.
- `task/mod.rs`: Defines AArch64 `Context` and context switching assembly.
- `syscall.rs`: Defines `SyscallFrame` with AArch64 register layout.

## Open Questions
- Should `los_hal` also be split by architecture for MMU/GIC logic? (Currently `los_hal` handles some abstraction but `mmu.rs` is AArch64-centric).

## Steps
1. **Step 1: Map Current Behavior and Boundaries**
   - Identify every file using `core::arch::global_asm!` or `aarch64-cpu`.
2. **Step 2: Lock in Golden Tests**
   - Verify `tests/golden_boot.txt` is current.
