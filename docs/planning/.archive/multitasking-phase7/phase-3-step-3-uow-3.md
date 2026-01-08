# Phase 3 / Step 3 / UoW 3: Switch Finish Hook

## Goal
Perform post-switch cleanup (locks, status updates).

## Parent Context
- [Step 3](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-3.md)

## Tasks
1. Implement `pub unsafe extern "C" fn switch_finish_hook()`.
2. This hook will release the `CONTEXT_SWITCH_LOCK`.
3. Ensure the assembly switch jumps to this hook or a caller that handles it.

## Expected Output
- System remains stable after multiple context switches.
