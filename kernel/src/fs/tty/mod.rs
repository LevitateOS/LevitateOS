//! TEAM_247: TTY and Terminal support for LevitateOS.
//!
//! Implements POSIX terminal features including line discipline,
//! termios configuration, and signal generation.

use alloc::sync::Arc;
use los_utils::Mutex;

/// TEAM_247: Number of control characters in termios.
pub const NCCS: usize = 32;

/// TEAM_247: termios structure (matches Linux AArch64 layout)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; NCCS],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

// Local mode flags (c_lflag)
pub const ISIG: u32 = 0x01;
pub const ICANON: u32 = 0x02;
pub const ECHO: u32 = 0x08;
pub const ECHOE: u32 = 0x10;
pub const ECHOK: u32 = 0x20;
pub const ECHONL: u32 = 0x40;
pub const NOFLSH: u32 = 0x80;
pub const TOSTOP: u32 = 0x100;
pub const IEXTEN: u32 = 0x8000;

// Output mode flags (c_oflag)
pub const OPOST: u32 = 0x01;
pub const ONLCR: u32 = 0x04;

// special characters (c_cc index)
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VTIME: usize = 5;
pub const VMIN: usize = 6;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;

pub mod pty;

// ioctl requests
pub const TCGETS: u64 = 0x5401;
pub const TCSETS: u64 = 0x5402;
pub const TCSETSW: u64 = 0x5403;
pub const TCSETSF: u64 = 0x5404;

pub const TIOCGPTN: u64 = 0x80045430;
pub const TIOCSPTLCK: u64 = 0x40045431;
pub const TIOCGWINSZ: u64 = 0x5413;
pub const TIOCSWINSZ: u64 = 0x5414;

impl Termios {
    pub const INITIAL_TERMIOS: Termios = {
        let mut cc = [0u8; NCCS];
        cc[VINTR] = 0x03; // Ctrl+C
        cc[VQUIT] = 0x1C; // Ctrl+\
        cc[VERASE] = 0x7F; // DEL
        cc[VKILL] = 0x15; // Ctrl+U
        cc[VEOF] = 0x04; // Ctrl+D
        cc[VSTART] = 0x11; // Ctrl+Q
        cc[VSTOP] = 0x13; // Ctrl+S
        cc[VSUSP] = 0x1A; // Ctrl+Z

        Termios {
            c_iflag: 0x0500, // ICRNL | IXON (common defaults)
            c_oflag: 0x0005, // OPOST | ONLCR (common defaults)
            c_cflag: 0x00BF, // B38400 | CS8 | CREAD | HUPCL
            c_lflag: ISIG | ICANON | ECHO | ECHOE | ECHOK | IEXTEN,
            c_line: 0,
            c_cc: cc,
            c_ispeed: 38400,
            c_ospeed: 38400,
        }
    };
}

impl Default for Termios {
    fn default() -> Self {
        Self::INITIAL_TERMIOS
    }
}

use alloc::collections::VecDeque;

/// TEAM_247: TTY state for a terminal device.
pub struct TtyState {
    pub termios: Termios,
    /// Chars ready to be read by the process
    pub input_buffer: VecDeque<u8>,
    /// Chars being edited in canonical mode
    pub canon_buffer: VecDeque<u8>,
    /// Whether output is stopped by IXON (Ctrl+S)
    pub stopped: bool,
    /// TEAM_247: Optional buffer for terminal emulator (PTY master)
    pub master_buffer: Option<Arc<Mutex<VecDeque<u8>>>>,
}

impl TtyState {
    pub fn new() -> Self {
        Self {
            termios: Termios::INITIAL_TERMIOS,
            input_buffer: VecDeque::new(),
            canon_buffer: VecDeque::new(),
            stopped: false,
            master_buffer: None,
        }
    }

    /// Process a raw input byte from hardware.
    /// Returns true if something was added to input_buffer.
    pub fn process_input(&mut self, mut byte: u8) -> bool {
        // 1. Input processing (iflag)
        const IXON: u32 = 0x0400;
        if (self.termios.c_iflag & IXON) != 0 {
            if byte == self.termios.c_cc[VSTOP] {
                self.stopped = true;
                return false;
            }
            if byte == self.termios.c_cc[VSTART] {
                self.stopped = false;
                return false;
            }
        }

        // If we were stopped and any char is received (with IXANY? but we'll just resume on any for now or specific)
        // Actually IXON usually means VSTART resumes.

        if byte == b'\r' && (self.termios.c_iflag & 0x0100) != 0 {
            // ICRNL
            byte = b'\n';
        }

        // 2. Local processing/Signals (lflag)
        if (self.termios.c_lflag & ISIG) != 0 {
            if byte == self.termios.c_cc[VINTR] {
                crate::syscall::signal::signal_foreground_process(crate::syscall::signal::SIGINT);
                return false;
            }
            if byte == self.termios.c_cc[VQUIT] {
                crate::syscall::signal::signal_foreground_process(3); // SIGQUIT = 3
                return false;
            }
            if byte == self.termios.c_cc[VSUSP] {
                crate::syscall::signal::signal_foreground_process(20); // SIGTSTP = 20
                return false;
            }
        }

        // 3. Echoing
        if (self.termios.c_lflag & ECHO) != 0 {
            if byte == b'\n' {
                self.echo(b'\r');
                self.echo(b'\n');
            } else if byte < 32 && byte != b'\t' {
                // Echo as ^X
                self.echo(b'^');
                self.echo(byte + 64);
            } else {
                self.echo(byte);
            }
        }

        // 4. Line Discipline (Canonical vs Non-canonical)
        if (self.termios.c_lflag & ICANON) != 0 {
            if byte == self.termios.c_cc[VERASE] {
                if let Some(_last) = self.canon_buffer.pop_back() {
                    if (self.termios.c_lflag & ECHOE) != 0 {
                        // Visual erase: backspace - space - backspace
                        self.echo(b'\x08');
                        self.echo(b' ');
                        self.echo(b'\x08');
                    }
                }
                return false;
            }
            if byte == self.termios.c_cc[VKILL] {
                self.canon_buffer.clear();
                // TODO: Visual kill
                return false;
            }
            if byte == self.termios.c_cc[VEOF] {
                // EOF - flush canon_buffer to input_buffer
                while let Some(b) = self.canon_buffer.pop_front() {
                    self.input_buffer.push_back(b);
                }
                return true;
            }

            self.canon_buffer.push_back(byte);
            if byte == b'\n' {
                // EOL - flush canon_buffer to input_buffer
                while let Some(b) = self.canon_buffer.pop_front() {
                    self.input_buffer.push_back(b);
                }
                return true;
            }
        } else {
            // Non-canonical: straight to input buffer
            self.input_buffer.push_back(byte);
            return true;
        }

        false
    }

    fn echo(&mut self, byte: u8) {
        if let Some(ref buffer) = self.master_buffer {
            buffer.lock().push_back(byte);
        } else {
            los_hal::print!("{}", byte as char);
        }
    }
}

/// TEAM_247: Global console TTY state.
pub static CONSOLE_TTY: Mutex<TtyState> = Mutex::new(TtyState {
    termios: Termios::INITIAL_TERMIOS,
    input_buffer: VecDeque::new(),
    canon_buffer: VecDeque::new(),
    stopped: false,
    master_buffer: None,
});
