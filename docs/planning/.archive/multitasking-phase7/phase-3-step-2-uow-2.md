# Phase 3 / Step 2 / UoW 2: TCB and Context Structures

## Goal
Implement the main `Task` structure.

## Parent Context
- [Step 2](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-2.md)

## Tasks
1. Define `Context` struct with x19-x30, sp, elr, spsr (AArch64 callee-saved).
2. Define `Task` struct (TCB) containing `TaskId`, `TaskState`, `Context`, and reference to a stack.
3. Implement `Task::new()` to initialize a default kernel task.

## Expected Output
- TCB can be instantiated in memory.
