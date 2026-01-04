# Phase 3 / Step 1 / UoW 3: Table Reclamation

## Goal
Automatically free intermediate page tables when they become empty after an unmap.

## Parent Context
- [Step 1](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1.md)

## Tasks
1. Enhance `unmap_page` to check if the current table is empty (all entries zero).
2. If empty and not the root, free the page via `PAGE_ALLOCATOR_PTR` and recurse to parent.
3. This may require `walk_to_entry` to return breadcrumbs or path info.

## Expected Output
- No memory leaks of page tables during repeated map/unmap of wide ranges.
