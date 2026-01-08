# Phase 1 — Understanding and Scoping

**Goal:** Make the bug understandable and bounded before touching code.

## Bug Summary
LevitateOS fails to execute code at high virtual addresses (0x0000FFFF80000000+). While data reads from these addresses succeed (e.g., `ldr` from high VA works), jumping to the address (`br x0`) causes an "Undefined Instruction" exception (ESR EC=0).

## Reproduction Status
- **Reproducible:** Yes.
- **Reproduction Steps:**
    1. Update `linker.ld` to map kernel code to high VA.
    2. Set up page tables with identity mapping for boot code and higher-half mapping for kernel code.
    3. Enable MMU.
    4. Jump to the high virtual address of `kmain`.
- **Expected Behavior:** Kernel reaches `kmain` and continues execution.
- **Actual Behavior:** CPU takes an "Undefined Instruction" exception at the jump.

## Context
- **Suspected Areas:**
    - `kernel/src/main.rs`: Boot assembly and MMU setup.
    - `levitate-hal/src/mmu.rs`: TCR/MAIR configuration and page table flags.
    - `linker.ld`: VMA/LMA alignment and layout.
- **Recent Findings:**
    - TEAM_024 identified missing TTBR1 mappings as a likely cause for an earlier attempt.
    - TEAM_025 tried a TTBR0-only approach but hit the same execution failure.

## Constraints
- Must maintain a clean separation between mechanism (HAL) and policy (Kernel).
- Should align with standard AArch64 higher-half practices (e.g., as seen in Theseus or Redox).

## Open Questions
- Is it a QEMU `-cpu cortex-a53` specific behavior?
- Are instruction caches stale despite invalidation?
- Is there a subtle SCTLR_EL1 bit (like WXN) preventing execution from a page that might be implicitly seen as writable?

---

## Steps

### [Step 1: Consolidate Bug Information](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-1-step-1.md)
Gather all logs and traces from previous attempts.

### [Step 2: Confirm or Improve Reproduction](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-1-step-2.md)
Create a minimal reproduction case.

### [Step 3: Identify Suspected Code Areas](file:///home/vince/Projects/LevitateOS/docs/planning/higher-half-kernel/plan/phase-1-step-3.md)
List files and specific lines for investigation.
