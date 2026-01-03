# Phase 4: Integration & Testing — Filesystem Feature

**Feature:** FAT32 Filesystem Support  
**Parent:** `/docs/planning/filesystem/`

---

## Steps

### Step 1: Update Golden Boot Log
- **File:** `phase-4-step-1.md`
- **Goal:** Add "Filesystem initialized." to `tests/golden_boot.txt`
- **UoW:** 1

### Step 2: Add Regression Test
- **File:** `phase-4-step-2.md`
- **Goal:** Add static check for fs.rs existence and fatfs dependency
- **UoW:** 1

### Step 3: Run Full Test Suite
- **File:** `phase-4-step-3.md`
- **Goal:** `cargo xtask test` passes
- **UoW:** 1

### Step 4: Manual QEMU Verification
- **File:** `phase-4-step-4.md`
- **Goal:** Boot kernel, verify file listing works
- **UoW:** 1

→ **Next:** Phase 5 (Polish & Docs)
