# Rust `std` Support Implementation Plan

**Team**: TEAM_222  
**Created**: 2026-01-07  
**Status**: Planning Complete

## Overview

This plan implements full Rust `std` compatibility for LevitateOS through 7 phases with 35 Units of Work (UoWs), each sized for a single SLM session.

## Phase Summary

| Phase | Name | Priority | UoWs | Status |
|-------|------|----------|------|--------|
| 1 | Discovery and Safeguards | — | 6 | Not Started |
| 2 | Auxv Verification | P0 | 1 | **COMPLETE** (TEAM_217) |
| 3 | mmap/munmap/mprotect | P0 | 9 | **IN PROGRESS** (TEAM_228) |
| 4 | Threading (clone, TLS) | P1 | 10 | **IN PROGRESS** (TEAM_228) |
| 5 | I/O (writev/readv kernel) | P1 | 0 | **COMPLETE** (TEAM_217) |
| 6 | Process Orchestration | P2 | 9 | Not Started |
| 7 | Cleanup and Validation | — | 9 | Not Started |

**Total UoWs**: ~44 (reduced from original due to existing implementations)

> **Note (TEAM_228)**: Phase 2 auxv and Phase 5 writev/readv already implemented by TEAM_217.


---

## Execution Order

### Critical Path (P0)
```
Phase 1 → Phase 2 → Phase 3
```
After Phase 3, a simple std binary can start and allocate memory.

### Threading Path (P1)
```
Phase 3 → Phase 4
```
After Phase 4, `std::thread` works.

### I/O Path (P1)
```
Phase 1 → Phase 5
```
Can be done in parallel with P0 work.

### Process Path (P2)
```
Phase 3 → Phase 6
```
Requires mmap for pipe buffers.

### Finalization
```
All phases → Phase 7
```

---

## UoW Index

### Phase 1: Discovery and Safeguards
| UoW | Name | Est. Time |
|-----|------|-----------|
| 1.1.1 | Inventory Existing Syscalls | 30 min |
| 1.1.2 | Map libsyscall Wrappers | 20 min |
| 1.2.1 | Run and Document Existing Tests | 30 min |
| 1.2.2 | Identify Coverage Gaps | 20 min |
| 1.3.1 | Map Memory Layout | 30 min |
| 1.3.2 | Map Thread/Process Context | 30 min |

### Phase 2: Auxv Verification (P0) — COMPLETE
| UoW | Name | Est. Time |
|-----|------|-----------|
| 2.1 | Verify Auxv with std Binary | 30 min |

> **Note (TEAM_228)**: All auxv implementation UoWs removed — already done by TEAM_217.


### Phase 3: mmap/munmap/mprotect (P0)
| UoW | Name | Est. Time |
|-----|------|-----------|
| 3.1.1 | Audit Current VM State | 30 min |
| 3.1.2 | Add VMA Tracking | 45 min |
| 3.2.1 | Add mmap Constants | 15 min |
| 3.2.2 | Implement Anonymous mmap | 60 min |
| 3.2.3 | Add mmap to Syscall Dispatch | 15 min |
| 3.3.1 | Implement munmap | 45 min |
| 3.4.1 | Implement mprotect | 30 min |
| 3.5.1 | Add mmap/munmap to libsyscall | 15 min |
| 3.5.2 | Add mmap Test Program | 30 min |

### Phase 4: Threading (P1)
| UoW | Name | Est. Time |
|-----|------|-----------|
| 4.1.1 | Add TPIDR_EL0 to Thread Context | 45 min |
| 4.1.2 | Add arch_prctl or Equivalent | 30 min |
| 4.2.1 | Define Clone Flags | 15 min |
| 4.2.2 | Implement Basic sys_clone | 60 min |
| 4.2.3 | Implement SETTID/CLEARTID | 30 min |
| 4.2.4 | Wire Clone to Syscall Dispatch | 15 min |
| 4.3.1 | Implement set_tid_address | 20 min |
| 4.4.1 | Implement Thread Exit | 30 min |
| 4.5.1 | Add Clone to libsyscall | 15 min |
| 4.5.2 | Add Threading Test | 30 min |

### Phase 5: I/O (P1) — Kernel Handlers Only
| UoW | Name | Est. Time |
|-----|------|-----------|
| 5.1.1 | Add IoVec to Kernel | 10 min |
| 5.2.1 | Implement writev Kernel Handler | 30 min |
| 5.3.1 | Implement readv Kernel Handler | 30 min |
| 5.4.1 | Add Vectored I/O Test | 20 min |

> **Note (TEAM_228)**: Userspace wrappers already exist (TEAM_217). Only kernel handlers needed.


### Phase 6: Process Orchestration (P2)
| UoW | Name | Est. Time |
|-----|------|-----------|
| 6.1.1 | Design Pipe Data Structure | 20 min |
| 6.1.2 | Implement Pipe Object | 45 min |
| 6.1.3 | Implement sys_pipe2 | 30 min |
| 6.1.4 | Wire Pipe to File Operations | 20 min |
| 6.2.1 | Implement sys_dup | 20 min |
| 6.2.2 | Implement sys_dup3 | 20 min |
| 6.3.1 | Add Pipe/Dup to libsyscall | 15 min |
| 6.3.2 | Add Pipe Test | 30 min |
| 6.3.3 | Add Dup/Redirect Test | 20 min |

### Phase 7: Cleanup and Validation
| UoW | Name | Est. Time |
|-----|------|-----------|
| 7.1.1 | Replace Hand-Rolled Constants | 30 min |
| 7.1.2 | Replace Hand-Rolled Structs | 30 min |
| 7.2.1 | Remove Unused Code | 20 min |
| 7.3.1 | Run Full Test Suite | 30 min |
| 7.3.2 | Update Golden Files | 15 min |
| 7.4.1 | Create std Test Program | 30 min |
| 7.4.2 | Run std Test Program | 20 min |
| 7.5.1 | Update Architecture Docs | 20 min |
| 7.5.2 | Create std-support Summary | 15 min |

---

## Dependencies Graph

```
Phase 1 (Discovery)
    │
    ├──────────────────┬─────────────────┐
    ▼                  ▼                 ▼
Phase 2 (Auxv)    Phase 5 (I/O)    [Parallel OK]
    │
    ▼
Phase 3 (mmap)
    │
    ├─────────────────┐
    ▼                 ▼
Phase 4 (Thread)  Phase 6 (Pipe)
    │                 │
    └────────┬────────┘
             ▼
      Phase 7 (Cleanup)
```

---

## Quick Start for SLMs

To work on this plan:

1. **Read the requirements first**: `docs/planning/std-support/requirements.md`
2. **Pick a UoW from the current phase**
3. **Read the phase file** for context
4. **Complete the UoW tasks**
5. **Run tests** before marking complete
6. **Update phase file** with completion status

---

## Files

- `requirements.md` — Requirements and external crate references
- `PLAN.md` — This file (index)
- `phase-1.md` — Discovery and Safeguards
- `phase-2.md` — Auxv Implementation
- `phase-3.md` — mmap/munmap/mprotect
- `phase-4.md` — Threading
- `phase-5.md` — I/O
- `phase-6.md` — Process Orchestration
- `phase-7.md` — Cleanup and Validation
