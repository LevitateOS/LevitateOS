//! `tui <app>`: run a terminal app from tui/ in this terminal; `tui list`: list them.

use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Command;

use distro_contract::tui_apps::{self, TuiApp};

/// xtask's own options on an app's line (before any `--`): `--stand-ins` runs the app
/// with its stand-in tools instead of the real ones, `--story NAME` (which implies it)
/// also starts it with that story's arguments and environment, for you to drive.
struct RunOptions {
    stand_ins: bool,
    story: Option<String>,
    args: Vec<String>,
}

fn run_options(line: Vec<String>) -> Result<RunOptions> {
    let mut options = RunOptions { stand_ins: false, story: None, args: Vec::new() };
    let mut line = line.into_iter();
    while let Some(arg) = line.next() {
        match arg.as_str() {
            "--" => {
                options.args.extend(line);
                break;
            }
            "--stand-ins" => options.stand_ins = true,
            "--story" => {
                options.story = Some(line.next().context("--story needs a story name")?);
            }
            _ => match arg.strip_prefix("--story=") {
                Some(name) => options.story = Some(name.to_string()),
                None => options.args.push(arg),
            },
        }
    }
    options.stand_ins |= options.story.is_some();
    Ok(options)
}

pub fn run(name: String, line: Vec<String>) -> Result<()> {
    // Before installing or building anything: without a terminal the app
    // would refuse anyway, and only xtask can name the headless alternative.
    crate::util::exit::require_terminal(
        "`cargo xtask tui`",
        &format!("cargo xtask inspect {name} renders it without one"),
    )?;
    let root = crate::util::repo::repo_root()?;
    let app = tui_apps::find(&root, &name)?;
    let options = run_options(line)?;
    let mut cmd = app_command(&root, &app)?;
    let mut args = Vec::new();
    if options.stand_ins {
        // app_command built the real tools; they stay reachable for stand-ins that defer
        // to them for what nothing recorded (disk-plan's recpart plans an edited layout
        // with the real one, and never applies).
        if !app.tools.is_empty() {
            cmd.env("LEVITATE_REAL_TOOLS", crate::util::repo::cargo_target_dir(&root).join("debug"));
        }
        cmd.env("PATH", inspect::stand_in_path(&app)?);
        if let Some(story) = &options.story {
            let story = app
                .inspect
                .stories
                .iter()
                .find(|s| &s.name == story)
                .with_context(|| {
                    let names: Vec<&str> = app.inspect.stories.iter().map(|s| s.name.as_str()).collect();
                    format!("{name} has no story '{story}' (stories: {})", names.join(", "))
                })?;
            args.extend(story.args.iter().cloned());
            cmd.envs(story.env.iter());
        }
        eprintln!("{name}: stand-in tools from {}; nothing real is changed", app.inspect.path.join(", "));
    }
    args.extend(options.args);
    let status = cmd.args(&args).status().context("running bun")?;
    if !status.success() {
        bail!("{name} exited with {status}");
    }
    Ok(())
}

/// Every app under tui/apps, as its manifest describes it.
pub fn list() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let apps = tui_apps::discover(&root)?;
    let width = apps.iter().map(|app| app.name.len()).max().unwrap_or(0) + 3;
    let indent = " ".repeat(2 + width);
    println!("Apps under tui/apps; run one with `cargo xtask tui <NAME> [ARGS]`:\n");
    for app in &apps {
        println!("  {:<width$}{}", app.name, app.description);
        let dir = app.dir.strip_prefix(&root).unwrap_or(&app.dir);
        println!("{indent}{}", dir.display());
        if !app.tools.is_empty() {
            let stand_ins = app.inspect.path.join(", ");
            println!(
                "{indent}runs: {} (the real ones here; stand-ins from {} for --stand-ins, inspect and check)",
                app.tools.join(", "),
                if stand_ins.is_empty() { "nowhere" } else { &stand_ins },
            );
        }
        if let Some(pages) = &app.inspect.pages {
            println!(
                "{indent}pages: `{}` lists them, `{} PAGE` opens one",
                pages.list.join(" "),
                pages.arg
            );
        }
        if !app.inspect.stories.is_empty() {
            let names: Vec<&str> = app.inspect.stories.iter().map(|s| s.name.as_str()).collect();
            println!("{indent}stories: {}", names.join(", "));
        }
        println!();
    }
    println!("`cargo xtask tui <NAME> --help` prints an app's flags and keys.");
    println!("`cargo xtask inspect <NAME> [--story STORY]` renders one headlessly.");
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
