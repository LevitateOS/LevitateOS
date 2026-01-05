# Phase 4 — Implementation and Tests

**Bug:** GPU Display Deadlock  
**Team:** TEAM_084  
**Status:** READY FOR IMPLEMENTATION

---

## 4.1 Overview

This phase contains the actual implementation work. Due to the scope (4 files, ~15 functions), it is split into steps with UoWs.

**Estimated Total Effort:** 2-3 UoWs

---

## 4.2 Steps

### Step 1 — Refactor Display struct in gpu.rs
**Size:** 1 UoW  
**File:** `phase-4-step-1-uow-1.md`

### Step 2 — Update terminal.rs  
**Size:** 1 UoW  
**File:** `phase-4-step-2-uow-1.md`

### Step 3 — Update console_gpu.rs and cursor.rs
**Size:** 1 UoW  
**File:** `phase-4-step-3-uow-1.md`

### Step 4 — Verification
**Size:** Part of Step 3 UoW  
**Tasks:** Build, boot, test dual console

---

## 4.3 Dependency Graph

```
Step 1 (gpu.rs)
    ↓
Step 2 (terminal.rs) ←── depends on new Display API
    ↓
Step 3 (console_gpu.rs, cursor.rs) ←── depends on terminal changes
    ↓
Step 4 (Verification)
```

Steps must be executed in order. Each step's UoW is self-contained but depends on prior steps being complete.

---

## 4.4 Success Criteria

After all steps complete:
- [ ] `cargo build` succeeds with no new warnings
- [ ] Kernel boots in QEMU without hangs
- [ ] Dual console enabled (UART + GPU output)
- [ ] Text appears on GPU screen during boot
- [ ] Cursor blinks on GPU screen
- [ ] No deadlocks during normal operation

---

**Proceed to UoW files for implementation details.**
