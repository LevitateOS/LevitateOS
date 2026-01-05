# Phase 1 — Understanding and Scoping

**TEAM_131 → TEAM_132** | Reduce Unsafe Code via Safe Abstractions

## Bug Summary

**Issue:** The codebase contains ~148 unsafe blocks scattered across kernel, levitate-hal, and levitate-virtio crates. Many of these follow repetitive patterns that could be encapsulated in safe abstractions per Rule 5 (Memory Safety & Type Hygiene).

**Severity:** Medium (code hygiene / maintainability)

**Impact:**
- Increased cognitive load for reviewers
- Higher risk of memory safety bugs
- Violates Rule 5: "Wrap unsafe in safe, idiomatic abstractions"

## Reproduction Status

Not a runtime bug — this is a code quality issue identified through static analysis.

**Audit Results:**
- **148 total unsafe blocks** across kernel crates
- **~30 inline asm! blocks** (system registers, barriers)
- **~12 volatile read/write operations** (MMIO, DMA)
- **~7 from_raw_parts calls** (slice creation from raw pointers)
- **~4 NonNull::as_mut() calls** (linked list operations)

## Context

### Affected Code Areas

| Crate | Unsafe Count | Primary Patterns |
|-------|--------------|------------------|
| `kernel/` | ~68 | asm!, from_raw_parts, ptr ops |
| `levitate-hal/` | ~122 | asm!, volatile MMIO, linked lists |
| `levitate-virtio/` | ~25 | volatile DMA, ptr ops |

### Unsafe Pattern Categories

1. **Volatile I/O** — `read_volatile`, `write_volatile` for MMIO/DMA
2. **Inline Assembly** — System register access, memory barriers, interrupts
3. **Raw Slice Creation** — `from_raw_parts` from physical addresses
4. **Pointer Dereference** — `NonNull::as_mut()`, raw pointer deref

## Constraints

- **No runtime behavior changes** — abstractions must be zero-cost
- **Must work in no_std** — kernel environment
- **Maintain performance** — volatile/asm patterns are performance-critical
- **Backwards compatible** — existing code should migrate incrementally

## Open Questions — RESOLVED

1. ~~Should we use existing crates (e.g., `volatile`, `safe-mmio`) or build our own?~~
   **RESOLVED:** Use existing crates. See "Golden Standard Crates" below.
2. ~~How do we handle architecture-specific asm! (aarch64 only currently)?~~
   **RESOLVED:** Use `aarch64-cpu` crate — provides all sysregs + barriers.
3. ~~Should linked list operations use a dedicated intrusive list crate?~~
   **RESOLVED:** Use `intrusive-collections` crate.

## Golden Standard Crates

| Purpose | Crate | Version | Status |
|---------|-------|---------|--------|
| System Registers + Barriers | `aarch64-cpu` | 11.2 | Add |
| Volatile MMIO | `safe-mmio` | 0.5+ | Already in Cargo.lock |
| Intrusive Lists | `intrusive-collections` | 0.10 | Add |

## Steps

- [x] Step 1 — Consolidate unsafe pattern inventory (this document)
- [ ] Step 2 — Categorize by abstraction opportunity (Phase 2)
- [ ] Step 3 — Design safe wrapper libraries (Phase 3)
