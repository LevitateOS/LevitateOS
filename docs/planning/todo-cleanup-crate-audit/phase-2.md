# Phase 2: Design - TODO Cleanup & Crate Audit

**Feature:** Address all known TODOs and audit for missing crates  
**Team:** TEAM_235  
**Status:** DRAFT - AWAITING ANSWERS

---

## 1. Proposed Solution Overview

This feature is split into two tracks:

### Track A: TODO Implementation
Implement missing functionality identified in Phase 1, prioritized by severity.

### Track B: Crate Audit
Evaluate and potentially replace hand-rolled implementations with established crates.

---

## 2. Track A: TODO Implementation Design

### 2.1 Memory Management TODOs (HIGH Priority)

#### A1: Page Table Teardown (`destroy_user_page_table`)

**Current State:** Function exists but is a no-op that leaks pages.

**Proposed Design:**
```rust
pub unsafe fn destroy_user_page_table(ttbr0_phys: usize) -> Result<(), MmuError> {
    // 1. Walk L0 table
    // 2. For each valid L1 entry:
    //    - If table: recurse to L2
    //    - If block: free block
    // 3. For each valid L2 entry:
    //    - If table: recurse to L3
    //    - If block: free block
    // 4. For each valid L3 entry:
    //    - Free page
    // 5. Free intermediate tables bottom-up
    // 6. Free L0 table itself
}
```

#### A2: VMA Tracking for munmap

**Current State:** No Virtual Memory Area tracking - munmap is a stub.

**Proposed Design Options:**

**Option 1: Simple List**
```rust
struct Vma {
    start: usize,
    end: usize,
    flags: VmaFlags,
    // No file backing for MVP
}
struct VmaList(Vec<Vma>);
```

**Option 2: Interval Tree (like Linux)**
More complex but O(log n) lookups.

**Option 3: Bitmap**
Simple but wastes memory for sparse mappings.

#### A3: mmap Failure Cleanup

**Current State:** Partial allocations leak on failure.

**Proposed Design:**
- Track allocated pages during mmap
- On failure, iterate and free all allocated pages
- Use RAII guard pattern for automatic cleanup

### 2.2 Process/Thread TODOs (MEDIUM Priority)

#### A4: fd_table Sharing (CLONE_FILES)

**Current State:** Threads always get separate fd tables.

**Proposed Design:**
- `Arc<Mutex<FdTable>>` already used
- Add flag check in `clone_thread()`
- If CLONE_FILES set: share parent's Arc
- If not: clone the table contents

#### A5: Real Entropy for AT_RANDOM

**Current State:** Hardcoded pattern `(i * 7) as u8`.

**Proposed Design Options:**
1. Use CPU cycle counter as entropy source
2. Add VirtIO-RNG driver
3. Use timer jitter (simple but low quality)

### 2.3 Filesystem TODOs (MEDIUM Priority)

#### A6: Permission Checking

**Current State:** Stub that only checks existence.

**Proposed Design:**
- Check inode uid/gid against process credentials
- Implement mode bit checking (rwx for owner/group/other)
- Support CAP_DAC_OVERRIDE equivalent for root

---

## 3. Track B: Crate Evaluation Design

### 3.1 ELF Parser Evaluation

**Current Implementation:** `kernel/src/loader/elf.rs` (~500 lines)
- Hand-rolled Elf64Header and Elf64ProgramHeader parsing
- Only supports ET_EXEC, AArch64, little-endian
- No section header parsing

**Candidate Crates:**

| Crate | no_std | Size | Features |
|-------|--------|------|----------|
| `goblin` | Yes (feature) | Large | Full ELF/Mach-O/PE support |
| `elf` | Yes | Medium | Pure ELF, good API |
| `xmas-elf` | Yes | Small | Minimal, read-only |

**Recommendation:** Keep custom OR use `xmas-elf`
- Our parser is simple and focused
- `goblin` adds unnecessary bloat
- `xmas-elf` is a reasonable alternative if we need more features

### 3.2 Intrusive List Migration

**Current Implementation:** `crates/hal/src/allocator/intrusive_list.rs` (~300 lines)

**Already Have Dependency:** `intrusive-collections = "0.10"`

**Recommendation:** Migrate to `intrusive-collections`
- Already a dependency (no size increase)
- More battle-tested
- Supports multiple list types

### 3.3 Ring Buffer Evaluation

**Current Implementation:** `crates/utils/src/lib.rs` (~70 lines)

**Candidate:** `heapless::spsc::Queue`

**Recommendation:** Keep custom
- Our implementation is trivial and well-tested
- Adding `heapless` for just a ring buffer is overkill

---

## 4. Open Questions (REQUIRES USER INPUT)

### Q1: VMA Tracking Complexity
**Question:** For munmap support, which VMA tracking approach should we use?
- **A)** Simple Vec<Vma> - O(n) operations, simple implementation
- **B)** Interval tree - O(log n) operations, complex implementation
- **C)** Defer munmap support entirely (keep stub)

**Recommendation:** Option A for MVP, can upgrade later.

---

### Q2: ELF Parser Replacement
**Question:** Should we replace the hand-rolled ELF parser?
- **A)** Keep custom - it works, is simple, and focused
- **B)** Replace with `xmas-elf` - more features, maintained
- **C)** Replace with `goblin` - full-featured but larger

**Recommendation:** Option A (keep custom) unless we need more ELF features.

---

### Q3: Entropy Source for AT_RANDOM
**Question:** What entropy source should we use for AT_RANDOM?
- **A)** CPU cycle counter (simple, available, moderate entropy)
- **B)** VirtIO-RNG driver (proper entropy, requires new driver)
- **C)** Timer jitter (simple, low quality)
- **D)** Keep current stub (security risk for programs needing randomness)

**Recommendation:** Option A for MVP, upgrade to B later.

---

### Q4: Intrusive List Migration Priority
**Question:** Should we migrate the buddy allocator's intrusive list to `intrusive-collections`?
- **A)** Yes, now - cleaner, already a dependency
- **B)** No - current implementation works, don't touch working allocator
- **C)** Later - add to backlog

**Recommendation:** Option B - the allocator is critical and well-tested.

---

### Q5: Permission Checking Scope
**Question:** For filesystem permission checking, what scope?
- **A)** Basic mode bits only (rwx checks)
- **B)** Full POSIX (uid/gid/mode + sticky bit)
- **C)** Linux-style (POSIX + capabilities)
- **D)** Defer - keep stub

**Recommendation:** Option A for MVP.

---

### Q6: TODO Prioritization
**Question:** Should we implement ALL TODOs or prioritize?
- **A)** Implement all (comprehensive but large scope)
- **B)** HIGH priority only (memory safety issues)
- **C)** HIGH + MEDIUM (functional completeness)
- **D)** Create tracking issues and implement incrementally

**Recommendation:** Option D - create issues, implement HIGH priority now.

---

## 5. Design Alternatives Considered

### VMA Tracking
- **Rejected: Bitmap** - Wastes memory, doesn't scale with sparse mappings
- **Rejected: B-tree** - Over-engineered for current needs

### ELF Parser
- **Rejected: goblin** - Too large for our needs
- **Considered: elf crate** - Reasonable but adds dependency for little gain

### Buddy Allocator
- **Rejected: External crate** - Our implementation is specialized and well-tested

---

## 6. Implementation Phases (Post-Questions)

Once questions are answered:

### Phase 3: Implementation
- Step 1: Create tracking issues for all TODOs
- Step 2: Implement HIGH priority memory TODOs
- Step 3: Implement MEDIUM priority TODOs (based on Q6 answer)
- Step 4: Crate migrations (based on Q2, Q4 answers)

### Phase 4: Integration & Testing
- Verify no regressions
- Update golden tests if needed
- Performance testing for memory changes

### Phase 5: Documentation
- Update ARCHITECTURE.md with VMA design
- Document entropy source
- Update crate dependency rationale

---

## 7. Awaiting User Input

**Please answer questions Q1-Q6 above to proceed with Phase 3 planning.**

Key decisions needed:
1. VMA complexity level
2. ELF parser decision
3. Entropy source
4. Intrusive list migration
5. Permission checking scope
6. TODO prioritization strategy
