# Phase 1: Discovery - TODO Cleanup & Crate Audit

**Feature:** Address all known TODOs and audit for missing crates  
**Team:** TEAM_235  
**Status:** COMPLETE

---

## 1. Feature Summary

### Problem Statement
The LevitateOS codebase contains scattered TODOs representing incomplete functionality, and some components are hand-rolled when battle-tested crates exist. This creates:
- Technical debt from incomplete features
- Potential bugs from reimplementing solved problems
- Maintenance burden from custom implementations

### Who Benefits
- **Developers:** Cleaner codebase, less maintenance
- **Users:** More robust, feature-complete OS
- **Future teams:** Clear understanding of what's done vs. incomplete

---

## 2. Success Criteria

1. All TODOs are either:
   - Implemented
   - Converted to tracked issues
   - Documented as intentionally deferred
2. Hand-rolled implementations evaluated for crate replacement
3. Crate replacements implemented where beneficial
4. No regression in functionality or tests

---

## 3. Current State Analysis

### 3.1 Discovered TODOs

#### Kernel TODOs (High Priority)

| File | Line | TODO | Severity |
|------|------|------|----------|
| `kernel/src/syscall/mm.rs:128` | Unmap pages on mmap failure | HIGH |
| `kernel/src/syscall/mm.rs:141` | Free pages on map failure | HIGH |
| `kernel/src/syscall/mm.rs:177` | Implement VMA tracking for munmap | HIGH |
| `kernel/src/syscall/mm.rs:206` | Implement mprotect | MEDIUM |
| `kernel/src/memory/user.rs:273` | Use actual entropy for AT_RANDOM | MEDIUM |
| `kernel/src/memory/user.rs:308` | Pass actual HWCAP | LOW |
| `kernel/src/memory/user.rs:390` | Full page table teardown | HIGH |
| `kernel/src/task/thread.rs:127` | Share fd_table with CLONE_FILES | MEDIUM |
| `kernel/src/task/mod.rs:291` | Inherit CWD from parent | LOW |
| `kernel/src/fs/vfs/dispatch.rs:284` | Proper permission checking | MEDIUM |
| `kernel/src/fs/vfs/inode.rs:159-166` | Real timestamps from clock | LOW |

#### Userspace TODOs (Lower Priority)

| File | Line | TODO | Severity |
|------|------|------|----------|
| `userspace/ulib/src/entry.rs:90` | Call _init if defined | LOW |
| `userspace/levbox/src/bin/mkdir.rs:42` | Support -p (parents) | LOW |
| `userspace/levbox/src/bin/rm.rs:91` | Handle -r recursive | LOW |
| `userspace/levbox/src/bin/cp.rs:92` | Write to destination | MEDIUM |

### 3.2 Current Crate Usage

**Already using well-chosen crates:**
- `spin` - Mutex/RwLock/Once/Lazy (good choice for no_std)
- `hashbrown` - HashMap/HashSet (good choice for no_std)
- `linked_list_allocator` - Kernel heap (proven, minimal)
- `fdt` - Device tree parsing (specialized, correct)
- `virtio-drivers` - VirtIO support (well-maintained)
- `embedded-graphics` - Display rendering (industry standard)
- `bitflags` - Flag types (ubiquitous)
- `aarch64-cpu` - CPU operations (arch-specific, appropriate)
- `intrusive-collections` - (available but using custom IntrusiveList)
- `embedded-sdmmc` - FAT filesystem
- `ext4-view` - ext4 filesystem

### 3.3 Hand-Rolled Implementations

| Component | Location | Potential Crate |
|-----------|----------|-----------------|
| ELF Parser | `kernel/src/loader/elf.rs` | `goblin`, `elf`, `xmas-elf` |
| CPIO Parser | `crates/utils/src/cpio.rs` | `cpio` (limited options) |
| Ring Buffer | `crates/utils/src/lib.rs` | `heapless::spsc::Queue` |
| Buddy Allocator | `crates/hal/src/allocator/buddy.rs` | Keep (well-tested, specialized) |
| Intrusive List | `crates/hal/src/allocator/intrusive_list.rs` | `intrusive-collections` (already dep) |

---

## 4. Codebase Reconnaissance

### 4.1 Code Areas Affected

**Memory Management:**
- `kernel/src/memory/user.rs` - Page table teardown
- `kernel/src/syscall/mm.rs` - mmap/munmap/mprotect

**Process Management:**
- `kernel/src/task/mod.rs` - CWD inheritance
- `kernel/src/task/thread.rs` - fd_table sharing

**Filesystem:**
- `kernel/src/fs/vfs/` - Permissions, timestamps

**Loaders:**
- `kernel/src/loader/elf.rs` - ELF parsing (crate candidate)

**Utilities:**
- `crates/utils/src/cpio.rs` - CPIO parsing (evaluate)
- `crates/hal/src/allocator/` - Keep custom (well-tested)

### 4.2 Tests That May Be Impacted

- Golden boot tests (`tests/golden_boot.txt`)
- Buddy allocator unit tests
- CPIO parser unit tests
- ELF parser unit tests (if replaced)

### 4.3 Non-Obvious Constraints

1. **no_std requirement:** All crates must work without std
2. **AArch64 target:** Some crates may not support this arch
3. **Binary size:** Embedded target, minimize bloat
4. **Stability:** Prefer well-maintained crates

---

## 5. Constraints

### 5.1 Technical Constraints
- Must remain `#![no_std]` compatible
- AArch64 target support required
- Minimal binary size preferred
- No breaking changes to existing APIs

### 5.2 Quality Constraints
- All existing tests must pass
- No behavioral regressions
- Maintain code traceability (TEAM_XXX comments)

---

## 6. Recommendations

### 6.1 TODOs to Implement (Phase 3)

**Must Have:**
1. Page table teardown (`destroy_user_page_table`)
2. mmap failure cleanup
3. VMA tracking for munmap

**Should Have:**
4. Real entropy for AT_RANDOM
5. fd_table sharing (CLONE_FILES)
6. Permission checking

**Nice to Have:**
7. Real timestamps
8. HWCAP detection
9. Levbox utility completions

### 6.2 Crate Replacements to Evaluate (Phase 2)

| Component | Recommendation |
|-----------|----------------|
| ELF Parser | **Evaluate `goblin`** - widely used, no_std support |
| CPIO Parser | **Keep custom** - simple, well-tested, few alternatives |
| Ring Buffer | **Keep custom** - simple, adequate |
| Intrusive List | **Migrate to `intrusive-collections`** - already a dependency |
| Buddy Allocator | **Keep custom** - specialized, well-tested |

---

## 7. Next Steps

Proceed to **Phase 2: Design** to:
1. Answer questions about TODO implementations
2. Evaluate crate candidates with concrete API comparisons
3. Design VMA tracking system
4. Define page table teardown approach
