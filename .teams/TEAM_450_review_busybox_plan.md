# TEAM_450: Review BusyBox Integration Plan

**Date:** 2026-01-12  
**Task:** Review and refine busybox-integration plan  
**Status:** Complete

---

## User Answers Received

| Question | Answer | Implication |
|----------|--------|-------------|
| Q1: Shell exit | A (Respawn) | Use `::respawn` in inittab |
| Q2: Mount proc/sys | A (Yes) | Add mount commands to init |
| Q3: Test mode | A (Separate initramfs) | No test runner in production init |
| Q4: Applets | B (Standard ~1MB) | Current applet list is appropriate |
| Q5: vi editor | A (Yes) | Keep vi in applet list |

---

## Review Phases

### Phase 1: Questions & Answers Audit
- [x] Verify plan reflects user answers

### Phase 2: Scope & Complexity Check
- [x] Check for overengineering
- [x] Check for oversimplification

### Phase 3: Architecture Alignment
- [x] Verify fits existing codebase structure

### Phase 4: Global Rules Compliance
- [x] Check all rules followed

### Phase 5: Verification
- [x] Verify technical claims

### Phase 6: Final Refinements
- [x] Apply corrections to plan

---

## Findings

### Phase 1: Q&A Audit

**Issues Found:**
1. ⚠️ Missing `/proc` and `/sys` mount commands in inittab (Q2=A)
2. ⚠️ No implementation details for separate test initramfs (Q3=A)

**Corrections Applied:**
- Added mount commands to inittab in phase-2.md and phase-3.md
- Added test initramfs section to phase-4.md
- Updated questions file with user's answers

### Phase 2: Scope & Complexity Check

**Verdict: APPROPRIATE**

- 5 phases is reasonable for replacing 3 components (init, shell, coreutils)
- UoWs are appropriately SLM-sized
- No unnecessary abstractions detected
- Cleanup phase (5) exists - good
- Test strategy documented - good

### Phase 3: Architecture Alignment

**Verdict: ALIGNED**

- New `busybox.rs` follows existing `c_apps.rs` pattern ✓
- Output paths follow `toolchain/<name>-out/<arch>/` convention ✓
- Build commands follow existing xtask patterns ✓
- Removes old code cleanly (Rule 6 compliant) ✓

### Phase 4: Global Rules Compliance

| Rule | Status |
|------|--------|
| Rule 0 (Quality) | ✓ Clean solution, no hacks |
| Rule 4 (Regression) | ✓ Golden logs update mentioned |
| Rule 5 (Breaking Changes) | ✓ Clean removal, no shims |
| Rule 6 (No Dead Code) | ✓ Phase 5 removes old code |
| Rule 10 (Handoff) | ✓ Phase 5 has handoff notes |

### Phase 5: Technical Verification

**Verified Claims:**
- BusyBox static musl build: ✓ Standard practice (Alpine Linux)
- ~1MB binary size: ✓ Typical for defconfig
- Syscall compatibility: ✓ Uses standard POSIX syscalls

**Note:** aarch64 cross-compilation mentioned as risk - plan acknowledges this.

---

## Corrections Made

1. `docs/questions/TEAM_449_busybox_integration.md` - Added user's answers
2. `docs/planning/busybox-integration/phase-2.md` - Added proc/sys mounts, updated status
3. `docs/planning/busybox-integration/phase-3.md` - Added proc/sys mounts to inittab
4. `docs/planning/busybox-integration/phase-4.md` - Added test initramfs section

---

## Final Assessment

**Plan Quality: GOOD**

The plan is well-structured, appropriately scoped, and architecturally sound.
Minor gaps (proc/sys mounting, test initramfs details) have been corrected.

**Recommendation: PROCEED TO IMPLEMENTATION**

---

## Session 2: Implementing Syscall Stubs (2026-01-12)

### Objective
Add missing syscalls required for BusyBox utilities to run.

### Implementation

Added the following syscalls as no-ops for single-user OS:

| Syscall | Purpose |
|---------|---------|
| setuid | Set user ID |
| setgid | Set group ID |
| setreuid | Set real and effective user IDs |
| setregid | Set real and effective group IDs |
| setresuid | Set real, effective, and saved user IDs |
| setresgid | Set real, effective, and saved group IDs |
| getresuid | Get real, effective, and saved user IDs |
| getresgid | Get real, effective, and saved group IDs |

Also added dispatch entries for fchmodat and fchownat (implementations already existed).

### Files Modified
- `syscall/src/process/identity.rs` - Added syscall implementations
- `syscall/src/process/mod.rs` - Updated exports
- `arch/x86_64/src/lib.rs` - Added syscall numbers and from_u64 cases
- `arch/aarch64/src/lib.rs` - Added syscall numbers and from_u64 cases
- `syscall/src/lib.rs` - Added dispatch entries

### Commit
`3dea4d5` - feat(syscall): add user identity syscalls for BusyBox compatibility
