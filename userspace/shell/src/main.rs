//! `TEAM_081`: `LevitateOS` Shell (`lsh`)
//!
//! Interactive shell for `LevitateOS` Phase 8b.
//! Supports builtin commands: `echo`, `help`, `clear`, `exit`
//!
//! `TEAM_118`: Refactored to use `libsyscall`.
//!
//! TEAM_158: Behavior IDs [SH1]-[SH7] for traceability.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libsyscall::{common_panic_handler, print, println};

// ============================================================================
// Panic Handler
// ============================================================================

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    common_panic_handler(info)
}

// ============================================================================
// Shell Logic
// ============================================================================

/// Trim whitespace from both ends of a byte slice.
fn trim(s: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = s.len();
    while start < end && matches!(s[start], b' ' | b'\t' | b'\n' | b'\r') {
        start += 1;
    }
    while end > start && matches!(s[end - 1], b' ' | b'\t' | b'\n' | b'\r') {
        end -= 1;
    }
    &s[start..end]
}

/// Check if two byte slices are equal.
fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

/// Check if slice starts with prefix.
fn starts_with(s: &[u8], prefix: &[u8]) -> bool {
    s.len() >= prefix.len() && bytes_eq(&s[..prefix.len()], prefix)
}

/// Execute a command.
fn execute(line: &[u8]) {
    let cmd = trim(line);
    if cmd.is_empty() {
        return;
    }

    // [SH7] Builtin: exit command terminates shell
    // exit         -> graceful shutdown (minimal output)
    // exit --verbose -> graceful shutdown with golden file output
    if bytes_eq(cmd, b"exit") {
        println!("Goodbye!");
        libsyscall::shutdown(libsyscall::shutdown_flags::NORMAL); // [SH7]
    }
    if bytes_eq(cmd, b"exit --verbose") {
        println!("Goodbye! (verbose shutdown for golden file)");
        libsyscall::shutdown(libsyscall::shutdown_flags::VERBOSE); // [SH7]
    }

    // Builtin: test (POSIX: exit 0 if no args, used for shell scripting)
    if bytes_eq(cmd, b"test") {
        // No output, exit 0 (implicit return in shell)
        return;
    }

    // [SH6] Builtin: help command shows available commands
    if bytes_eq(cmd, b"help") {
        println!("LevitateOS Shell (lsh) v0.1");
        println!("Commands:");
        println!("  echo <text>    - Print text");
        println!("  help           - Show this help");
        println!("  clear          - Clear screen");
        println!("  exit           - Shutdown system");
        println!("  exit --verbose - Shutdown with detailed output");
        println!("  test           - Exit 0 (for scripting)");
        return; // [SH6]
    }

    // Builtin: clear (ANSI escape)
    if bytes_eq(cmd, b"clear") {
        print!("\x1b[2J\x1b[H");
        return;
    }

    // [SH5] Builtin: echo command outputs text
    if starts_with(cmd, b"echo ") {
        if let Ok(s) = core::str::from_utf8(&cmd[5..]) {
            println!("{}", s); // [SH5] echo outputs text
        }
        return;
    }
    if bytes_eq(cmd, b"echo") {
        println!(); // [SH5] empty echo
        return;
    }

    // Unknown command
    print!("Unknown: ");
    if let Ok(s) = core::str::from_utf8(cmd) {
        println!("{}", s);
    } else {
        println!("<invalid utf8>");
    }
}

/// [SH1] Entry point for the shell - prints banner on startup.
/// [SH2] Prints # prompt.
/// [SH3] Reads input line.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // [SH1] Shell prints banner on startup
    println!();
    println!("LevitateOS Shell (lsh) v0.1");
    println!("Type 'help' for commands.");
    println!();

    let mut buf = [0u8; 256];

    loop {
        print!("# "); // [SH2] Shell prints # prompt
        let mut line_len = 0; // [SH3] reads input line
        'inner: loop {
            let mut c_buf = [0u8; 1];
            let n = libsyscall::read(0, &mut c_buf);
            if n > 0 {
                let b = c_buf[0];
                
                if b == b'\n' || b == b'\r' {
                    // Echo newline and execute
                    libsyscall::write(1, b"\n");
                    if line_len > 0 {
                        execute(&buf[..line_len]);
                    }
                    break 'inner;
                } else if b == 0x08 || b == 0x7f {
                    // Backspace: erase character from buffer and screen
                    if line_len > 0 {
                        line_len -= 1;
                        // Move back, overwrite with space, move back again
                        libsyscall::write(1, b"\x08 \x08");
                    }
                } else if line_len < buf.len() {
                    // Normal character: add to buffer and echo
                    buf[line_len] = b;
                    line_len += 1;
                    libsyscall::write(1, &c_buf[..1]);
                }
            }
        }
    }
}
