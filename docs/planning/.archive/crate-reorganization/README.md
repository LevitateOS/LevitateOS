# Crate Reorganization Refactor Plan

**TEAM_101** | Created: 2026-01-05

## Overview

Full reorganization of LevitateOS crates to establish correct abstraction layers.

### Decision Summary

| Decision | Choice |
|----------|--------|
| GPU Consolidation | **Option A**: Fix `levitate-virtio-gpu`, delete `levitate-gpu` |
| Refactor Scope | **FULL**: Complete reorganization |

### Goals

1. Fix VirtIO GPU driver VirtQueue DMA bugs
2. Delete legacy `levitate-gpu` crate  
3. Clean up HAL layer boundaries
4. Extract drivers from kernel into separate crates
5. Establish clear naming convention
6. Remove all external dependencies from kernel

### Target Architecture

```
KERNEL (binary)
    ↓
SUBSYSTEMS (terminal, fs)
    ↓  
DRIVERS (drivers-gpu, drivers-blk, drivers-net, drivers-input)
    ↓
TRANSPORT (virtio)
    ↓
HAL (mmu, gic, timer, uart, console)
    ↓
UTILS (spinlock, ringbuffer, cpio)
```

### Phases

| Phase | Title | Status |
|-------|-------|--------|
| 1 | Discovery and Safeguards | Planned |
| 2 | Structural Extraction | Planned |
| 3 | Migration | Planned |
| 4 | Cleanup | Planned |
| 5 | Hardening and Handoff | Planned |

---

## Files

- `phase-1.md` - Discovery and Safeguards
- `phase-2.md` - Structural Extraction  
- `phase-3.md` - Migration
- `phase-4.md` - Cleanup
- `phase-5.md` - Hardening and Handoff
