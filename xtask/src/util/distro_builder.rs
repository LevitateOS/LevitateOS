//! distro-builder is the producer of every product. xtask runs its binary with a
//! typed `distro_builder::cli::Command`, so the arguments are checked by the compiler.

use anyhow::{Context, Result, bail};
use distro_builder::cli::Command;
use std::path::Path;

/// Run distro-builder.
pub fn run(root: &Path, command: &Command) -> Result<()> {
    let bin = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let args = command.to_args(root);
    let mut cmd = std::process::Command::new(&bin);
    cmd.args(&args);
    let status = cmd
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        let args: Vec<_> = args.iter().map(|arg| arg.to_string_lossy()).collect();
        bail!("distro-builder {} failed ({status})", args.join(" "));
    }
    Ok(())
}
