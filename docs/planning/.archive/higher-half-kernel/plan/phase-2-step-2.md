# Phase 2 — Step 2: Narrow Down Faulty Region

**Parent Phase:** [Phase 2](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2.md)

## Unit of Work: QEMU Log Analysis
Extract and analyze detailed fault information from QEMU logs.

### Tasks
- [ ] Run reproduction with `-d int,mmu,guest_errors -D qemu_debug.log`.
- [ ] Inspect `qemu_debug.log` for the translation walk of the high VA.
- [ ] Verify if the "Undefined Instruction" is triggered by a failed fetch or a specific opcode.

### Expected Output
Exact physical address and page table entry used for the failing fetch.
