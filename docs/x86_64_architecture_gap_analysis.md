# x86_64 Architecture Gap Analysis: LevitateOS vs Redox

## 1. Overview
This document details the architectural differences identified between LevitateOS and the Redox kernel regarding x86_64 support. The comparison reveals execution-critical gaps in how LevitateOS handles per-CPU state and context switching, which are the likely root causes of current instability (e.g., `invalid opcode` crashes).

## 2. Per-CPU State Management

### The "Right" Way (Redox)
Redox uses the x86_64 `GS` segment register to store per-CPU data. This is the standard architectural pattern for SMP-safe OS design on x86_64.

*   **Structure**: `ProcessorControlRegion` (PCR) contains GDT, TSS, and per-CPU scratch space.
*   **Mechanism**:
    *   `IA32_GS_BASE` MSR is set to point to the PCR.
    *   `swapgs` instruction is used on kernel entry/exit to swap the user's GS base with the kernel's GS base (the PCR).
    *   Data access: `mov gs:[offset], reg` accesses per-CPU data without locks or atomics.

**Code Reference (Redox):**
```rust
// src/arch/x86_shared/gdt.rs
pub struct ProcessorControlRegion {
    pub self_ref: *mut ProcessorControlRegion,
    pub user_rsp_tmp: usize,
    pub tss: TaskStateSegment,
    // ...
}
```

```asm
// src/arch/x86_64/interrupt/syscall.rs
swapgs
mov gs:[user_rsp_tmp], rsp  // Save user stack to per-CPU scratch
mov rsp, gs:[tss.rsp0]      // Load kernel stack from per-CPU TSS
```

### The Current Way (LevitateOS)
LevitateOS currently uses **global static** variables for what should be per-CPU state.

*   **Structure**: Naked static mut variables in `syscall.rs`.
*   **Mechanism**:
    *   No `swapgs` usage.
    *   RIP-relative addressing to access globals.

**Code Reference (LevitateOS):**
```rust
// syscall.rs
pub static mut CURRENT_KERNEL_STACK: usize = 0;
pub static mut USER_RSP_SCRATCH: usize = 0;
```

```asm
// syscall_entry
mov [rip + USER_RSP_SCRATCH], rsp
mov rsp, [rip + CURRENT_KERNEL_STACK]
```

**Risk**: This is fundamentally broken for SMP (Multi-Core) and introduces race conditions even on single-core if interrupts or context switches update the global state unexpectedly relative to the execution flow of a suspended task.

## 3. Context Switching

### Saved State comparison

| Register | Redox | LevitateOS | Significance |
|----------|-------|------------|--------------|
| RBX, RBP, R12-R15 | ✅ Saved | ✅ Saved | Callee-saved registers. |
| **RFLAGS** | ✅ **Saved** | ❌ **Missing** | **CRITICAL**. Controls direction flag (DF), interrupt flag (IF), etc. |
| RSP | Saved to Context | Saved to Context | Stack pointer. |

### The RFLAGS Gap
LevitateOS does **not** save/restore RFLAGS during `cpu_switch_to`.
*   **Consequence**: If a task (or the kernel during a task's slot) changes RFLAGS (e.g., `std` to set Direction Flag), and we switch to another task that expects `cld` (Clear Direction Flag), memory operations like `rep movsb` will go backwards, causing massive memory corruption.
*   **Redox Approach**: Explicitly pushes/pops RFLAGS during switch.

```asm
// Redox context/arch/x86_64.rs
pushfq
pop QWORD PTR [rdi + {off_rflags}] // Save old
push QWORD PTR [rsi + {off_rflags}] // Load new
popfq
```

## 4. Kernel Stack Management

### Redox
*   **Storage**: Inside `Context` struct as `kstack: Kstack`.
*   **TSS Update**: In `switch_to` (Rust), calls `gdt::set_tss_stack` to update the PCR's TSS with `next.kstack.initial_top()`.

### LevitateOS
*   **Storage**: `kernel_stack_top` in `TaskControlBlock`.
*   **TSS Update**: in `cpu_switch_to` (ASM), updates `TSS.rsp0`.

**Difference**: LevitateOS logic is generally correct for single-core, but the reliance on the global `CURRENT_KERNEL_STACK` for the *next* syscall entry is risky if the restore logic doesn't align perfectly with the `yield` point.

## 5. Recommendations for Future Teams

1.  **Adopt the PCR Pattern**: Stop using global statics for core state. Implement `ProcessorControlRegion` and use `GS`-relative addressing.
2.  **Save RFLAGS**: Immediately add `pushfq`/`popfq` to the context switch assembly. This is a likely source of "silent" corruption.
3.  **Audit `sysretq` canonicality**: Redox explicitly handles non-canonical RCX addresses (security exploit vector). LevitateOS should check this too.
