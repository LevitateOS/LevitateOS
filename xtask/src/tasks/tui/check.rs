//! `check tui`: the checks that tell whether the terminal apps work.
//!
//! For every TUI package: typecheck, lint and format check. For every app: it must
//! draw at a full console size without crashing (on every distro, for an app that is
//! told its distro); the install screen's hub and docs must also draw, on every
//! distro, at the size their pane really has on each console (the docs at their
//! page); snapshots under an app's `snapshots/[<distro>/]<columns>x<rows>/<page>.txt`
//! must match, at sizes the app is really shown at; and each app's declared story
//! (keys typed in turn, text the screens must show) must hold.

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

use super::inspect::{CaptureError, Shot, Size, Wait, capture};
use super::{Launch, Tools};
use crate::cli::{Distro, INSPECT_SIZE, INSPECT_WAIT};
use crate::util::pty_capture::parse_keys;
use distro_builder::tui_apps::{Story, TuiApp, Workspace};
use distro_contract::install_ux::{DOCS, DOCS_PAGE, HUB};

/// Console sizes the install screen runs on: a serial terminal, and the 1920x1080
/// framebuffer the live kernel command line asks for (`video=1920x1080`) in 8x16 cells.
const CONSOLES: &[(u16, u16)] = &[(80, 24), (240, 67)];
const FULL: Size = Size {
    columns: INSPECT_SIZE.0,
    rows: INSPECT_SIZE.1,
};
const WAIT: Wait = Wait {
    settle: std::time::Duration::from_millis(INSPECT_WAIT.0),
    timeout: std::time::Duration::from_secs(INSPECT_WAIT.1),
};
/// Screens that mean the app is broken even though it is still running. (A crash
/// exits non-zero, which a capture already reports.)
const BROKEN: &[&str] = &["Terminal too small", "Invalid hook call"];

pub fn run(bless: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    crate::tasks::check::install_js_workspace(&root)?;
    let workspace = Workspace::load(&root)?;
    let distros = Distro::all()?;
    let mut failed = Vec::new();
    let mut step = |name: String, result: Result<()>| match result {
        Ok(()) => eprintln!("[ok]   {name}"),
        Err(e) => {
            eprintln!("[FAIL] {name}: {e:#}");
            failed.push(name);
        }
    };

    for dir in &workspace.packages {
        let name = dir.strip_prefix(&root).unwrap_or(dir).display().to_string();
        for script in ["typecheck", "lint", "format:check"] {
            step(format!("{script} {name}"), bun_run(dir, script));
        }
    }

    step(
        "install screen apps".to_string(),
        distro_builder::install_ux::Screen::resolve(&workspace).map(drop),
    );
    for app in &workspace.apps {
        let name = &app.name;
        let launch = match Launch::new(&root, app, Tools::StandIns) {
            Ok(launch) => launch,
            Err(e) => {
                step(format!("prepare {name}"), Err(e));
                continue;
            }
        };
        let told = told_distro(app);
        for distro in if told { &distros[..] } else { &[] } {
            let args = launch.distro_args(distro);
            step(
                format!("render {name} on {} {}x{}", distro.id(), FULL.columns, FULL.rows),
                render(&launch, &args, FULL),
            );
        }
        if !told {
            step(format!("render {name} {}x{}", FULL.columns, FULL.rows), render(&launch, &[], FULL));
        }
        let pane = match name.as_str() {
            HUB => Some(0),
            DOCS => Some(1),
            _ => None,
        };
        for (pane, distro) in pane.into_iter().flat_map(|pane| distros.iter().map(move |d| (pane, d))) {
            let mut args = launch.distro_args(distro);
            if pane == 1 {
                args.extend([app.page_arg()?.to_string(), DOCS_PAGE.to_string()]);
            }
            for &(columns, rows) in CONSOLES {
                let (pane_columns, pane_rows) = install_split_pane::pane_sizes(columns, rows)[pane];
                let size = Size {
                    columns: pane_columns,
                    rows: pane_rows,
                };
                step(
                    format!("render {name} in {}'s install screen on a {columns}x{rows} console ({pane_columns}x{pane_rows})", distro.id()),
                    render(&launch, &args, size),
                );
            }
        }
        for story_spec in &app.inspect.stories {
            step(format!("story {name}: {}", story_spec.name), story(&launch, story_spec));
        }
        let snapshots = app.dir.join("snapshots");
        if snapshots.is_dir() {
            if told {
                for distro in &distros {
                    step(
                        format!("snapshots {name} on {}", distro.id()),
                        compare_snapshots(&launch, &snapshots.join(distro.id()), Some(distro), bless),
                    );
                }
            } else {
                step(format!("snapshots {name}"), compare_snapshots(&launch, &snapshots, None, bless));
            }
        }
    }

    if !failed.is_empty() {
        bail!("{} check(s) failed: {}", failed.len(), failed.join(", "));
    }
    Ok(())
}

/// Whether the app is told which distro it runs on (its manifest declares the flags).
fn told_distro(app: &TuiApp) -> bool {
    app.distro_args.os_name.is_some() || app.distro_args.shell.is_some()
}

fn bun_run(dir: &Path, script: &str) -> Result<()> {
    let output = Command::new("bun")
        .current_dir(dir)
        .args(["run", script])
        .output()
        .with_context(|| format!("running bun run {script}"))?;
    if !output.status.success() {
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        let tail: Vec<&str> = text.lines().rev().take(20).collect();
        bail!("{}", tail.into_iter().rev().collect::<Vec<_>>().join("\n"));
    }
    Ok(())
}

fn render(launch: &Launch, args: &[String], size: Size) -> Result<()> {
    let shot = capture(launch, args, &[], &[], size, WAIT).map_err(capture_error)?;
    healthy(&shot)
}

fn healthy(shot: &Shot) -> Result<()> {
    if let Some(marker) = BROKEN.iter().find(|marker| shot.plain.contains(*marker)) {
        bail!("the screen shows '{marker}':\n{}", shot.plain);
    }
    Ok(())
}

fn capture_error(error: CaptureError) -> anyhow::Error {
    match error.shot {
        Some(shot) => anyhow::anyhow!("{}\nscreen:\n{}", error.message, shot.plain),
        None => anyhow::anyhow!(error.message),
    }
}

/// Runs the story: the screen must be healthy after its keys, and show each step's
/// expected text once that step's keys are typed.
fn story(launch: &Launch, spec: &Story) -> Result<()> {
    let start = launch.start(Some(spec), None)?;
    let mut keys = Vec::new();
    let mut typed = String::new();
    let last = spec.steps.len().saturating_sub(1);
    if spec.steps.is_empty() {
        let shot = capture(launch, &start.args, &start.env, &keys, FULL, WAIT).map_err(capture_error)?;
        return healthy(&shot);
    }
    for (index, step) in spec.steps.iter().enumerate() {
        keys.extend(parse_keys(&step.keys)?);
        typed.push_str(&step.keys);
        if step.expect.is_none() && index != last {
            continue;
        }
        let shot = capture(launch, &start.args, &start.env, &keys, FULL, WAIT).map_err(capture_error)?;
        healthy(&shot)?;
        if let Some(expect) = &step.expect
            && !shot.plain.contains(expect) {
                bail!(
                    "step {} (keys so far '{typed}'): expected '{expect}' on screen:\n{}",
                    index + 1,
                    shot.plain
                );
            }
    }
    Ok(())
}

/// `<dir>/<columns>x<rows>/<page>.txt`: render each page at that size (on `distro`)
/// and compare. A size the app is not shown at (the full size, or for the docs a pane
/// size of a console) is stale.
fn compare_snapshots(launch: &Launch, dir: &Path, distro: Option<&Distro>, bless: bool) -> Result<()> {
    let app = launch.app;
    let masks = app
        .inspect
        .mask
        .iter()
        .map(|pattern| Regex::new(pattern).with_context(|| format!("mask '{pattern}'")))
        .collect::<Result<Vec<_>>>()?;
    let mask = |text: &str| {
        masks
            .iter()
            .fold(text.to_string(), |text, re| re.replace_all(&text, "<masked>").into_owned())
    };
    let mut shown = vec![(FULL.columns, FULL.rows)];
    if app.name == DOCS {
        shown.extend(CONSOLES.iter().map(|&(columns, rows)| install_split_pane::docs_pane_size(columns, rows)));
    }
    let distro_args = distro.map(|distro| launch.distro_args(distro)).unwrap_or_default();
    let page_arg = app.page_arg()?.to_string();
    let mut mismatches = Vec::new();
    let mut sizes: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("reading '{}'", dir.display()))?
        .collect::<std::io::Result<_>>()?;
    sizes.sort_by_key(|entry| entry.file_name());
    for size_dir in sizes {
        let size_name = size_dir.file_name().to_string_lossy().into_owned();
        let size = parse_size(&size_name)
            .with_context(|| format!("snapshot directory '{size_name}' is not <columns>x<rows>"))?;
        if !shown.contains(&(size.columns, size.rows)) {
            let shown: Vec<String> = shown.iter().map(|(c, r)| format!("{c}x{r}")).collect();
            bail!(
                "snapshots at {size_name}, a size {} is not shown at (it is at {}): move them",
                app.name,
                shown.join(", ")
            );
        }
        let mut pages: Vec<_> = fs::read_dir(size_dir.path())?.collect::<std::io::Result<_>>()?;
        pages.sort_by_key(|entry| entry.file_name());
        for page_file in pages {
            let path = page_file.path();
            if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                continue;
            }
            let Some(page) = path.file_stem().and_then(|s| s.to_str()).map(str::to_string) else {
                continue;
            };
            let args = [distro_args.clone(), vec![page_arg.clone(), page.clone()]].concat();
            let shot = capture(launch, &args, &[], &[], size, WAIT).map_err(capture_error)?;
            let actual = format!("{}\n", mask(&shot.plain));
            if bless {
                fs::write(&path, &actual)?;
                continue;
            }
            let expected = fs::read_to_string(&path)?;
            if expected != actual {
                mismatches.push(format!(
                    "{size_name}/{page}.txt:\n{}",
                    line_diff(&expected, &actual)
                ));
            }
        }
    }
    if !mismatches.is_empty() {
        bail!(
            "{} snapshot(s) differ (rerun with --bless to accept):\n{}",
            mismatches.len(),
            mismatches.join("\n")
        );
    }
    Ok(())
}

fn parse_size(name: &str) -> Option<Size> {
    let (columns, rows) = name.split_once('x')?;
    Some(Size {
        columns: columns.parse().ok()?,
        rows: rows.parse().ok()?,
    })
}

/// The lines that differ, as `-expected` / `+actual` pairs.
fn line_diff(expected: &str, actual: &str) -> String {
    let (e, a): (Vec<&str>, Vec<&str>) = (expected.lines().collect(), actual.lines().collect());
    let mut out = String::new();
    for i in 0..e.len().max(a.len()) {
        let (x, y) = (e.get(i).copied(), a.get(i).copied());
        if x != y {
            out.push_str(&format!("  line {}:\n  -{}\n  +{}\n", i + 1, x.unwrap_or(""), y.unwrap_or("")));
        }
    }
    out
}
