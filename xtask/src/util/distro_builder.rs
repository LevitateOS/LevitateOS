//! distro-builder is the producer of every product. xtask runs its binary with a
//! typed `distro_builder::cli::Command`, so the arguments are checked by the compiler.

use anyhow::{Context, Result, bail};
use distro_builder::cli::Command;
use std::path::Path;

/// Run distro-builder with the repo's tool environment.
pub fn run(root: &Path, command: &Command) -> Result<()> {
    run_with(root, command, true)
}

/// Run distro-builder without the tool environment (to install the tools).
pub fn run_bare(root: &Path, command: &Command) -> Result<()> {
    run_with(root, command, false)
}

fn run_with(root: &Path, command: &Command, tools_env: bool) -> Result<()> {
    let bin = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let args = command.to_args(root);
    let mut cmd = std::process::Command::new(&bin);
    cmd.args(&args);
    if tools_env {
        crate::util::tools_env::apply_to_command(&mut cmd, root)?;
    }
    let status = cmd
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        let args: Vec<_> = args.iter().map(|arg| arg.to_string_lossy()).collect();
        bail!("distro-builder {} failed ({status})", args.join(" "));
    }
    Ok(())
}
