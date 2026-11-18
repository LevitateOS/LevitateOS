//! distro-builder is the producer of every product. xtask runs its binary with a
//! typed `distro_builder::cli::Command`, so the arguments are checked by the compiler.

use anyhow::{Context, Result, bail};
use distro_builder::cli::Command;
use std::path::Path;

/// Run distro-builder.
pub fn run(root: &Path, command: &Command) -> Result<()> {
    let (bin, args, mut cmd) = prepare(root, command)?;
    let status = cmd
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    check(status, &args)
}

/// Run distro-builder like [`run`] (same elevation, environment and `--repo-root`), and
/// return what it printed on standard output; its standard error still goes to the terminal.
pub fn run_capture(root: &Path, command: &Command) -> Result<String> {
    let (bin, args, mut cmd) = prepare(root, command)?;
    let out = cmd
        .stderr(std::process::Stdio::inherit())
        .output()
        .with_context(|| format!("running {}", bin.display()))?;
    check(out.status, &args)?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The builder binary, its arguments and the command that starts it (elevated when the
/// current user is not root).
fn prepare(root: &Path, command: &Command) -> Result<(std::path::PathBuf, Vec<std::ffi::OsString>, std::process::Command)> {
    let bin = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let args = command.to_args(root);
    let mut cmd = if elevated() {
        sudo_command(root, &bin)
    } else {
        std::process::Command::new(&bin)
    };
    cmd.args(&args);
    Ok((bin, args, cmd))
}

fn elevated() -> bool {
    (unsafe { libc::geteuid() }) != 0
}

/// Fails with the builder's command line when it did not succeed, and with the sudo
/// message when sudo itself refused to start it.
fn check(status: std::process::ExitStatus, args: &[std::ffi::OsString]) -> Result<()> {
    if elevated() && status.code() == Some(1) && !sudo_works_without_password() {
        bail!(
            "sudo needs a password, so the builder (which needs root: the Ginux coordinator \
             runs privileged) did not start; configure passwordless sudo for this user \
             (`sudo -n true` must succeed), or run `cargo xtask` as root"
        );
    }
    if !status.success() {
        let args: Vec<_> = args.iter().map(|arg| arg.to_string_lossy()).collect();
        bail!("distro-builder {} failed ({status})", args.join(" "));
    }
    Ok(())
}

/// Run distro-builder as the current user, for commands that need no root (prebuilt keys
/// and publishing, which uses the user's own `gh` login).
pub fn run_unprivileged(root: &Path, command: &Command) -> Result<()> {
    let bin = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let args = command.to_args(root);
    let status = std::process::Command::new(&bin)
        .args(&args)
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        let args: Vec<_> = args.iter().map(|arg| arg.to_string_lossy()).collect();
        bail!("distro-builder {} failed ({status})", args.join(" "));
    }
    Ok(())
}

/// `sudo -n env ... <bin>`: the builder as root, with the user's toolchain and a
/// target directory of its own (root's cargo builds of Recipe and the Ginux launcher
/// then write no root-owned files into `target/` or the user's registry, and run
/// offline). The compiler image selection variables and `RECIPE_BIN` pass through
/// when set; the builder picks the pinned image itself otherwise.
fn sudo_command(root: &Path, bin: &Path) -> std::process::Command {
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from);
    let user_dir = |var: &str, default: &str| {
        std::env::var_os(var)
            .or_else(|| home.as_ref().map(|home| home.join(default).into_os_string()))
            .unwrap_or_default()
    };
    let mut cmd = std::process::Command::new("sudo");
    cmd.args(["-n", "env"]);
    let mut set = |name: &str, value: &std::ffi::OsStr| {
        let mut pair = std::ffi::OsString::from(format!("{name}="));
        pair.push(value);
        cmd.arg(pair);
    };
    set("PATH", &std::env::var_os("PATH").unwrap_or_default());
    set("CARGO_HOME", &user_dir("CARGO_HOME", ".cargo"));
    set("RUSTUP_HOME", &user_dir("RUSTUP_HOME", ".rustup"));
    set("CARGO_NET_OFFLINE", "true".as_ref());
    // The directory root's builds have always used. Another path rebuilds Recipe
    // and the Ginux launcher into different bytes (debug info records it), and
    // their digests are part of every Recipe cache identity.
    set(
        "CARGO_TARGET_DIR",
        root.join(".artifacts/work/iso-runtime-fixes/cargo-host").as_os_str(),
    );
    for name in [
        "RUSTUP_TOOLCHAIN",
        "GINUX_ROOT",
        "GINUX_ROOTFS",
        "GINUX_MINIMAL_ROOT",
        "GINUX_MINIMAL_ROOTFS",
        "RECIPE_BIN",
        "RECIPE_FRAME_TIMEOUT_SECS",
    ] {
        if let Some(value) = std::env::var_os(name) {
            set(name, &value);
        }
    }
    cmd.arg(bin);
    cmd
}

/// Whether `sudo -n` runs a command without asking for a password.
fn sudo_works_without_password() -> bool {
    std::process::Command::new("sudo")
        .args(["-n", "true"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
