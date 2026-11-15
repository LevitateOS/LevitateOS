//! `recupdate apply`: install a release on a mutable installation (docs/release-engineering.md
//! §6b and §10). Root only; an A/B installation is refused and pointed at `recab update`.
//!
//! 1. Fetch: the channel's release is downloaded and verified into the cache before any file
//!    changes. Nothing newer than the installed release: nothing to do.
//! 2. The intent record (`<state>/applying.json`) is written, atomically, before the first
//!    change.
//! 3. `recipe --system apply-release <folder>`: Recipe applies the release to `/`.
//! 4. The release's kernel and initramfs become a new default boot entry; the entry the system
//!    booted with stays as the second (see `boot`).
//! 5. The intent record is removed; done, with a restart needed.
//!
//! Run again after an interruption, `apply` reads the intent record and resumes: step 3 again
//! when it had not finished (Recipe resumes its own part), else step 4.
//!
//! Recipe is found on `PATH`; `RECUPDATE_RECIPE_BIN` names another program (a stand-in).

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::{BufRead, BufReader, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::release::Manifest;

const STEPS: u32 = 5;
/// The intent record's file name, in the state folder.
const INTENT: &str = "applying.json";
/// One `apply` at a time.
const LOCK: &str = "/run/lock/recupdate-apply.lock";

/// The record of an apply that has begun changing the system.
#[derive(Serialize, Deserialize, Clone)]
pub struct Intent {
    pub schema: u32,
    pub version: String,
    pub serial: u64,
    /// The fetched release folder (`<cache>/<serial>`).
    pub folder: PathBuf,
    /// The step to run next.
    pub step: Step,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Step {
    /// Recipe's `apply-release` has not finished.
    ApplyRelease,
    /// Recipe finished; the boot entry is not in place yet.
    Boot,
}

pub fn intent_path() -> PathBuf {
    crate::check::state_dir().join(INTENT)
}

/// The intent record, if an apply was interrupted.
pub fn read_intent() -> Result<Option<Intent>> {
    let path = intent_path();
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(anyhow!(e).context(format!("reading {}", path.display()))),
    };
    let intent: Intent =
        serde_json::from_str(&text).with_context(|| format!("{} is not JSON", path.display()))?;
    Ok(Some(intent))
}

fn write_intent(intent: &Intent) -> Result<()> {
    let mut text = serde_json::to_string(intent)?;
    text.push('\n');
    crate::util::write_atomic(&intent_path(), text.as_bytes(), 0o644)
}

fn clear_intent() -> Result<()> {
    let path = intent_path();
    match fs::remove_file(&path) {
        Err(e) if e.kind() != ErrorKind::NotFound => {
            Err(anyhow!(e).context(format!("removing {}", path.display())))
        }
        _ => Ok(()),
    }
}

/// Human lines on stdout, or the JSON lines of §10.
#[derive(Clone, Copy)]
struct Report {
    json: bool,
}

impl Report {
    fn step(self, n: u32, text: &str) {
        if self.json {
            println!("{}", json!({"event":"step","n":n,"of":STEPS,"text":text}));
        } else {
            println!("Step {n} of {STEPS}: {text}");
        }
    }

    /// A step of something this step runs: indented (text) or left out (JSON).
    fn sub(self, text: &str) {
        if !self.json {
            println!("  {text}");
        }
    }

    fn progress(self, done: u64, total: u64) {
        if self.json {
            println!("{}", json!({"event":"progress","done":done,"total":total}));
        }
    }

    fn done(self, restart_needed: bool, version: &str, serial: u64, text: &str) {
        if self.json {
            println!(
                "{}",
                json!({"event":"done","restart_needed":restart_needed,"version":version,"serial":serial})
            );
        } else {
            println!("{text}");
        }
    }

    fn failed(self, error: &anyhow::Error) {
        if self.json {
            let line = format!("{error:#}").replace('\n', " ");
            println!("{}", json!({"event":"failed","error":line}));
        }
    }
}

pub fn run(json: bool) -> Result<()> {
    let report = Report { json };
    match apply(report) {
        Ok(()) => Ok(()),
        Err(error) => {
            report.failed(&error);
            Err(error)
        }
    }
}

/// Hold the apply lock until the returned file is dropped.
fn lock() -> Result<fs::File> {
    use std::os::fd::AsRawFd;
    let path = Path::new(LOCK);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    let file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    // SAFETY: flock on a file descriptor this function owns.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        anyhow::bail!("another `recupdate apply` is running");
    }
    Ok(file)
}

/// Download and verify the channel's release into the cache, with nested reporting.
fn fetch(report: Report, channel: &crate::release::Verified) -> Result<PathBuf> {
    let fetch_report = crate::fetch::Report {
        step: &|_, text| report.sub(text),
        progress: &|done, total| report.progress(done, total),
    };
    crate::fetch::into_cache(channel, &fetch_report)
}

fn apply(report: Report) -> Result<()> {
    // SAFETY: geteuid has no preconditions.
    anyhow::ensure!(
        unsafe { libc::geteuid() } == 0,
        "recupdate apply must run as root"
    );
    crate::boot::require_mutable()?;
    let _lock = lock()?;
    // Everything the boot step needs is checked before the first change.
    crate::boot::prepare()?;

    let unchanged = |error: anyhow::Error| anyhow!("{error:#}; nothing on this system changed");
    let intent = read_intent()?;
    let (manifest, folder, step): (Manifest, PathBuf, Step) = match intent {
        Some(intent) => {
            report.step(
                1,
                &format!(
                    "Checking the downloaded release of the interrupted update ({} {})",
                    intent.version, intent.serial
                ),
            );
            match crate::release::verify_fetched(&intent.folder).and_then(|m| {
                anyhow::ensure!(
                    m.serial == intent.serial,
                    "{} holds serial {}, the update record names {}",
                    intent.folder.display(),
                    m.serial,
                    intent.serial
                );
                Ok(m)
            }) {
                Ok(manifest) => (manifest, intent.folder, intent.step),
                Err(error) => {
                    // The interrupted update's files are gone or unusable: continue it with
                    // the channel's release (never older: the channel refuses serials below
                    // the installed one).
                    report.sub(&format!(
                        "{error:#}; downloading the channel's release to finish the update"
                    ));
                    let channel = crate::release::load_channel().map_err(|e| {
                        anyhow!(
                            "{e:#}; the interrupted update is still recorded in {}, run \
`recupdate apply` again",
                            intent_path().display()
                        )
                    })?;
                    let folder = fetch(report, &channel).map_err(|e| {
                        anyhow!(
                            "{e:#}; the interrupted update is still recorded in {}, run \
`recupdate apply` again",
                            intent_path().display()
                        )
                    })?;
                    (channel.manifest, folder, Step::ApplyRelease)
                }
            }
        }
        None => {
            report.step(1, "Downloading and checking the release");
            let channel = crate::release::load_channel().map_err(unchanged)?;
            let installed = crate::release::installed().map_err(unchanged)?;
            let m = &channel.manifest;
            if m.serial <= installed.serial {
                report.done(
                    false,
                    &installed.version,
                    installed.serial,
                    &format!(
                        "{} {} is up to date; there is nothing to apply.",
                        m.os, installed.version
                    ),
                );
                return Ok(());
            }
            let folder = fetch(report, &channel).map_err(unchanged)?;
            (channel.manifest, folder, Step::ApplyRelease)
        }
    };

    report.step(2, "Recording the update before changing anything");
    let mut intent = Intent {
        schema: 1,
        version: manifest.version.clone(),
        serial: manifest.serial,
        folder: folder.clone(),
        step,
    };
    write_intent(&intent).map_err(unchanged)?;

    let recorded = |error: anyhow::Error| {
        anyhow!(
            "{error:#}; the update is recorded in {}: run `recupdate apply` again to finish it",
            intent_path().display()
        )
    };
    if step == Step::ApplyRelease {
        report.step(3, "Applying the release to this system");
        let program = std::env::var_os("RECUPDATE_RECIPE_BIN")
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "recipe".into());
        let mut command = Command::new(program);
        command.args(["--system", "apply-release"]).arg(&folder);
        run_child(report, command).map_err(recorded)?;
        intent.step = Step::Boot;
        write_intent(&intent).map_err(recorded)?;
    } else {
        report.step(3, "Applying the release to this system (already done)");
    }

    report.step(4, "Installing the new kernel as the default boot entry");
    let installed = crate::boot::prepare()
        .and_then(|boot| boot.install(&folder, &manifest))
        .map_err(|error| {
            anyhow!(
                "{error:#}; the release is applied, but the previous kernel stays the default \
boot entry. The update is recorded in {}: run `recupdate apply` again to install the new kernel",
                intent_path().display()
            )
        })?;

    report.step(5, "Removing the update record");
    clear_intent()?;
    report.done(
        true,
        &manifest.version,
        manifest.serial,
        &format!(
            "{} {} (serial {}) is installed. Restart to use it: the boot entry {} is the default, \
and '{}' stays in the boot menu as the previous kernel.",
            manifest.os, manifest.version, manifest.serial, installed.entry, installed.previous
        ),
    );
    Ok(())
}

/// Run `command` with its stdout read line by line. A JSON `progress` line is passed through
/// (JSON) or dropped (text); a JSON `step` line is shown indented (text) or dropped (JSON,
/// where recupdate's own steps count); a `failed` line's error names the failure; any other
/// line goes to stdout (text) or stderr (JSON). Stderr is recupdate's.
fn run_child(report: Report, mut command: Command) -> Result<()> {
    let shown = format!("{command:?}");
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot run {shown}"))?;
    let stdout = child.stdout.take().context("the child's stdout")?;
    let mut failure = None;
    for line in BufReader::new(stdout).lines() {
        let line = line.with_context(|| format!("reading the output of {shown}"))?;
        let parsed = serde_json::from_str::<serde_json::Value>(&line)
            .ok()
            .filter(serde_json::Value::is_object);
        match parsed {
            Some(value) => match value["event"].as_str() {
                Some("progress") => {
                    if report.json {
                        println!("{line}");
                    }
                }
                Some("step") => {
                    if let Some(text) = value["text"].as_str() {
                        report.sub(text);
                    }
                }
                Some("failed") => failure = value["error"].as_str().map(str::to_string),
                _ if report.json => eprintln!("{line}"),
                _ => println!("{line}"),
            },
            None if report.json => eprintln!("{line}"),
            None => println!("{line}"),
        }
    }
    let status = child
        .wait()
        .with_context(|| format!("waiting for {shown}"))?;
    if status.success() {
        return Ok(());
    }
    Err(anyhow!(match failure {
        Some(error) => format!("{shown} failed: {error}"),
        None => format!("{shown} failed ({status})"),
    }))
}
