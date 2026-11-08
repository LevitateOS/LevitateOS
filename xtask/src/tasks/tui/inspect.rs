//! `inspect <app>`: render a TUI app headlessly, optionally typing keys, and write its screen.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::{Launch, Tools};
use crate::cli::InspectArgs;
use crate::util::pty_capture::{PtySession, Settled, parse_keys, plain_text};
use distro_builder::tui_apps::Workspace;

pub fn run(options: InspectArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let workspace = Workspace::load(&root)?;
    let app = workspace.app(&options.app)?;
    let launch = Launch::new(&root, app, Tools::StandIns)?;
    let story = options.story.as_deref().map(|story| app.story(story)).transpose()?;
    let distro = options.distro.clone();
    let start = launch.start(story, distro.as_ref())?;
    let mut keys = Vec::new();
    for step in story.iter().flat_map(|story| &story.steps) {
        keys.extend(parse_keys(&step.keys)?);
    }
    if let Some(spec) = &options.input {
        keys.extend(parse_keys(spec)?);
    }
    let size = Size {
        columns: options.columns,
        rows: options.rows,
    };
    let wait = Wait {
        settle: Duration::from_millis(options.settle_ms),
        timeout: Duration::from_secs(options.timeout_seconds),
    };

    // (file name, arguments): a story's file is its number and a slug of its name.
    let runs: Vec<(String, Vec<String>)> = if options.all_pages {
        let distro_args = match &distro {
            Some(distro) => launch.distro_args(distro),
            None => start.args.clone(),
        };
        pages(&launch, &distro_args)?
            .into_iter()
            .enumerate()
            .map(|(index, (page, args))| {
                let args = [distro_args.clone(), args, options.args.clone()].concat();
                (format!("{:02}-{}", index + 1, slug(&page)), args)
            })
            .collect()
    } else {
        let name = match (&options.story, story) {
            (Some(_), Some(story)) => {
                let index = app.inspect.stories.iter().position(|s| s.name == story.name).unwrap_or(0);
                format!("{:02}-{}", index + 1, slug(&story.name))
            }
            _ => slug(&app.name),
        };
        vec![(name, [start.args.clone(), options.args.clone()].concat())]
    };

    let base = options
        .out_dir
        .clone()
        .unwrap_or_else(|| root.join(".artifacts/tui-inspect"));
    let run_dir = new_run_dir(&base, &app.name)?;

    let mut manifest = format!(
        "app={}\nsize={}x{}\ninput={}\n",
        app.name,
        size.columns,
        size.rows,
        options.input.as_deref().unwrap_or("")
    );
    for (index, (name, args)) in runs.iter().enumerate() {
        if runs.len() > 1 {
            eprintln!("[inspect] {}/{} {name}", index + 1, runs.len());
        }
        let capture = capture(&launch, args, &start.env, &keys, size, wait);
        let shot = match &capture {
            Ok(shot) | Err(CaptureError { shot: Some(shot), .. }) => Some(shot),
            Err(_) => None,
        };
        if let Some(shot) = shot {
            write(&run_dir.join(format!("{name}.txt")), shot.plain.as_bytes())?;
            if options.ansi {
                write(&run_dir.join(format!("{name}.ansi")), &shot.formatted)?;
            }
            if options.keep_transcript {
                write(&run_dir.join(format!("{name}.transcript")), &shot.transcript)?;
            }
            if options.stdout {
                println!("{}", shot.plain);
            }
        }
        if let Err(error) = capture {
            bail!("{}\n(screen written to '{}')", error.message, run_dir.display());
        }
        manifest.push_str(&format!("{name}: {}\n", launch.argv(args).join(" ")));
    }
    write(&run_dir.join("manifest.txt"), manifest.as_bytes())?;
    println!("inspect: wrote {} screen(s) to {}", runs.len(), run_dir.display());
    Ok(())
}

/// A fresh `<app>-<unix seconds>[-N]` under `base`; never one an earlier run made.
fn new_run_dir(base: &Path, app: &str) -> Result<PathBuf> {
    fs::create_dir_all(base).with_context(|| format!("creating '{}'", base.display()))?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before 1970")?
        .as_secs();
    for n in 1.. {
        let name = if n == 1 { format!("{app}-{stamp}") } else { format!("{app}-{stamp}-{n}") };
        let dir = base.join(name);
        match fs::create_dir(&dir) {
            Ok(()) => return Ok(dir),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e).with_context(|| format!("creating '{}'", dir.display())),
        }
    }
    unreachable!("an unbounded range ends")
}

/// `name` as a file name: lowercase letters, digits and single dashes.
fn slug(name: &str) -> String {
    let mut slug = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-');
    if slug.is_empty() { "screen".to_string() } else { slug.to_string() }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Clone, Copy)]
pub struct Wait {
    /// How long the screen must stay unchanged to count as drawn.
    pub settle: Duration,
    /// Give up if it never settles.
    pub timeout: Duration,
}

pub struct Shot {
    pub plain: String,
    pub formatted: Vec<u8>,
    pub transcript: Vec<u8>,
}

pub struct CaptureError {
    pub message: String,
    /// The screen at the time, when there was one.
    pub shot: Option<Shot>,
}

/// Every page the app lists (run with `distro_args` first), with the arguments that open it.
pub fn pages(launch: &Launch, distro_args: &[String]) -> Result<Vec<(String, Vec<String>)>> {
    let app = launch.app;
    let Some(pages) = &app.inspect.pages else {
        bail!("{} declares no pages to render (levitate.inspect.pages)", app.name);
    };
    let output = Command::new("bun")
        .current_dir(&app.dir)
        .arg(&app.entry)
        .args(distro_args)
        .args(&pages.list)
        .envs(launch.env().iter().map(|(k, v)| (k, v)))
        .output()
        .with_context(|| format!("listing {} pages", app.name))?;
    if !output.status.success() {
        bail!(
            "{} {} failed: {}",
            app.name,
            pages.list.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|page| !page.is_empty())
        .map(|page| (page.to_string(), vec![pages.arg.clone(), page.to_string()]))
        .collect())
}

/// Starts the app, waits for it to draw, types `keys` one at a time (waiting for
/// each to draw), and returns the screen. An exit before the end is an error.
pub fn capture(
    launch: &Launch,
    args: &[String],
    extra_env: &[(String, String)],
    keys: &[Vec<u8>],
    size: Size,
    wait: Wait,
) -> std::result::Result<Shot, CaptureError> {
    let argv = launch.argv(args);
    let label = format!("inspect {}", launch.app.name);
    let env = [launch.env(), extra_env].concat();
    let mut session = PtySession::spawn(&label, &launch.app.dir, &argv, &env, size.columns, size.rows)
        .map_err(|e| CaptureError {
            message: format!("{e:#}"),
            shot: None,
        })?;
    let shot = |session: &PtySession| Shot {
        plain: plain_text(session.screen()),
        formatted: session.screen().contents_formatted(),
        transcript: session.transcript().to_vec(),
    };
    let fail = |session: &PtySession, message: String| CaptureError {
        message,
        shot: Some(shot(session)),
    };

    for step in 0..=keys.len() {
        match session.settle(wait.settle, wait.timeout, step == 0) {
            Ok(Settled::Quiet) => {}
            Ok(Settled::Exited(status)) => {
                let message = format!(
                    "{label}: `{}` exited ({status:?}) before the capture finished",
                    argv.join(" ")
                );
                return Err(fail(&session, message));
            }
            Err(e) => return Err(fail(&session, format!("{e:#}"))),
        }
        if let Some(key) = keys.get(step) {
            session
                .send_key(key)
                .map_err(|e| fail(&session, format!("{e:#}")))?;
        }
    }
    let result = shot(&session);
    session.finish().map_err(|e| CaptureError {
        message: format!("{e:#}"),
        shot: None,
    })?;
    Ok(result)
}

fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).with_context(|| format!("writing '{}'", path.display()))
}
