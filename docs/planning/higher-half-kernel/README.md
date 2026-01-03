# Higher-Half Kernel Implementation

**Status:** 🔴 BLOCKED — Execute permission issue unresolved  
**Priority:** High (Required for Phase 3 completion)  
**Teams:** TEAM_024 (Review), TEAM_025 (Implementation)

---

## Quick Summary

**Goal:** Run kernel at high virtual address `0x0000FFFF80000000+` using TTBR0.

**Current State:** Identity-mapped kernel works. Higher-half attempt fails on code execution despite data access working.

**The Bug:** `br x0` to high VA causes "Undefined Instruction" exception, but `ldr` from same address works.

---

## Files in This Directory

| File | Purpose |
|------|---------|
| `README.md` | This overview |
| `INVESTIGATION_NOTES.md` | Detailed technical findings and debugging guide |

---

## Quick Start for Next Team

```bash
# 1. Read the investigation notes
cat docs/planning/higher-half-kernel/INVESTIGATION_NOTES.md

# 2. Read previous team files
cat .teams/TEAM_024_review_impl_high_memory.md
cat .teams/TEAM_025_implement_higher_half_ttbr0.md

# 3. Run the debug script
./scripts/debug_higher_half.sh

# 4. Examine QEMU debug log
less qemu_higher_half_debug.log
```

---

## The Core Issue

```
┌─────────────────────────────────────────────────────────────┐
│  Data Read:   ldr w1, [x0]  where x0 = 0x0000FFFF80089870  │
│  Result:      ✅ WORKS — returns correct instruction bytes  │
├─────────────────────────────────────────────────────────────┤
│  Code Jump:   br x0         where x0 = 0x0000FFFF80089870  │
│  Result:      ❌ FAILS — "Undefined Instruction" exception  │
└─────────────────────────────────────────────────────────────┘
```

This is unusual because both operations use the same virtual address and the same page table entry.

---

## Key Resources

### Internal
- `levitate-hal/src/mmu.rs` — MMU abstraction (reference for page table flags)
- `kernel/src/main.rs` — Current boot assembly (identity-mapped)
- `linker.ld` — Current linker script

### External References
- `.external-kernels/theseus/` — Working higher-half kernel (x86_64 and aarch64)
- `.external-kernels/redox-kernel/` — Alternative approach
- ARMv8 Architecture Reference Manual (ARM DDI 0487)

---

## Success Criteria

The higher-half kernel is complete when:

1. [ ] Kernel code runs at VA `0x0000FFFF80000000+`
2. [ ] Boot code runs at PA `0x40080000` (identity mapped)
3. [ ] Jump from boot to kernel succeeds
4. [ ] `./scripts/test_behavior.sh` passes
5. [ ] Identity mapping can be removed after jump (optional)

---

## Estimated Effort

Based on TEAM_025's experience:
- **Minimum:** 2-4 hours (if root cause is simple)
- **Expected:** 1-2 days (debugging + implementation)
- **Maximum:** 1 week (if architectural changes needed)

The issue is likely a subtle configuration detail, not a fundamental approach problem.
