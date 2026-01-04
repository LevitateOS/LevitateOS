# Phase 3 / Step 1 / UoW 2: Basic Unmap Implementation

## Goal
Implement `unmap_page` using the refactored walking logic.

## Parent Context
- [Step 1](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1.md)

## Tasks
1. Implement `pub fn unmap_page(root: &mut PageTable, va: usize) -> Result<(), &'static str>` in `mmu.rs`.
2. The function should:
   - Walk to L3 entry with `create = false`.
   - If missing, return `Err` (Rule 14).
   - Clear the entry.
   - Call `tlb_flush_page(va)`.
3. Add a unit test in `mmu.rs` verifying a map -> unmap cycle.

## Expected Output
- Unit tests pass. Virtual address is inaccessible after unmap.
