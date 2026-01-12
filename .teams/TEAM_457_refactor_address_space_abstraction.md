# TEAM_457: Refactor Address Space Abstraction

## Objective

Create a unified `AddressSpace` abstraction that encapsulates all process memory state, preventing the synchronization bugs discovered in TEAM_455 and TEAM_456.

## Problem Statement

The last two major bugfixes (TEAM_455: fork VMA fix, TEAM_456: execve ttbr0 fix) both stem from the same root cause: **address space state is fragmented across multiple locations with no single owner**.

### Current Architecture (Problematic)

```
┌─────────────────────────────────────────────────────────────────┐
│                     TaskControlBlock                             │
├─────────────────────────────────────────────────────────────────┤
│ ttbr0: AtomicUsize          ← Physical page table address       │
│ vmas: IrqSafeLock<VmaList>  ← VMA tracking (separate!)          │
│ heap: IrqSafeLock<ProcessHeap>  ← Heap state (separate!)        │
│ tls: AtomicUsize            ← TLS base (separate!)              │
│ ...                                                              │
└─────────────────────────────────────────────────────────────────┘
        ↓                      ↓                    ↓
    ┌────────┐          ┌──────────┐          ┌─────────┐
    │ CR3/   │          │ VMA List │          │ Heap    │
    │ TTBR0  │          │ (Vec)    │          │ State   │
    │ Reg    │          │          │          │         │
    └────────┘          └──────────┘          └─────────┘
         ↑
    Can get out of sync!
```

### Bugs Caused by This Design

1. **TEAM_455 (Fork VMA bug)**: ELF loader mapped pages to page table but didn't add VMAs. Fork iterated over empty VMA list → copied no pages → child crashed.

2. **TEAM_456 (execve ttbr0 bug)**: execve switched CR3 to new page table but didn't update `task.ttbr0`. Mmap scanned OLD page table → returned wrong addresses → page fault.

Both bugs share the pattern: **one component was updated, but related state wasn't**.

### Current Code Locations

| State | Location | Used By |
|-------|----------|---------|
| Page table (ttbr0) | `TCB.ttbr0: AtomicUsize` | switch_to, fork, execve, mmap |
| VMA list | `TCB.vmas: IrqSafeLock<VmaList>` | fork, mmap, munmap, mprotect |
| Heap state | `TCB.heap: IrqSafeLock<ProcessHeap>` | brk syscall |
| TLS base | `TCB.tls: AtomicUsize` | context switch, clone |
| Page table ops | `los_mm::user::*` functions | scattered |

## Proposed Solution

### Create `AddressSpace` Struct

```rust
/// A process's complete virtual address space.
/// All memory state in one place - impossible to have inconsistent state.
pub struct AddressSpace {
    /// Physical address of L0/PML4 page table
    ttbr0: usize,
    /// Tracked virtual memory areas
    vmas: VmaList,
    /// Heap state (brk)
    heap: ProcessHeap,
    /// TLS base address
    tls: usize,
}

impl AddressSpace {
    /// Map pages AND track VMA in one atomic operation.
    pub fn map(&mut self, vaddr: usize, size: usize, flags: VmaFlags) -> Result<(), MmError>;

    /// Unmap pages AND remove VMA in one atomic operation.
    pub fn unmap(&mut self, vaddr: usize, size: usize) -> Result<(), MmError>;

    /// Deep copy entire address space (for fork).
    pub fn fork(&self) -> Result<AddressSpace, MmError>;

    /// Replace address space (for execve).
    /// Returns old address space for cleanup.
    pub fn replace(&mut self, new: AddressSpace) -> AddressSpace;

    /// Get page table physical address for CR3/TTBR0 switch.
    pub fn page_table(&self) -> usize { self.ttbr0 }
}
```

### Update TCB to Use AddressSpace

```rust
pub struct TaskControlBlock {
    // ... other fields ...

    /// TEAM_457: Unified address space abstraction
    /// All memory state in one place - always consistent.
    pub address_space: IrqSafeLock<AddressSpace>,

    // REMOVED:
    // pub ttbr0: AtomicUsize,       // → address_space.ttbr0
    // pub vmas: IrqSafeLock<VmaList>,  // → address_space.vmas
    // pub heap: IrqSafeLock<ProcessHeap>, // → address_space.heap
    // pub tls: AtomicUsize,         // → address_space.tls
}
```

### Migration Strategy

**Phase 1**: Create AddressSpace struct in `los_mm` crate
- New file: `crates/kernel/mm/src/address_space.rs`
- Define struct with ttbr0, vmas, heap, tls
- Implement map/unmap/fork methods that keep everything in sync
- Add comprehensive tests

**Phase 2**: Add AddressSpace to TCB alongside old fields
- `TCB.address_space: IrqSafeLock<AddressSpace>`
- Keep old fields temporarily for compatibility
- New code uses AddressSpace, old code continues working

**Phase 3**: Migrate syscalls to use AddressSpace
- `sys_mmap` → `address_space.map()`
- `sys_munmap` → `address_space.unmap()`
- `sys_brk` → `address_space.brk()`
- `sys_mprotect` → `address_space.protect()`
- Fork → `address_space.fork()`
- Execve → `address_space.replace()`

**Phase 4**: Remove old fields from TCB
- Remove `ttbr0`, `vmas`, `heap`, `tls` from TCB
- All access goes through AddressSpace

**Phase 5**: Add invariant checks
- Debug assertions that page table and VMA list are consistent
- Optionally: verify all mapped pages have corresponding VMAs

## Key Invariants to Enforce

1. **Map invariant**: Every mapped page has a corresponding VMA
2. **Unmap invariant**: Unmapping a region removes both pages AND VMA
3. **Fork invariant**: Child gets complete copy of parent's AddressSpace
4. **Exec invariant**: execve atomically replaces entire AddressSpace
5. **CR3/TTBR0 invariant**: Hardware register always matches address_space.ttbr0

## Files to Modify

| File | Change |
|------|--------|
| `mm/src/address_space.rs` | **NEW** - AddressSpace struct and methods |
| `mm/src/lib.rs` | Export AddressSpace |
| `sched/src/lib.rs` | Add address_space field to TCB |
| `sched/src/fork.rs` | Use AddressSpace::fork() |
| `sched/src/user.rs` | Use AddressSpace in UserTask |
| `syscall/src/mm.rs` | Use AddressSpace for mmap/munmap/brk |
| `syscall/src/process/lifecycle.rs` | Use AddressSpace for execve |
| `loader/elf.rs` | Return AddressSpace instead of separate fields |

## Rollback Strategy

Each phase is one commit. If issues are found:
1. Phase 2 can be reverted without affecting old code (AddressSpace is just added)
2. Phases 3-4 can be reverted if syscalls break
3. All tests must pass before each merge

## Success Criteria

- [ ] No more "forgot to update X when changing Y" bugs
- [ ] AddressSpace owns all memory state
- [ ] Single point of truth for page table address
- [ ] Fork/exec use AddressSpace methods
- [ ] All tests pass on both x86_64 and aarch64

## Related Work

- TEAM_422: Kernel architecture redesign (this is a specific piece)
- TEAM_455: Fork VMA bug (root cause addressed here)
- TEAM_456: execve ttbr0 bug (root cause addressed here)

## Progress Log

### Session 1 (2026-01-12)

- Analyzed TEAM_455 and TEAM_456 bugfixes
- Identified root cause: fragmented address space state
- Designed AddressSpace abstraction
- Created this planning document

## Remaining Work

- [ ] Phase 1: Create AddressSpace struct
- [ ] Phase 2: Add to TCB alongside old fields
- [ ] Phase 3: Migrate syscalls
- [ ] Phase 4: Remove old fields
- [ ] Phase 5: Add invariant checks

## Handoff Notes

The core insight is that `ttbr0`, `vmas`, `heap`, and `tls` are all part of the same logical entity (a process's address space) but are stored separately with no enforced relationship. The solution is to bundle them into one struct with methods that maintain consistency.

Start with Phase 1 - creating the AddressSpace struct with proper encapsulation. This is the foundation for everything else.
