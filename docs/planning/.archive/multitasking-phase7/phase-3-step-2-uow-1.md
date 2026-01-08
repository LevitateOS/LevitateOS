# Phase 3 / Step 2 / UoW 1: Basic Task Types

## Goal
Define the foundational enums for task management.

## Parent Context
- [Step 2](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-2.md)

## Tasks
1. Create `kernel/src/task.rs`.
2. Define `TaskId` (Newtype over `u64`).
3. Define `TaskState` enum: `Ready`, `Running`, `Blocked`, `Exited`.
4. Register the module in `kernel/src/main.rs`.

## Expected Output
- `kernel` compiles with new types available.
