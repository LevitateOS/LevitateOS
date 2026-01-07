# Rust `std` Support Implementation Plan

**Team**: TEAM_222  
**Created**: 2026-01-07  
**Status**: Planning Complete

## Overview

This plan implements full Rust `std` compatibility for LevitateOS through 7 phases with 35 Units of Work (UoWs), each sized for a single SLM session.

## Phase Summary

| Phase | Name | Priority | UoWs | Blocking |
|-------|------|----------|------|----------|
| 1 | Discovery and Safeguards | — | 6 | Nothing |
| 2 | Auxv Implementation | P0 | 6 | All std binaries |
| 3 | mmap/munmap/mprotect | P0 | 8 | std allocator |
| 4 | Threading (clone, TLS) | P1 | 10 | std::thread |
| 5 | I/O (writev/readv) | P1 | 5 | println! |
| 6 | Process Orchestration | P2 | 7 | Command, pipes |
| 7 | Cleanup and Validation | — | 7 | Release |

**Total UoWs**: 49

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

### Phase 2: Auxv Implementation (P0)
| UoW | Name | Est. Time |
|-----|------|-----------|
| 2.1.1 | Add AT_* Constants to Kernel | 15 min |
| 2.2.1 | Locate Stack Setup Code | 20 min |
| 2.2.2 | Implement Auxv Push | 45 min |
| 2.2.3 | Add ELF Header Auxv Entries | 30 min |
| 2.3.1 | Add Auxv Reader to libsyscall | 20 min |
| 2.3.2 | Add Auxv Test Program | 30 min |

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

### Phase 5: I/O (P1)
| UoW | Name | Est. Time |
|-----|------|-----------|
| 5.1.1 | Add iovec to Kernel | 10 min |
| 5.2.1 | Implement writev Core Logic | 30 min |
| 5.2.2 | Wire writev to Syscall Dispatch | 10 min |
| 5.3.1 | Implement readv Core Logic | 30 min |
| 5.3.2 | Wire readv to Syscall Dispatch | 10 min |
| 5.4.1 | Verify libsyscall Wrappers | 15 min |
| 5.4.2 | Add Vectored I/O Test | 20 min |

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
