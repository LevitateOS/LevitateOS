# Phase 3 / Step 5 / UoW 1: Timer Configuration

## Goal
Set up the AArch64 Generic Timer to fire at 100Hz (every 10ms).

## Parent Context
- [Step 5](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-5.md)

## Tasks
1. Update `levitate-hal/src/timer.rs` to support periodic interrupts.
2. Configure `CNTP_TVAL_EL0` for the desired interval.

## Expected Output
- Timer interrupts fire regularly.
