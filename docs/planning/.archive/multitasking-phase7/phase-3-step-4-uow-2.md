# Phase 3 / Step 4 / UoW 2: Round-Robin Selection

## Goal
Implement the algorithm to pick the next runnable task.

## Parent Context
- [Step 4](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-4.md)

## Tasks
1. Implement `Scheduler::next_task()`.
2. Algorithm:
   - Pop front of `ReadyList`.
   - If runnable, return it.
   - If none runnable, return `idle_task`.

## Expected Output
- Scheduler correctly cycles through tasks.
