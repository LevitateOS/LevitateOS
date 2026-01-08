//! x86_64 Memory Management Compartment
//!
//! This module handles all memory-related functionality:
//! - **Paging** - 4-level page table structures (PML4, PDPT, PD, PT)
//! - **MMU** - Memory mapping, virtual-to-physical translation
//! - **Frame Allocator** - Physical frame allocation for page tables

pub mod paging;
pub mod mmu;
pub mod frame_alloc;

pub use paging::*;
pub use mmu::*;
pub use frame_alloc::*;
