# Phase 1: Discovery — TTBR0 Restoration

## Feature Summary

**Problem:** Blocking syscalls (e.g., `sys_read` waiting for keyboard input) cannot yield to other tasks because the syscall return path (`eret`) doesn't restore TTBR0. This forces single-tasking behavior during I/O waits.

**Solution:** Save TTBR0 on syscall entry, restore on `eret`, enabling proper preemptive multitasking.

**Who Benefits:** 
- Multiple concurrent user processes
- Interactive shell responsiveness
- Background tasks during I/O

---

## Success Criteria

1. `yield_now()` can be called from syscall handlers without crashes
2. Multiple user tasks can run concurrently during blocking syscalls
3. All existing tests continue to pass
4. No performance regression for non-yielding syscalls

---

## Current State Analysis

### How It Works Today

1. User task (PID=2) calls `svc #0` (syscall)
2. CPU traps to `sync_lower_el_entry` with TTBR0 = task2's page tables
3. Syscall handler runs in kernel mode (TTBR1)
4. Handler calls `yield_now()` → `switch_to(task1)`
5. `CURRENT_TASK` changes but TTBR0 is NOT switched
6. Handler returns, `eret` executed
7. **BUG:** CPU returns to EL0 with wrong TTBR0 → crash

### Current Exception Handler (sync_lower_el_entry)

```asm
sync_lower_el_entry:
    sub     sp, sp, #272        // Save context
    stp     x0, x1, [sp, #0]
    ...
    mrs     x0, sp_el0          // Save SP_EL0
    mrs     x0, elr_el1         // Save return address
    mrs     x0, spsr_el1        // Save status
    
    mov     x0, sp
    bl      handle_sync_lower_el
    
    // Restore and eret
    ldr     x0, [sp, #264]
    msr     spsr_el1, x0
    ldr     x0, [sp, #256]
    msr     elr_el1, x0
    ...
    eret                        // ← No TTBR0 restore!
```

### Missing Piece

TTBR0 is not saved/restored in the syscall frame. The `eret` returns with whatever TTBR0 was last set.

---

## Codebase Reconnaissance

### Files to Modify

| File | Purpose |
|------|---------|
| `kernel/src/exceptions.rs` | Syscall entry/exit assembly, save/restore TTBR0 |
| `kernel/src/syscall.rs` | SyscallFrame struct, add ttbr0 field |
| `kernel/src/task/mod.rs` | `switch_to` may need to update saved ttbr0 |

### Key Data Structures

```rust
// kernel/src/syscall.rs - Current SyscallFrame
pub struct SyscallFrame {
    pub x: [u64; 31],    // General registers
    pub sp_el0: u64,     // User stack pointer
    pub elr_el1: u64,    // Return address
    pub spsr_el1: u64,   // Status register
    // MISSING: ttbr0_el1!
}
```

### Tests Affected

- `cargo xtask test behavior` - Must still pass
- New test: Multiple tasks yielding during syscall

---

## Constraints

1. **Performance:** Saving/restoring TTBR0 adds ~2 instructions per syscall
2. **TLB:** May need TLB flush after TTBR0 switch (already done in `switch_ttbr0`)
3. **Interrupt Safety:** IRQ from userspace also needs similar handling
4. **Backward Compatibility:** Existing single-task code must still work

---

## Open Questions (Phase 1)

None — Phase 1 is discovery only. Questions will emerge in Phase 2 (Design).
