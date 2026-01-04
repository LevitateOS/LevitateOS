# Phase 3 / Step 5 / UoW 2: Interrupt-Driven Scheduling

## Goal
Force a task switch when the timer fires.

## Parent Context
- [Step 5](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-5.md)

## Tasks
1. In `handle_irq` (or timer-specific handler), call `schedule()`.
2. Ensure the exception context is saved such that `switch_to` can resume it later.
    - Specifically, the `Context::sp` should point to the exception frame on the stack created by `vector.s`.

## Expected Output
- The CPU switches tasks automatically without explicit `yield_now()` calls.
