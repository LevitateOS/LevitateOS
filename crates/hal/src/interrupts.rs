/// AArch64 interrupt control.
/// Behaviors: [I1]-[I6] interrupt enable/disable/restore cycle
/// TEAM_132: Migrate DAIF to aarch64-cpu
///
/// [I1] Disables interrupts, [I2] returns previous state
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn disable() -> u64 {
    use aarch64_cpu::registers::{Readable, DAIF};
    let state = DAIF.get(); // [I2] capture prev state
    // SAFETY: daifset is a special immediate-only instruction not provided by aarch64-cpu
    unsafe { core::arch::asm!("msr daifset, #2") }; // [I1] disable
    state
}

/// [I7] Unconditionally enables interrupts
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn enable() {
    // SAFETY: daifclr is a special immediate-only instruction not provided by aarch64-cpu
    unsafe { core::arch::asm!("msr daifclr, #2") };
}

#[cfg(not(target_arch = "aarch64"))]
#[cfg(feature = "std")]
mod mock {
    use std::cell::Cell;
    thread_local! {
        pub static ENABLED: Cell<bool> = Cell::new(true);
    }
}

/// [I1] Disables interrupts, [I2] returns previous state (mock impl)
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn disable() -> u64 {
    #[cfg(feature = "std")]
    {
        let prev = is_enabled(); // [I2] capture prev
        mock::ENABLED.with(|e| e.set(false)); // [I1] disable
        prev as u64
    }
    #[cfg(not(feature = "std"))]
    0 // Stub for no-std non-aarch64
}

/// [I7] Unconditionally enables interrupts (mock impl)
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub unsafe fn enable() {
    #[cfg(feature = "std")]
    mock::ENABLED.with(|e| e.set(true));
}

/// [I3] Restores previous interrupt state (mock impl)
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn restore(_state: u64) {
    #[cfg(feature = "std")]
    mock::ENABLED.with(|e| e.set(_state != 0)); // [I3] restore
}

/// [I3] Restores previous interrupt state
/// TEAM_132: Migrate DAIF to aarch64-cpu
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn restore(state: u64) {
    use aarch64_cpu::registers::{Writeable, DAIF};
    DAIF.set(state); // [I3] restore
}

/// [I4] Returns true when enabled, [I5] returns false when disabled
/// TEAM_132: Migrate DAIF to aarch64-cpu
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn is_enabled() -> bool {
    use aarch64_cpu::registers::{Readable, DAIF};
    // IRQ is bit 7 (zero-indexed)
    (DAIF.get() & (1 << 7)) == 0 // [I4][I5] check enabled state
}

/// [I4] Returns true when enabled, [I5] returns false when disabled (mock impl)
#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn is_enabled() -> bool {
    #[cfg(feature = "std")]
    return mock::ENABLED.with(|e| e.get()); // [I4][I5]
    #[cfg(not(feature = "std"))]
    true // Stub for no-std non-aarch64
}
