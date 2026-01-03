# Phase 1 — Discovery: AArch64 Generic Timer Driver

## Feature Summary
- **Description**: Implement a formal AArch64 Generic Timer driver within the `levitate-hal` crate to replace the current ad-hoc timer logic in the kernel.
- **Problem Statement**: The current timer logic is likely embedded in the kernel or minimally implemented, making it hard to manage interrupts and reuse for multitasking.
- **Who benefits**: The kernel (for scheduling) and the HAL (for hardware abstraction).

## Success Criteria
- Timer interrupts are successfully handled by the kernel.
- The timer driver resides in `levitate-hal`.
- The kernel can initialize and use the timer through a clean HAL API.

## Current State Analysis
- `kernel/src/timer.rs` contains the current implementation.
- `kernel/src/main.rs` initializes the GIC and enables IRQ 30 (often the physical timer IRQ).

## Codebase Reconnaissance
- `levitate-hal/src/lib.rs`: Needs to export the new timer module.
- `kernel/src/timer.rs`: Existing code to be migrated.
- `kernel/src/main.rs`: Needs to be refactored to use the new HAL API.
- `levitate-hal/src/gic.rs`: Interaction for interrupt enabling.

## Constraints
- Must work in QEMU `virt` machine (AArch64).
- Must adhere to the Kernel Development SOP (Modular Scope).

## Steps

### Step 1 – Capture Feature Intent
- [x] Initial summary and success criteria drafted.

### Step 2 – Analyze Current State
- [x] Read and document `kernel/src/timer.rs` logic.
- [x] Determine IRQ numbers and register definitions used.
    - IRQ 30 is the Physical Timer IRQ for AArch64 QEMU `virt`.
    - Registers: `cntpct_el0` (counter), `cntfrq_el0` (frequency), `cntp_tval_el0` (timer value), `cntp_ctl_el0` (control).

### Step 3 – Source Code Reconnaissance
- [x] Identify necessary `levitate-hal` exports.
    - `timer` module needs to be added to `levitate-hal`.
- [x] Check if `gic` driver needs updates to support timer interrupts.
    - `gic` already supports `enable_irq(30)`.
