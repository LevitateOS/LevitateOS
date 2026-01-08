// TEAM_259: CPU Exception Handlers for x86_64.

use crate::x86_64::idt::IDT;
use core::arch::{asm, naked_asm};

#[repr(C)]
#[derive(Debug)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

macro_rules! exception_handler {
    ($name:ident, $handler:ident) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
                "push rax",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push rbp",     // TEAM_299: Save RBP for alignment trick

                // TEAM_299: Conditional swapgs
                // Check CS of the interrupted frame. Offset 88 = 10 regs * 8 bytes + RBP.
                // ExceptionStackFrame starts at [rsp + 88]
                // CS is at [rsp + 88 + 8] = [rsp + 96]
                // Wait, let's re-verify:
                // Pushed: rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11, rbp (10 regs)
                // RSP + 0  : rbp
                // RSP + 8  : r11
                // ...
                // RSP + 72 : rax
                // RSP + 80 : ExceptionStackFrame.RIP
                // RSP + 88 : ExceptionStackFrame.CS
                "test qword ptr [rsp + 88], 3",
                "jz 1f",
                "swapgs",
                "1:",

                "mov rbp, rsp", // Save stack pointer before alignment
                "and rsp, -16", // Ensure 16-byte alignment for Rust handler
                "mov rdi, rbp",
                "add rdi, 80",  // Point to ExceptionStackFrame (10 regs * 8)
                "call {handler}",
                "mov rsp, rbp", // Restore stack pointer

                // TEAM_299: Conditional swapgs back
                "test qword ptr [rsp + 96], 3",
                "jz 2f",
                "swapgs",
                "2:",

                "pop rbp",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rax",
                "iretq",
                handler = sym $handler,
            );
        }
    };
}

macro_rules! exception_handler_err {
    ($name:ident, $handler:ident) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
                "push rax",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push rbp",     // TEAM_299: Save RBP for alignment trick

                // TEAM_299: Conditional swapgs
                // Pushed: rax, rcx, rdx, rsi, rdi, r8, r9, r10, r11, rbp (10 regs)
                // RSP + 0  : rbp
                // ...
                // RSP + 72 : rax
                // RSP + 80 : Error Code
                // RSP + 88 : ExceptionStackFrame.RIP
                // RSP + 96 : ExceptionStackFrame.CS
                "test qword ptr [rsp + 96], 3",
                "jz 1f",
                "swapgs",
                "1:",

                "mov rbp, rsp", // Save stack pointer before alignment
                "and rsp, -16", // Ensure 16-byte alignment for Rust handler
                "mov rdi, rbp",
                "add rdi, 80",  // Point to error code
                "mov rsi, [rdi]", // Error code
                "add rdi, 8",   // Point to ExceptionStackFrame
                "call {handler}",
                "mov rsp, rbp", // Restore stack pointer

                // TEAM_299: Conditional swapgs back
                "test qword ptr [rsp + 96], 3",
                "jz 2f",
                "swapgs",
                "2:",

                "pop rbp",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rdx",
                "pop rcx",
                "pop rax",
                "add rsp, 8", // Clean up error code
                "iretq",
                handler = sym $handler,
            );
        }
    };
}

macro_rules! irq_handler {
    ($name:ident, $vector:expr) => {
        #[unsafe(naked)]
        pub unsafe extern "C" fn $name() {
            naked_asm!(
                "push rax",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push rbp",

                // TEAM_299: Conditional swapgs
                "test qword ptr [rsp + 88], 3",
                "jz 1f",
                "swapgs",
                "1:",

                "mov rbp, rsp",
                "and rsp, -16",
                "mov rdi, {vector}",
                "call {handler}",
                "mov rsp, rbp",

                // TEAM_299: Conditional swapgs back
                "test qword ptr [rsp + 88], 3",
                "jz 2f",
                "swapgs",
                "2:",

                "pop rbp",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rax",
                "iretq",
                vector = const $vector,
                handler = sym irq_dispatch,
            );
        }
    };
}

extern "C" fn divide_error_handler(frame: &ExceptionStackFrame) {
    panic!("EXCEPTION: DIVIDE ERROR\n{:#?}", frame);
}

extern "C" fn debug_handler(frame: &ExceptionStackFrame) {
    panic!("EXCEPTION: DEBUG\n{:#?}", frame);
}

extern "C" fn breakpoint_handler(frame: &ExceptionStackFrame) {
    // Just print for now to verify IDT works
    // TODO: Use a proper logger once serial/VGA are wired
    let _ = frame;
}

extern "C" fn invalid_opcode_handler(frame: &ExceptionStackFrame) {
    let rip = frame.instruction_pointer;

    // TEAM_301: Dump bytes around RIP to debug corruption
    unsafe {
        crate::println!("EXCEPTION: INVALID OPCODE at {:x}", rip);

        // Dump Code
        let ptr = rip as *const u8;
        crate::print!("Code at RIP: ");
        for i in 0..16 {
            crate::print!("{:02x} ", *ptr.add(i));
        }
        crate::println!();

        if rip > 16 {
            let ptr_prev = (rip - 16) as *const u8;
            crate::print!("Code before: ");
            for i in 0..16 {
                crate::print!("{:02x} ", *ptr_prev.add(i));
            }
            crate::println!();
        }

        // TEAM_301: Dump Stack (Backtraceish)
        let rsp = frame.stack_pointer;
        crate::println!("Stack at {:x}:", rsp);
        if rsp > 0x1000 && rsp < 0x0000_8000_0000_0000 {
            let sp_ptr = rsp as *const u64;
            for i in 0..16 {
                // Check alignment/validity roughly
                let val = *sp_ptr.add(i);
                crate::println!("  SP+{:<2}: {:x}", i * 8, val);
            }
        } else {
            crate::println!("  Invalid RSP/Kernel Stack?");
        }
    }

    panic!(
        "EXCEPTION: INVALID OPCODE\n{instruction_pointer:x}",
        instruction_pointer = frame.instruction_pointer
    );
}

extern "C" fn double_fault_handler(frame: &ExceptionStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: DOUBLE FAULT\nError Code: {}\n{:#?}",
        error_code, frame
    );
}

extern "C" fn general_protection_fault_handler(frame: &ExceptionStackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: GENERAL PROTECTION FAULT\nError Code: {}\n{:#?}",
        error_code, frame
    );
}

extern "C" fn page_fault_handler(frame: &ExceptionStackFrame, error_code: u64) {
    let cr2: u64;
    unsafe {
        asm!("mov {}, cr2", out(reg) cr2);
    }
    panic!(
        "EXCEPTION: PAGE FAULT\nAccessed Address: {cr2:x}\nError Code: {error_code:?}\n{frame:#?}"
    );
}

#[unsafe(no_mangle)]
extern "C" fn irq_dispatch(vector: u64) {
    // 1. Dispatch to registered handler
    if !crate::x86_64::apic::dispatch(vector as u8) {
        // TEAM_303: Log unhandled IRQs to serial
    }

    // 2. Signal EOI to APIC
    crate::x86_64::apic::APIC.signal_eoi();
}

exception_handler!(de_wrapper, divide_error_handler);
exception_handler!(db_wrapper, debug_handler);
exception_handler!(bp_wrapper, breakpoint_handler);
exception_handler!(ud_wrapper, invalid_opcode_handler);
exception_handler_err!(df_wrapper, double_fault_handler);
exception_handler_err!(gp_wrapper, general_protection_fault_handler);
exception_handler_err!(pf_wrapper, page_fault_handler);

irq_handler!(irq32_wrapper, 32); // PIT Timer
irq_handler!(irq36_wrapper, 36); // COM1 Serial

pub fn init() {
    let mut idt = IDT.lock();
    idt.set_handler(0, de_wrapper as *const () as u64);
    idt.set_handler(1, db_wrapper as *const () as u64);
    idt.set_handler(3, bp_wrapper as *const () as u64);
    idt.set_handler(6, ud_wrapper as *const () as u64);
    idt.set_handler(8, df_wrapper as *const () as u64);
    idt.set_handler(13, gp_wrapper as *const () as u64);
    idt.set_handler(14, pf_wrapper as *const () as u64);

    // IRQs
    idt.set_handler(32, irq32_wrapper as *const () as u64);
    idt.set_handler(36, irq36_wrapper as *const () as u64);
}
