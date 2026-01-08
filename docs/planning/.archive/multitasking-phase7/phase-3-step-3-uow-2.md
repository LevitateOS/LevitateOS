# Phase 3 / Step 3 / UoW 2: Naked Rust Wrapper

## Goal
Expose the assembly switch to Rust code.

## Parent Context
- [Step 3](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-3.md)

## Tasks
1. In `kernel/src/task.rs`, implement `unsafe fn switch_to(prev: &mut Context, next: &mut Context)`.
2. Use `naked_asm!` to handle the transition safely (matching Redox pattern).

## Expected Output
- Rust code can trigger a context switch.
