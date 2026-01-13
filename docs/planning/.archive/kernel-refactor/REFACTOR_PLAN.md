# Kernel Refactor Plan

**TEAM_459 Audit Results** - January 2026

This document captures all hardcoded values and scalability issues that need refactoring.

## Overview

The kernel has accumulated technical debt from rapid feature development. This refactor focuses on:
1. **Hardcoded values** that should be configurable or dynamic
2. **Data structures** that don't scale beyond toy workloads
3. **Architectural patterns** that block multi-core support

## Priority Tiers

| Tier | Impact | When to Fix |
|------|--------|-------------|
| **P0** | Blocks >50 processes or multi-core | Must fix before production |
| **P1** | Performance degrades noticeably | Fix in next major release |
| **P2** | Technical debt, minor impact | Fix opportunistically |

---

## Phase 1: Critical Data Structure Fixes (P0)

These O(n) scans become O(n²) in aggregate and will crater performance.

### 1.1 VMA List → Interval Tree

**Location**: `crates/kernel/mm/src/vma.rs:99-105`

**Current Code**:
```rust
// O(n) scan on EVERY mmap, brk, munmap
for existing in &self.vmas {
    if existing.overlaps(vma.start, vma.end) {
        return Err(VmaError::Overlapping);
    }
}
```

**Problem**:
- Every `mmap()` call scans all existing VMAs
- A process with 1000 VMAs (common for large apps) = 1000 comparisons per mmap
- Aggregate: O(n²) to build address space

**Solution**: Interval tree (red-black tree keyed by address range)
- Insert: O(log n)
- Overlap check: O(log n + k) where k = overlapping intervals
- Range lookup: O(log n)

**Reference**: Linux uses `struct rb_root_cached` for VMAs

**Files to Modify**:
- `mm/src/vma.rs` - Replace `Vec<Vma>` with interval tree
- Add `los_utils/src/interval_tree.rs` or use existing crate

---

### 1.2 FD Table → Bitmap + Vec

**Location**: `crates/kernel/sched/src/fd_table.rs:142-159`

**Current Code**:
```rust
const MAX_FDS: usize = 64;  // Way too low!

pub fn alloc(&mut self, fd_type: FdType) -> Option<usize> {
    for (i, slot) in self.entries.iter_mut().enumerate() {
        if slot.is_none() {  // O(n) scan for free slot
            *slot = Some(FdEntry { fd_type, flags: 0 });
            return Some(i);
        }
    }
    None
}
```

**Problems**:
1. `MAX_FDS = 64` is absurdly low (Linux default: 1024, can go to 1M)
2. O(n) scan to find free FD on every `open()`
3. Fixed array size wastes memory for simple processes

**Solution**: Bitmap-based allocation
```rust
struct FdTable {
    bitmap: BitVec,           // Track free/used slots
    entries: Vec<Option<FdEntry>>,
    next_free: usize,         // Hint for next free slot
}
```
- Alloc: O(1) amortized with `next_free` hint
- Free: O(1)
- Dynamic growth when needed

**Files to Modify**:
- `sched/src/fd_table.rs` - Rewrite allocation strategy
- Update `MAX_FDS` to 1024 (immediate) or dynamic (full fix)

---

### 1.3 Mount Table → Radix Trie

**Location**: `crates/kernel/vfs/src/mount.rs:198-200`

**Current Code**:
```rust
pub fn lookup<'a>(&'a self, path: &'a str) -> Option<(&'a Mount, &'a str)> {
    for mount in &self.mounts {  // O(n) on EVERY path operation
        if path.starts_with(&mount.mountpoint) {
            ...
        }
    }
}
```

**Problem**:
- Every `open()`, `stat()`, `readdir()` scans all mount points
- With 20 mounts, that's 20 string comparisons per syscall
- Gets worse with bind mounts, overlay filesystems

**Solution**: Radix trie (path-component trie)
- Insert mount at `/foo/bar`: Create nodes for `/`, `foo`, `bar`
- Lookup: Walk trie by path components, O(path_depth)
- Most lookups: O(3-5) regardless of mount count

**Files to Modify**:
- `vfs/src/mount.rs` - Replace `Vec<Mount>` with trie
- Add `los_utils/src/path_trie.rs`

---

### 1.4 Epoll Interest List → Hash Map

**Location**: `crates/kernel/sched/src/epoll.rs` (estimated)

**Problem**: Epoll with O(n) scan defeats its entire purpose.

**Current** (suspected):
```rust
// On epoll_wait, scan all registered FDs
for entry in &self.interest_list {
    if entry.events_ready() {
        results.push(entry);
    }
}
```

**Solution**:
- Hash map: FD → interest entry
- Ready list: Only FDs with pending events
- Callback-based: When FD becomes ready, add to ready list

**Files to Modify**:
- `sched/src/epoll.rs` - Restructure around ready list

---

## Phase 2: Hardcoded Values → Config (P1)

### 2.1 Create Kernel Config Module

**New File**: `crates/kernel/config/src/lib.rs`

```rust
//! Kernel configuration constants
//!
//! All tunables in one place. Future: runtime config via /proc/sys

/// Maximum file descriptors per process (Linux default: 1024)
pub const MAX_FDS_PER_PROCESS: usize = 1024;

/// Maximum processes system-wide
pub const MAX_PROCESSES: usize = 4096;

/// Tmpfs limits (percentage of total RAM)
pub const TMPFS_MAX_SIZE_PERCENT: u8 = 50;
pub const TMPFS_MAX_FILE_PERCENT: u8 = 25;

/// VMA limits
pub const MAX_VMAS_PER_PROCESS: usize = 65535;

/// Screen defaults
pub const DEFAULT_SCREEN_WIDTH: u32 = 1280;
pub const DEFAULT_SCREEN_HEIGHT: u32 = 800;

/// Scheduler
pub const DEFAULT_TIMESLICE_MS: u64 = 10;
```

---

### 2.2 Hardcoded Values Inventory

| Value | Location | Current | Should Be |
|-------|----------|---------|-----------|
| `MAX_FDS` | `sched/src/fd_table.rs:18` | 64 | 1024+ |
| `MAX_FILE_SIZE` | `fs/tmpfs/src/node.rs:15` | 16MB | % of RAM |
| `MAX_TOTAL_SIZE` | `fs/tmpfs/src/node.rs:16` | 64MB | % of RAM |
| Screen fallback | `levitate/src/init.rs:341-342` | 1280x800 | config const |
| Screen fallback | `levitate/src/input.rs:128` | 1024x768 | **mismatch!** |
| `MAX_REGIONS` | `mm/src/vma.rs` (if exists) | 32/64 | 65535+ |
| Timer frequency | various | hardcoded | config const |
| Stack size | `loader/src/` | hardcoded | config const |
| Heap initial size | `mm/src/` | hardcoded | config const |

---

### 2.3 Specific Fixes Needed

#### Screen Resolution Mismatch

**Bug**: Two different fallback resolutions in codebase

```rust
// init.rs:341-342
const FALLBACK_WIDTH: u32 = 1280;
const FALLBACK_HEIGHT: u32 = 800;

// input.rs:128
const FALLBACK_WIDTH: u32 = 1024;  // Different!
const FALLBACK_HEIGHT: u32 = 768;
```

**Fix**: Use single source from config module

#### Tmpfs Limits

**Current**: Fixed 16MB/64MB regardless of system RAM

**Fix**: Calculate from available memory:
```rust
let total_ram = memory::total_pages() * PAGE_SIZE;
let max_tmpfs = total_ram * config::TMPFS_MAX_SIZE_PERCENT / 100;
```

---

## Phase 3: Multi-Core Scalability (P0 for SMP)

### 3.1 Single Global Scheduler Lock

**Location**: `crates/kernel/sched/src/scheduler.rs:7-11`

**Current Code**:
```rust
pub struct Scheduler {
    pub ready_list: IrqSafeLock<VecDeque<Arc<TaskControlBlock>>>,
}
pub static SCHEDULER: Scheduler = Scheduler::new();
```

**Problem**: Single lock serializes ALL scheduling across ALL cores

**Solution**: Per-CPU run queues
```rust
pub struct PerCpuScheduler {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    current_task: Option<Arc<TaskControlBlock>>,
}

// Each CPU has its own scheduler state
static PER_CPU_SCHEDULERS: PerCpu<SpinLock<PerCpuScheduler>>;
```

**Benefits**:
- No lock contention on same-CPU scheduling
- Load balancing via work stealing
- Affinity support

**Files to Modify**:
- `sched/src/scheduler.rs` - Per-CPU queues
- `sched/src/lib.rs` - CPU-local task selection
- `hal/src/` - Per-CPU storage primitives

---

### 3.2 Process Table Linear Scan

**Location**: `sched/src/` (waitpid implementation)

**Problem**: `waitpid(-1)` scans ALL processes looking for zombie children

**Solution**:
- Per-process child list
- Zombie notification via parent pointer
- O(1) to check if children exist

---

### 3.3 Global Locks Inventory

| Lock | Location | Contention Risk | Fix |
|------|----------|-----------------|-----|
| Scheduler | `sched/scheduler.rs` | **CRITICAL** | Per-CPU queues |
| Mount table | `vfs/mount.rs` | High | RCU or per-mount lock |
| Dentry cache | `vfs/dentry.rs` | High | Sharded hash table |
| TTY | `fs/tty/` | Medium | Per-TTY lock (already?) |
| Memory allocator | `mm/` | High | Per-CPU page cache |

---

## Phase 4: Code Quality Cleanup (P2)

### 4.1 Magic Numbers to Constants

Search pattern: `\b\d{3,}\b` (3+ digit numbers in code)

Known issues:
- `0o755`, `0o644` permissions scattered (define `MODE_DIR_DEFAULT`, `MODE_FILE_DEFAULT`)
- Page sizes sometimes `4096`, sometimes `PAGE_SIZE`
- Signal numbers sometimes numeric, sometimes `SIGKILL` etc.

### 4.2 Duplicate Code

- Path parsing logic duplicated across VFS functions
- Error mapping (`VfsError` → errno) duplicated in each syscall
- Buffer validation code repeated in every syscall

### 4.3 Dead Code Removal

Run: `cargo +nightly udeps` to find unused dependencies
Run: `cargo clippy -- -W dead_code` to find unused functions

---

## Implementation Order

```
Week 1: Data Structures (highest impact)
├── 1.2 FD Table bitmap (quick win, immediate benefit)
├── 1.1 VMA interval tree (complex but critical)
└── 1.3 Mount table trie (medium complexity)

Week 2: Configuration
├── 2.1 Create config module
├── 2.2 Migrate hardcoded values
└── 2.3 Fix screen resolution mismatch

Week 3: Multi-core prep (if targeting SMP)
├── 3.1 Per-CPU scheduler (major change)
├── 3.2 Process table optimization
└── 3.3 Lock audit and fixes

Ongoing: Code quality
├── 4.1 Magic numbers cleanup
├── 4.2 Dedup common patterns
└── 4.3 Dead code removal
```

---

## Testing Strategy

### Before Starting
```bash
cargo xtask test           # Baseline: all tests pass
cargo xtask build all      # Both architectures build
```

### After Each Change
1. Unit tests for modified module
2. Behavior tests (boot + shell interaction)
3. Stress tests for scalability changes:
   - FD table: Open 1000 files
   - VMA: Map 1000 regions
   - Scheduler: Fork 100 processes

### Performance Benchmarks (future)
- `mmap` latency with varying VMA count
- `open` latency with varying FD count
- Context switch latency
- Path lookup latency with varying mount count

---

## References

- Linux VMA management: `mm/mmap.c`, uses `rb_root` interval tree
- Linux FD table: `fs/file.c`, bitmap + RCU
- Linux scheduler: `kernel/sched/`, per-CPU run queues
- Linux mount table: `fs/namespace.c`, hash table by path

---

## Change Log

| Date | Author | Change |
|------|--------|--------|
| 2026-01-12 | TEAM_459 | Initial audit and plan |
