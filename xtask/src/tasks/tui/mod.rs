//! The TUI harness: `tui <app>` runs a terminal app from tui/ in this terminal, `tui list`
//! lists them, `inspect` renders one headlessly and `check tui` checks them all.
//!
//! All three start an app the one way [`Launch`] defines: the same environment (PATH
//! with the real tools or the stand-ins, and what the stand-ins read), the distro's
//! facts through the flags the app's manifest declares, and one story lookup.

use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::Path;
use std::process::Command;

use distro_builder::tui_apps::{Story, TuiApp, Workspace};
use distro_contract::install_ux::SHELL;

use crate::cli::Distro;

/// Which tools an app finds on PATH.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tools {
    /// This checkout's builds of the binaries the manifest lists: disk-plan's apply
    /// really partitions and recab-tui's actions really change boot entries.
    Real,
    /// The manifest's stand-ins (`inspect.path`) first, one for every binary it lists,
    /// so nothing real is reached; stand-ins that defer what nothing recorded to the
    /// real tool (disk-plan's recpart plans an edited layout, and never applies) find
    /// this checkout's builds through `LEVITATE_REAL_TOOLS`.
    StandIns,
}

/// An app ready to start: its environment and how its command line begins.
pub struct Launch<'a> {
    pub app: &'a TuiApp,
    env: Vec<(String, String)>,
}

impl<'a> Launch<'a> {
    /// Installs the workspace and sets up `app`'s environment for `tools`.
    pub fn new(root: &Path, app: &'a TuiApp, tools: Tools) -> Result<Self> {
        crate::tasks::check::install_js_workspace(root)?;
        let mut first = Vec::new();
        let mut env = Vec::new();
        if tools == Tools::StandIns {
            for dir in &app.inspect.path {
                first.push(app.dir.join(dir));
            }
            for tool in &app.tools {
                if !first.iter().any(|dir| dir.join(tool).is_file()) {
                    bail!(
                        "{}: no stand-in for '{tool}' in levitate.inspect.path ({}); \
                         stand-in runs never run the real tool",
                        app.name,
                        app.inspect.path.join(", ")
                    );
                }
            }
        }
        if !app.tools.is_empty() {
            let real = build_tools(root, app)?;
            match tools {
                Tools::Real => first.push(real),
                Tools::StandIns => env.push((
                    "LEVITATE_REAL_TOOLS".to_string(),
                    real.to_string_lossy().into_owned(),
                )),
            }
        }
        if !first.is_empty() {
            first.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
            let path = std::env::join_paths(first)?;
            env.push(("PATH".to_string(), path.to_string_lossy().into_owned()));
        }
        Ok(Self { app, env })
    }

    /// The environment the app gets on top of xtask's.
    pub fn env(&self) -> &[(String, String)] {
        &self.env
    }

    /// `bun <entry> <args>`, run in the app's directory.
    pub fn argv(&self, args: &[String]) -> Vec<String> {
        let mut argv = vec!["bun".to_string(), self.app.entry.display().to_string()];
        argv.extend(args.iter().cloned());
        argv
    }

    /// The arguments that tell the app it runs on `distro` (the flags its manifest
    /// declares, as the install screen passes them).
    pub fn distro_args(&self, distro: &Distro) -> Vec<String> {
        self.app.distro_argv(distro.name(), SHELL)
    }

    /// What a run starts with: a story's arguments and environment, or, without one,
    /// the distro's facts, which an app that declares flags for them needs.
    pub fn start(&self, story: Option<&Story>, distro: Option<&Distro>) -> Result<Start> {
        if let Some(story) = story {
            return Ok(Start {
                args: story.args.clone(),
                env: story.env.clone().into_iter().collect(),
            });
        }
        let declared = self.app.distro_args.os_name.is_some() || self.app.distro_args.shell.is_some();
        let args = match distro {
            Some(distro) => self.distro_args(distro),
            None if declared => bail!(
                "{} is told which distro it runs on: give --distro ({}) or a --story",
                self.app.name,
                crate::cli::distro_ids().join("|")
            ),
            None => Vec::new(),
        };
        Ok(Start { args, env: Vec::new() })
    }
}

/// The start of a run: arguments before the caller's, and extra environment.
pub struct Start {
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
}

/// Builds the binaries `app` runs and returns the directory holding them.
fn build_tools(root: &Path, app: &TuiApp) -> Result<std::path::PathBuf> {
    let mut build = Command::new("cargo");
    build.current_dir(root).args(["build", "-q"]);
    for tool in &app.tools {
        build.args(["--bin", tool]);
    }
    if !build.status().context("running cargo build")?.success() {
        bail!("building {} for {} failed", app.tools.join(", "), app.name);
    }
    Ok(crate::util::repo::cargo_target_dir(root).join("debug"))
}

/// xtask's own options on an app's line; the rest goes to the app.
#[derive(Parser)]
#[command(name = "cargo xtask tui <APP>", disable_help_flag = true, disable_version_flag = true)]
struct RunLine {
    /// Run with the app's stand-in tools instead of the real ones.
    #[arg(long)]
    stand_ins: bool,
    /// Start with this story's arguments and environment (implies --stand-ins).
    #[arg(long, value_name = "NAME|NUMBER")]
    story: Option<String>,
    /// The distro the app is told it runs on, for an app that takes one.
    #[arg(long, value_name = "ID")]
    distro: Option<String>,
    /// Arguments for the app.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

pub fn run(name: String, line: Vec<String>) -> Result<()> {
    // Before installing or building anything: without a terminal the app
    // would refuse anyway, and only xtask can name the headless alternative.
    crate::util::exit::require_terminal(
        "`cargo xtask tui`",
        &format!("cargo xtask inspect {name} renders it without one"),
    )?;
    let options = RunLine::try_parse_from(std::iter::once(name.clone()).chain(line))?;
    let root = crate::util::repo::repo_root()?;
    let workspace = Workspace::load(&root)?;
    let app = workspace.app(&name)?;
    let story = options.story.as_deref().map(|story| app.story(story)).transpose()?;
    let tools = if options.stand_ins || story.is_some() { Tools::StandIns } else { Tools::Real };
    let launch = Launch::new(&root, app, tools)?;
    let distro = options.distro.as_deref().map(Distro::load).transpose()?;
    let start = launch.start(story, distro.as_ref())?;
    if tools == Tools::StandIns {
        eprintln!("{name}: stand-in tools from {}; nothing real is changed", app.inspect.path.join(", "));
    }
    let argv = launch.argv(&[start.args, options.args].concat());
    let status = Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(&app.dir)
        .envs(launch.env().iter().map(|(k, v)| (k, v)))
        .envs(start.env.iter().map(|(k, v)| (k, v)))
        .status()
        .context("running bun")?;
    if !status.success() {
        bail!("{name} exited with {status}");
    }
    Ok(())
}

/// Every app under tui/apps, as its manifest describes it.
pub fn list() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let workspace = Workspace::load(&root)?;
    let apps = &workspace.apps;
    let width = apps.iter().map(|app| app.name.len()).max().unwrap_or(0) + 3;
    let indent = " ".repeat(2 + width);
    println!("Apps under tui/apps; run one with `cargo xtask tui <NAME> [ARGS]`:\n");
    for app in apps {
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
        let flags: Vec<&str> = [&app.distro_args.os_name, &app.distro_args.shell]
            .into_iter()
            .flatten()
            .map(String::as_str)
            .collect();
        if !flags.is_empty() {
            println!("{indent}told the distro by: {} (--distro ID passes them)", flags.join(", "));
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

/// Bold, dim and colour on a terminal; plain text in a pipe or a file.
struct Style(bool);

impl Style {
    fn paint(&self, codes: &str, text: &str) -> String {
        if self.0 { format!("\x1b[{codes}m{text}\x1b[0m") } else { text.to_string() }
    }
    fn title(&self, text: &str) -> String {
        self.paint("1;36", text)
    }
    fn heading(&self, text: &str) -> String {
        self.paint("1", text)
    }
    fn dim(&self, text: &str) -> String {
        self.paint("2", text)
    }
    fn command(&self, text: &str) -> String {
        self.paint("32", text)
    }
    fn number(&self, text: &str) -> String {
        self.paint("1;33", text)
    }
}

/// One app's stories in detail: each one's number and name as a heading, what it shows,
/// the command that opens it, how it starts and its steps; then every name again in a
/// short numbered list, the part left on screen.
pub fn stories(name: &str) -> Result<()> {
    use std::io::IsTerminal;
    let style = Style(std::io::stdout().is_terminal());
    let root = crate::util::repo::repo_root()?;
    let workspace = Workspace::load(&root)?;
    let app = workspace.app(name)?;
    let dir = app.dir.strip_prefix(&root).unwrap_or(&app.dir);
    println!("{} {}", style.title(&app.name), style.dim(&format!("· {}", app.description)));
    println!("{}", style.dim(&dir.display().to_string()));
    let stories = &app.inspect.stories;
    if stories.is_empty() {
        println!("\nNo stories (levitate.inspect.stories in its package.json).");
        return Ok(());
    }
    let digits = stories.len().to_string().len();
    let open = |story: &str| format!("cargo xtask tui {name} --story {}", shell_word(story));
    for (n, story) in stories.iter().enumerate() {
        let number = format!("{:>digits$}", n + 1);
        let indent = " ".repeat(digits + 2);
        println!("\n{}", style.dim(&"─".repeat(72)));
        println!("{}  {}", style.number(&number), style.heading(&story.name));
        if let Some(about) = &story.about {
            println!("{indent}{about}");
        }
        println!("{indent}{} {}", style.dim("open:"), style.command(&open(&story.name)));
        let env: Vec<String> = story.env.iter().map(|(k, v)| format!("{k}={v}")).collect();
        let starts = [story.args.join(" "), env.join(" ")]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" · ");
        if !starts.is_empty() {
            println!("{indent}{} {starts}", style.dim("with:"));
        }
        let keys: Vec<String> = story
            .steps
            .iter()
            .enumerate()
            .map(|(i, step)| match (i, step.keys.is_empty()) {
                (0, true) => Ok("(as it opens)".to_string()),
                (_, true) => Ok("(no keys)".to_string()),
                _ => readable_keys(&step.keys),
            })
            .collect::<Result<_>>()?;
        if keys.is_empty() {
            continue;
        }
        println!("{indent}{}", style.dim("steps (keys → what the screen shows):"));
        let width = keys.iter().map(|k| k.chars().count()).max().unwrap_or(0);
        for (i, (step, keys)) in story.steps.iter().zip(&keys).enumerate() {
            let shows = step
                .expect
                .as_deref()
                .map(|e| format!("  {}  {}", style.dim("→"), e.trim().trim_start_matches('│')))
                .unwrap_or_default();
            println!("{indent}  {} {keys:<width$}{shows}", style.dim(&format!("{:>2}.", i + 1)));
        }
    }
    println!("\n{}", style.dim(&"─".repeat(72)));
    println!(
        "{} {}",
        style.title(&format!("{} stories", stories.len())),
        style.dim(&format!("(details above) · stand-in {} from {}: nothing real is changed", app.tools.join(", "), app.inspect.path.join(", ")))
    );
    let name_width = stories.iter().map(|s| s.name.chars().count()).max().unwrap_or(0);
    for (n, story) in stories.iter().enumerate() {
        let about = story.about.as_deref().unwrap_or("");
        let short: String = if about.chars().count() > 60 {
            format!("{}…", about.chars().take(59).collect::<String>().trim_end())
        } else {
            about.to_string()
        };
        println!(
            "  {}  {}  {}",
            style.number(&format!("{:>digits$}", n + 1)),
            style.heading(&format!("{:<name_width$}", story.name)),
            style.dim(&short)
        );
    }
    println!(
        "\n{} {}   {}",
        style.dim("open one:"),
        style.command(&format!("cargo xtask tui {name} --story <number or name>")),
        style.dim(&format!("e.g. --story 1 or --story {}", shell_word(&stories[0].name)))
    );
    Ok(())
}

/// A story's keys (printf escapes, as `parse_keys` reads them) as names: `jj Enter ↓ Enter`.
fn readable_keys(spec: &str) -> Result<String> {
    let mut words: Vec<String> = Vec::new();
    let mut text = String::new();
    for key in crate::util::pty_capture::parse_keys(spec)? {
        let name = match key.as_slice() {
            b"\x1b[A" | b"\x1bOA" => "↑".to_string(),
            b"\x1b[B" | b"\x1bOB" => "↓".to_string(),
            b"\x1b[C" | b"\x1bOC" => "→".to_string(),
            b"\x1b[D" | b"\x1bOD" => "←".to_string(),
            b"\x1b" => "Esc".to_string(),
            b"\x7f" => "Backspace".to_string(),
            b"\r" | b"\n" => "Enter".to_string(),
            b"\t" => "Tab".to_string(),
            b" " => "Space".to_string(),
            other => match std::str::from_utf8(other) {
                Ok(c) if !c.chars().any(char::is_control) => {
                    text.push_str(c);
                    continue;
                }
                _ => other.escape_ascii().to_string(),
            },
        };
        if !text.is_empty() {
            words.push(std::mem::take(&mut text));
        }
        words.push(name);
    }
    if !text.is_empty() {
        words.push(text);
    }
    Ok(words.join(" "))
}

/// `name` quoted for a shell when it needs it.
fn shell_word(name: &str) -> String {
    if name.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c)) {
        name.to_string()
    } else {
        format!("\"{}\"", name.replace('"', "\\\""))
    }
}

pub mod check;
pub mod inspect;
