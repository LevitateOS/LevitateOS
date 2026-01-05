# LevitateOS TODO List

Global tracking of incomplete tasks per Rule 11.

---

## Critical

*No critical issues*

---

## High Priority

### Re-enable Dual Console — TEAM_086
**File:** `kernel/src/main.rs`  
**Description:** GPU mirroring via `set_secondary_output()` can now be re-enabled. The deadlock issue has been fixed.

**Blocked By:** Nothing - GPU Display Deadlock fixed by TEAM_086

---

## Medium Priority

### Boot Hijack Code Still Present — TEAM_081
**File:** `kernel/src/main.rs:612`  
**Description:** TEAM_073's demo code `run_from_initramfs("hello")` is commented out but not removed. Should be cleaned up once userspace shell is working properly.

---

## Low Priority

### Stale Golden Tests — TEAM_081
**File:** `tests/golden_boot.txt`  
**Description:** Behavior tests reflect old boot sequence. Need update after Phase 8b is complete.

---

## Completed

- [x] TEAM_082: Linker script conflict for userspace builds
- [x] TEAM_083: UART debug spam from stale binary
- [x] TEAM_083: Timer "T" debug output flooding console
- [x] TEAM_083: GPU reference compilation error
- [x] TEAM_086: GPU Display Deadlock — Refactored Display to accept `&mut GpuState`
