# Phase 2 — Design: AArch64 Generic Timer Driver

## Proposed Solution
Implement the `Timer` driver as a module in `levitate-hal`. It will provide a safe abstraction over the AArch64 generic timer registers.

## User-facing Behavior
- The kernel will use the HAL to set up periodic or one-shot timer interrupts.
- Reliable `uptime` and `delay` functions will be available.

## API Design

The timer will be implemented using the `tock-registers` or `bitflags` pattern for type-safe register access.

```rust
use bitflags::bitflags;

bitflags! {
    pub struct TimerCtrlFlags: u64 {
        const ENABLE = 1 << 0;
        const IMASK  = 1 << 1;
        const ISTATUS = 1 << 2;
    }
}

pub trait Timer {
    /// Read the current system counter value (CNTPCT_EL0).
    fn read_counter(&self) -> u64;

    /// Read the system counter frequency (CNTFRQ_EL0).
    fn read_frequency(&self) -> u64;

    /// Set the timer value for a one-shot interrupt (CNTP_TVAL_EL0).
    fn set_timeout(&self, ticks: u64);

    /// Configure the timer control register (CNTP_CTL_EL0).
    fn configure(&self, flags: TimerCtrlFlags);

    /// Convenience: Enable the timer and unmask its interrupt.
    fn enable(&self) {
        self.configure(TimerCtrlFlags::ENABLE);
    }

    /// Convenience: Disable the timer or mask its interrupt.
    fn disable(&self) {
        self.configure(TimerCtrlFlags::IMASK);
    }

    /// Check if the timer interrupt is pending.
    fn is_pending(&self) -> bool {
        // ISTATUS bit
        false // Implementation will read register
    }
}

pub struct AArch64Timer;
impl Timer for AArch64Timer {
    // ... implementation ...
}

pub static API: AArch64Timer = AArch64Timer;
```

## System Behavior
- The timer driver will use `volatile` access (via `asm!`) to interact with standard AArch64 system registers.
- It will NOT manage the GIC; the kernel is responsible for routing IRQ 30 through the GIC.

## Behavioral Decisions (Questions)
1. **Should the HAL manage IRQ 30 implicitly?**
   - *Recommendation*: No. The GIC driver handles IRQ routing. The Timer driver should only handle the local CPU timer state.
2. **One-shot vs Periodic?**
   - *Recommendation*: The hardware supports `TVAL` (decrementing timer) which is naturally one-shot. Periodic timers can be implemented by reloading `TVAL` in the interrupt handler.

## Open Questions
- **Q1**: Do we want the HAL to provide a `delay_ms` function, or should that stay in the kernel/utils?
   - *Hypothesis*: HAL should provide low-level `delay_cycles`, and a higher-level `delay_ms` can be in `levitate-utils` or the HAL.

## Steps

### Step 1 – Draft Initial Design
- [x] API contract drafted.

### Step 2 – Define Behavioral Contracts
- [ ] Document handling of frequency-based conversions (ticks to ms).

### Step 3 – Review Design Against Architecture
- [ ] Ensure consistency with `Gic` and `Console`.
