# Phase 1: Discovery and Safeguards

**Parent:** [README.md](./README.md)  
**Status:** Planned

---

## Refactor Summary

Reorganize LevitateOS crates to establish correct abstraction layers:
- Fix and consolidate GPU drivers (remove duplication)
- Clean HAL layer (remove driver code)
- Extract drivers from kernel monolith
- Establish consistent naming convention

### Pain Points

1. Two GPU crates doing the same thing
2. HAL contains VirtIO driver code (wrong layer)
3. Kernel directly depends on external crates
4. No clear naming convention
5. Drivers embedded in kernel, not extractable

---

## Success Criteria

### Before
```
kernel → levitate-gpu (virtio-drivers wrapper)
       → levitate-virtio-gpu (broken, unused)
       → levitate-hal (contains virtio.rs)
       → virtio-drivers, embedded-graphics (direct external deps)
```

### After
```
kernel → levitate-drivers-gpu (fixed, canonical)
       → levitate-drivers-blk
       → levitate-drivers-net  
       → levitate-drivers-input
       → levitate-terminal
       → levitate-fs
       
(no direct external deps in kernel)
```

---

## Behavioral Contracts

### Must Preserve

| Contract | Description | Test |
|----------|-------------|------|
| GPU Init | GPU initializes with 1280x800 resolution | Golden boot test |
| Terminal | Terminal renders text on GPU framebuffer | Visual + golden |
| Block | VirtIO block device reads/writes | Behavior test |
| Net | VirtIO net initializes with MAC | Golden boot test |
| Input | Keyboard input works | Behavior test |
| Boot | Full boot sequence completes | Golden boot test |

### Golden Tests

- `tests/golden_boot.txt` - Must match exactly after refactor
- `cargo xtask test` - All 22 regression tests must pass

---

## Current Architecture Notes

### Dependency Graph (Current)

```
levitate-kernel
├── levitate-hal ─────────────┬── levitate-utils
│                             ├── levitate-virtio
│                             └── virtio-drivers ❌ (external in HAL!)
├── levitate-gpu ─────────────┬── levitate-hal
│                             ├── virtio-drivers ❌ 
│                             └── embedded-graphics
├── levitate-virtio-gpu ──────┬── levitate-virtio
│                             └── embedded-graphics
├── levitate-terminal ────────┬── levitate-utils
│                             └── embedded-graphics
├── virtio-drivers ❌ (direct dep!)
├── embedded-graphics ❌ (direct dep!)
├── embedded-sdmmc ❌
└── ext4-view ❌
```

### Problem Areas

1. **levitate-hal/src/virtio.rs** - VirtIO HAL impl should be in levitate-virtio
2. **levitate-gpu** - Wrapper around virtio-drivers, has less visibility
3. **levitate-virtio-gpu** - Our driver, but VirtQueue has DMA bugs
4. **kernel/src/block.rs** - Block driver embedded in kernel
5. **kernel/src/net.rs** - Net driver embedded in kernel
6. **kernel/src/input.rs** - Input driver embedded in kernel

---

## Constraints

1. **Behavioral regression protection** - All golden tests must pass
2. **One phase at a time** - Complete and verify each phase before next
3. **No feature freeze** - Refactor incrementally alongside other work
4. **VirtIO GPU fix first** - Must work before deleting levitate-gpu

---

## Open Questions

1. Should `levitate-fs` wrap both FAT32 and ext4, or separate crates?
2. Should drivers use a trait-based interface for testing?
3. How to handle the virtio-drivers dependency during transition?

---

## Steps

### Step 1: Verify Current State
- Run `cargo xtask test` and confirm all pass
- Document current test coverage
- Capture golden baselines

### Step 2: Analyze VirtQueue DMA Bugs
- Review levitate-virtio/src/queue.rs
- Identify root cause of timeout issues
- Document fix approach

### Step 3: Lock in Golden Tests
- Ensure golden_boot.txt is current
- Add any missing behavioral contracts as tests
