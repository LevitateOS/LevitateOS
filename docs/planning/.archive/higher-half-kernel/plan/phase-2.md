# Phase 2 — Root Cause Analysis

**Goal:** Isolate where and why the bug occurs.

## Hypotheses List
- **H1: Instruction Cache Stale.** The instruction cache is serving stale/invalid data even after MMU enable. (Confidence: Medium)
- **H2: Execute Permission (PXN).** Implicit or explicit PXN bits are preventing instruction fetch. (Confidence: High)
- **H3: Alignment/Fetch Boundary.** High VA fetch is crossing a boundary or is misaligned in a way that causes "Undefined Instruction". (Confidence: Low)
- **H4: SCTLR_EL1 Bit Mismatch.** A bit like `WXN` (Write implies Never-Execute) is set, and the page is writable. (Confidence: High)

## Key Code Areas
- `levitate-hal/src/mmu.rs`: TCR and PageFlags definitions.
- `kernel/src/main.rs`: `global_asm!` block where MMU is enabled and jump happens.

## Investigation Strategy
1. **Map Execution Path:** Use GDB to trace exactly which instruction causes the ESR to trigger.
2. **Validate SCTLR:** Check SCTLR_EL1 and TCR_EL1 values at runtime.

---

## Steps

### [Step 1: Map Execution Path](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2-step-1.md)
Trace from MMU enable to the exception using GDB.

### [Step 2: Narrow Down Faulty Region](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2-step-2.md)
Identify the exact faulting address and instruction word in QEMU logs.

### [Step 3: Validate Hypotheses](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2-step-3.md)
Test H1-H4 individually, including MAIR/SCTLR attribute consistency.
