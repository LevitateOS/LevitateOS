//! `inspect <app>`: render a TUI app headlessly, optionally typing keys, and write its screen.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::util::pty_capture::{PtySession, Settled, parse_keys, plain_text};
use distro_contract::tui_apps::{self, TuiApp};

pub struct Options {
    pub app: String,
    pub args: Vec<String>,
    pub input: Option<String>,
    pub all_pages: bool,
    pub columns: u16,
    pub rows: u16,
    pub timeout_seconds: u64,
    pub settle_ms: u64,
    pub out_dir: Option<PathBuf>,
    pub stdout: bool,
    pub ansi: bool,
    pub keep_transcript: bool,
}

pub fn run(options: Options) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let launch = Launch::prepare(&root, tui_apps::find(&root, &options.app)?)?;
    let keys = match &options.input {
        Some(spec) => parse_keys(spec)?,
        None => Vec::new(),
    };
    let size = Size {
        columns: options.columns.max(1),
        rows: options.rows.max(1),
    };
    let wait = Wait {
        settle: Duration::from_millis(options.settle_ms.max(1)),
        timeout: Duration::from_secs(options.timeout_seconds.max(1)),
    };

    let runs: Vec<(String, Vec<String>)> = if options.all_pages {
        launch
            .pages()?
            .into_iter()
            .map(|(page, args)| (page, [args, options.args.clone()].concat()))
            .collect()
    } else {
        vec![(launch.app.name.clone(), options.args.clone())]
    };

    let base = options
        .out_dir
        .clone()
        .unwrap_or_else(|| root.join(".artifacts/out/tui/inspect"));
    let run_dir = base.join(format!("{}-{}", launch.app.name, unix_timestamp()?));
    fs::create_dir_all(&run_dir).with_context(|| format!("creating '{}'", run_dir.display()))?;

    let mut manifest = format!(
        "app={}\nsize={}x{}\ninput={}\n",
        launch.app.name,
        size.columns,
        size.rows,
        options.input.as_deref().unwrap_or("")
    );
    for (index, (name, args)) in runs.iter().enumerate() {
        if runs.len() > 1 {
            eprintln!("[inspect] {}/{} {name}", index + 1, runs.len());
        }
        let capture = launch.capture(args, &[], &keys, size, wait);
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

/// An app ready to run headlessly: workspace installed, its tools built and on PATH,
/// refused tool calls shimmed out.
pub struct Launch {
    pub app: TuiApp,
    env: Vec<(String, String)>,
    _shims: Option<tempfile::TempDir>,
}

impl Launch {
    pub fn prepare(root: &Path, app: TuiApp) -> Result<Self> {
        crate::tasks::check::install_js_workspace(root)?;
        let mut path = super::tools_path(root, &app)?
            .unwrap_or_else(|| std::env::var_os("PATH").unwrap_or_default());
        if !app.inspect.path.is_empty() {
            let mut paths: Vec<PathBuf> = app.inspect.path.iter().map(|dir| app.dir.join(dir)).collect();
            paths.extend(std::env::split_paths(&path));
            path = std::env::join_paths(paths)?;
        }
        let shims = refuse_shims(&app, &path)?;
        if let Some(dir) = &shims {
            let mut paths = vec![dir.path().to_path_buf()];
            paths.extend(std::env::split_paths(&path));
            path = std::env::join_paths(paths)?;
        }
        Ok(Self {
            app,
            env: vec![("PATH".to_string(), path.to_string_lossy().into_owned())],
            _shims: shims,
        })
    }

    pub fn argv(&self, args: &[String]) -> Vec<String> {
        let mut argv = vec!["bun".to_string(), self.app.entry.display().to_string()];
        argv.extend(self.app.inspect.args.iter().cloned());
        argv.extend(args.iter().cloned());
        argv
    }

    /// Every page the app lists, with the arguments that open it.
    pub fn pages(&self) -> Result<Vec<(String, Vec<String>)>> {
        let Some(pages) = &self.app.inspect.pages else {
            bail!("{} declares no pages to render (levitate.inspect.pages)", self.app.name);
        };
        let output = Command::new("bun")
            .current_dir(&self.app.dir)
            .arg(&self.app.entry)
            .args(&pages.list)
            .envs(self.env.iter().map(|(k, v)| (k, v)))
            .output()
            .with_context(|| format!("listing {} pages", self.app.name))?;
        if !output.status.success() {
            bail!(
                "{} {} failed: {}",
                self.app.name,
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
        &self,
        args: &[String],
        extra_env: &[(String, String)],
        keys: &[Vec<u8>],
        size: Size,
        wait: Wait,
    ) -> std::result::Result<Shot, CaptureError> {
        let argv = self.argv(args);
        let label = format!("inspect {}", self.app.name);
        let mut session =
            PtySession::spawn(
                &label,
                &self.app.dir,
                &argv,
                &[self.env.as_slice(), extra_env].concat(),
                size.columns,
                size.rows,
            )
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
}

/// Writes one shim per refused tool: it exits 97 for a refused call and runs the real
/// tool (found on `path`) otherwise.
fn refuse_shims(app: &TuiApp, path: &std::ffi::OsStr) -> Result<Option<tempfile::TempDir>> {
    if app.inspect.refuse.is_empty() {
        return Ok(None);
    }
    let dir = tempfile::tempdir().context("creating the refuse-shim directory")?;
    let mut rules: Vec<(String, Vec<String>)> = Vec::new();
    for rule in &app.inspect.refuse {
        let mut words = rule.split_whitespace().map(str::to_string);
        let tool = words
            .next()
            .with_context(|| format!("{}: empty refuse rule", app.name))?;
        rules.push((tool, words.collect()));
    }
    let mut tools: Vec<&String> = rules.iter().map(|(tool, _)| tool).collect();
    tools.dedup();
    for tool in tools {
        let real = which::which_in(tool, Some(path), &app.dir)
            .with_context(|| format!("{}: '{tool}' is not on PATH", app.name))?;
        let mut script = String::from("#!/bin/sh\n");
        for (rule_tool, tokens) in &rules {
            if rule_tool != tool {
                continue;
            }
            let test = tokens
                .iter()
                .map(|token| format!("case \" $* \" in *\" {token} \"*) ;; *) false ;; esac"))
                .collect::<Vec<_>>()
                .join(" && ");
            script.push_str(&format!(
                "if {test}; then\n  echo \"cargo xtask inspect: refused '{tool} $*' (headless runs never do this)\" >&2\n  exit 97\nfi\n"
            ));
        }
        script.push_str(&format!("exec '{}' \"$@\"\n", real.display()));
        let shim = dir.path().join(tool);
        fs::write(&shim, script)?;
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&shim, fs::Permissions::from_mode(0o755))?;
    }
    Ok(Some(dir))
}

fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).with_context(|| format!("writing '{}'", path.display()))
}

fn unix_timestamp() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before 1970")?
        .as_secs())
}
