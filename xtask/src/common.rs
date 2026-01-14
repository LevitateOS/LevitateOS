use anyhow::{Context, Result};
use std::path::PathBuf;

/// Get the workspace root directory
pub fn workspace_root() -> Result<PathBuf> {
    let output = std::process::Command::new("cargo")
        .args(&["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .context("Failed to run cargo locate-project")?;

    let path = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 from cargo")?;

    let manifest_path = PathBuf::from(path.trim());
    manifest_path.parent()
        .map(|p| p.to_path_buf())
        .context("No parent directory")
}

/// Print message if verbose mode is enabled
pub fn verbose_println(verbose: bool, msg: &str) {
    if verbose {
        println!("[verbose] {}", msg);
    }
}

/// Print message unless quiet mode is enabled
pub fn info_println(quiet: bool, msg: &str) {
    if !quiet {
        println!("{}", msg);
    }
}
