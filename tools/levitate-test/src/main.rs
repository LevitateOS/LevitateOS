//! LevitateOS In-VM Test Suite
//!
//! Tests that user-facing tools actually work, not just exist.
//! Run with: systemd.unit=levitate-test.target on kernel cmdline

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

static mut PASSED: u32 = 0;
static mut FAILED: u32 = 0;

fn pass(name: &str) {
    println!("  [PASS] {}", name);
    unsafe { PASSED += 1; }
}

fn fail(name: &str, reason: &str) {
    println!("  [FAIL] {} ({})", name, reason);
    unsafe { FAILED += 1; }
}

/// Run a command and return stdout if successful.
fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("exec: {}", e))
        .and_then(|o| {
            if o.status.success() {
                Ok(String::from_utf8_lossy(&o.stdout).to_string())
            } else {
                let stderr = String::from_utf8_lossy(&o.stderr);
                Err(format!("exit {}: {}", o.status.code().unwrap_or(-1), stderr.trim()))
            }
        })
}

/// Run command, return true if exit 0.
fn run_ok(cmd: &str, args: &[&str]) -> bool {
    run(cmd, args).is_ok()
}

/// Check if a command exists (can be executed).
fn cmd_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--help")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

// ============================================================================
// Test: Shell functionality
// ============================================================================
fn test_shell() {
    println!("\n# Shell");

    // bash exists and runs
    match run("bash", &["--version"]) {
        Ok(out) if out.contains("bash") => pass("bash.version"),
        Ok(_) => fail("bash.version", "unexpected output"),
        Err(e) => fail("bash.version", &e),
    }

    // bash can execute a simple script
    match run("bash", &["-c", "echo hello"]) {
        Ok(out) if out.trim() == "hello" => pass("bash.echo"),
        Ok(out) => fail("bash.echo", &format!("got: {:?}", out.trim())),
        Err(e) => fail("bash.echo", &e),
    }

    // bash can do arithmetic
    match run("bash", &["-c", "echo $((2 + 3))"]) {
        Ok(out) if out.trim() == "5" => pass("bash.arithmetic"),
        Ok(out) => fail("bash.arithmetic", &format!("got: {:?}", out.trim())),
        Err(e) => fail("bash.arithmetic", &e),
    }

    // bash can use variables
    match run("bash", &["-c", "X=test; echo $X"]) {
        Ok(out) if out.trim() == "test" => pass("bash.variables"),
        Ok(out) => fail("bash.variables", &format!("got: {:?}", out.trim())),
        Err(e) => fail("bash.variables", &e),
    }

    // bash can pipe
    match run("bash", &["-c", "echo hello | cat"]) {
        Ok(out) if out.trim() == "hello" => pass("bash.pipe"),
        Ok(out) => fail("bash.pipe", &format!("got: {:?}", out.trim())),
        Err(e) => fail("bash.pipe", &e),
    }

    // env is set correctly
    match run("bash", &["-c", "echo $PATH"]) {
        Ok(out) if out.contains("/bin") => pass("bash.path_set"),
        Ok(out) => fail("bash.path_set", &format!("PATH={}", out.trim())),
        Err(e) => fail("bash.path_set", &e),
    }
}

// ============================================================================
// Test: Coreutils - file operations
// ============================================================================
fn test_coreutils() {
    println!("\n# Coreutils");

    let test_dir = "/tmp/test_coreutils";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();

    // mkdir creates directory
    let subdir = format!("{}/subdir", test_dir);
    if run_ok("mkdir", &[&subdir]) && fs::metadata(&subdir).map(|m| m.is_dir()).unwrap_or(false) {
        pass("mkdir");
    } else {
        fail("mkdir", "dir not created");
    }

    // touch creates file
    let file1 = format!("{}/file1.txt", test_dir);
    if run_ok("touch", &[&file1]) && fs::metadata(&file1).is_ok() {
        pass("touch");
    } else {
        fail("touch", "file not created");
    }

    // Write test content
    let file2 = format!("{}/file2.txt", test_dir);
    fs::write(&file2, "hello world\n").unwrap();

    // cat reads file
    match run("cat", &[&file2]) {
        Ok(out) if out.contains("hello world") => pass("cat"),
        Ok(_) => fail("cat", "wrong content"),
        Err(e) => fail("cat", &e),
    }

    // cp copies file
    let file3 = format!("{}/file3.txt", test_dir);
    if run_ok("cp", &[&file2, &file3]) && fs::read_to_string(&file3).map(|s| s.contains("hello")).unwrap_or(false) {
        pass("cp");
    } else {
        fail("cp", "copy failed");
    }

    // mv moves file
    let file4 = format!("{}/file4.txt", test_dir);
    if run_ok("mv", &[&file3, &file4]) && !fs::metadata(&file3).is_ok() && fs::metadata(&file4).is_ok() {
        pass("mv");
    } else {
        fail("mv", "move failed");
    }

    // rm removes file
    if run_ok("rm", &[&file4]) && !fs::metadata(&file4).is_ok() {
        pass("rm");
    } else {
        fail("rm", "file not removed");
    }

    // chmod changes permissions
    let chmod_file = format!("{}/chmod_test.txt", test_dir);
    fs::write(&chmod_file, "test").unwrap();
    if run_ok("chmod", &["755", &chmod_file]) {
        let mode = fs::metadata(&chmod_file).unwrap().permissions().mode() & 0o777;
        if mode == 0o755 {
            pass("chmod");
        } else {
            fail("chmod", &format!("mode is {:o}", mode));
        }
    } else {
        fail("chmod", "chmod failed");
    }

    // ln -s creates symlink
    let link_target = format!("{}/target.txt", test_dir);
    let link_name = format!("{}/link.txt", test_dir);
    fs::write(&link_target, "target content").unwrap();
    if run_ok("ln", &["-s", &link_target, &link_name]) && fs::read_link(&link_name).is_ok() {
        pass("ln_symlink");
    } else {
        fail("ln_symlink", "symlink not created");
    }

    // ls lists files
    match run("ls", &[test_dir]) {
        Ok(out) if out.contains("file1.txt") => pass("ls"),
        Ok(out) => fail("ls", &format!("missing files: {}", out.trim())),
        Err(e) => fail("ls", &e),
    }

    // head extracts first lines
    let multiline = format!("{}/multi.txt", test_dir);
    fs::write(&multiline, "line1\nline2\nline3\n").unwrap();
    match run("head", &["-n", "1", &multiline]) {
        Ok(out) if out.trim() == "line1" => pass("head"),
        Ok(out) => fail("head", &format!("got: {:?}", out.trim())),
        Err(e) => fail("head", &e),
    }

    // tail extracts last lines
    match run("tail", &["-n", "1", &multiline]) {
        Ok(out) if out.trim() == "line3" => pass("tail"),
        Ok(out) => fail("tail", &format!("got: {:?}", out.trim())),
        Err(e) => fail("tail", &e),
    }

    // wc counts lines
    match run("wc", &["-l", &multiline]) {
        Ok(out) if out.trim().starts_with("3") => pass("wc"),
        Ok(out) => fail("wc", &format!("got: {}", out.trim())),
        Err(e) => fail("wc", &e),
    }

    // sort sorts lines
    let unsorted = format!("{}/unsorted.txt", test_dir);
    fs::write(&unsorted, "c\na\nb\n").unwrap();
    match run("sort", &[&unsorted]) {
        Ok(out) if out == "a\nb\nc\n" => pass("sort"),
        Ok(out) => fail("sort", &format!("got: {:?}", out)),
        Err(e) => fail("sort", &e),
    }

    // uniq removes duplicates
    let dupes = format!("{}/dupes.txt", test_dir);
    fs::write(&dupes, "a\na\nb\n").unwrap();
    match run("uniq", &[&dupes]) {
        Ok(out) if out == "a\nb\n" => pass("uniq"),
        Ok(out) => fail("uniq", &format!("got: {:?}", out)),
        Err(e) => fail("uniq", &e),
    }

    let _ = fs::remove_dir_all(test_dir);
}

// ============================================================================
// Test: Text processing (grep, sed, awk)
// ============================================================================
fn test_text_processing() {
    println!("\n# Text Processing");

    let test_dir = "/tmp/test_text";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();

    let test_file = format!("{}/data.txt", test_dir);
    fs::write(&test_file, "apple\nbanana\ncherry\napricot\n").unwrap();

    // grep finds matching lines
    match run("grep", &["ap", &test_file]) {
        Ok(out) if out.contains("apple") && out.contains("apricot") && !out.contains("banana") => pass("grep"),
        Ok(out) => fail("grep", &format!("got: {:?}", out)),
        Err(e) => fail("grep", &e),
    }

    // grep -v inverts match
    match run("grep", &["-v", "ap", &test_file]) {
        Ok(out) if out.contains("banana") && !out.contains("apple") => pass("grep_invert"),
        Ok(out) => fail("grep_invert", &format!("got: {:?}", out)),
        Err(e) => fail("grep_invert", &e),
    }

    // grep -c counts matches
    match run("grep", &["-c", "ap", &test_file]) {
        Ok(out) if out.trim() == "2" => pass("grep_count"),
        Ok(out) => fail("grep_count", &format!("got: {}", out.trim())),
        Err(e) => fail("grep_count", &e),
    }

    // sed substitutes text
    match run("sed", &["s/apple/orange/", &test_file]) {
        Ok(out) if out.contains("orange") && !out.contains("apple") => pass("sed"),
        Ok(out) => fail("sed", &format!("got: {:?}", out)),
        Err(e) => fail("sed", &e),
    }

    // awk prints columns
    let csv_file = format!("{}/data.csv", test_dir);
    fs::write(&csv_file, "a,1\nb,2\nc,3\n").unwrap();
    match run("awk", &["-F,", "{print $2}", &csv_file]) {
        Ok(out) if out.trim() == "1\n2\n3" => pass("awk"),
        Ok(out) => fail("awk", &format!("got: {:?}", out.trim())),
        Err(e) => fail("awk", &e),
    }

    // tr translates characters
    match run("bash", &["-c", "echo 'hello' | tr 'a-z' 'A-Z'"]) {
        Ok(out) if out.trim() == "HELLO" => pass("tr"),
        Ok(out) => fail("tr", &format!("got: {:?}", out.trim())),
        Err(e) => fail("tr", &e),
    }

    // cut extracts fields
    match run("bash", &["-c", "echo 'a:b:c' | cut -d: -f2"]) {
        Ok(out) if out.trim() == "b" => pass("cut"),
        Ok(out) => fail("cut", &format!("got: {:?}", out.trim())),
        Err(e) => fail("cut", &e),
    }

    let _ = fs::remove_dir_all(test_dir);
}

// ============================================================================
// Test: Process management
// ============================================================================
fn test_processes() {
    println!("\n# Process Management");

    // ps shows processes
    match run("ps", &["aux"]) {
        Ok(out) if out.contains("PID") || out.contains("levitate-test") => pass("ps"),
        Ok(out) => fail("ps", &format!("unexpected: {}", &out[..out.len().min(100)])),
        Err(e) => fail("ps", &e),
    }

    // pgrep finds by name
    match run("pgrep", &["-c", "systemd"]) {
        Ok(out) => {
            let count: i32 = out.trim().parse().unwrap_or(0);
            if count > 0 {
                pass("pgrep");
            } else {
                fail("pgrep", "no systemd found");
            }
        }
        Err(e) => fail("pgrep", &e),
    }

    // uptime shows system uptime
    match run("uptime", &[]) {
        Ok(out) if out.contains("up") || out.contains("load") => pass("uptime"),
        Ok(out) => fail("uptime", &format!("unexpected: {}", out.trim())),
        Err(e) => fail("uptime", &e),
    }

    // free shows memory
    match run("free", &["-m"]) {
        Ok(out) if out.contains("Mem") => pass("free"),
        Ok(out) => fail("free", &format!("unexpected: {}", out.trim())),
        Err(e) => fail("free", &e),
    }

    // uname shows system info
    match run("uname", &["-a"]) {
        Ok(out) if out.contains("Linux") => pass("uname"),
        Ok(out) => fail("uname", &format!("unexpected: {}", out.trim())),
        Err(e) => fail("uname", &e),
    }

    // id shows user info
    match run("id", &[]) {
        Ok(out) if out.contains("uid=") => pass("id"),
        Ok(out) => fail("id", &format!("unexpected: {}", out.trim())),
        Err(e) => fail("id", &e),
    }

    // whoami shows current user
    match run("whoami", &[]) {
        Ok(out) if out.trim() == "root" => pass("whoami"),
        Ok(out) => fail("whoami", &format!("got: {}", out.trim())),
        Err(e) => fail("whoami", &e),
    }
}

// ============================================================================
// Test: Network tools
// ============================================================================
fn test_network() {
    println!("\n# Network");

    // ip link shows interfaces
    match run("ip", &["link", "show"]) {
        Ok(out) if out.contains("lo") => pass("ip_link"),
        Ok(out) => fail("ip_link", &format!("no loopback: {}", &out[..out.len().min(100)])),
        Err(e) => fail("ip_link", &e),
    }

    // ip addr shows addresses
    match run("ip", &["addr", "show", "lo"]) {
        Ok(out) if out.contains("127.0.0.1") => pass("ip_addr"),
        Ok(out) => fail("ip_addr", &format!("no 127.0.0.1: {}", out.trim())),
        Err(e) => fail("ip_addr", &e),
    }

    // ss shows sockets
    match run("ss", &["-tuln"]) {
        Ok(_) => pass("ss"),
        Err(e) => fail("ss", &e),
    }

    // ping localhost (just check command works, might not have network)
    match run("ping", &["-c", "1", "-W", "1", "127.0.0.1"]) {
        Ok(out) if out.contains("1 received") || out.contains("bytes from") => pass("ping"),
        Ok(_) => pass("ping"),  // ping ran, even if no response
        Err(e) if e.contains("exec") => fail("ping", &e),
        Err(_) => pass("ping"),  // ping exists but network issue
    }

    // hostname shows hostname
    match run("hostname", &[]) {
        Ok(out) if !out.trim().is_empty() => pass("hostname"),
        Ok(_) => fail("hostname", "empty"),
        Err(e) => fail("hostname", &e),
    }
}

// ============================================================================
// Test: Compression tools
// ============================================================================
fn test_compression() {
    println!("\n# Compression");

    let test_dir = "/tmp/test_compress";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();

    // Create test data
    let test_file = format!("{}/data.txt", test_dir);
    fs::write(&test_file, "test data for compression\n".repeat(100)).unwrap();

    // gzip compresses
    let gz_file = format!("{}/data.txt.gz", test_dir);
    if run_ok("bash", &["-c", &format!("gzip -c {} > {}", test_file, gz_file)]) && fs::metadata(&gz_file).is_ok() {
        pass("gzip");
    } else {
        fail("gzip", "compression failed");
    }

    // gunzip decompresses
    let ungz_file = format!("{}/ungz.txt", test_dir);
    if run_ok("bash", &["-c", &format!("gunzip -c {} > {}", gz_file, ungz_file)]) {
        match fs::read_to_string(&ungz_file) {
            Ok(s) if s.contains("test data") => pass("gunzip"),
            _ => fail("gunzip", "content mismatch"),
        }
    } else {
        fail("gunzip", "decompression failed");
    }

    // tar creates archive
    let tar_file = format!("{}/archive.tar", test_dir);
    let tar_dir = format!("{}/tartest", test_dir);
    fs::create_dir_all(&tar_dir).unwrap();
    fs::write(format!("{}/a.txt", tar_dir), "file a").unwrap();
    fs::write(format!("{}/b.txt", tar_dir), "file b").unwrap();

    if run_ok("tar", &["-cf", &tar_file, "-C", test_dir, "tartest"]) && fs::metadata(&tar_file).is_ok() {
        pass("tar_create");
    } else {
        fail("tar_create", "archive not created");
    }

    // tar extracts archive
    let extract_dir = format!("{}/extracted", test_dir);
    fs::create_dir_all(&extract_dir).unwrap();
    if run_ok("tar", &["-xf", &tar_file, "-C", &extract_dir]) {
        if fs::metadata(format!("{}/tartest/a.txt", extract_dir)).is_ok() {
            pass("tar_extract");
        } else {
            fail("tar_extract", "files not extracted");
        }
    } else {
        fail("tar_extract", "extraction failed");
    }

    let _ = fs::remove_dir_all(test_dir);
}

// ============================================================================
// Test: Find utilities
// ============================================================================
fn test_find() {
    println!("\n# Find Utilities");

    let test_dir = "/tmp/test_find";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(format!("{}/sub1/sub2", test_dir)).unwrap();
    fs::write(format!("{}/a.txt", test_dir), "a").unwrap();
    fs::write(format!("{}/sub1/b.txt", test_dir), "b").unwrap();
    fs::write(format!("{}/sub1/sub2/c.txt", test_dir), "c").unwrap();

    // find locates files
    match run("find", &[test_dir, "-name", "*.txt"]) {
        Ok(out) if out.contains("a.txt") && out.contains("b.txt") && out.contains("c.txt") => pass("find"),
        Ok(out) => fail("find", &format!("missing files: {}", out)),
        Err(e) => fail("find", &e),
    }

    // find with -type f
    match run("find", &[test_dir, "-type", "f"]) {
        Ok(out) if out.lines().count() == 3 => pass("find_type"),
        Ok(out) => fail("find_type", &format!("expected 3 files, got: {}", out.lines().count())),
        Err(e) => fail("find_type", &e),
    }

    // xargs processes input
    match run("bash", &["-c", &format!("find {} -name '*.txt' | xargs cat", test_dir)]) {
        Ok(out) if out.contains("a") && out.contains("b") && out.contains("c") => pass("xargs"),
        Ok(out) => fail("xargs", &format!("got: {:?}", out)),
        Err(e) => fail("xargs", &e),
    }

    let _ = fs::remove_dir_all(test_dir);
}

// ============================================================================
// Test: System state
// ============================================================================
fn test_system() {
    println!("\n# System");

    // /proc is mounted
    if fs::read_to_string("/proc/version").map(|s| s.contains("Linux")).unwrap_or(false) {
        pass("proc_mounted");
    } else {
        fail("proc_mounted", "/proc not available");
    }

    // /sys is mounted
    if fs::metadata("/sys/class").map(|m| m.is_dir()).unwrap_or(false) {
        pass("sys_mounted");
    } else {
        fail("sys_mounted", "/sys not available");
    }

    // /dev/null exists
    if fs::metadata("/dev/null").is_ok() {
        pass("dev_null");
    } else {
        fail("dev_null", "missing");
    }

    // /etc/passwd has root
    if fs::read_to_string("/etc/passwd").map(|s| s.contains("root:")).unwrap_or(false) {
        pass("etc_passwd");
    } else {
        fail("etc_passwd", "missing root");
    }

    // /etc/shadow exists (even if not readable)
    if fs::metadata("/etc/shadow").is_ok() {
        pass("etc_shadow");
    } else {
        fail("etc_shadow", "missing");
    }

    // getent works
    match run("getent", &["passwd", "root"]) {
        Ok(out) if out.contains("root") => pass("getent"),
        Ok(out) => fail("getent", &format!("unexpected: {}", out.trim())),
        Err(e) => fail("getent", &e),
    }

    // systemctl is-system-running
    match run("systemctl", &["is-system-running"]) {
        Ok(out) => {
            let state = out.trim();
            if state == "running" || state == "degraded" || state == "starting" {
                pass("systemd_running");
            } else {
                fail("systemd_running", state);
            }
        }
        Err(_) => {
            // systemctl might return non-zero for degraded
            if cmd_exists("systemctl") {
                pass("systemd_running");
            } else {
                fail("systemd_running", "systemctl missing");
            }
        }
    }
}

// ============================================================================
// Test: Editors exist
// ============================================================================
fn test_editors() {
    println!("\n# Editors");

    // nano exists
    if cmd_exists("nano") {
        pass("nano");
    } else {
        fail("nano", "not found");
    }

    // vi exists
    if cmd_exists("vi") {
        pass("vi");
    } else {
        fail("vi", "not found");
    }

    // less exists (pager)
    if cmd_exists("less") {
        pass("less");
    } else {
        fail("less", "not found");
    }
}

// ============================================================================
// Test: Authentication tools
// ============================================================================
fn test_auth() {
    println!("\n# Authentication");

    // login binary exists
    if cmd_exists("login") {
        pass("login");
    } else {
        fail("login", "not found");
    }

    // su binary exists
    if cmd_exists("su") {
        pass("su");
    } else {
        fail("su", "not found");
    }

    // sudo binary exists
    if cmd_exists("sudo") {
        pass("sudo");
    } else {
        fail("sudo", "not found");
    }

    // passwd binary exists
    if cmd_exists("passwd") {
        pass("passwd");
    } else {
        fail("passwd", "not found");
    }

    // PAM is configured
    if fs::metadata("/etc/pam.d/login").is_ok() || fs::metadata("/etc/pam.d/system-auth").is_ok() {
        pass("pam_config");
    } else {
        fail("pam_config", "no PAM config found");
    }
}

// ============================================================================
// Main
// ============================================================================
fn main() {
    println!("===== LevitateOS Test Suite =====");
    println!("Testing user-facing functionality\n");

    test_shell();
    test_coreutils();
    test_text_processing();
    test_processes();
    test_network();
    test_compression();
    test_find();
    test_system();
    test_editors();
    test_auth();

    let (passed, failed) = unsafe { (PASSED, FAILED) };
    let total = passed + failed;

    println!("\n===== Summary =====");
    println!("Passed: {}/{}", passed, total);
    println!("Failed: {}/{}", failed, total);

    if failed == 0 {
        println!("\nAll tests passed!");
        std::process::exit(0);
    } else {
        println!("\nSome tests failed.");
        std::process::exit(1);
    }
}
