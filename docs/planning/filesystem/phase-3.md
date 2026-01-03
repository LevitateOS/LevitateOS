# Phase 3: Implementation — Filesystem Feature

**Feature:** FAT32 Filesystem Support  
**Parent:** `/docs/planning/filesystem/`

---

## Implementation Overview

Build the filesystem module step-by-step, each step producing working code.

---

## Steps

### Step 1: Add Dependency and Create Module
- **File:** `phase-3-step-1.md`
- **Goal:** Add `fatfs` to Cargo.toml, create empty `fs.rs`
- **UoW:** 1

### Step 2: Implement BlockDeviceIO Struct
- **File:** `phase-3-step-2.md`
- **Goal:** Create struct with `new()` and basic fields
- **UoW:** 1

### Step 3: Implement Read Trait
- **File:** `phase-3-step-3.md`
- **Goal:** Implement `fatfs::Read` using block::read_block
- **UoW:** 1 (may need 2 if complex)

### Step 4: Implement Seek Trait
- **File:** `phase-3-step-4.md`
- **Goal:** Implement `fatfs::Seek` with position tracking
- **UoW:** 1

### Step 5: Implement Write Trait (Stub)
- **File:** `phase-3-step-5.md`
- **Goal:** Stub `fatfs::Write` (read-only for now)
- **UoW:** 1

### Step 6: Implement init()
- **File:** `phase-3-step-6.md`
- **Goal:** Parse FAT32, store global FileSystem
- **UoW:** 1

### Step 7: Implement read_file()
- **File:** `phase-3-step-7.md`
- **Goal:** Open file by path, read contents to Vec
- **UoW:** 1

### Step 8: Implement list_dir()
- **File:** `phase-3-step-8.md`
- **Goal:** List directory entries
- **UoW:** 1

### Step 9: Wire into kmain
- **File:** `phase-3-step-9.md`
- **Goal:** Call fs::init(), demo file listing
- **UoW:** 1

→ **Next:** Phase 4 (Integration & Testing)
