# TEAM_461: Kernel Data Structure Refactoring

## Objective

Continue the refactoring work from `docs/planning/kernel-refactor/REFACTOR_PLAN.md`:
- Phase 1.2: FD Table bitmap (already done by TEAM_459)
- Phase 1.1: VMA binary search optimization
- Phase 1.3: Mount table optimization
- Phase 2: Kernel config module

## Progress Log

### Session 1 (2026-01-12)
- Discovered FD table refactor was already completed by TEAM_459:
  - MAX_FDS increased from 64 to 1024
  - Bitmap-based allocation for O(1) lookups
  - next_free hint for fast path

- Implemented VMA binary search optimization in `mm/src/vma.rs`:
  - Added `search_insert_point()` - O(log n) binary search for insertion
  - Added `search_containing()` - O(log n) binary search for point queries
  - Updated `insert()` - now O(log n) for overlap check (was O(n))
  - Updated `find()` - now O(log n) (was O(n))
  - Updated `find_overlapping()` - now O(log n + k) where k = overlapping VMAs

- Fixed aarch64 build issue:
  - Added `#[cfg(target_arch = "x86_64")]` around `Mkdir` syscall dispatch
  - aarch64 doesn't have mkdir syscall (uses mkdirat directly)

- Added tests for binary search:
  - `test_binary_search_many_vmas` - 100 VMAs, verifies all can be found
  - `test_find_overlapping_many` - range queries with binary search
  - `test_insert_maintains_sorted` - verifies sorted order after random inserts
  - `test_overlap_detection_edge_cases` - adjacent and overlapping cases

## Key Decisions

1. **Binary search over interval tree**: Chose to optimize with binary search rather
   than a full red-black interval tree. The sorted Vec is already maintained, so binary
   search gives O(log n) for most operations with much simpler code. A true interval
   tree could be added later if needed for extreme workloads (thousands of VMAs).

2. **Adjacent VMA overlap check**: When inserting, only need to check adjacent VMAs
   (prev and next) since the list is sorted. This makes overlap detection O(1) after
   finding the insertion point.

## Files Modified

| File | Change |
|------|--------|
| `mm/src/vma.rs` | Binary search helpers, optimized insert/find/find_overlapping |
| `syscall/src/lib.rs` | Added cfg for x86_64-only Mkdir syscall |
| `levitate/src/config.rs` | **NEW** - Centralized kernel configuration module |
| `levitate/src/main.rs` | Added `pub mod config` |
| `levitate/src/init.rs` | Use config for fallback screen resolution |
| `levitate/src/input.rs` | Use config for fallback screen resolution |

## Remaining Work

- [ ] Mount table optimization (Phase 1.3) - deferred, low priority for small mount counts
- [ ] Make tmpfs limits dynamic (% of RAM)
- [ ] Per-CPU scheduler (Phase 3 - SMP prep)

## Handoff Notes

### VMA Optimization
VMA operations are now O(log n) instead of O(n). This is a significant improvement
for processes with many memory mappings. The implementation is straightforward
binary search on a sorted Vec, which is cache-friendly and simple to maintain.

### Config Module
Created `levitate/src/config.rs` with centralized constants:
- `display::FALLBACK_WIDTH/HEIGHT` - Screen resolution fallback (was mismatched: 1280x800 vs 1024x768)
- `tmpfs::MAX_FILE_SIZE/MAX_TOTAL_SIZE` - Tmpfs limits (16MB/64MB)
- `process::MAX_FDS/MAX_PROCESSES/MAX_VMAS` - Process limits
- `scheduler::DEFAULT_TIMESLICE_MS` - Scheduler tuning

### Mount Table
Deferred mount table optimization. Current linear scan is O(n) but:
- Typical systems have < 10 mounts
- Already sorted by length for correct longest-prefix matching
- Adding a radix trie would add complexity for minimal benefit

### Next Steps
1. Consider making tmpfs limits dynamic based on available RAM
2. For SMP support, implement per-CPU scheduler queues (Phase 3)
3. If mount count grows significantly, revisit trie implementation
