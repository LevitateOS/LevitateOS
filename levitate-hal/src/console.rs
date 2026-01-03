use core::fmt::{self, Write};
use levitate_utils::Spinlock;

pub const UART0_BASE: usize = 0x0900_0000;

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            core::ptr::write_volatile(self.base as *mut u8, byte);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

static WRITER: Spinlock<Uart> = Spinlock::new(Uart::new(UART0_BASE));

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn print_hex(val: u64) {
    let mut writer = WRITER.lock();
    let _ = writer.write_str("0x");
    for i in (0..16).rev() {
        let nibble = (val >> (i * 4)) & 0xf;
        let c = if nibble < 10 {
            (b'0' + nibble as u8) as char
        } else {
            (b'a' + (nibble - 10) as u8) as char
        };
        let _ = writer.write_str(core::str::from_utf8(&[c as u8]).unwrap());
    }
}
