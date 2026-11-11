//! distro-builder is the producer of every product. xtask runs its binary with a
//! typed `distro_builder::cli::Command`, so the arguments are checked by the compiler.

use anyhow::{Context, Result, bail};
use distro_builder::cli::Command;
use std::path::Path;

/// Run distro-builder.
pub fn run(root: &Path, command: &Command) -> Result<()> {
    let bin = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let args = command.to_args(root);
    let elevated = unsafe { libc::geteuid() } != 0;
    let mut cmd = if elevated {
        sudo_command(root, &bin)
    } else {
        std::process::Command::new(&bin)
    };
    cmd.args(&args);
    let status = cmd
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if elevated && status.code() == Some(1) && !sudo_works_without_password() {
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
    for name in ["RUSTUP_TOOLCHAIN", "GINUX_ROOT", "GINUX_ROOTFS", "RECIPE_BIN"] {
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
