# Phase 3 / Step 4 / UoW 1: Scheduler State

## Goal
Implement the `Scheduler` data structure.

## Parent Context
- [Step 4](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-4.md)

## Tasks
1. Create `kernel/src/scheduler.rs`.
2. Define `Scheduler` struct.
3. Use a thread-safe queue (e.g., `VecDeque` protected by an `IrqSafeLock`) for the `ReadyList`.
    - This ensures `handle_irq` can safely access the scheduler without deadlocks (Rule 7).
4. Initialize the global scheduler in `kmain`.

## Expected Output
- Global scheduler is accessible throughout the kernel.
