//! TEAM_222: x86_64 CPU stubs

pub fn wait_for_interrupt() {
    // x86: hlt
    unsafe { core::arch::asm!("hlt") };
}

pub fn halt() -> ! {
    loop {
        wait_for_interrupt();
    }
}
