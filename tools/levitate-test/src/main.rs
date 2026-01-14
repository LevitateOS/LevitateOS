//! LevitateOS In-VM Test Suite
//! Tests that tools actually work, not just exist.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

static mut PASSED: u32 = 0;
static mut FAILED: u32 = 0;

fn pass(name: &str) {
    println!("{}: PASS", name);
    unsafe {
        PASSED += 1;
    }
}

fn fail(name: &str, err: &str) {
    println!("{}: FAIL ({})", name, err);
    unsafe {
        FAILED += 1;
    }
}

fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())
        .and_then(|o| {
            if o.status.success() {
                Ok(String::from_utf8_lossy(&o.stdout).to_string())
            } else {
                Err(format!("exit {}", o.status.code().unwrap_or(-1)))
            }
        })
}

fn run_ok(cmd: &str, args: &[&str]) -> bool {
    run(cmd, args).is_ok()
}

// ============================================================================
// Group 0: Smoke tests (binaries exist and run)
// ============================================================================
fn test_group_smoke() {
    println!("# smoke");

    let cmds = [
        ("ls", &["--version"][..]),
        ("cat", &["--version"]),
        ("cp", &["--version"]),
        ("mv", &["--version"]),
        ("rm", &["--version"]),
        ("mkdir", &["--version"]),
        ("chmod", &["--version"]),
        ("find", &["--version"]),
        ("xargs", &["--version"]),
        ("diff", &["--version"]),
        ("cmp", &["--version"]),
        ("ps", &["--version"]),
        ("free", &["--version"]),
        ("ip", &["-V"]),
        ("ss", &["-V"]),
        ("ping", &["-V"]),
        ("hx", &["--version"]),
        ("brush", &["--version"]),
        ("sudo", &["--version"]),
    ];

    for (cmd, args) in cmds {
        match run(cmd, args) {
            Ok(_) => pass(&format!("smoke.{}", cmd)),
            Err(_) => {
                // Some commands exit non-zero for --version, just check they exist
                if Command::new(cmd)
                    .arg("--help")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .is_ok()
                {
                    pass(&format!("smoke.{}", cmd));
                } else {
                    fail(&format!("smoke.{}", cmd), "not found");
                }
            }
        }
    }
}

// ============================================================================
// Group 1: Coreutils - file operations actually work
// ============================================================================
fn test_group_coreutils() {
    println!("# coreutils");

    let test_dir = "/tmp/test_coreutils";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();

    // mkdir - creates directory
    let subdir = format!("{}/subdir", test_dir);
    if run_ok("mkdir", &[&subdir]) && fs::metadata(&subdir).map(|m| m.is_dir()).unwrap_or(false) {
        pass("coreutils.mkdir");
    } else {
        fail("coreutils.mkdir", "dir not created");
    }

    // touch - creates file
    let file1 = format!("{}/file1.txt", test_dir);
    if run_ok("touch", &[&file1]) && fs::metadata(&file1).is_ok() {
        pass("coreutils.touch");
    } else {
        fail("coreutils.touch", "file not created");
    }

    // echo + redirect simulation: write content via shell
    let file2 = format!("{}/file2.txt", test_dir);
    fs::write(&file2, "hello world\n").unwrap();

    // cat - reads file content
    match run("cat", &[&file2]) {
        Ok(out) if out.contains("hello world") => pass("coreutils.cat"),
        Ok(_) => fail("coreutils.cat", "wrong content"),
        Err(e) => fail("coreutils.cat", &e),
    }

    // cp - copies file
    let file3 = format!("{}/file3.txt", test_dir);
    if run_ok("cp", &[&file2, &file3]) {
        match fs::read_to_string(&file3) {
            Ok(s) if s.contains("hello world") => pass("coreutils.cp"),
            _ => fail("coreutils.cp", "content mismatch"),
        }
    } else {
        fail("coreutils.cp", "cp failed");
    }

    // mv - moves file
    let file4 = format!("{}/file4.txt", test_dir);
    if run_ok("mv", &[&file3, &file4])
        && !fs::metadata(&file3).is_ok()
        && fs::metadata(&file4).is_ok()
    {
        pass("coreutils.mv");
    } else {
        fail("coreutils.mv", "move failed");
    }

    // rm - removes file
    if run_ok("rm", &[&file4]) && !fs::metadata(&file4).is_ok() {
        pass("coreutils.rm");
    } else {
        fail("coreutils.rm", "file not removed");
    }

    // rmdir - removes empty directory
    let emptydir = format!("{}/emptydir", test_dir);
    fs::create_dir(&emptydir).unwrap();
    if run_ok("rmdir", &[&emptydir]) && !fs::metadata(&emptydir).is_ok() {
        pass("coreutils.rmdir");
    } else {
        fail("coreutils.rmdir", "dir not removed");
    }

    // chmod - changes permissions
    let chmod_file = format!("{}/chmod_test.txt", test_dir);
    fs::write(&chmod_file, "test").unwrap();
    if run_ok("chmod", &["755", &chmod_file]) {
        let mode = fs::metadata(&chmod_file).unwrap().permissions().mode() & 0o777;
        if mode == 0o755 {
            pass("coreutils.chmod");
        } else {
            fail("coreutils.chmod", &format!("mode is {:o}", mode));
        }
    } else {
        fail("coreutils.chmod", "chmod failed");
    }

    // ln - creates symlink
    let link_target = format!("{}/link_target.txt", test_dir);
    let link_name = format!("{}/link_name.txt", test_dir);
    fs::write(&link_target, "link content").unwrap();
    if run_ok("ln", &["-s", &link_target, &link_name]) {
        match fs::read_link(&link_name) {
            Ok(_) => pass("coreutils.ln_symlink"),
            Err(e) => fail("coreutils.ln_symlink", &e.to_string()),
        }
    } else {
        fail("coreutils.ln_symlink", "ln -s failed");
    }

    // ls - lists directory contents
    match run("ls", &[test_dir]) {
        Ok(out) if out.contains("file1.txt") && out.contains("subdir") => pass("coreutils.ls"),
        Ok(out) => fail(
            "coreutils.ls",
            &format!("missing expected files: {}", out.trim()),
        ),
        Err(e) => fail("coreutils.ls", &e),
    }

    // head - extracts first lines
    let multiline = format!("{}/multiline.txt", test_dir);
    fs::write(&multiline, "line1\nline2\nline3\nline4\nline5\n").unwrap();
    match run("head", &["-n", "2", &multiline]) {
        Ok(out) if out == "line1\nline2\n" => pass("coreutils.head"),
        Ok(out) => fail("coreutils.head", &format!("got: {:?}", out)),
        Err(e) => fail("coreutils.head", &e),
    }

    // tail - extracts last lines
    match run("tail", &["-n", "2", &multiline]) {
        Ok(out) if out == "line4\nline5\n" => pass("coreutils.tail"),
        Ok(out) => fail("coreutils.tail", &format!("got: {:?}", out)),
        Err(e) => fail("coreutils.tail", &e),
    }

    // wc - counts lines/words/chars
    match run("wc", &["-l", &multiline]) {
        Ok(out) if out.trim().starts_with("5") => pass("coreutils.wc"),
        Ok(out) => fail("coreutils.wc", &format!("got: {}", out.trim())),
        Err(e) => fail("coreutils.wc", &e),
    }

    // basename - extracts filename
    match run("basename", &["/path/to/file.txt"]) {
        Ok(out) if out.trim() == "file.txt" => pass("coreutils.basename"),
        Ok(out) => fail("coreutils.basename", &format!("got: {}", out.trim())),
        Err(e) => fail("coreutils.basename", &e),
    }

    // dirname - extracts directory
    match run("dirname", &["/path/to/file.txt"]) {
        Ok(out) if out.trim() == "/path/to" => pass("coreutils.dirname"),
        Ok(out) => fail("coreutils.dirname", &format!("got: {}", out.trim())),
        Err(e) => fail("coreutils.dirname", &e),
    }

    // echo - outputs text
    match run("echo", &["test output"]) {
        Ok(out) if out.trim() == "test output" => pass("coreutils.echo"),
        Ok(out) => fail("coreutils.echo", &format!("got: {:?}", out)),
        Err(e) => fail("coreutils.echo", &e),
    }

    // pwd - prints working directory
    match run("pwd", &[]) {
        Ok(out) if out.trim().starts_with("/") => pass("coreutils.pwd"),
        Ok(out) => fail("coreutils.pwd", &format!("got: {}", out.trim())),
        Err(e) => fail("coreutils.pwd", &e),
    }

    // Cleanup
    let _ = fs::remove_dir_all(test_dir);
}

// ============================================================================
// Group: System checks (files, mounts, etc)
// ============================================================================
fn test_group_system() {
    println!("# system");

    // /proc mounted
    if fs::read_to_string("/proc/version")
        .map(|s| s.contains("Linux"))
        .unwrap_or(false)
    {
        pass("system.proc");
    } else {
        fail("system.proc", "/proc not mounted");
    }

    // /sys mounted
    if fs::metadata("/sys/class")
        .map(|m| m.is_dir())
        .unwrap_or(false)
    {
        pass("system.sys");
    } else {
        fail("system.sys", "/sys not mounted");
    }

    // /dev/null works
    if fs::metadata("/dev/null").is_ok() {
        pass("system.devnull");
    } else {
        fail("system.devnull", "missing");
    }

    // /etc/passwd exists with root
    if fs::read_to_string("/etc/passwd")
        .map(|s| s.contains("root"))
        .unwrap_or(false)
    {
        pass("system.passwd");
    } else {
        fail("system.passwd", "missing root");
    }

    // hostname set
    if fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().len() > 0)
        .unwrap_or(false)
    {
        pass("system.hostname");
    } else {
        fail("system.hostname", "empty");
    }
}

fn main() {
    println!("---TESTS---");

    test_group_smoke();
    test_group_coreutils();
    test_group_system();

    let (p, f) = unsafe { (PASSED, FAILED) };
    println!("---SUMMARY---");
    println!("passed:{} failed:{}", p, f);
    std::process::exit(if f == 0 { 0 } else { 1 });
}
