# Phase 3 — Fix Design and Validation Plan

**Goal:** Decide *how* to fix the bug and *how* to prove it stays fixed.

## Root Cause Summary
(To be populated after Phase 2)

## Fix Strategy
(To be populated after Phase 2)

## Reversal Strategy
- If fix involves SCTLR configuration, reversal entails restoring original bits.
- If fix involves page table flags, reversal entails reverting `mmu.rs` changes.

## Test Strategy
- **Regression Test:** Add a new test in `scripts/test_behavior.sh` that specifically checks for high-VA execution.
- **Bootloader Verification:** Ensure "MMU enabled (higher-half)" is printed correctly.

---

## Steps

### [Step 1: Define Fix Requirements](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-3-step-1.md)
List invariants that must be maintained.

### [Step 2: Propose Fix Options](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-3-step-2.md)
Draft approaches based on root cause.

### [Step 3: Choose Fix and Test Strategy](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-3-step-3.md)
Finalize implementation plan for Phase 4.
