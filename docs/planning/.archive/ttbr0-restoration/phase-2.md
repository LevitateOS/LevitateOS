# Phase 2: Design — TTBR0 Restoration

## Proposed Solution

### High-Level Design

1. **Extend SyscallFrame** to include `ttbr0_el1`
2. **Save TTBR0** on syscall entry (in assembly)
3. **Restore TTBR0** before `eret` (in assembly)
4. **Update `switch_to`** to modify saved TTBR0 in frame (optional, see Q1)

### Syscall Flow (After Change)

```
User task (PID=2) calls svc #0
  │
  ▼
sync_lower_el_entry:
  ├── Save x0-x30, sp_el0, elr_el1, spsr_el1
  ├── NEW: mrs x0, ttbr0_el1 → save to frame   ◄── Save TTBR0
  ├── bl handle_sync_lower_el
  │     └── syscall handler may call yield_now()
  │           └── switch_to changes CURRENT_TASK and TTBR0
  ├── NEW: ldr x0, [sp, #TTBR0_OFFSET] → msr ttbr0_el1   ◄── Restore TTBR0
  ├── Restore other regs
  └── eret
```

---

## API / Data Model Changes

### SyscallFrame (Modified)

```rust
#[repr(C)]
pub struct SyscallFrame {
    pub x: [u64; 31],      // General registers (0-240)
    pub sp_el0: u64,       // User stack (248)
    pub elr_el1: u64,      // Return address (256)
    pub spsr_el1: u64,     // Status (264)
    pub ttbr0_el1: u64,    // NEW: User page table (272)
}
// Total size: 280 bytes (was 272)
```

### Assembly Changes

```diff
 sync_lower_el_entry:
-    sub     sp, sp, #272
+    sub     sp, sp, #280
     ...
+    mrs     x0, ttbr0_el1
+    str     x0, [sp, #272]
     ...
     bl      handle_sync_lower_el
     ...
+    ldr     x0, [sp, #272]
+    msr     ttbr0_el1, x0
+    isb
     ...
+    add     sp, sp, #280
     eret
```

---

## Behavioral Decisions

### Decision 1: What happens when `switch_to` is called during syscall?

**Options:**
- **A)** `switch_to` doesn't touch the saved frame — only changes hardware TTBR0
- **B)** `switch_to` updates the saved TTBR0 in the current task's syscall frame

**Chosen:** Option A — Simpler. The saved TTBR0 in the frame is correct from entry. When we return, we restore the original task's TTBR0.

**Rationale:** If we yield to task1, task1 runs with its TTBR0. When task1 yields back to task2, task2's syscall frame still has task2's original TTBR0.

### Decision 2: TLB flush after TTBR0 restore?

**Options:**
- **A)** Always flush TLB after restore (`tlbi vmalle1; dsb sy; isb`)
- **B)** No flush — assume ASID handles it
- **C)** Flush only if TTBR0 changed

**Recommendation:** Option C — Only flush if the restored TTBR0 differs from current. This is what `switch_ttbr0` already does.

### Decision 3: IRQ from userspace — same treatment?

**Options:**
- **A)** Yes — save/restore TTBR0 in `irq_lower_el_entry` too
- **B)** No — IRQ handlers shouldn't yield

**Recommendation:** Option B for now. IRQ handlers should be fast and not yield. Can add later if needed.

---

## Open Questions

> [!IMPORTANT]
> **Q1:** Should `switch_to()` update the saved TTBR0 in the yielding task's frame, or leave it unchanged?
> 
> **Context:** When task2 yields to task1 during a syscall, task2's saved `ttbr0_el1` still points to task2's page tables. When task1 later returns via eret (its own syscall), it restores its own frame. When task2 is eventually resumed, its original frame is correct.
> 
> **Recommendation:** Leave unchanged (Option A). The frame captures the state at syscall entry.

> [!IMPORTANT]  
> **Q2:** Should we handle the case where a task is terminated during a syscall yield?
>
> **Context:** If task2 calls sys_read, yields, and is then killed by another task before resuming, what happens?
>
> **Options:**
> - A) Don't handle — killing mid-syscall is undefined behavior for now
> - B) Add cleanup logic to detect and handle this case
>
> **Recommendation:** Option A for initial implementation.

> [!IMPORTANT]
> **Q3:** Should IRQ handlers also save/restore TTBR0?
>
> **Context:** If a timer IRQ fires during userspace and the handler calls schedule(), the same issue could occur.
>
> **Recommendation:** Not initially — keep IRQ handlers simple.

---

## Verification Plan

1. **Unit Test:** Syscall from task2, yield to task1, return to task2 — verify no crash
2. **Behavior Test:** Shell prints prompt, waits for input (with yield), receives input correctly
3. **Regression:** All existing tests must pass
