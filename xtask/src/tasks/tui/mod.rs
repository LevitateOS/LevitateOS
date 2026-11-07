//! `tui <app>`: run a terminal app from tui/ in this terminal.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

use distro_contract::tui_apps::{self, TuiApp};

pub fn run(name: String, args: Vec<String>) -> Result<()> {
    // Before installing or building anything: without a terminal the app
    // would refuse anyway, and only xtask can name the headless alternative.
    crate::util::exit::require_terminal(
        "`cargo xtask tui`",
        &format!("cargo xtask inspect {name} renders it without one"),
    )?;
    let root = crate::util::repo::repo_root()?;
    let app = tui_apps::find(&root, &name)?;
    let mut cmd = app_command(&root, &app)?;
    let status = cmd.args(&args).status().context("running bun")?;
    if !status.success() {
        bail!("{name} exited with {status}");
    }
    Ok(())
}

/// `bun <entry>` in the app's directory, after installing the workspace and
/// building the app's Rust tools onto the front of PATH.
pub fn app_command(root: &Path, app: &TuiApp) -> Result<Command> {
    crate::tasks::check::install_js_workspace(root)?;
    let mut cmd = Command::new("bun");
    cmd.current_dir(&app.dir).arg(&app.entry);
    if let Some(path) = tools_path(root, app)? {
        cmd.env("PATH", path);
    }
    Ok(cmd)
}

/// PATH with this checkout's builds of the app's tools first, or None if it needs none.
pub fn tools_path(root: &Path, app: &TuiApp) -> Result<Option<std::ffi::OsString>> {
    if app.tools.is_empty() {
        return Ok(None);
    }
    let mut build = Command::new("cargo");
    build.current_dir(root).args(["build", "-q"]);
    for tool in &app.tools {
        build.args(["-p", tool]);
    }
    if !build.status()?.success() {
        bail!("building {} for {} failed", app.tools.join(", "), app.name);
    }
    let mut paths = vec![crate::util::repo::cargo_target_dir(root).join("debug")];
    paths.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
    Ok(Some(std::env::join_paths(paths)?))
}

pub mod check;
pub mod inspect;
