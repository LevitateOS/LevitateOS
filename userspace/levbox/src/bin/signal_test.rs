#![no_std]
#![no_main]

extern crate ulib;
use ulib::libsyscall;
use ulib::prelude::println;
use ulib::{kill, pause, signal, Signal};

#[no_mangle]
pub fn main() -> i32 {
    println!("Signal test starting...");

    // Register a handler for SIGINT (2)
    signal(Signal::SIGINT, Some(handle_sigint));

    println!("Registered SIGINT handler. Sending SIGINT to self...");

    // Send SIGINT to self
    let pid = libsyscall::getpid() as i32;
    kill(pid, Signal::SIGINT);

    println!("Signal sent. If handled, we should see handler output.");

    // The signal should be delivered on the return from kill() or during pause()
    println!("Waiting for signal in pause()...");
    pause();

    println!("pause() returned (interrupted by signal).");
    println!("Signal test complete.");
    0
}

extern "C" fn handle_sigint(sig: i32) {
    // We are in a signal handler!
    // Using direct syscall write for async-signal-safety
    let msg = b"*** HANDLER: Received signal ";
    let _ = libsyscall::write(1, msg);

    // Simple hex print for signal number
    let mut buf = [0u8; 2];
    buf[0] = (sig as u8 / 10) + b'0';
    buf[1] = (sig as u8 % 10) + b'0';
    let _ = libsyscall::write(1, &buf);
    let _ = libsyscall::write(1, b" ***\n");
}
