# Phase 1: Discovery — Multitasking & Scheduler

## Feature Summary
**Phase 7** introduces multi-tasking to LevitateOS. Currently, the kernel runs as a single execution flow in `kmain`. This phase will allow the kernel to manage multiple independent paths of execution (tasks), starting with kernel-level multitasking and eventually supporting userspace.

### Key Components
1. **Virtual Memory Reclamation (`unmap_page`)**: Essential for freeing memory when a task terminates. Requires walking page tables, clearing entries, and invalidating the TLB.
2. **Task Primitives**: Defining the `Task` struct and `TaskControlBlock` (TCB) to hold state (registers, stack, status).
3. **Context Switching**: The mechanism to save/restore CPU state in assembly (`cpu_switch_to`).
4. **Scheduler**: The logic to decide which task runs next, starting with Round-Robin.

## Success Criteria
- [ ] **SC1: Memory Reclamation**: `unmap_page(va)` correctly clears L3 entries and invalidates TLB.
- [ ] **SC2: Table Reclamation**: Intermediate page tables are freed when they become empty.
- [ ] **SC3: Context Switch**: Aarch64 assembly `cpu_switch_to` correctly saves/restores callee-saved registers and SP.
- [ ] **SC4: Task Lifecycle**: Tasks can be created, run, and terminated gracefully.
- [ ] **SC5: Preemption**: Timer-based interrupts trigger the scheduler to switch tasks.

## Current State Analysis
- **Execution Flow**: Single-threaded in `kmain`. Interrupts are supported but only for handlers, not for task preemption yet.
- **MMU**: `levitate-hal/src/mmu.rs` supports mapping pages and 2MB blocks. `map_page` and `map_range` are functional. No unmapping support exists.
- **Allocation**: Buddy Allocator (Phase 5) is integrated and provides physical page frames. `PageAllocator` trait is ready.
- **Context**: No task context structures or assembly switching logic present.

## Codebase Reconnaissance
- `levitate-hal/src/mmu.rs`: Main site for mapping logic. Needs refactoring to expose walking logic for unmapping.
- `levitate-hal/src/lib.rs`: Entry point for HAL.
- `kernel/src/main.rs`: Boot entry point where scheduler initialization will occur.
- `docs/planning/multitasking-phase7/overview.md`: Project-provided context.
- `docs/planning/multitasking-phase7/task-7.1-unmap-page.md`: Specific implementation notes for MMU part.

## Constraints
- **AArch64 Consistency**: Must follow AArch64 exception model and register usage conventions (calling convention for callee-saved registers).
- **Safety**: `unsafe` blocks in MMU and context switching must be documented with `// SAFETY:` comments.
- **Efficiency**: TLB flushes should be granular (`tlbi vae1`) rather than global where possible.

## Steps
1. **Step 1 – Capture Feature Intent**
   - High-level goals clearly stated in `overview.md`.
2. **Step 2 – Analyze Current State**
   - Verified `mmu.rs` lacks `unmap_page`.
   - Verified `PageAllocator` has `free_page` ready.
3. **Step 3 – Source Code Reconnaissance**
   - Identified `mmu.rs` as the first major target.
   - Identified the need for a new `scheduler` module in `kernel/src`.
