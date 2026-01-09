# VirtIO Driver Reorganization Plan

**TEAM_332** | Created: 2026-01-09

## Overview

Reorganize scattered VirtIO driver code into a clean, modular crate structure with proper abstractions.

## Phases

| Phase | Name | Status | Description |
|-------|------|--------|-------------|
| 1 | [Discovery and Safeguards](phase-1.md) | 📋 TODO | Map behavior, lock in tests |
| 2 | [Structural Extraction](phase-2.md) | 📋 TODO | Create new crates |
| 3 | [Migration](phase-3.md) | 📋 TODO | Move call sites |
| 4 | [Cleanup](phase-4.md) | 📋 TODO | Remove dead code |
| 5 | [Hardening and Handoff](phase-5.md) | 📋 TODO | Final verification |

## Estimated Effort

| Phase | Steps | Est. UoWs | Complexity |
|-------|-------|-----------|------------|
| 1 | 3 | 3-5 | Low |
| 2 | 5 | 8-12 | High |
| 3 | 5 | 5-8 | Medium |
| 4 | 4 | 4-6 | Low |
| 5 | 4 | 4-5 | Low |
| **Total** | **21** | **24-36** | |

## Key Deliverables

1. **`crates/virtio-transport/`** - Unified MMIO/PCI transport abstraction
2. **`crates/drivers/virtio-input/`** - Input driver crate
3. **`crates/drivers/virtio-blk/`** - Block driver crate
4. **`crates/drivers/virtio-net/`** - Network driver crate
5. **`crates/drivers/virtio-gpu/`** - GPU driver crate (moved from `crates/gpu/`)
6. **`kernel/src/drivers/`** - Thin kernel integration layer

## Dependencies

- Phase 2 depends on Phase 1 completion
- Phase 3 depends on Phase 2 (each step can start as corresponding Phase 2 step completes)
- Phase 4 depends on Phase 3 completion
- Phase 5 depends on Phase 4 completion

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Break x86_64 display | Medium | High | Screenshot tests catch early |
| Break aarch64 boot | Low | High | Golden log tests catch |
| Keyboard input regression | Medium | High | Manual testing on both arches |
| Long migration time | Medium | Medium | Incremental migration per driver |

## Success Criteria

- [ ] All VirtIO drivers in dedicated crates
- [ ] Unified transport abstraction works for MMIO and PCI
- [ ] All tests pass (screenshot, behavior, unit)
- [ ] Dead code removed (`crates/virtio/`, old kernel drivers)
- [ ] Architecture documentation updated
- [ ] No behavior regressions

## Open Questions

1. Should `virtio-transport` wrap or replace `virtio-drivers` transports?
2. Should driver crates support `std` for unit testing?
3. Add PCI support to block/net drivers in this refactor or defer?

---

## Quick Start

To begin implementation:

1. Read `phase-1.md` thoroughly
2. Run baseline tests: `cargo xtask test levitate && cargo xtask test behavior`
3. Start with `phase-1-step-1.md` tasks
