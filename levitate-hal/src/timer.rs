use bitflags::bitflags;

bitflags! {
    /// Control register bits for the AArch64 generic timer (CNTP_CTL_EL0 / CNTV_CTL_EL0).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TimerCtrlFlags: u64 {
        /// Timer enabled.
        const ENABLE = 1 << 0;
        /// Timer interrupt masked.
        const IMASK  = 1 << 1;
        /// Timer interrupt status (read-only).
        const ISTATUS = 1 << 2;
    }
}

/// A generic interface for a hardware timer.
pub trait Timer {
    /// Read the current system counter value (CNTPCT_EL0).
    fn read_counter(&self) -> u64;

    /// Read the system counter frequency (CNTFRQ_EL0).
    fn read_frequency(&self) -> u64;

    /// Set the timer value for a one-shot interrupt (CNTP_TVAL_EL0).
    /// The timer will decrement this value and fire when it reaches zero.
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
    fn is_pending(&self) -> bool;
}

/// Implementation of the AArch64 Generic Physical Timer.
pub struct AArch64Timer;

impl Timer for AArch64Timer {
    fn read_counter(&self) -> u64 {
        let val: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntvct_el0", out(reg) val);
        }
        val
    }

    fn read_frequency(&self) -> u64 {
        let val: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntfrq_el0", out(reg) val);
        }
        val
    }

    fn set_timeout(&self, ticks: u64) {
        unsafe {
            core::arch::asm!("msr cntv_tval_el0, {}", in(reg) ticks);
        }
    }

    fn configure(&self, flags: TimerCtrlFlags) {
        unsafe {
            core::arch::asm!("msr cntv_ctl_el0, {}", in(reg) flags.bits());
        }
    }

    fn is_pending(&self) -> bool {
        let val: u64;
        unsafe {
            core::arch::asm!("mrs {}, cntv_ctl_el0", out(reg) val);
        }
        TimerCtrlFlags::from_bits_truncate(val).contains(TimerCtrlFlags::ISTATUS)
    }
}

/// Global instance of the AArch64 physical timer.
pub static API: AArch64Timer = AArch64Timer;

/// Returns the uptime in seconds using the global timer.
pub fn uptime_seconds() -> u64 {
    let cnt = API.read_counter();
    let freq = API.read_frequency();
    if freq == 0 { 0 } else { cnt / freq }
}

/// Spin-wait for a certain number of cycles.
pub fn delay_cycles(cycles: u64) {
    let start = API.read_counter();
    while API.read_counter() - start < cycles {
        core::hint::spin_loop();
    }
}
