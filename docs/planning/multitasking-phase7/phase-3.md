# Phase 3: Implementation — Multitasking & Scheduler

This phase focuses on building the core multitasking capabilities of LevitateOS, following the designs established in Phase 2.

## Implementation Overview

The implementation is broken down into five major steps, each further subdivided into manageable Units of Work (UoW).

### [Step 1: Virtual Memory Reclamation](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-1.md)
Refactor `mmu.rs` and implement `unmap_page` with TLB flushing.

### [Step 2: Task Primitives](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-2.md)
Define the `Task`, `TCB`, and `Context` structures.

### [Step 3: AArch64 Context Switching](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-3.md)
Implement the assembly-level `cpu_switch_to` and naked Rust wrappers.

### [Step 4: Basic Scheduler](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-4.md)
Implement the `Scheduler` struct, `TaskQueue`, and cooperative yielding.

### [Step 5: Preemptive Scheduling](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-3-step-5.md)
Integrate the Generic Timer to trigger task switches.

## Design Reference
All implementation follows the decisions documented in [phase-2.md](file:///home/vince/Projects/LevitateOS/docs/planning/multitasking-phase7/phase-2.md).
