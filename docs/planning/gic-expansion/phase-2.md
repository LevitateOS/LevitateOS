# Phase 2 — Design: GICv2/v3 Expansion

**TEAM_015** | **Feature:** Clean IRQ routing abstraction

---

## Proposed Solution

### High-Level Design

Replace the hardcoded IRQ dispatch in `handle_irq()` with a **typed IRQ mapping** and **handler dispatch table**.

```
┌─────────────────────────────────────────────────────────┐
│                     GIC Hardware                         │
└─────────────────────┬───────────────────────────────────┘
                      │ IRQ number (raw u32)
                      ▼
┌─────────────────────────────────────────────────────────┐
│              levitate-hal/src/gic.rs                     │
│  - IrqId enum (Timer, Uart, VirtioGpu, ...)             │
│  - Gic::acknowledge() → IrqId                           │
│  - Handler registry: set_handler(IrqId, fn)             │
└─────────────────────┬───────────────────────────────────┘
                      │ IrqId (typed)
                      ▼
┌─────────────────────────────────────────────────────────┐
│           kernel/src/exceptions.rs                       │
│  - handle_irq() → lookup handler by IrqId → call        │
│  - No hardcoded numbers!                                 │
└─────────────────────────────────────────────────────────┘
```

### Components

1. **`IrqId` enum** — Typed IRQ identifiers
2. **IRQ number mapping** — `const` array mapping IrqId → u32
3. **Handler table** — Static array of function pointers
4. **Registration API** — `gic::register_handler(IrqId, fn())`

---

## API Design

### New Types (`levitate-hal/src/gic.rs`)

```rust
/// Known IRQ sources in LevitateOS
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IrqId {
    VirtualTimer = 0,
    Uart = 1,
    // Future: VirtioGpu, VirtioInput, VirtioBlk, VirtioNet
}

impl IrqId {
    /// Hardware IRQ number for this source (QEMU virt machine)
    pub const fn irq_number(self) -> u32 {
        match self {
            IrqId::VirtualTimer => 27,
            IrqId::Uart => 33,
        }
    }
    
    /// Try to convert raw IRQ number to IrqId
    pub fn from_irq_number(irq: u32) -> Option<Self> {
        match irq {
            27 => Some(IrqId::VirtualTimer),
            33 => Some(IrqId::Uart),
            _ => None,
        }
    }
}
```

### Handler Registration

```rust
type IrqHandler = fn();

static mut HANDLERS: [Option<IrqHandler>; 16] = [None; 16];

/// Register a handler for an IRQ. Panics if already registered.
pub fn register_handler(irq: IrqId, handler: IrqHandler) {
    unsafe {
        let idx = irq as usize;
        if HANDLERS[idx].is_some() {
            panic!("IRQ handler already registered");
        }
        HANDLERS[idx] = Some(handler);
    }
}

/// Called from handle_irq() in kernel
pub fn dispatch(irq_num: u32) -> bool {
    if let Some(irq_id) = IrqId::from_irq_number(irq_num) {
        unsafe {
            if let Some(handler) = HANDLERS[irq_id as usize] {
                handler();
                return true;
            }
        }
    }
    false
}
```

---

## Behavioral Decisions

| Question | Decision |
|----------|----------|
| What happens if an unknown IRQ fires? | Log warning, EOI anyway |
| What if a handler panics? | Kernel panic (no recovery yet) |
| Can handlers be unregistered? | Not initially (add later if needed) |
| Thread safety? | Static mut with no lock (single-core assumption) |
| GICv3 support scope? | Not in this PR, but design allows future extension |

---

## Design Alternatives Considered

1. **Trait-based GIC abstraction** — Too complex for current needs; defer until GICv3 actually needed.
2. **Dynamic allocation for handlers** — Requires heap in HAL; avoid for now.
3. **No registry, just typed IrqId** — Simpler but still leaves dispatch logic in kernel. Chosen hybrid approach.

---

## Open Questions

> **None blocking.** All decisions made above based on current project state.

If user disagrees with any of the behavioral decisions above, please note them before implementation.

---

## Next Steps

Proceed to Phase 3 (Implementation) with the following units of work:

1. Add `IrqId` enum and registration API to `gic.rs`
2. Update `exceptions.rs` to use `gic::dispatch()`
3. Register timer and UART handlers in `main.rs`
4. Verify in QEMU
