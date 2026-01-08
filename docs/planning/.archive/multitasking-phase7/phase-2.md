# Phase 2: Design — Multitasking & Scheduler

## Proposed Solution

### 1. Virtual Memory Reclamation (`unmap_page`)
- **Helper**: `fn walk_to_entry(root: &mut PageTable, va: usize, create: bool) -> Result<&mut PageTableEntry, Error>`
  - This refactors `get_or_create_table`.
- **Logic**:
  - `unmap_page` walks to L3.
  - Clears `VALID` bit and `TABLE` bit (if set).
  - Calls `tlb_flush_page(va)`.
  - (Phase B) Recurse back if table is empty.

### 2. Task Control Block (TCB)
Refining the structure based on Redox and Theseus:
```rust
pub struct Task {
    id: TaskId,
    state: TaskState,
    context: Context,          // Saved x19-x30, SP, ELR, SPSR
    stack: KernelStack,        // 16KB stack
    page_table: RootPageTable, // TTBR0_EL1 value
}

pub struct Context {
    // Callee-saved GPRs (x19-x29)
    x19: u64, x20: u64, x21: u64, x22: u64, 
    x23: u64, x24: u64, x25: u64, x26: u64, 
    x27: u64, x28: u64, x29: u64,
    
    // Link register (x30)
    lr: u64,    
    // Stack pointer (SP_EL1)
    sp: u64,
    // Exception Link Register (ELR_EL1) - where to return to
    elr: u64,
    // Saved Process Status Register (SPSR_EL1)
    spsr: u64,
}
```

### 3. Scheduler Logic
- **Storage**: `VecDeque<TaskRef>` (inspired by Theseus) or a linked-list queue.
- **Preemption**: Every 10ms (approx), the Generic Timer interrupt will:
  1. Acknowledge the interrupt.
  2. Call `schedule()`.
- **Global Lock**: Use a `static CONTEXT_SWITCH_LOCK: AtomicBool` (Redox pattern) to ensure only one CPU core (if SMP) or one context is switching at a time, protecting the stack switch.

## Design References
- **Redox**: Uses `naked_asm!` for `switch_to`. Saves `elr_el1` and `spsr_el1` explicitly to handle nested exceptions or userspace return context accurately.
- **Theseus**: Modular scheduler allows swapping policies (RR, Priority). We will stick to RR for now (Rule 20: Simplicity).

## Behavioral Decisions & Questions (EXPECTED FOR USER REVIEW)

### Q1: Handling `unmap_page` on non-mapped address
- **Option A**: Error (return `Result::Err`).
- **Option B**: Silent Ignore (return `Result::Ok`).
- **Decision**: **Option A**. 
- **Justification**: **Rule 14 (Fail Loud, Fail Fast)** and **Rule 6 (Robust Error Handling)**. Attempting to unmap a non-existent mapping suggests a bug in the caller's state tracking. Masking this error leads to inconsistent state.

### Q2: Task Termination & Cleanup
- When a task exits, its stack and TCB memory must be reclaimed.
- **Question**: Who reclaims the "current" task's memory if it is currently running?
- **Decision**: A "Reaper" task or the next task in `schedule()` checks for `Exited` status and cleans up.
- **Justification**: **Rule 17 (Resilience & Self-Healing)**. We need a closed-loop recovery mechanism to prevent resource leaks that would eventually crash the kernel.

### Q3: Idle Task Behavior
- **Question**: What should the CPU do when no tasks are ready?
- **Decision**: A dedicated `idle_task` that runs `wfi` (Wait For Interrupt) in a loop.
- **Justification**: **Rule 16 (Energy Awareness & Power Efficiency)**. Implementing a "Race to Sleep" strategy requires the CPU to enter a low-power state whenever no work is pending.

### Q4: Scheduler Re-entrancy
- **Question**: Can `schedule()` be called from an interrupt handler while another `schedule()` is running?
- **Decision**: Disable interrupts during the critical section of the scheduler and use IRQ-safe primitives.
- **Justification**: **Rule 7 (Concurrency & Sync)**. Use `IrqSafeLock` or similar for data shared between threads and interrupt handlers.

### Q5: Task Stack Size
- **Question**: What should be the default kernel stack size?
- **Decision**: 16KB (4 pages) for initial development.
- **Justification**: **Rule 20 (Simplicity > Perfection)**. A larger stack is simpler to manage initially than optimizing for minimal stack usage which could lead to hard-to-debug stack overflows.

## Design Alternatives Considered
- **Stack per Task vs. Shared Interrupt Stack**: We will use a dedicated stack per task for simplicity and isolation.
- **Cooperative vs. Preemptive**: We will implement cooperative first (`yield_now()`), then add preemption.

## Steps
1. **Step 1 – Draft Initial Design** (Done above)
2. **Step 2 – Define Behavioral Contracts**
   - Address Q1-Q5.
3. **Step 3 – Review Design Against Architecture**
   - Integration with `Buddy Allocator` for stacks.
4. **Step 4 – Finalize Design After Questions Answered**
