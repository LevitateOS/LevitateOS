# Phase 3 / Step 1: Virtual Memory Reclamation

This step implements the ability to safely remove memory mappings, which is critical for task cleanup and swapping.

## Objective
Implement `unmap_page()` in `levitate-hal/src/mmu.rs`.

## Units of Work

- [ ] [UoW 1: MMU Refactoring](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1-uow-1.md)
- [ ] [UoW 2: Basic Unmap Implementation](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1-uow-2.md)
- [ ] [UoW 3: Table Reclamation](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1-uow-3.md)
