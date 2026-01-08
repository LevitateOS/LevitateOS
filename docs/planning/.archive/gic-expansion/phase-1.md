# Phase 1 — Discovery: GICv2/v3 Expansion

**TEAM_015** | **Feature:** Expand GIC support to handle specific IRQ routing cleanly.

---

## Feature Summary

**Problem:** The current GIC implementation is minimal—it initializes the GICv2, enables/disables IRQs by number, and acknowledges interrupts. However, IRQ handling in `exceptions.rs` uses hardcoded magic numbers (27 = Timer, 33 = UART), making it brittle and difficult to extend.

**Goal:** Create a clean IRQ routing abstraction that:
1. Maps symbolic IRQ identifiers to GIC IRQ numbers
2. Supports handler registration (callback-style or dispatch table)
3. Prepares for potential GICv3 support (used on Raspberry Pi 4/5)

**Who Benefits:**
- Future driver development becomes easier (no more magic numbers)
- Path to GICv3 support for real hardware (Pi 4/5)
- Cleaner separation between HAL and kernel

---

## Success Criteria

1. No hardcoded IRQ numbers in `kernel/src/exceptions.rs`
2. IRQ handlers registered via a typed API
3. Existing functionality preserved (Timer IRQ 27, UART IRQ 33)
4. Build passes, runtime verification shows no regressions

---

## Current State Analysis

### Existing Implementation

| File | Purpose | Lines |
|------|---------|-------|
| `levitate-hal/src/gic.rs` | GICv2 driver (init, enable/disable, ack, EOI) | 152 |
| `levitate-hal/src/interrupts.rs` | IRQ enable/disable via DAIF | 29 |
| `kernel/src/exceptions.rs` | Vector table + IRQ dispatch | 166 |

### Current IRQ Flow
```
IRQ → vectors (irq_entry) → handle_irq() → hardcoded switch on IRQ number → end_interrupt()
```

### Hardcoded IRQs in `handle_irq()`
```rust
if irq == 27 { /* Timer */ }
else if irq == 33 { /* UART */ }
else if irq < 1020 { println!(...) }
```

### GIC Addresses
- `GICD_BASE = 0x08000000` (Distributor)
- `GICC_BASE = 0x08010000` (CPU Interface)
- These are QEMU `virt` machine GICv2 addresses

---

## Codebase Reconnaissance

### Files Likely Touched
- **`levitate-hal/src/gic.rs`** — Add IRQ ID types, possibly handler registry
- **`kernel/src/exceptions.rs`** — Replace hardcoded dispatch with registry lookup
- **`levitate-hal/src/lib.rs`** — Export new types

### Tests
- `levitate-hal/src/timer.rs` has a `#[cfg(test)]` module (uptime calculation)
- No GIC-specific unit tests exist
- Primary verification: runtime in QEMU

### Non-Obvious Constraints
- GICv2 limits to 8 CPUs (not a concern for single-core QEMU)
- GICv3 uses system registers instead of MMIO for CPU interface
- QEMU `-M virt` defaults to GICv2; use `-M virt,gic-version=3` for GICv3

---

## GICv2 vs GICv3 Key Differences

| Aspect | GICv2 | GICv3 |
|--------|-------|-------|
| Max CPUs | 8 | 512 |
| CPU Interface | Memory-mapped (GICC) | System registers (ICC_*) |
| Redistributor | N/A | Required (per-CPU) |
| LPIs (Message-based IRQs) | No | Yes (via ITS) |

### Implications for LevitateOS
- **Short term:** Focus on clean IRQ routing for GICv2
- **Medium term:** Abstract GIC operations behind a trait for v2/v3 switching
- **QEMU testing:** Use `-M virt,gic-version=3` to test GICv3 path

---

## Constraints

1. **No breaking changes** to existing timer/UART functionality
2. **Minimal complexity** — don't over-engineer for multi-core until needed
3. **Kernel philosophy** — IRQ routing is mechanism; handler policy stays in kernel
4. **No runtime detection** yet — configure GIC version at compile time or boot param initially
