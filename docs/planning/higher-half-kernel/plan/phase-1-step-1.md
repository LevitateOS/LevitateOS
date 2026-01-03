# Phase 1 — Step 1: Consolidate Bug Information

**Parent Phase:** [Phase 1](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-1.md)

## Unit of Work: Consolidate Logs
This UoW is focused on gathering existing debug info into a single reference.

### Tasks
- [ ] Read `docs/planning/higher-half-kernel/INVESTIGATION_NOTES.md` again for exact ESR/ESR_EL1 values.
- [ ] Check `qemu_int.log` if it exists in the root.
- [ ] Document the exact failing instruction word if known.

### Expected Output
A detailed summary of the failure state (Registers, ESR, Faulting Address).
