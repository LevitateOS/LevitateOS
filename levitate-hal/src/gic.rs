use core::ptr::{read_volatile, write_volatile};

pub const GICD_BASE: usize = 0x08000000;
pub const GICC_BASE: usize = 0x08010000;

// Distributor registers
const GICD_CTLR: usize = 0x000;
const GICD_TYPER: usize = 0x004;
const GICD_ISENABLER: usize = 0x100;
const GICD_ICENABLER: usize = 0x180;
const GICD_ICPENDR: usize = 0x280;
const GICD_IPRIORITYR: usize = 0x400;
const GICD_ITARGETSR: usize = 0x800;
const GICD_ICFGR: usize = 0xC00;

// CPU Interface registers
const GICC_CTLR: usize = 0x000;
const GICC_PMR: usize = 0x004;
const GICC_IAR: usize = 0x00C;
const GICC_EOIR: usize = 0x010;

pub const GIC_MAX_IRQ: u32 = 256;
pub const GIC_SPI_START: u32 = 32;

pub struct Gic {
    dist_base: usize,
    cpu_base: usize,
}

// GIC is safe to share between threads if we are careful (or if we trust single-core + interrupt logic)
unsafe impl Sync for Gic {}

pub static API: Gic = Gic::new(GICD_BASE, GICC_BASE);

impl Gic {
    pub const fn new(dist_base: usize, cpu_base: usize) -> Self {
        Self {
            dist_base,
            cpu_base,
        }
    }

    unsafe fn gicd_write(&self, offset: usize, value: u32) {
        unsafe {
            write_volatile((self.dist_base + offset) as *mut u32, value);
            core::arch::asm!("dmb sy");
        }
    }

    unsafe fn gicd_read(&self, offset: usize) -> u32 {
        unsafe {
            let val = read_volatile((self.dist_base + offset) as *const u32);
            core::arch::asm!("dmb sy");
            val
        }
    }

    unsafe fn gicc_write(&self, offset: usize, value: u32) {
        unsafe {
            write_volatile((self.cpu_base + offset) as *mut u32, value);
            core::arch::asm!("dmb sy");
        }
    }

    unsafe fn gicc_read(&self, offset: usize) -> u32 {
        unsafe {
            let val = read_volatile((self.cpu_base + offset) as *const u32);
            core::arch::asm!("dmb sy");
            val
        }
    }

    pub fn init(&self) {
        unsafe {
            // Disable distributor
            self.gicd_write(GICD_CTLR, 0);

            let typer = self.gicd_read(GICD_TYPER);
            let num_irqs = ((typer & 0x1F) + 1) * 32;
            let num_irqs = if num_irqs > GIC_MAX_IRQ {
                GIC_MAX_IRQ
            } else {
                num_irqs
            };

            // Disable all interrupts
            for i in 0..(num_irqs / 32) {
                self.gicd_write(GICD_ICENABLER + (i as usize * 4), 0xFFFFFFFF);
            }

            // Clear all pending
            for i in 0..(num_irqs / 32) {
                self.gicd_write(GICD_ICPENDR + (i as usize * 4), 0xFFFFFFFF);
            }

            // Set priority to lowest
            for i in 0..(num_irqs / 4) {
                self.gicd_write(GICD_IPRIORITYR + (i as usize * 4), 0xA0A0A0A0);
            }

            // Target SPIs to CPU0
            for i in (GIC_SPI_START / 4)..(num_irqs / 4) {
                self.gicd_write(GICD_ITARGETSR + (i as usize * 4), 0x01010101);
            }

            // Configure level-triggered
            for i in (GIC_SPI_START / 16)..(num_irqs / 16) {
                self.gicd_write(GICD_ICFGR + (i as usize * 4), 0);
            }

            // Enable distributor
            self.gicd_write(GICD_CTLR, 1);

            // CPU Interface init
            self.gicc_write(GICC_PMR, 0xFF);
            self.gicc_write(GICC_CTLR, 1);
        }
    }

    pub fn acknowledge(&self) -> u32 {
        unsafe { self.gicc_read(GICC_IAR) & 0x3FF }
    }

    pub fn end_interrupt(&self, irq: u32) {
        unsafe {
            self.gicc_write(GICC_EOIR, irq);
        }
    }

    pub fn enable_irq(&self, irq: u32) {
        if irq >= GIC_MAX_IRQ {
            return;
        }
        let reg = irq / 32;
        let bit = irq % 32;
        unsafe {
            self.gicd_write(GICD_ISENABLER + (reg as usize * 4), 1 << bit);
        }
    }

    pub fn disable_irq(&self, irq: u32) {
        if irq >= GIC_MAX_IRQ {
            return;
        }
        let reg = irq / 32;
        let bit = irq % 32;
        unsafe {
            self.gicd_write(GICD_ICENABLER + (reg as usize * 4), 1 << bit);
        }
    }
}
