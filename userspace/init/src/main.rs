//! TEAM_120: LevitateOS Init Process (PID 1)
//!
//! Responsible for starting the system and managing services.
//! Currently just starts the shell.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use libsyscall::{common_panic_handler, println, spawn};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    common_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let pid = libsyscall::getpid();
    println!("[INIT] PID {} starting...", pid);

    // Spawn the shell
    println!("[INIT] Spawning shell...");
    let shell_pid = spawn("shell");

    if shell_pid < 0 {
        println!("[INIT] ERROR: Failed to spawn shell: {}", shell_pid);
    } else {
        println!("[INIT] Shell spawned as PID {}", shell_pid);
    }

    // PID 1 must never exit
    loop {
        // In a real OS, we would wait for children here
        // For now, just yield or loop
        for _ in 0..1000000 {
            core::hint::spin_loop();
        }
    }
}
