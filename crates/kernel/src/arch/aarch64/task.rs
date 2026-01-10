/// TEAM_409: NEON/FPU state buffer for AArch64.
/// Contains V0-V31 (32 × 128-bit registers = 512 bytes) plus FPCR/FPSR.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct FpuState {
    /// V0-V31: 32 128-bit vector registers
    pub v_regs: [[u64; 2]; 32], // 512 bytes
    /// Floating-point Control Register
    pub fpcr: u32,
    /// Floating-point Status Register
    pub fpsr: u32,
}

impl Default for FpuState {
    fn default() -> Self {
        Self {
            v_regs: [[0u64; 2]; 32],
            fpcr: 0,
            fpsr: 0,
        }
    }
}

impl core::fmt::Debug for FpuState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FpuState").finish_non_exhaustive()
    }
}

/// TEAM_162: Saved CPU context for AArch64.
/// TEAM_409: Added NEON/FPU state for floating-point preservation across context switches.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Context {
    // Callee-saved general purpose registers (offsets 0-104)
    pub x19: u64,       // 0
    pub x20: u64,       // 8
    pub x21: u64,       // 16
    pub x22: u64,       // 24
    pub x23: u64,       // 32
    pub x24: u64,       // 40
    pub x25: u64,       // 48
    pub x26: u64,       // 56
    pub x27: u64,       // 64
    pub x28: u64,       // 72
    pub x29: u64,       // 80: Frame Pointer
    pub lr: u64,        // 88: Link Register (x30)
    pub sp: u64,        // 96: Stack Pointer
    pub tpidr_el0: u64, // 104: TEAM_217: Thread Local Storage pointer
    // TEAM_409: NEON/FPU state (offset 112, 520 bytes)
    pub fpu_state: FpuState,
}

impl Context {
    pub fn new(stack_top: usize, entry_wrapper: usize) -> Self {
        let mut ctx = Self::default();
        ctx.sp = stack_top as u64;
        ctx.lr = task_entry_trampoline as *const () as u64;
        ctx.x19 = entry_wrapper as u64;
        ctx
    }

    // TEAM_258: Abstract TLS setting for architecture independence
    pub fn set_tls(&mut self, addr: u64) {
        self.tpidr_el0 = addr;
    }
}

/// TEAM_162: Enter user mode at the specified entry point.
pub unsafe fn enter_user_mode(entry_point: usize, user_sp: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "msr elr_el1, {entry}",
            "msr spsr_el1, xzr",
            "msr sp_el0, {sp}",
            "mov x0, xzr", "mov x1, xzr", "mov x2, xzr", "mov x3, xzr",
            "mov x4, xzr", "mov x5, xzr", "mov x6, xzr", "mov x7, xzr",
            "mov x8, xzr", "mov x9, xzr", "mov x10, xzr", "mov x11, xzr",
            "mov x12, xzr", "mov x13, xzr", "mov x14, xzr", "mov x15, xzr",
            "mov x16, xzr", "mov x17, xzr", "mov x18, xzr", "mov x19, xzr",
            "mov x20, xzr", "mov x21, xzr", "mov x22, xzr", "mov x23, xzr",
            "mov x24, xzr", "mov x25, xzr", "mov x26, xzr", "mov x27, xzr",
            "mov x28, xzr", "mov x29, xzr", "mov x30, xzr",
            "eret",
            entry = in(reg) entry_point,
            sp = in(reg) user_sp,
            options(noreturn)
        );
    }
    #[allow(unreachable_code)]
    loop {
        core::hint::spin_loop();
    }
}

/// TEAM_162: Switch to a new user address space.
pub unsafe fn switch_mmu_config(config_phys: usize) {
    unsafe {
        los_hal::mmu::switch_ttbr0(config_phys);
    }
}

unsafe extern "C" {
    pub fn cpu_switch_to(old: *mut Context, new: *const Context);
    pub fn task_entry_trampoline();
}

use core::arch::global_asm;
global_asm!(include_str!("asm/task.S"));
