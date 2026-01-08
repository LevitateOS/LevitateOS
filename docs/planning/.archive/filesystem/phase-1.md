# Phase 1: Discovery — Filesystem Feature

**Feature:** FAT32 Filesystem Support  
**Parent:** `/docs/planning/filesystem/`

---

## Feature Summary

- **Problem:** LevitateOS can read/write disk sectors but cannot interpret filesystem structure
- **Goal:** Read files from a FAT32-formatted disk image
- **Who benefits:** Future userspace loader (Phase 8) needs to load ELF binaries

---

## Success Criteria

1. ✅ Can mount a FAT32 disk image
2. ✅ Can list root directory contents
3. ✅ Can read a file by path and return its contents
4. ✅ Boot log shows filesystem initialization message

---

## Current State Analysis

- **Block Driver:** `kernel/src/block.rs` provides `read_block()`/`write_block()` at 512-byte granularity
- **Test Disk:** `tinyos_disk.img` (17MB) exists but is raw data (not FAT32)
- **Heap:** Kernel has `linked_list_allocator`, so `alloc`-based crates work

---

## External Kernel Analysis

| Kernel | FS Approach | Copy-Paste Viable? |
|--------|-------------|-------------------|
| **Theseus** | Custom VFS traits (`FsNode`, `Directory`, `File`) + `memfs` | ❌ No — too abstract, no FAT32 |
| **Redox** | Scheme-based (`syscall/fs.rs`) | ❌ No — deeply integrated with Redox scheme system |
| **Tock** | Minimal embedded FS | ❌ No — different target |

**Conclusion:** Use `fatfs` crate — it's self-contained and `no_std` compatible.

---

## Codebase Reconnaissance

### Files to Create
- `kernel/src/fs.rs` — Filesystem module

### Files to Modify
- `kernel/Cargo.toml` — Add `fatfs` dependency
- `kernel/src/main.rs` — Call `fs::init()`

### Tests Impacted
- `tests/golden_boot.txt` — Add filesystem init message

---

## Constraints

- **Read-only first:** Write support adds complexity; defer
- **Single disk:** Only support one block device initially
- **Synchronous:** No async I/O (no runtime)

---

## Steps

### Step 1: Document Block Driver Interface
- **File:** `phase-1-step-1.md`
- **Goal:** Understand exact API of `block.rs`
- **UoW:** 1 (small, single file)

### Step 2: Research fatfs Crate Requirements
- **File:** `phase-1-step-2.md`  
- **Goal:** Document trait requirements for `fatfs`
- **UoW:** 1 (research only)

### Step 3: Prepare Test Disk Image
- **File:** `phase-1-step-3.md`
- **Goal:** Format `tinyos_disk.img` as FAT32 with test files
- **UoW:** 1 (shell commands)

→ **Next:** Phase 2 (Design)
