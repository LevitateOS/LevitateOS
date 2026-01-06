//! TEAM_073: User process spawning and management.
//!
//! This module provides the high-level interface for creating and
//! running user processes.

use crate::loader::elf::Elf;
use crate::loader::elf::ElfError;
use crate::task::user::UserTask;
use crate::task::user_mm;
use levitate_hal::mmu::MmuError;

/// TEAM_073: Error type for process spawning.
/// TEAM_152: Updated to preserve inner errors (0x03xx) per unified error system plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnError {
    /// ELF parsing/loading failed (0x0301)
    Elf(ElfError),
    /// Page table creation failed (0x0302)
    PageTable(MmuError),
    /// Stack setup failed (0x0303)
    Stack(MmuError),
}

impl SpawnError {
    /// TEAM_152: Get numeric error code for debugging
    pub const fn code(&self) -> u16 {
        match self {
            Self::Elf(_) => 0x0301,
            Self::PageTable(_) => 0x0302,
            Self::Stack(_) => 0x0303,
        }
    }

    /// TEAM_152: Get error name for logging
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Elf(_) => "ELF loading failed",
            Self::PageTable(_) => "Page table creation failed",
            Self::Stack(_) => "Stack setup failed",
        }
    }
}

impl core::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Elf(inner) => write!(f, "E{:04X}: {} ({})", self.code(), self.name(), inner),
            Self::PageTable(inner) => write!(f, "E{:04X}: {} ({})", self.code(), self.name(), inner),
            Self::Stack(inner) => write!(f, "E{:04X}: {} ({})", self.code(), self.name(), inner),
        }
    }
}

impl core::error::Error for SpawnError {}

impl From<ElfError> for SpawnError {
    fn from(e: ElfError) -> Self {
        SpawnError::Elf(e)  // TEAM_152: Now preserves context
    }
}

/// TEAM_073: Spawn a user process from an ELF binary in memory.
///
/// # Arguments
/// * `elf_data` - Raw ELF file contents
///
/// # Returns
/// A `UserTask` ready to be scheduled, or an error.
pub fn spawn_from_elf(elf_data: &[u8]) -> Result<UserTask, SpawnError> {
    levitate_hal::println!("[SPAWN] Parsing ELF header ({} bytes)...", elf_data.len());
    // 1. Parse ELF
    let elf = Elf::parse(elf_data)?;
    levitate_hal::println!("[SPAWN] ELF parsed.");

    // 2. Create user page table
    levitate_hal::println!("[SPAWN] Creating user page table...");
    let ttbr0_phys = user_mm::create_user_page_table().ok_or(SpawnError::PageTable(MmuError::AllocationFailed))?;

    // 3. Load ELF segments into user address space
    levitate_hal::println!("[SPAWN] Loading segments...");
    let (entry_point, brk) = elf.load(ttbr0_phys)?;

    // 4. Set up user stack
    levitate_hal::println!("[SPAWN] Setting up stack...");
    let stack_pages = user_mm::layout::STACK_SIZE / levitate_hal::mmu::PAGE_SIZE;
    let user_sp = unsafe {
        user_mm::setup_user_stack(ttbr0_phys, stack_pages).map_err(SpawnError::Stack)?
    };

    // 5. Create UserTask
    let task = UserTask::new(entry_point, user_sp, ttbr0_phys, brk);

    levitate_hal::println!(
        "[SPAWN] Success: PID={} entry=0x{:x} sp=0x{:x}",
        task.pid.0,
        entry_point,
        user_sp
    );

    Ok(task)
}
