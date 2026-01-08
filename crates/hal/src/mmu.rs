// TEAM_260: Generic MMU delegation.
// Provides arch-agnostic access to MMU functions where possible.

pub use crate::traits::PageAllocator;
#[cfg(target_arch = "aarch64")]
pub use crate::arch::mmu::*;
#[cfg(target_arch = "x86_64")]
pub use crate::arch::mem::mmu::*;
