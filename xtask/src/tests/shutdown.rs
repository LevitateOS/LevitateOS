//! Shutdown Behavior Test
//!
//! TEAM_142: Tests graceful shutdown by sending "exit --verbose" to shell
//! and verifying the output matches golden_shutdown.txt

use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const GOLDEN_SHUTDOWN_FILE: &str = "tests/golden_shutdown.txt";
const ACTUAL_SHUTDOWN_FILE: &str = "tests/actual_shutdown.txt";

/// Run the shutdown behavior test
pub fn run(arch: &str) -> Result<()> {
    println!("=== Shutdown Behavior Test for {} ===\n", arch);

    // Build everything first
    crate::build::build_kernel_verbose(arch)?;

    let qemu_bin = match arch {
        "aarch64" => "qemu-system-aarch64",
        "x86_64" => "qemu-system-x86_64",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    // Kill any existing QEMU
    let _ = Command::new("pkill")
        .args(["-f", qemu_bin])
        .status();

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
        "-nographic",
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
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to start QEMU")?;

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let mut stdout = child.stdout.take().expect("Failed to get stdout");

    // Set stdout to non-blocking
    use std::os::unix::io::AsRawFd;
    let fd = stdout.as_raw_fd();
    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL);
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    }

    let mut all_output = String::new();
    let mut buf = [0u8; 4096];

    // Wait for shell prompt
    println!("Waiting for shell prompt...");
    let start = Instant::now();
    let timeout = Duration::from_secs(30);

    while start.elapsed() < timeout {
        match stdout.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                all_output.push_str(&chunk);
                
                // Look for shell prompt after banner
                if all_output.contains("# ") && all_output.contains("LevitateOS Shell") {
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => break,
        }
    }

    if !all_output.contains("# ") {
        let _ = child.kill();
        let _ = child.wait();
        bail!("Shell prompt not found within timeout");
    }

    println!("Shell ready. Sending 'exit --verbose'...");
    
    // Send exit --verbose command
    stdin.write_all(b"exit --verbose\n")?;
    stdin.flush()?;

    // Capture shutdown output - wait longer since kernel may take time
    let shutdown_start = Instant::now();
    let shutdown_timeout = Duration::from_secs(15);
    let mut shutdown_output = String::new();
    let mut consecutive_empty = 0;

    while shutdown_start.elapsed() < shutdown_timeout {
        match stdout.read(&mut buf) {
            Ok(0) => break, // EOF - QEMU exited (good!)
            Ok(n) => {
                consecutive_empty = 0;
                let chunk = String::from_utf8_lossy(&buf[..n]);
                shutdown_output.push_str(&chunk);
                
                // Check if shutdown completed
                if shutdown_output.contains("[SHUTDOWN] Goodbye!") {
                    println!("Shutdown sequence completed.");
                    // Wait a bit more to capture any trailing output
                    std::thread::sleep(Duration::from_millis(500));
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                consecutive_empty += 1;
                // If we've had output and now nothing for a while, QEMU might have halted
                if consecutive_empty > 40 && !shutdown_output.is_empty() {
                    println!("No more output after {}ms, checking result...", consecutive_empty * 50);
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => break,
        }
    }

    // Wait for QEMU to exit (shutdown halts the CPU)
    std::thread::sleep(Duration::from_millis(500));
    let _ = child.kill();
    let _ = child.wait();

    // Save actual output
    fs::write(ACTUAL_SHUTDOWN_FILE, &shutdown_output)
        .context("Failed to write actual shutdown output")?;

    // Read golden file
    let golden = fs::read_to_string(GOLDEN_SHUTDOWN_FILE)
        .context("Failed to read golden shutdown file")?;

    // Normalize and compare
    let golden_lines: Vec<&str> = golden.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .collect();
    
    let actual_lines: Vec<&str> = shutdown_output.lines()
        .filter(|l| l.contains("exit") || l.contains("Goodbye") || l.contains("[SHUTDOWN]"))
        .collect();

    println!("\n--- Expected Shutdown Sequence ---");
    for line in &golden_lines {
        println!("  {}", line);
    }
    
    println!("\n--- Actual Shutdown Output ---");
    for line in &actual_lines {
        println!("  {}", line);
    }

    // Check that all golden lines are present in actual output
    let mut all_found = true;
    for expected in &golden_lines {
        if !actual_lines.iter().any(|a| a.contains(expected.trim())) {
            println!("❌ Missing: {}", expected);
            all_found = false;
        }
    }

    if all_found && !actual_lines.is_empty() {
        println!("\n✅ SUCCESS: Shutdown sequence matches golden file!");
        Ok(())
    } else {
        println!("\n❌ FAILURE: Shutdown sequence mismatch!");
        println!("\nFull shutdown output:\n{}", shutdown_output);
        bail!("Shutdown behavior test failed")
    }
}
