# Phase 3 / Step 1 / UoW 1: MMU Refactoring

## Goal
Extract the page table walking logic from `map_page` into a reusable helper function.

## Parent Context
- [Phase 3](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3.md)
- [Step 1](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1.md)

## Tasks
1. Refactor `levitate-hal/src/mmu.rs`.
2. Implement `fn walk_to_entry(root: &mut PageTable, va: usize, create: bool) -> Result<WalkResult, &'static str>`.
   - `WalkResult` should contain the leaf entry AND breadcrumbs (parent pointers/indices) for reclamation.
3. Update `map_page` and `map_block_2mb` to use this new helper.

## Expected Output
- `mmu.rs` compiles and behavior tests still pass (no regressions in mapping).
