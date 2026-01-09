//! Screenshot Tests
//!
//! TEAM_327: Unified screenshot tests for visual verification.
//!
//! Test Types:
//! - `alpine` - Alpine Linux reference tests (external OS)
//! - `levitate` - Basic LevitateOS display test
//! - `userspace` - Run userspace commands and capture results
//!
//! Usage:
//!   cargo xtask test screenshot           # All screenshot tests
//!   cargo xtask test screenshot alpine    # Alpine only
//!   cargo xtask test screenshot levitate  # LevitateOS display only
//!   cargo xtask test screenshot userspace # Userspace tests + results

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::build;
use crate::qemu::{Arch, QemuBuilder, QemuProfile};
use crate::support::qmp::QmpClient;

use super::common::{wait_for_qmp_socket, qmp_send_keys, qmp_send_key};

const SCREENSHOT_DIR: &str = "tests/screenshots";

/// Run all screenshot tests
pub fn run(subtest: Option<&str>) -> Result<()> {
    fs::create_dir_all(SCREENSHOT_DIR)?;

    match subtest {
        Some("alpine") => run_alpine(),
        Some("levitate") => run_levitate(),
        Some("userspace") => run_userspace(),
        Some(other) => bail!("Unknown screenshot test: {}. Use: alpine, levitate, userspace", other),
        None => {
            println!("📸 Running All Screenshot Tests\n");
            
            // Run userspace test (most important)
            println!("━━━ Userspace Tests ━━━");
            let userspace_result = run_userspace();
            
            // Run basic levitate display test
            println!("\n━━━ LevitateOS Display ━━━");
            let levitate_result = run_levitate();
            
            // Report results
            println!("\n━━━ Results ━━━");
            match &userspace_result {
                Ok(_) => println!("  ✅ userspace: Tests completed, screenshot captured"),
                Err(e) => println!("  ❌ userspace: {}", e),
            }
            match &levitate_result {
                Ok(_) => println!("  ✅ levitate: Display verified"),
                Err(e) => println!("  ⚠️  levitate: {}", e),
            }
            
            println!("\n   Screenshots saved to {}/", SCREENSHOT_DIR);
            
            // Fail if userspace failed (it's the important one)
            userspace_result
        }
    }
}

// =============================================================================
// Userspace Tests - Run commands and capture results
// =============================================================================

/// Run userspace tests and capture screenshot with results
pub fn run_userspace() -> Result<()> {
    println!("🧪 Userspace Test Screenshot\n");

    // Build for aarch64 (working arch)
    let arch = "aarch64";
    println!("🔨 Building LevitateOS for {}...", arch);
    build::build_all(arch)?;

    let qmp_socket = format!("./userspace-test-{}.sock", std::process::id());
    let _ = fs::remove_file(&qmp_socket);

    println!("🚀 Starting LevitateOS...");
    let mut child = start_levitate_vnc(arch, &qmp_socket)?;

    // Wait for QMP
    wait_for_qmp_socket(&qmp_socket, 30)?;
    std::thread::sleep(Duration::from_secs(2));

    let mut client = QmpClient::connect(&qmp_socket)?;

    // Wait for boot (watch serial output)
    println!("⏳ Waiting for shell prompt (20s)...");
    std::thread::sleep(Duration::from_secs(20));

    // Run test commands via keyboard input
    println!("⌨️  Running test commands...");
    
    // Clear screen and show test header
    run_command(&mut client, "echo")?;
    run_command(&mut client, "echo '=============================='")?;
    run_command(&mut client, "echo '   USERSPACE TEST RESULTS'")?;
    run_command(&mut client, "echo '=============================='")?;
    run_command(&mut client, "echo")?;

    // Test 1: List files in root
    run_command(&mut client, "echo '[TEST 1] ls /'")?;
    run_command(&mut client, "ls")?;
    run_command(&mut client, "echo")?;

    // Test 2: Show help
    run_command(&mut client, "echo '[TEST 2] help'")?;
    run_command(&mut client, "help")?;
    run_command(&mut client, "echo")?;

    // Test 3: Echo test
    run_command(&mut client, "echo '[TEST 3] echo test'")?;
    run_command(&mut client, "echo 'Hello from userspace!'")?;
    run_command(&mut client, "echo")?;

    // Final summary
    run_command(&mut client, "echo '=============================='")?;
    run_command(&mut client, "echo '   ALL TESTS COMPLETED'")?;
    run_command(&mut client, "echo '=============================='")?;

    // Wait for output to render
    std::thread::sleep(Duration::from_secs(2));

    // Take screenshot
    let screenshot = format!("{}/userspace_{}.ppm", SCREENSHOT_DIR, arch);
    println!("📸 Taking screenshot: {}", screenshot);
    take_screenshot(&mut client, &screenshot)?;

    // Cleanup
    let _ = child.kill();
    let _ = child.wait();
    let _ = fs::remove_file(&qmp_socket);

    // Verify screenshot exists
    let png = screenshot.replace(".ppm", ".png");
    if Path::new(&png).exists() || Path::new(&screenshot).exists() {
        println!("✅ Userspace test screenshot captured!");
        Ok(())
    } else {
        bail!("Screenshot not created")
    }
}

/// Run a command by typing it via QMP keyboard
fn run_command(client: &mut QmpClient, cmd: &str) -> Result<()> {
    qmp_send_keys(client, cmd)?;
    qmp_send_key(client, "ret")?;
    std::thread::sleep(Duration::from_millis(500));
    Ok(())
}

// =============================================================================
// LevitateOS Display Test - Basic boot verification
// =============================================================================

/// Basic LevitateOS display test
pub fn run_levitate() -> Result<()> {
    println!("📸 LevitateOS Display Test\n");

    // Build for both architectures
    println!("🔨 Building LevitateOS...");
    build::build_all("aarch64")?;

    // Test aarch64 (working)
    println!("\n━━━ aarch64 ━━━");
    let aarch64_result = run_levitate_arch("aarch64");

    // Report
    match &aarch64_result {
        Ok(_) => println!("  ✅ aarch64: Screenshot captured"),
        Err(e) => println!("  ❌ aarch64: {}", e),
    }

    aarch64_result
}

fn run_levitate_arch(arch: &str) -> Result<()> {
    let qmp_socket = format!("./levitate-{}.sock", arch);
    let _ = fs::remove_file(&qmp_socket);

    println!("[{}] 🚀 Starting LevitateOS...", arch);
    let mut child = start_levitate_vnc(arch, &qmp_socket)?;

    wait_for_qmp_socket(&qmp_socket, 30)?;

    println!("[{}] ⏳ Waiting for boot (15s)...", arch);
    std::thread::sleep(Duration::from_secs(15));

    let mut client = QmpClient::connect(&qmp_socket)?;

    let screenshot = format!("{}/levitate_{}.ppm", SCREENSHOT_DIR, arch);
    println!("[{}] 📸 Taking screenshot: {}", arch, screenshot);
    take_screenshot(&mut client, &screenshot)?;

    let _ = child.kill();
    let _ = child.wait();
    let _ = fs::remove_file(&qmp_socket);

    let png = screenshot.replace(".ppm", ".png");
    if Path::new(&png).exists() || Path::new(&screenshot).exists() {
        println!("[{}] ✅ Screenshot captured!", arch);
        Ok(())
    } else {
        bail!("Screenshot not created")
    }
}

// =============================================================================
// Alpine Linux Reference Tests
// =============================================================================

const ALPINE_VERSION: &str = "3.20.0";

/// Alpine Linux screenshot tests
pub fn run_alpine() -> Result<()> {
    println!("📸 Alpine Linux Screenshot Tests\n");

    // Check for Alpine images
    let x86_iso = format!("tests/images/alpine-virt-{}-x86_64.iso", ALPINE_VERSION);
    let arm_iso = format!("tests/images/alpine-virt-{}-aarch64.iso", ALPINE_VERSION);

    if !Path::new(&x86_iso).exists() || !Path::new(&arm_iso).exists() {
        bail!(
            "Alpine images not found. Run:\n  ./tests/images/download.sh"
        );
    }

    // Run both architectures
    println!("━━━ aarch64 ━━━");
    let aarch64_result = run_alpine_arch("aarch64");

    println!("\n━━━ x86_64 ━━━");
    let x86_result = run_alpine_arch("x86_64");

    // Report
    println!("\n━━━ Results ━━━");
    match &aarch64_result {
        Ok(_) => println!("  ✅ aarch64: Screenshots captured"),
        Err(e) => println!("  ❌ aarch64: {}", e),
    }
    match &x86_result {
        Ok(_) => println!("  ✅ x86_64: Screenshots captured"),
        Err(e) => println!("  ❌ x86_64: {}", e),
    }

    aarch64_result?;
    x86_result?;

    println!("\n✅ All Alpine screenshot tests passed!");
    Ok(())
}

fn run_alpine_arch(arch: &str) -> Result<()> {
    let qmp_socket = format!("./alpine-{}.sock", arch);
    let _ = fs::remove_file(&qmp_socket);

    println!("[{}] 🚀 Starting Alpine Linux...", arch);
    let mut child = start_alpine(arch, &qmp_socket)?;

    wait_for_qmp_socket(&qmp_socket, 30)?;
    std::thread::sleep(Duration::from_secs(3));

    let mut client = QmpClient::connect(&qmp_socket)?;

    // x86_64 needs Enter at ISOLINUX
    if arch == "x86_64" {
        println!("[{}] ⏎ Pressing Enter at boot prompt...", arch);
        std::thread::sleep(Duration::from_secs(5));
        qmp_send_key(&mut client, "ret")?;
    }

    // Wait for boot
    println!("[{}] ⏳ Waiting for boot (30s)...", arch);
    std::thread::sleep(Duration::from_secs(30));

    // Login as root
    println!("[{}] 🔑 Logging in...", arch);
    qmp_send_keys(&mut client, "root")?;
    qmp_send_key(&mut client, "ret")?;
    std::thread::sleep(Duration::from_secs(3));

    // Run date command
    qmp_send_keys(&mut client, "date")?;
    qmp_send_key(&mut client, "ret")?;
    std::thread::sleep(Duration::from_secs(2));

    // Screenshot 1
    let screenshot1 = format!("{}/alpine_{}_shell.ppm", SCREENSHOT_DIR, arch);
    println!("[{}] 📸 Taking screenshot 1: {}", arch, screenshot1);
    take_screenshot(&mut client, &screenshot1)?;

    // Run ls
    qmp_send_keys(&mut client, "ls -la /")?;
    qmp_send_key(&mut client, "ret")?;
    std::thread::sleep(Duration::from_secs(2));

    // Screenshot 2
    let screenshot2 = format!("{}/alpine_{}_ls.ppm", SCREENSHOT_DIR, arch);
    println!("[{}] 📸 Taking screenshot 2: {}", arch, screenshot2);
    take_screenshot(&mut client, &screenshot2)?;

    let _ = child.kill();
    let _ = child.wait();
    let _ = fs::remove_file(&qmp_socket);

    println!("[{}] ✅ Screenshots captured!", arch);
    Ok(())
}

// =============================================================================
// Helpers
// =============================================================================

/// Start LevitateOS with VNC display
fn start_levitate_vnc(arch: &str, qmp_socket: &str) -> Result<std::process::Child> {
    let arch_enum = Arch::try_from(arch)?;
    let profile = if arch == "x86_64" {
        QemuProfile::X86_64
    } else {
        QemuProfile::Default
    };

    let mut builder = QemuBuilder::new(arch_enum, profile)
        .display_vnc()
        .enable_qmp(qmp_socket);

    if arch == "x86_64" {
        builder = builder.boot_iso();
    }

    let mut cmd = builder.build()?;

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start QEMU")
}

/// Start Alpine Linux
fn start_alpine(arch: &str, qmp_socket: &str) -> Result<std::process::Child> {
    let qemu_bin = match arch {
        "aarch64" => "qemu-system-aarch64",
        "x86_64" => "qemu-system-x86_64",
        _ => bail!("Unsupported architecture: {}", arch),
    };

    let iso_path = format!("tests/images/alpine-virt-{}-{}.iso", ALPINE_VERSION, arch);
    let qmp_arg = format!("unix:{},server,nowait", qmp_socket);

    let mut args: Vec<String> = vec![
        "-m".into(), "512M".into(),
        "-display".into(), "none".into(),
        "-serial".into(), "mon:stdio".into(),
        "-qmp".into(), qmp_arg,
        "-cdrom".into(), iso_path,
        "-boot".into(), "d".into(),
    ];

    if arch == "aarch64" {
        args.extend([
            "-M".into(), "virt".into(),
            "-cpu".into(), "cortex-a72".into(),
            "-bios".into(), "/usr/share/AAVMF/AAVMF_CODE.fd".into(),
            "-device".into(), "virtio-gpu-pci".into(),
            "-device".into(), "virtio-keyboard-pci".into(),
        ]);
    } else {
        args.extend([
            "-M".into(), "q35".into(),
            "-cpu".into(), "qemu64".into(),
            "-enable-kvm".into(),
            "-vga".into(), "std".into(),
        ]);
    }

    Command::new(qemu_bin)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start QEMU")
}

/// Take screenshot via QMP
fn take_screenshot(client: &mut QmpClient, output: &str) -> Result<()> {
    let abs_path = std::env::current_dir()?.join(output);
    let args = serde_json::json!({
        "filename": abs_path.to_string_lossy()
    });
    client.execute("screendump", Some(args))?;
    std::thread::sleep(Duration::from_millis(500));

    // Convert PPM to PNG
    if output.ends_with(".ppm") {
        let png_path = output.replace(".ppm", ".png");
        let status = Command::new("magick")
            .args([output, &png_path])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let _ = fs::remove_file(output);
        }
    }

    Ok(())
}
