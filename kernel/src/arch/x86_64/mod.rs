//! TEAM_162: x86_64 Architecture Stub
//!
//! This module provides stubs for x86_64 to verify the architecture abstraction.

pub mod boot;
pub mod cpu;
pub mod exceptions;
pub mod power;
pub mod task;
pub mod time;

// Re-export Context and other items from task
pub use self::boot::*;
pub use self::exceptions::*;
pub use self::task::*;

// TEAM_162: Stubs for types that need to be provided by the architecture
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SyscallFrame {
    pub regs: [u64; 31],
    pub sp: u64,
}

impl SyscallFrame {
    pub fn syscall_number(&self) -> u64 {
        0
    }
    pub fn arg0(&self) -> u64 {
        0
    }
    pub fn arg1(&self) -> u64 {
        0
    }
    pub fn arg2(&self) -> u64 {
        0
    }
    pub fn arg3(&self) -> u64 {
        0
    }
    pub fn arg4(&self) -> u64 {
        0
    }
    pub fn arg5(&self) -> u64 {
        0
    }
    pub fn arg6(&self) -> u64 {
        0
    }
    pub fn set_return(&mut self, _value: i64) {}
}

pub unsafe fn switch_mmu_config(_config_phys: usize) {
    // unimplemented!("x86_64 switch_mmu_config")
}
