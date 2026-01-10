//! TEAM_222: Architecture-specific CPU instructions
//! TEAM_409: Added Processor Control Region (PCR) for per-CPU state

use core::arch::asm;

/// TEAM_409: Processor Control Region (PCR) for AArch64.
/// This structure is pointed to by TPIDR_EL1 in kernel mode.
/// It contains per-CPU state that must be accessed quickly and safely.
///
/// IMPORTANT: The layout of this struct must match the offsets used in assembly.
#[repr(C, align(16))]
pub struct ProcessorControlRegion {
    /// Self-reference to this structure (offset 0)
    pub self_ptr: *const ProcessorControlRegion,
    /// Temporary scratch space for user SP during exception entry (offset 8)
    pub user_sp_scratch: usize,
    /// The current kernel stack top for this CPU (offset 16)
    pub kernel_stack: usize,
    /// Pointer to the current TaskControlBlock (offset 24)
    pub current_task_ptr: usize,
    /// CPU ID for multi-core support (offset 32)
    pub cpu_id: usize,
    /// Padding for future use and alignment (offset 40)
    pub _reserved: [usize; 3],
}

pub const PCR_SELF_OFFSET: usize = 0;
pub const PCR_USER_SP_OFFSET: usize = 8;
pub const PCR_KSTACK_OFFSET: usize = 16;
pub const PCR_CURRENT_TASK_OFFSET: usize = 24;
pub const PCR_CPU_ID_OFFSET: usize = 32;

impl ProcessorControlRegion {
    pub const fn new() -> Self {
        Self {
            self_ptr: core::ptr::null(),
            user_sp_scratch: 0,
            kernel_stack: 0,
            current_task_ptr: 0,
            cpu_id: 0,
            _reserved: [0; 3],
        }
    }
}

/// TEAM_409: Get the current PCR for the calling CPU.
/// Uses TPIDR_EL1 which holds the per-CPU pointer.
#[inline(always)]
pub unsafe fn get_pcr() -> &'static mut ProcessorControlRegion {
    let ptr: *mut ProcessorControlRegion;
    asm!("mrs {}, tpidr_el1", out(reg) ptr, options(nostack, nomem, preserves_flags));
    &mut *ptr
}

/// TEAM_409: Per-CPU PCR storage.
/// In a multi-core system, this would be an array or allocated per CPU.
pub static mut PCR: ProcessorControlRegion = ProcessorControlRegion::new();

/// TEAM_409: Initialize the PCR for the boot CPU.
pub unsafe fn init_pcr() {
    let pcr_ptr = &raw mut PCR;
    PCR.self_ptr = pcr_ptr;
    PCR.cpu_id = 0; // Boot CPU

    // Set TPIDR_EL1 to point to our PCR
    asm!("msr tpidr_el1, {}", in(reg) pcr_ptr, options(nostack, nomem));

    log::info!("[CPU] AArch64 PCR initialized at {:p}", pcr_ptr);
}

/// Wait for interrupt (WFI/WFE).
///
/// Puts the CPU into a low-power state until an interrupt occurs.
#[inline]
pub fn wait_for_interrupt() {
    aarch64_cpu::asm::wfe();
}

/// Halt the CPU indefinitely.
///
/// This enters a loop of `wait_for_interrupt`.
pub fn halt() -> ! {
    loop {
        wait_for_interrupt();
    }
}
