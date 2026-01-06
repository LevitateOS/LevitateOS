# Phase 3: Implementation — TTBR0 Restoration

> **Prerequisites:** Phase 2 questions must be answered before starting.

## Implementation Overview

| Step | Description | Files | UoWs |
|------|-------------|-------|------|
| 1 | Extend SyscallFrame struct | `syscall.rs` | 1 |
| 2 | Update assembly to save/restore TTBR0 | `exceptions.rs` | 1 |
| 3 | Re-enable yield_now in sys_read | `syscall.rs` | 1 |

---

## Step 1: Extend SyscallFrame

**File:** `kernel/src/syscall.rs`

### Changes:

1. Add `ttbr0_el1: u64` field to `SyscallFrame`
2. Update any code that creates/accesses the frame

```rust
#[repr(C)]
pub struct SyscallFrame {
    pub x: [u64; 31],      // 0-247
    pub sp_el0: u64,       // 248
    pub elr_el1: u64,      // 256  
    pub spsr_el1: u64,     // 264
    pub ttbr0_el1: u64,    // 272 (NEW)
}
```

### Exit Criteria:
- Code compiles
- Frame size is 280 bytes

---

## Step 2: Update Assembly

**File:** `kernel/src/exceptions.rs`

### Changes to `sync_lower_el_entry`:

```asm
sync_lower_el_entry:
    sub     sp, sp, #280        // Was 272

    // ... existing register saves ...
    
    // NEW: Save TTBR0
    mrs     x0, ttbr0_el1
    str     x0, [sp, #272]
    
    mov     x0, sp
    bl      handle_sync_lower_el
    
    // NEW: Restore TTBR0 (before other restores)
    ldr     x0, [sp, #272]
    msr     ttbr0_el1, x0
    isb
    
    // ... existing register restores ...
    
    add     sp, sp, #280        // Was 272
    eret
```

### Exit Criteria:
- Kernel boots without crash
- Syscalls still work

---

## Step 3: Re-enable yield_now in sys_read

**File:** `kernel/src/syscall.rs`

### Changes:

```rust
// In sys_read, restore yield_now:
if bytes_read == 0 {
    crate::task::yield_now();  // Now works!
    #[cfg(target_arch = "aarch64")]
    aarch64_cpu::asm::wfi();
}
```

### Exit Criteria:
- Shell waits for input without crashing
- Multiple tasks can run during blocking read

---

## Design References

All implementation follows [Phase 2 Design](phase-2.md):
- Decision 1: Don't modify saved TTBR0 in switch_to
- Decision 2: TLB flush via isb is sufficient
- Decision 3: IRQ handlers not modified initially
