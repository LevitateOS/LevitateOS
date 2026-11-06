//! `check tui`: the checks that tell whether the terminal apps work.
//!
//! For every TUI package: typecheck, lint and format check. For every app: it must
//! draw at a full console size without crashing; an app a variant shows in its
//! install screen (`[install_ux]` docs) must also draw, at that variant's page, at the
//! size the docs pane really has on each console; snapshots under an app's
//! `snapshots/<columns>x<rows>/<page>.txt` must match; and each app's declared walk
//! (keys typed in turn, text each screen must show) must hold.

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

use super::inspect::{CaptureError, Launch, Shot, Size, Wait};
use crate::util::pty_capture::parse_keys;
use distro_contract::tui_apps::{self, TuiApp};

/// Console sizes the install screen runs on: a serial terminal, and the 1920x1080
/// framebuffer the live kernel command line asks for (`video=1920x1080`) in 8x16 cells.
const CONSOLES: &[(u16, u16)] = &[(80, 24), (240, 67)];
const FULL: Size = super::inspect::DEFAULT_SIZE;
const WAIT: Wait = super::inspect::DEFAULT_WAIT;
/// Screens that mean the app is broken even though it is still running. (A crash
/// exits non-zero, which a capture already reports.)
const BROKEN: &[&str] = &["Terminal too small", "Invalid hook call"];

pub fn run(bless: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    crate::tasks::check::install_js_workspace(&root)?;
    let mut failed = Vec::new();
    let mut step = |name: String, result: Result<()>| match result {
        Ok(()) => eprintln!("[ok]   {name}"),
        Err(e) => {
            eprintln!("[FAIL] {name}: {e:#}");
            failed.push(name);
        }
    };

    for dir in tui_apps::tui_packages(&root)? {
        let name = dir.strip_prefix(&root).unwrap_or(&dir).display().to_string();
        for script in ["typecheck", "lint", "format:check"] {
            step(format!("{script} {name}"), bun_run(&dir, script));
        }
    }

    let hosted = install_screen_docs(&root)?;
    for app in tui_apps::discover(&root)? {
        let name = app.name.clone();
        let launch = match Launch::prepare(&root, app) {
            Ok(launch) => launch,
            Err(e) => {
                step(format!("prepare {name}"), Err(e));
                continue;
            }
        };
        step(format!("render {name} {}x{}", FULL.columns, FULL.rows), render(&launch, &[], FULL));
        for (variant, page) in hosted.iter().filter(|(_, (app, _))| *app == name).map(|(v, (_, p))| (v, p)) {
            let args = match page {
                Some(page) => vec![page_arg(&launch.app)?, page.clone()],
                None => Vec::new(),
            };
            for &(columns, rows) in CONSOLES {
                let (pane_columns, pane_rows) = install_split_pane::docs_pane_size(columns, rows);
                let size = Size {
                    columns: pane_columns,
                    rows: pane_rows,
                };
                step(
                    format!("render {name} as {variant}'s docs pane on a {columns}x{rows} console ({pane_columns}x{pane_rows})"),
                    render(&launch, &args, size),
                );
            }
        }
        for walk_spec in &launch.app.inspect.walks {
            step(format!("walk {name}: {}", walk_spec.name), walk(&launch, walk_spec));
        }
        let snapshots = launch.app.dir.join("snapshots");
        if snapshots.is_dir() {
            step(format!("snapshots {name}"), compare_snapshots(&launch, &snapshots, bless));
        }
    }

    if !failed.is_empty() {
        bail!("{} check(s) failed: {}", failed.len(), failed.join(", "));
    }
    Ok(())
}

/// Each variant whose install screen shows a TUI app: variant, (app, page).
fn install_screen_docs(root: &Path) -> Result<Vec<(String, (String, Option<String>))>> {
    let mut hosted = Vec::new();
    let mut variants: Vec<_> = fs::read_dir(root.join("distro/variants"))?
        .collect::<std::io::Result<_>>()?;
    variants.sort_by_key(|entry| entry.file_name());
    for entry in variants {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('_') || !entry.path().join("ring2/products.toml").is_file() {
            continue;
        }
        let contract = distro_contract::load_variant_contract_for_distro_from(root, &name)
            .with_context(|| format!("loading variant '{name}'"))?;
        if let distro_contract::InstallDocs::TuiApp { app, page } =
            contract.product_config.install_ux.docs
        {
            hosted.push((name, (app, page)));
        }
    }
    Ok(hosted)
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
    let shot = launch.capture(args, &[], &[], size, WAIT).map_err(capture_error)?;
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

fn walk(launch: &Launch, spec: &distro_contract::tui_apps::Walk) -> Result<()> {
    let env: Vec<(String, String)> = spec.env.clone().into_iter().collect();
    let mut keys = Vec::new();
    let mut typed = String::new();
    for (index, step) in spec.steps.iter().enumerate() {
        keys.extend(parse_keys(&step.keys)?);
        typed.push_str(&step.keys);
        let shot = launch.capture(&[], &env, &keys, FULL, WAIT).map_err(capture_error)?;
        healthy(&shot)?;
        if !shot.plain.contains(&step.expect) {
            bail!(
                "step {} (keys so far '{typed}'): expected '{}' on screen:\n{}",
                index + 1,
                step.expect,
                shot.plain
            );
        }
    }
    Ok(())
}

/// `snapshots/<columns>x<rows>/<page>.txt`: render each page at that size and compare.
fn compare_snapshots(launch: &Launch, dir: &Path, bless: bool) -> Result<()> {
    let masks = launch
        .app
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
    let page_arg = page_arg(&launch.app)?;
    let mut mismatches = Vec::new();
    let mut sizes: Vec<_> = fs::read_dir(dir)?.collect::<std::io::Result<_>>()?;
    sizes.sort_by_key(|entry| entry.file_name());
    for size_dir in sizes {
        let size_name = size_dir.file_name().to_string_lossy().into_owned();
        let size = parse_size(&size_name)
            .with_context(|| format!("snapshot directory '{size_name}' is not <columns>x<rows>"))?;
        let mut pages: Vec<_> = fs::read_dir(size_dir.path())?.collect::<std::io::Result<_>>()?;
        pages.sort_by_key(|entry| entry.file_name());
        for page_file in pages {
            let path = page_file.path();
            let Some(page) = path.file_stem().and_then(|s| s.to_str()).map(str::to_string) else {
                continue;
            };
            let args = vec![page_arg.clone(), page.clone()];
            let shot = launch.capture(&args, &[], &[], size, WAIT).map_err(capture_error)?;
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

fn page_arg(app: &TuiApp) -> Result<String> {
    app.inspect
        .pages
        .as_ref()
        .map(|pages| pages.arg.clone())
        .with_context(|| format!("{} has snapshots but declares no inspect.pages", app.name))
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
