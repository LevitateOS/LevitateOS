# Phase 3 / Step 5 / UoW 3: Preemption Verification

## Goal
Verify that a task in an infinite loop does not starve other tasks.

## Parent Context
- [Step 5](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-5.md)

## Tasks
1. Create two tasks that print different characters in an infinite loop (`while true {}`).
2. Run them and observe the interleaved output.

## Expected Output
- Interleaved output on the console (e.g., `ABABABAB...`).
