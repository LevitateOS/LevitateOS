# Phase 3 / Step 4 / UoW 3: Cooperative Yield

## Goal
Allow tasks to voluntarily give up the CPU.

## Parent Context
- [Step 4](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-4.md)

## Tasks
1. Implement `yield_now()`.
2. This function should:
   - Disable interrupts.
   - Pick next task.
   - Perform `switch_to`.
   - Enable interrupts (via `switch_finish_hook`).

## Expected Output
- Multiple kernel tasks can multiplex the CPU by calling `yield_now()`.
