# Phase 1: Understanding and Scoping

**Team:** TEAM_077  
**Parent:** `plan.md`  
**Status:** COMPLETE (inherited from TEAM_076 investigation)

---

## 1. Bug Summary

**Short Description:** Device MMIO (UART, VirtIO, GIC) uses identity mapping via TTBR0. When TTBR0 is switched to a user page table for userspace execution, device access causes a translation fault and the kernel hangs.

**Severity:** **CRITICAL** - Userspace execution is completely blocked.

**Impact:**
- Kernel cannot print after switching to user page table
- Syscall handlers cannot access console
- Userspace execution is completely broken
- Phase 8 (Userspace & Syscalls) is blocked

---

## 2. Reproduction Status

**Reproducible:** YES (100% reproducible)

### Reproduction Steps
1. `cargo xtask run`
2. Boot kernel to spawn user process
3. Observe hang after `[TASK] Before switch_ttbr0(0x48011000)`

### Expected Behavior
User process executes and prints "Hello from userspace!"

### Actual Behavior
System hangs indefinitely after `switch_ttbr0()` call.

### Evidence
```
[SPAWN] Created user process PID=1 entry=0x10000 sp=0x7fffffff0000
[SPAWN] Starting user process...
[TASK] Before switch_ttbr0(0x48011000)
<hang>
```

---

## 3. Context

### Suspected Code Areas
| File | Component | Role |
|------|-----------|------|
| `levitate-hal/src/mmu.rs` | MMU/Page Tables | Device mapping strategy |
| `levitate-hal/src/console.rs` | Console Driver | Uses UART at identity-mapped address |
| `levitate-hal/src/gic.rs` | GIC Driver | Uses GIC at identity-mapped address |
| `levitate-hal/src/virtio/*.rs` | VirtIO Drivers | Use MMIO at identity-mapped addresses |
| `kernel/src/task/process.rs` | User Process | Calls `switch_ttbr0()` |

### Recent Related Changes
- TEAM_072-076: Phase 8 userspace work
- TEAM_070-071: Phase 7 multitasking/scheduler

### Reference Materials
- `@/home/vince/Projects/LevitateOS/.teams/TEAM_076_investigate_userspace_hang.md` - Full investigation
- `@/home/vince/Projects/LevitateOS/kernel/src/task/process.rs:75` - CONFIRMED breadcrumb

---

## 4. Constraints

| Constraint | Notes |
|------------|-------|
| Time Sensitivity | High - blocks Phase 8 progress |
| Backwards Compatibility | Must not break existing kernel boot |
| Platform | QEMU virt (primary), future RPi4/Pixel6 |
| Safety | Device mappings must be kernel-only (not user-accessible) |

---

## 5. Open Questions

None - root cause is confirmed by TEAM_076.

---

## 6. Steps (Completed)

| Step | Description | Status |
|------|-------------|--------|
| 1 | Consolidate bug information | ✅ Done by TEAM_076 |
| 2 | Confirm reproduction | ✅ Done by TEAM_076 |
| 3 | Identify suspected code areas | ✅ Done by TEAM_076 |

**Phase 1 Complete.** Proceed to Phase 2.
