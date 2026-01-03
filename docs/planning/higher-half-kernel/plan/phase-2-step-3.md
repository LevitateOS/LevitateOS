# Phase 2 — Step 3: Validate Hypotheses

**Parent Phase:** [Phase 2](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-2.md)

## Unit of Work: Hypothesis Testing
Test H1-H4 by modifying configuration and observing effects.

### Tasks
- [ ] **H1:** Add explicit `ic ialluis` and `tlbi vmalle1is` with `dsb ish` / `isb` sequences.
- [ ] **H2/H4:** Temporarily set all RAM pages to `RWX` (no PXN/UXN, AP_RW_EL1) and disable `WXN` in SCTLR_EL1.
- [ ] **H3:** Check if the jump target is 4-byte or 8-byte aligned as required.

### Expected Output
Results for each hypothesis (Confirmed/Refuted).
