# TEAM_389: Review of Graceful OOM Plan

**Date:** 2026-01-10  
**Task:** Review `/docs/planning/graceful-oom/` per review-a-plan workflow

---

## Phase 1 — Questions and Answers Audit

### Findings

**No questions file exists** in `.questions/` for this plan.

The plan (phase-2.md) lists 5 design questions (Q1-Q5) that were "answered via kernel-development.md" without explicit user input. While deriving answers from project philosophy is reasonable, this assumes:
- The user agrees with the interpretations
- No edge cases were missed

**Verdict:** Acceptable. The derived answers are sensible and align with Unix philosophy.

---

## Phase 2 — Scope and Complexity Check

### Overengineering Signals: **None detected**

- 3 phases is appropriate (Discovery → Design → Implementation)
- Steps are reasonably sized
- No unnecessary abstractions proposed

### Oversimplification Signals: **Critical issues found**

#### Issue 1: `sys_sbrk` Returns 0, Not ENOMEM

**Plan claim (phase-1.md:34):**
> `sys_brk`/`sys_mmap` return `ENOMEM` ✓ (this part works)

**Actual code (mm.rs:104-114):**
```rust
if mm_user::alloc_and_map_heap_page(task.ttbr0, va).is_err() {
    heap.current = old_break;
    return 0; // null  <-- NOT ENOMEM!
}
...
Err(()) => 0,  // <-- Also 0, not ENOMEM
```

`sys_sbrk` returns **0 (NULL)**, not `-ENOMEM (-12)`.

**Impact:** The plan's assumption that "syscalls already return ENOMEM correctly" is **incorrect for sbrk**. Step 1 verification will reveal this needs fixing.

#### Issue 2: Eyra is an External Dependency

**Plan claim (phase-2.md:23, phase-3.md:73-76):**
> Make Eyra's userspace allocator handle `sbrk` returning ENOMEM gracefully.

**Actual code (coreutils/Cargo.toml:294):**
```toml
eyra = { version = "0.22", features = ["experimental-relocate"] }
```

Eyra is an **external crates.io dependency**, not code in this repository. The plan cannot "fix Eyra's allocator" without:
- Forking Eyra and patching it
- Contributing upstream
- Using a custom allocator wrapper

**Impact:** Step 5 in phase-3.md may not be feasible as written. Needs clarification on approach.

#### Issue 3: Missing Regression Test Plan

Phase-3.md mentions testing but lacks:
- Specific golden log expectations
- Baseline behavior to preserve
- How to test OOM without breaking the system

---

## Phase 3 — Architecture Alignment

### Findings

**Current heap constant locations:**
- `crates/kernel/src/memory/heap.rs:6` - `USER_HEAP_MAX_SIZE = 64MB`
- `crates/kernel/src/task/user.rs:60` - Duplicate constant (dead code warning)

**Plan correctly identifies** both locations need updating (phase-3.md:50).

**Architecture compliance:** ✓ Changes respect existing module boundaries.

---

## Phase 4 — Global Rules Compliance

| Rule | Status | Notes |
|------|--------|-------|
| Rule 0 (Quality > Speed) | ✓ | No shortcuts proposed |
| Rule 1 (SSOT) | ✓ | Plan in docs/planning/ |
| Rule 2 (Team Registration) | ✓ | TEAM_388 registered |
| Rule 3 (Pre-work) | ⚠ | Questions answered implicitly |
| Rule 4 (Regression Protection) | ⚠ | Missing test plan details |
| Rule 5 (Breaking Changes) | ✓ | No adapters proposed |
| Rule 6 (No Dead Code) | N/A | |
| Rule 7 (Modular Refactoring) | ✓ | |
| Rule 8 (Ask Questions) | ⚠ | No questions file |
| Rule 10 (Before Finishing) | ✓ | Acceptance criteria exist |
| Rule 11 (TODO Tracking) | ⚠ | No TODOs documented |

---

## Phase 5 — Verification and References

### Verified Claims

| Claim | Status | Evidence |
|-------|--------|----------|
| `sys_mmap` returns ENOMEM | ✓ | mm.rs:176, 191, 205 |
| `sys_sbrk` returns ENOMEM | ✗ | mm.rs:106,113 return 0 |
| USER_HEAP_MAX_SIZE = 64MB | ✓ | heap.rs:6 |
| TEAM_387 added diagnostic OOM | Unverified | Need to check |

### Unverified Claims

| Claim | Risk |
|-------|------|
| Eyra can be fixed | Medium - External dependency |
| Userspace panic can be caught | Medium - Requires runtime changes |

---

## Phase 6 — Final Refinements

### Critical Corrections Needed

1. **Fix phase-1.md:34** — Remove claim that sbrk returns ENOMEM. Change to:
   > `sys_mmap` returns ENOMEM ✓, but `sys_sbrk` returns 0 (needs fix)

2. **Add to phase-3.md Step 1** — Actually fix `sys_sbrk` to return ENOMEM:
   ```rust
   // Instead of: return 0;
   return ENOMEM; // -12
   ```

3. **Clarify phase-3.md Step 4-5** — Address Eyra external dependency:
   - Option A: Fork Eyra, patch allocator, use path dependency
   - Option B: Add custom panic handler that exits gracefully
   - Option C: Accept that Eyra's allocator will panic, focus on clean process termination

### Important Corrections

4. **Add regression test plan** — Before/after behavior for:
   - Normal allocation (should work)
   - Heap exhaustion (should return error, not panic)
   - Kernel vs userspace OOM distinction

5. **Create questions file** if user input needed on Eyra approach.

---

## Summary

| Category | Findings |
|----------|----------|
| **Critical** | sys_sbrk returns 0, not ENOMEM (plan assumption wrong) |
| **Critical** | Eyra is external crate, cannot "fix" without forking |
| **Important** | Missing regression test specifics |
| **Minor** | Implicit question answering (acceptable) |

### Recommendation

The plan is **mostly sound** but needs corrections before implementation:
1. Fix the incorrect assumption about sbrk
2. Decide Eyra approach (fork vs panic handler)
3. Add concrete test plan

---

## Implementation Completed

**User decision:** Option B — Custom panic handler

### Changes Made

1. **OOM Panic Handler** — `crates/userspace/eyra/coreutils/src/uucore/src/lib/mods/panic.rs`
   - Added `is_oom_panic()` function to detect allocation failures
   - Extended `mute_sigpipe_panic()` to catch OOM panics
   - On OOM: prints "Error: out of memory" and exits with code 134

2. **Fixed sys_sbrk** — `crates/kernel/src/syscall/mm.rs`
   - Changed return value from 0 (NULL) to ENOMEM (-12) on allocation failure
   - Two locations fixed: page allocation failure and heap bounds exceeded

3. **Updated Plan Documents**
   - `phase-1.md`: Corrected claim about sys_sbrk returning ENOMEM
   - `phase-3.md`: Updated Step 1, 4, 5 with actual findings and implementation

### Implementation Complete

All steps implemented:

- [x] Step 1: Fix sys_sbrk to return ENOMEM
- [x] Step 2: Increase USER_HEAP_MAX_SIZE to 256MB
- [x] Step 3: Add debug logging for userspace OOM
- [x] Step 4: Eyra allocator investigation
- [x] Step 5: OOM panic handler
- [x] Step 6: Integration testing (behavior tests pass)

### Handoff Checklist

- [x] Plan reviewed and corrected
- [x] Critical issues identified and fixed (sys_sbrk, panic handler)
- [x] All implementation steps completed
- [x] Team file updated
- [x] Kernel builds cleanly
- [x] Behavior tests pass (golden logs updated)
