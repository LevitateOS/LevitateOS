# Phase 3 / Step 2 / UoW 3: Stack Management

## Goal
Allocate and manage memory for task stacks.

## Parent Context
- [Step 2](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-2.md)

## Tasks
1. Define `KernelStack` struct.
2. Implement allocation of 16KB (4 pages) using `BuddyAllocator` via `levitate-hal`.
3. Implement `Drop` for `KernelStack` to free memory when task is destroyed.

## Expected Output
- Tasks have unique stacks that are safely reclaimed.
