//! `tui <app>`: run a terminal app from tui/ in this terminal.

use anyhow::{Context, Result, bail};
use std::process::Command;

use crate::cli::TuiApp;

pub fn run(app: TuiApp, args: Vec<String>) -> Result<()> {
    crate::util::exit::require_terminal(
        "`cargo xtask tui`",
        "cargo xtask inspect tui <app> --stdout renders it without one",
    )?;
    let root = crate::util::repo::repo_root()?;
    let mut cmd = match app {
        TuiApp::InstallDocs => {
            let dir = root.join("tui/apps/live-tools/install-docs");
            crate::tasks::check::ensure_node_modules(&dir)?;
            let mut cmd = Command::new("bun");
            cmd.current_dir(dir).arg("src/main.ts");
            cmd.env_remove("NO_COLOR").env("FORCE_COLOR", "3");
            cmd
        }
        TuiApp::InstallDiskPlan => {
            // The app runs this checkout's recpart.
            let status = Command::new("cargo")
                .current_dir(&root)
                .args(["build", "-q", "-p", "recpart"])
                .status()?;
            if !status.success() {
                bail!("building recpart failed");
            }
            let dir = root.join("tui/apps/install/disk-plan");
            crate::tasks::check::ensure_node_modules(&dir)?;
            let target_dir = std::env::var_os("CARGO_TARGET_DIR")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|| root.join("target"));
            let mut paths = vec![target_dir.join("debug")];
            paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
            let mut cmd = Command::new("bun");
            cmd.current_dir(dir)
                .args(["run", "start", "--"])
                .env("PATH", std::env::join_paths(paths)?);
            cmd
        }
    };
    let status = cmd.args(&args).status().context("running bun")?;
    if !status.success() {
        bail!("the app exited with {status}");
    }
    Ok(())
}
