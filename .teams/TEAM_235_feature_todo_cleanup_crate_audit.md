# TEAM_235: Feature Plan - TODO Cleanup & Crate Audit

## Status: AWAITING USER INPUT

## Summary

Created comprehensive feature plan to:
1. Address all known TODOs in the codebase
2. Audit hand-rolled implementations for potential crate replacements

## Planning Location

`docs/planning/todo-cleanup-crate-audit/`

## Progress

- [x] Registered as TEAM_235
- [x] Discovered all TODOs in kernel, crates, and userspace
- [x] Analyzed current crate dependencies
- [x] Identified hand-rolled implementations
- [x] Phase 1 - Discovery document
- [x] Phase 2 - Design document with questions

## Discovered TODOs (11 kernel, 4 userspace)

### HIGH Priority (Memory Safety)
1. `destroy_user_page_table` - leaks pages
2. mmap failure cleanup - leaks on partial failure
3. VMA tracking for munmap - stub implementation

### MEDIUM Priority
4. fd_table sharing (CLONE_FILES)
5. Real entropy for AT_RANDOM
6. mprotect implementation
7. Permission checking in VFS

### LOW Priority
8. Real timestamps, HWCAP, CWD inheritance, etc.

## Crate Audit Results

| Component | Recommendation |
|-----------|----------------|
| ELF Parser | Keep custom (simple, focused) |
| CPIO Parser | Keep custom (adequate, few alternatives) |
| Ring Buffer | Keep custom (trivial) |
| Intrusive List | Consider migration to `intrusive-collections` |
| Buddy Allocator | Keep custom (specialized, well-tested) |

## Open Questions (6 total in phase-2.md)

User must answer Q1-Q6 before Phase 3 can begin.

## Handoff

Phase 1 & 2 complete. Awaiting user answers to proceed to Phase 3 (Implementation).
