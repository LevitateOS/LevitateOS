# Phase 2 — Step 1: Map Execution Path

**Parent Phase:** [Phase 2](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2.md)

## Unit of Work: GDB Execution Trace
Use GDB to trace the execution from the point of MMU enable up to the Undefined Instruction exception.

### Tasks
- [ ] Launch QEMU with `-s -S`.
- [ ] Connect GDB and set breakpoint at `mmu::enable_mmu` or the `msr sctlr_el1` instruction.
- [ ] Single-step through the `br x0` instruction.
- [ ] Capture register state (`info reg`) and ESR_EL1 at the moment of failure.

### Expected Output
Confirmed execution flow and register values at the point of failure.
