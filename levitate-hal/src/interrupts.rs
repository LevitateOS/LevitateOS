/// AArch64 interrupt control.

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn disable() -> u64 {
    let state: u64;
    unsafe {
        core::arch::asm!("mrs {}, daif", out(reg) state);
        core::arch::asm!("msr daifset, #2");
    }
    state
}

#[cfg(not(target_arch = "aarch64"))]
#[cfg(feature = "std")]
mod mock {
    use std::cell::Cell;
    thread_local! {
        pub static ENABLED: Cell<bool> = Cell::new(true);
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn disable() -> u64 {
    #[cfg(feature = "std")]
    {
        let prev = is_enabled();
        mock::ENABLED.with(|e| e.set(false));
        prev as u64
    }
    #[cfg(not(feature = "std"))]
    0 // Stub for no-std non-aarch64
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn restore(state: u64) {
    #[cfg(feature = "std")]
    mock::ENABLED.with(|e| e.set(state != 0));
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn restore(state: u64) {
    unsafe {
        core::arch::asm!("msr daif, {}", in(reg) state);
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn is_enabled() -> bool {
    let state: u64;
    unsafe {
        core::arch::asm!("mrs {}, daif", out(reg) state);
    }
    // IRQ is bit 7 (zero-indexed)
    (state & (1 << 7)) == 0
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn is_enabled() -> bool {
    #[cfg(feature = "std")]
    return mock::ENABLED.with(|e| e.get());
    #[cfg(not(feature = "std"))]
    true // Stub for no-std non-aarch64
}
