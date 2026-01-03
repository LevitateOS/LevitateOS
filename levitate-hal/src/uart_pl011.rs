use bitflags::bitflags;
use core::ptr::{read_volatile, write_volatile};

bitflags! {
    /// Flag register bits (FR).
    pub struct FlagFlags: u32 {
        /// Transmit FIFO full.
        const TXFF = 1 << 5;
        /// Receive FIFO empty.
        const RXFE = 1 << 4;
        /// UART busy.
        const BUSY = 1 << 3;
    }
}

bitflags! {
    /// Control register bits (CR).
    pub struct ControlFlags: u32 {
        /// UART enable.
        const UARTEN = 1 << 0;
        /// Transmit enable.
        const TXE    = 1 << 8;
        /// Receive enable.
        const RXE    = 1 << 9;
    }
}

bitflags! {
    /// Line Control register bits (LCR_H).
    pub struct LineControlFlags: u32 {
        /// Enable FIFOs.
        const FEN    = 1 << 4;
        /// Word length: 8 bits.
        const WLEN_8 = 0b11 << 5;
    }
}

bitflags! {
    /// Interrupt Mask Set/Clear register (IMSC).
    pub struct InterruptFlags: u32 {
        /// Receive interrupt mask.
        const RXIM = 1 << 4;
        /// Transmit interrupt mask.
        const TXIM = 1 << 5;
        /// Receive timeout interrupt mask.
        const RTIM = 1 << 6;
    }
}

// TEAM_017: FIFO interrupt level configuration
const IFLS_RX4_8: u32 = 2 << 3; // RX FIFO becomes 4/8 full
const IFLS_TX4_8: u32 = 2 << 0; // TX FIFO becomes 4/8 empty

#[repr(transparent)]
struct Reg<T>(T);

impl<T> Reg<T> {
    fn read(&self) -> T {
        unsafe { read_volatile(&self.0) }
    }
    fn write(&mut self, val: T) {
        unsafe { write_volatile(&mut self.0, val) }
    }
}

#[repr(C)]
struct Registers {
    dr: Reg<u32>,     // 0x00
    rsrecr: Reg<u32>, // 0x04
    _reserved0: [u32; 4],
    fr: Reg<u32>, // 0x18
    _reserved1: u32,
    ilpr: Reg<u32>,  // 0x20
    ibrd: Reg<u32>,  // 0x24
    fbrd: Reg<u32>,  // 0x28
    lcr_h: Reg<u32>, // 0x2C
    cr: Reg<u32>,    // 0x30
    ifls: Reg<u32>,  // 0x34
    imsc: Reg<u32>,  // 0x38
    ris: Reg<u32>,   // 0x3C
    mis: Reg<u32>,   // 0x40
    icr: Reg<u32>,   // 0x44
}

pub struct Pl011Uart {
    base: usize,
}

impl Pl011Uart {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    fn regs(&self) -> &Registers {
        unsafe { &*(self.base as *const Registers) }
    }

    fn regs_mut(&mut self) -> &mut Registers {
        unsafe { &mut *(self.base as *mut Registers) }
    }

    pub fn init(&mut self) {
        // 1. Disable UART
        self.regs_mut().cr.write(0);

        // 2. Clear interrupts
        self.regs_mut().icr.write(0x7FF);

        // 3. Configure FIFO interrupt levels (TEAM_017)
        self.regs_mut().ifls.write(IFLS_RX4_8 | IFLS_TX4_8);

        // 4. Line Control (8n1, FIFOs enabled)
        self.regs_mut()
            .lcr_h
            .write((LineControlFlags::WLEN_8 | LineControlFlags::FEN).bits());

        // 5. Enable UART, TX, RX
        self.regs_mut()
            .cr
            .write((ControlFlags::UARTEN | ControlFlags::TXE | ControlFlags::RXE).bits());

        // 6. Drain stale data from RX FIFO (TEAM_017)
        self.drain_fifo();
    }

    /// Drain any stale bytes from the RX FIFO.
    /// TEAM_017: Matches Redox drain_fifo() behavior.
    pub fn drain_fifo(&mut self) {
        for _ in 0..32 {
            if FlagFlags::from_bits_truncate(self.regs().fr.read()).contains(FlagFlags::RXFE) {
                break;
            }
            let _ = self.regs().dr.read();
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        // Wait while TX FIFO is full
        while FlagFlags::from_bits_truncate(self.regs().fr.read()).contains(FlagFlags::TXFF) {
            core::hint::spin_loop();
        }
        self.regs_mut().dr.write(byte as u32);
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if FlagFlags::from_bits_truncate(self.regs().fr.read()).contains(FlagFlags::RXFE) {
            None
        } else {
            Some(self.regs().dr.read() as u8)
        }
    }

    pub fn enable_rx_interrupt(&mut self) {
        let mut imsc = InterruptFlags::from_bits_truncate(self.regs().imsc.read());
        imsc.insert(InterruptFlags::RXIM);
        imsc.insert(InterruptFlags::RTIM);
        self.regs_mut().imsc.write(imsc.bits());
    }

    pub fn clear_interrupts(&mut self) {
        self.regs_mut().icr.write(0x7FF);
    }
}
