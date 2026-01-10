//! TEAM_222: x86_64 power management
//! TEAM_409: Implemented QEMU debug exit for proper VM termination

/// Exit QEMU using the debug exit device.
/// 
/// This uses I/O port 0xf4 which is the QEMU isa-debug-exit device.
/// The exit code will be (value << 1) | 1, so writing 0 gives exit code 1.
/// 
/// If QEMU wasn't started with `-device isa-debug-exit`, this is a no-op
/// and we fall back to HLT loop.
pub fn system_off() -> ! {
    // QEMU debug exit via I/O port 0xf4
    // This requires QEMU to be started with: -device isa-debug-exit,iobase=0xf4,iosize=0x04
    // Writing any value causes QEMU to exit with code (value << 1) | 1
    unsafe {
        // Write 0 to port 0xf4 -> QEMU exits with code 1
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xf4u16,
            in("al") 0u8,
            options(nomem, nostack, preserves_flags)
        );
    }
    
    // Fallback: if debug exit device not available, halt the CPU
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}
