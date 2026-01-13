//! # LevitateOS Vendor Manager
//!
//! Manages vendored external sources (Linux kernel, etc.) to avoid
//! repeated downloads on bandwidth-constrained connections.
//!
//! ## Usage
//!
//! ```bash
//! vendor fetch linux      # Clone Linux kernel to vendor/linux/
//! vendor status           # Show vendored sources and versions
//! vendor update linux     # Pull latest changes
//! vendor list             # List available sources
//! ```

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::Command;

/// Vendor directory path
const VENDOR_DIR: &str = "vendor";

/// Known sources that can be vendored
const SOURCES: &[Source] = &[
    Source {
        name: "linux",
        repo: "https://github.com/torvalds/linux.git",
        branch: Some("master"),
        shallow: true,
        description: "Linux kernel source",
    },
    Source {
        name: "busybox",
        repo: "https://git.busybox.net/busybox",
        branch: Some("master"),
        shallow: true,
        description: "BusyBox source",
    },
    Source {
        name: "openrc",
        repo: "https://github.com/OpenRC/openrc.git",
        branch: Some("master"),
        shallow: true,
        description: "OpenRC init system source",
    },
];

struct Source {
    name: &'static str,
    repo: &'static str,
    branch: Option<&'static str>,
    shallow: bool,
    description: &'static str,
}

#[derive(Parser)]
#[command(name = "vendor", about = "LevitateOS vendor management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch a source into vendor/
    Fetch {
        /// Source name (linux, busybox, openrc)
        name: String,
        /// Clone specific tag/branch
        #[arg(long)]
        tag: Option<String>,
    },
    /// Show status of vendored sources
    Status,
    /// Update a vendored source
    Update {
        /// Source name
        name: String,
    },
    /// List available sources
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { name, tag } => fetch(&name, tag.as_deref())?,
        Commands::Status => status()?,
        Commands::Update { name } => update(&name)?,
        Commands::List => list(),
    }

    Ok(())
}

fn find_source(name: &str) -> Result<&'static Source> {
    SOURCES
        .iter()
        .find(|s| s.name == name)
        .ok_or_else(|| anyhow::anyhow!("Unknown source: {name}. Run 'vendor list' to see available sources."))
}

fn fetch(name: &str, tag: Option<&str>) -> Result<()> {
    let source = find_source(name)?;
    let dest = format!("{VENDOR_DIR}/{name}");

    if Path::new(&dest).exists() {
        bail!("{name} already exists at {dest}. Use 'vendor update {name}' to update.");
    }

    // Ensure vendor directory exists
    std::fs::create_dir_all(VENDOR_DIR)?;

    println!("Fetching {name} from {}...", source.repo);

    let mut args = vec!["clone"];

    if source.shallow {
        args.extend(["--depth", "1"]);
    }

    if let Some(t) = tag {
        args.extend(["--branch", t]);
    } else if let Some(b) = source.branch {
        args.extend(["--branch", b]);
    }

    args.push(source.repo);
    args.push(&dest);

    let status = Command::new("git")
        .args(&args)
        .status()
        .context("Failed to run git clone")?;

    if !status.success() {
        bail!("git clone failed");
    }

    // Show result
    let size = dir_size(&dest).unwrap_or(0);
    println!("  Fetched {name} to {dest} ({:.1} MB)", size as f64 / 1_000_000.0);

    Ok(())
}

fn status() -> Result<()> {
    println!("Vendored sources:\n");

    let vendor_path = Path::new(VENDOR_DIR);
    if !vendor_path.exists() {
        println!("  (none - run 'vendor fetch <name>' to fetch sources)");
        return Ok(());
    }

    let mut found = false;
    for source in SOURCES {
        let path = vendor_path.join(source.name);
        if path.exists() {
            found = true;
            let version = get_git_version(&path).unwrap_or_else(|_| "unknown".to_string());
            let size = dir_size(&path.to_string_lossy()).unwrap_or(0);
            println!(
                "  {} ({:.1} MB)",
                source.name,
                size as f64 / 1_000_000.0
            );
            println!("    {}", version);
            println!();
        }
    }

    if !found {
        println!("  (none - run 'vendor fetch <name>' to fetch sources)");
    }

    Ok(())
}

fn update(name: &str) -> Result<()> {
    let _source = find_source(name)?;
    let dest = format!("{VENDOR_DIR}/{name}");

    if !Path::new(&dest).exists() {
        bail!("{name} not found at {dest}. Run 'vendor fetch {name}' first.");
    }

    println!("Updating {name}...");

    let status = Command::new("git")
        .current_dir(&dest)
        .args(["pull", "--ff-only"])
        .status()
        .context("Failed to run git pull")?;

    if !status.success() {
        bail!("git pull failed");
    }

    println!("  Updated {name}");
    Ok(())
}

fn list() {
    println!("Available sources:\n");
    for source in SOURCES {
        let status = if Path::new(&format!("{VENDOR_DIR}/{}", source.name)).exists() {
            "[fetched]"
        } else {
            ""
        };
        println!("  {:12} {} {}", source.name, source.description, status);
    }
    println!("\nUse 'vendor fetch <name>' to fetch a source.");
}

fn get_git_version(path: &Path) -> Result<String> {
    let output = Command::new("git")
        .current_dir(path)
        .args(["describe", "--tags", "--always"])
        .output()
        .context("Failed to get git version")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn dir_size(path: &str) -> Result<u64> {
    let output = Command::new("du")
        .args(["-sb", path])
        .output()
        .context("Failed to get directory size")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let size_str = stdout.split_whitespace().next().unwrap_or("0");
    Ok(size_str.parse().unwrap_or(0))
}
