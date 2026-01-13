//! # LevitateOS - Rust-native Linux distribution builder
//!
//! Build minimal Linux systems from source with type-safe, fast Rust tooling.
//!
//! ## Usage
//!
//! ```bash
//! levitate build all           # Build Linux + BusyBox + OpenRC + initramfs
//! levitate build linux         # Build Linux kernel only
//! levitate build busybox       # Build BusyBox only
//! levitate build openrc        # Build OpenRC only
//! levitate build initramfs     # Build initramfs CPIO only
//! ```
//!
//! ## Components Built
//!
//! - **Linux kernel** (from submodule)
//! - **BusyBox** (shell + utilities, static musl)
//! - **OpenRC** (init system, static musl)
//! - **initramfs** (CPIO archive)

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

mod builder;

#[derive(Parser)]
#[command(name = "levitate", about = "Rust-native Linux distribution builder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Target architecture
    #[arg(long, global = true, default_value = "x86_64")]
    arch: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Build components
    #[command(subcommand)]
    Build(builder::BuildCommands),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let arch = cli.arch.as_str();

    if arch != "aarch64" && arch != "x86_64" {
        bail!("Unsupported architecture: {arch}. Use 'aarch64' or 'x86_64'");
    }

    match cli.command {
        Commands::Build(cmd) => match cmd {
            builder::BuildCommands::All => builder::build_all(arch)?,
            builder::BuildCommands::Initramfs => builder::create_initramfs(arch)?,
            builder::BuildCommands::Busybox => builder::busybox::build(arch)?,
            builder::BuildCommands::Linux => builder::linux::build_linux_kernel(arch)?,
            builder::BuildCommands::Openrc => builder::openrc::build(arch)?,
        },
    }

    Ok(())
}
