# Phase 1 — Step 2: Confirm or Improve Reproduction

**Parent Phase:** [Phase 1](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-1.md)

## Unit of Work: Minimal Reproduction Case
This UoW creates a bare-bones reproduction to remove Rust complexity.

### Tasks
- [ ] Create `kernel/src/repro_boot.s` (assembly only).
- [ ] Configure `linker.ld` for the repro.
- [ ] Create `scripts/test_repro.sh`.

### Expected Output
A script that reliably Reproduces the "Undefined Instruction" fault with minimal code.
