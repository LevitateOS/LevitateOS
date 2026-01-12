//! Coreutils Test Suite
//!
//! TEAM_465: Automated test runner for /root/test-core.sh
//! Boots LevitateOS, runs the coreutils test script, and reports results.

use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader, Write};
use std::process::Stdio;
use std::time::{Duration, Instant};

use crate::qemu::{Arch, QemuBuilder, QemuProfile};

/// Run the coreutils test suite
///
/// # Arguments
/// * `arch` - Target architecture (x86_64 or aarch64)
/// * `phase` - Which phase(s) to run: "all", "1", "2", "1-5", etc.
pub fn run(arch: &str, phase: Option<&str>) -> Result<()> {
    let phase_arg = phase.unwrap_or("all");
    println!("=== Coreutils Test Suite for {} (Phase: {}) ===\n", arch, phase_arg);

    // Build everything first (including ISO for x86_64)
    println!("Building kernel and userspace...");
    if arch == "x86_64" {
        crate::build::build_iso(arch)?;
    } else {
        crate::build::build_all(arch)?;
    }

    // Use QemuBuilder for proper configuration
    let qemu_arch = Arch::try_from(arch)?;
    let profile = match arch {
        "x86_64" => QemuProfile::X86_64,
        _ => QemuProfile::Default,
    };

    let mut builder = QemuBuilder::new(qemu_arch, profile)
        .display_nographic();

    // x86_64 boots from ISO
    if arch == "x86_64" {
        builder = builder.boot_iso();
    }

    let mut cmd = builder.build()?;

    println!("Starting QEMU...");
    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start QEMU")?;

    let mut stdin = child.stdin.take().expect("Failed to get stdin");
    let stdout = child.stdout.take().expect("Failed to get stdout");

    // Create a channel for stdout lines
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let reader = BufReader::new(stdout);

    let stdout_thread = std::thread::spawn(move || {
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
                let _ = tx.send(line);
            }
        }
    });

    // Wait for shell prompt (LevitateOS#)
    println!("\nWaiting for shell prompt...");
    let start = Instant::now();
    let mut shell_ready = false;

    while start.elapsed() < Duration::from_secs(60) {
        if let Ok(line) = rx.try_recv() {
            if line.contains("LevitateOS#") || (line.contains("SUCCESS") && line.contains("System Ready")) {
                shell_ready = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    if !shell_ready {
        let _ = child.kill();
        let _ = stdout_thread.join();
        bail!("Timed out waiting for shell prompt");
    }

    // TEAM_466: Give the shell extra time to be ready after prompt appears
    // The shell may still be initializing even after printing the prompt
    std::thread::sleep(Duration::from_millis(1000));

    // Drain any remaining output before sending command
    while rx.try_recv().is_ok() {}

    // Run the test script with the phase argument
    // TEAM_466: Script at root level (kernel has issues with subdirectory files)
    // TEAM_466: Send characters slowly to avoid serial input mangling
    let test_cmd = format!("sh /test-core.sh {}\n", phase_arg);
    let test_cmd_trimmed = test_cmd.trim();
    println!("\nSending: {}", test_cmd_trimmed);

    // Send character by character with 20ms delays to avoid input dropping
    // TEAM_466: Increased from 10ms after observing intermittent character loss
    for byte in test_cmd.bytes() {
        stdin.write_all(&[byte])?;
        stdin.flush()?;
        std::thread::sleep(Duration::from_millis(20));
    }
    // Extra delay after newline to ensure command is processed
    std::thread::sleep(Duration::from_millis(200));

    // Wait for test completion
    let start = Instant::now();
    let mut test_passed = false;
    let mut test_completed = false;
    let mut pass_count = 0u32;
    let mut fail_count = 0u32;

    while start.elapsed() < Duration::from_secs(120) {
        if let Ok(line) = rx.try_recv() {
            // Look for result summary
            if line.contains("ALL TESTS PASSED") {
                test_passed = true;
                test_completed = true;
            } else if line.contains("TESTS FAILED") {
                test_completed = true;
            } else if line.contains("Passed:") {
                if let Some(n) = line.split_whitespace().last() {
                    pass_count = n.parse().unwrap_or(0);
                }
            } else if line.contains("Failed:") {
                if let Some(n) = line.split_whitespace().last() {
                    fail_count = n.parse().unwrap_or(0);
                }
            }

            if test_completed {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    let _ = child.kill();
    let _ = stdout_thread.join();

    // Report results
    println!("\n=== Test Results ===");
    println!("Passed: {}", pass_count);
    println!("Failed: {}", fail_count);

    if test_passed {
        println!("\n✅ SUCCESS: All coreutils tests passed!");
        Ok(())
    } else if test_completed {
        bail!("❌ FAILED: {} test(s) failed", fail_count);
    } else {
        bail!("❌ TIMEOUT: Tests did not complete within 120 seconds");
    }
}
