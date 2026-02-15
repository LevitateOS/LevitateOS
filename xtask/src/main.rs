use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "LevitateOS repo developer tasks (scaffolding; complements justfile)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print the environment exports that the justfile sets for QEMU/tooling.
    ///
    /// Usage:
    ///   eval "$(cargo xtask env bash)"
    Env {
        #[arg(value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },

    /// Check that the local toolchain/tools match what the justfile expects.
    Doctor,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Shell {
    Bash,
    Sh,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Env { shell } => cmd_env(shell),
        Cmd::Doctor => cmd_doctor(),
    }
}

fn repo_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("xtask is expected at <repo>/xtask")
}

fn tools_prefix(root: &Path) -> PathBuf {
    root.join("leviso/downloads/.tools")
}

fn ovmf_path(root: &Path) -> PathBuf {
    tools_prefix(root).join("usr/share/edk2/ovmf/OVMF_CODE.fd")
}

fn cmd_env(shell: Shell) -> Result<()> {
    let root = repo_root()?;
    let tools = tools_prefix(&root);

    let usr_bin = tools.join("usr/bin");
    let usr_libexec = tools.join("usr/libexec");
    let ld_library_path = tools.join("usr/lib64");
    let ovmf = ovmf_path(&root);

    // This is intentionally the same wiring as the justfile.
    // Keep it as pure string exports so users can `eval` it.
    let path_export = format!("{}:{}:$PATH", usr_bin.display(), usr_libexec.display());

    match shell {
        Shell::Bash | Shell::Sh => {
            println!("export PATH=\"{}\"", path_export);
            println!("export LD_LIBRARY_PATH=\"{}\"", ld_library_path.display());
            println!("export OVMF_PATH=\"{}\"", ovmf.display());
        }
    }

    Ok(())
}

fn cmd_doctor() -> Result<()> {
    let root = repo_root()?;
    let tools = tools_prefix(&root);
    let ovmf = ovmf_path(&root);

    let mut ok = true;

    if which::which("just").is_err() {
        eprintln!("[FAIL] missing `just` in PATH");
        ok = false;
    } else {
        eprintln!("[OK] just");
    }

    let want_dirs = [
        tools.join("usr/bin"),
        tools.join("usr/libexec"),
        tools.join("usr/lib64"),
    ];
    for d in want_dirs {
        if d.is_dir() {
            eprintln!("[OK] {}", d.display());
        } else {
            eprintln!("[FAIL] missing directory: {}", d.display());
            ok = false;
        }
    }

    if ovmf.is_file() {
        eprintln!("[OK] {}", ovmf.display());
    } else {
        eprintln!("[FAIL] missing OVMF firmware: {}", ovmf.display());
        ok = false;
    }

    if !ok {
        bail!("doctor checks failed");
    }
    Ok(())
}
