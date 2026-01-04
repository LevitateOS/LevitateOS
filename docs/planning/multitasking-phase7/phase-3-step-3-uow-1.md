# Phase 3 / Step 3 / UoW 1: Assembly Switch Logic

## Goal
Implement the core register swapping in AArch64 assembly.

## Parent Context
- [Step 3](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-3.md)

## Tasks
1. Create `kernel/src/switch.s` or use `global_asm!`.
2. Implement `cpu_switch_to_inner`:
   - Store x19-x29, lr, sp on `prev` context.
   - Load x19-x29, lr, sp from `next` context.
   - Jump to `lr`.

## Expected Output
- Assembly logic correctly transitions from one register state to another.
