//! x86_64 CPU Compartment
//!
//! This module handles CPU-level structures and state management:
//! - **GDT** (Global Descriptor Table) - Segment descriptors for code/data
//! - **TSS** (Task State Segment) - Ring transition stack pointers
//! - **IDT** (Interrupt Descriptor Table) - Interrupt/exception handlers
//! - **Exceptions** - CPU exception handling (page fault, GP fault, etc.)

pub mod gdt;
pub mod idt;
pub mod exceptions;

pub use gdt::*;
pub use idt::{Idt, IdtEntry, IDT, init as idt_init};
pub use exceptions::*;
