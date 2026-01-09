//! Serial Input Test
//!
//! TEAM_139: Automated test for verifying serial console input works.
//! Starts QEMU with -nographic, pipes input, verifies echo.

use anyhow::bail;
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub fn run(arch: &str) -> Result<()> {
    println!("=== Serial Input Test for {} ===\n", arch);

    // Build everything first
    crate::build::build_kernel_verbose(arch)?;

    let qemu_bin = match arch {
        "aarch64" => "qemu-system-aarch64",
        "x86_64" => "qemu-system-x86_64",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    let kernel_bin = if arch == "aarch64" {
        "kernel64_rust.bin"
    } else {
        "target/x86_64-unknown-none/release/levitate-kernel"
    };

    let args = vec![
        "-M", if arch == "aarch64" { "virt" } else { "q35" },
        "-cpu", if arch == "aarch64" { "cortex-a72" } else { "qemu64" },
        "-m", "512M",
        "-kernel", kernel_bin,
        "-nographic", // Critical for serial input
        "-device", "virtio-gpu-pci",
        "-device", "virtio-keyboard-device",
        "-device", "virtio-tablet-device",
        "-device", "virtio-net-device,netdev=net0",
        "-netdev", "user,id=net0",
        "-drive", "file=tinyos_disk.img,format=raw,if=none,id=hd0",
        "-device", "virtio-blk-device,drive=hd0",
        // TEAM_327: Use arch-specific initramfs
        "-initrd", "initramfs_aarch64.cpio",
        "-serial", "mon:stdio",
        "-no-reboot",
    ];

    println!("Starting QEMU...");
    let mut child = Command::new(qemu_bin)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start QEMU")?;

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let mut stdout = child.stdout.take().expect("Failed to get stdout");

    // We need to read stdout non-blocking to check for prompt
    // For simplicity, we'll spawn a thread to read stdout and print/buffer it
    let (tx, rx) = std::sync::mpsc::channel();
    let stdout_thread = std::thread::spawn(move || {
        let mut buf = [0u8; 128];
        loop {
            if let Ok(n) = stdout.read(&mut buf) {
                if n == 0 { break; }
                let s = String::from_utf8_lossy(&buf[..n]);
                print!("{}", s); // Mirror to our stdout
                let _ = tx.send(s.into_owned());
            } else {
                break;
            }
        }
    });

    // Send "help" and check for output
    // Wait a bit for boot
    std::thread::sleep(Duration::from_secs(5));
    
    println!("\nSending 'help' command...");
    stdin.write_all(b"help\n")?;
    stdin.flush()?;

    // Check captured output for response
    let start = Instant::now();
    let mut found = false;
    let mut buffer = String::new();
    
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(chunk) = rx.try_recv() {
            buffer.push_str(&chunk);
            if buffer.contains("Available commands:") {
                found = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    let _ = child.kill();
    let _ = stdout_thread.join();

    if found {
        println!("✅ SUCCESS: Serial input received and echoed!");
        Ok(())
    } else {
        bail!("Failed to get expected response from serial input");
    }
}
