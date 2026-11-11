//! `cargo xtask progress [distro] [--once]`: where a running OS build is.
//!
//! The builder (`distro_builder::timeline`) keeps `.artifacts/progress/<distro>.json`
//! (the step stack, its pid, the active Recipe progress file) and appends every
//! step's start and end to `timeline.jsonl` in the run directory. Recipe keeps
//! the progress file of one invocation (`done`/`total`/`what`). This view reads
//! those files and prints, per distro: the product and step stack, `done/total
//! what` with a bar, the rate, the elapsed times, an ETA, and the last finished
//! steps. It only reads; it works in a plain terminal over SSH.
//!
//! - Rate: items per minute over the last [`WINDOW`] seconds of this process's
//!   samples of the Recipe file (one per change of its `updated`); with fewer
//!   than two samples (`--once`, the first redraw) the average since the
//!   invocation started.
//! - ETA with a count: remaining items over the rate. Without one (a step that
//!   reports no total): the median duration of the same step in earlier runs'
//!   `timeline.jsonl` (`.artifacts/out/<distro>/releases/*/*/`) less the time
//!   already spent, marked "~ from earlier runs".

use anyhow::Result;
use distro_builder::timeline::{self, Event, State};
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use std::fmt::Write as _;
use std::io::Write as _;
use std::path::Path;
use std::time::Duration;

use crate::cli::Distro;

/// How far back the rate looks.
const WINDOW: f64 = 300.0;
/// A state this old, with no live process to vouch for it, looks stopped.
const STALE: i64 = 300;
/// How many finished steps are listed.
const FINISHED: usize = 6;

/// Recipe's `--progress-file` (read leniently: a field it adds is ignored).
#[derive(Debug, Deserialize)]
struct RecipeProgress {
    #[serde(default)]
    recipe: String,
    #[serde(default)]
    phase: String,
    #[serde(default)]
    what: String,
    #[serde(default)]
    done: u64,
    #[serde(default)]
    total: u64,
    #[serde(default)]
    started: String,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    state: String,
}

/// Samples of one Recipe invocation's `done`, as (file time, done).
#[derive(Default)]
struct History {
    key: String,
    samples: VecDeque<(f64, u64)>,
}

impl History {
    fn add(&mut self, key: &str, at: f64, done: u64) {
        if self.key != key || self.samples.back().is_some_and(|&(_, last)| done < last) {
            self.key = key.to_string();
            self.samples.clear();
        }
        if self.samples.back().map(|&(time, _)| time) != Some(at) {
            self.samples.push_back((at, done));
        }
        while self.samples.front().is_some_and(|&(time, _)| at - time > WINDOW) {
            self.samples.pop_front();
        }
    }

    /// Items per second over the window, when there are two samples apart.
    fn rate(&self) -> Option<f64> {
        let (first, last) = (self.samples.front()?, self.samples.back()?);
        let span = last.0 - first.0;
        (span >= 5.0 && last.1 > first.1).then(|| (last.1 - first.1) as f64 / span)
    }
}

pub fn run(distro: Option<&Distro>, once: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let mut histories: HashMap<String, History> = HashMap::new();
    loop {
        let text = render(&root, distro.map(Distro::id), &mut histories);
        if once {
            print!("{text}");
            return Ok(());
        }
        // Home, the lines (each cleared to its end), then clear below.
        let mut out = String::from("\x1b[H");
        for line in text.lines() {
            let _ = writeln!(out, "{line}\x1b[K");
        }
        out.push_str("\x1b[J");
        print!("{out}");
        let _ = std::io::stdout().flush();
        std::thread::sleep(Duration::from_secs(2));
    }
}

/// One line per running build, for `cargo xtask status`.
pub fn summary_lines(root: &Path, distro: &str) -> Vec<String> {
    let Some(state) = read_state(root, distro) else {
        return Vec::new();
    };
    if state.state != "running" || !alive(state.pid) {
        return Vec::new();
    }
    let now = timeline::now_epoch();
    let started = timeline::parse_rfc3339(&state.started).unwrap_or(now);
    let mut line = format!(
        "build running: {} for {}, {}",
        state.product,
        duration((now - started) as f64),
        step_path(&state)
    );
    if let Some(recipe) = read_recipe(&state).filter(|recipe| recipe.total > 0) {
        let _ = write!(line, ", {}/{} {}", recipe.done, recipe.total, recipe.what);
    }
    vec![line, "  details: cargo xtask progress".into()]
}

fn render(root: &Path, only: Option<&str>, histories: &mut HashMap<String, History>) -> String {
    let ids: Vec<String> = match only {
        Some(id) => vec![id.to_string()],
        None => crate::cli::distro_ids(),
    };
    let mut out = format!("cargo xtask progress   {}\n", timeline::clock());
    let mut shown = 0;
    for id in &ids {
        let Some(state) = read_state(root, id) else {
            if only.is_some() {
                let _ = writeln!(out, "\n{id}: no build running (no {})", relative(root, &timeline::state_path(root, id)));
            }
            continue;
        };
        shown += 1;
        out.push('\n');
        out.push_str(&render_distro(root, &state, histories));
    }
    if shown == 0 {
        out.push_str(if only.is_some() { "" } else { "\nno build running (no state files under .artifacts/progress)\n" });
    }
    out
}

fn render_distro(root: &Path, state: &State, histories: &mut HashMap<String, History>) -> String {
    let now = timeline::now_epoch();
    let started = timeline::parse_rfc3339(&state.started).unwrap_or(now);
    let updated = timeline::parse_rfc3339(&state.updated).unwrap_or(now);
    let alive = alive(state.pid);
    let mut out = String::new();

    let (verdict, running) = match state.state.as_str() {
        "finished" => (format!("last build finished {} ago, in {}", duration((now - updated) as f64), duration((updated - started) as f64)), false),
        "failed" => (format!("last build FAILED {} ago, after {}", duration((now - updated) as f64), duration((updated - started) as f64)), false),
        _ if alive => ("running".to_string(), true),
        _ => (
            format!("looks stopped: the builder (pid {}) is gone and the state was last updated {} ago", state.pid, duration((now - updated) as f64)),
            false,
        ),
    };
    let _ = writeln!(out, "{}  {}  {}", state.distro, state.product, verdict);
    let _ = writeln!(out, "  run       {}", relative(root, Path::new(&state.run_dir)));
    if running {
        let _ = writeln!(out, "  build     {} elapsed (started {})", duration((now - started) as f64), timeline::clock_at(started));
        let _ = writeln!(out, "  now       {}", step_path(state));
        if let Some(step) = state.steps.last() {
            let step_started = timeline::parse_rfc3339(&step.started).unwrap_or(now);
            let _ = writeln!(out, "  step      {} elapsed", duration((now - step_started) as f64));
        }
        let recipe = read_recipe(state).filter(|recipe| recipe.state == "running" || recipe.state.is_empty());
        let counted = recipe.as_ref().filter(|recipe| recipe.total > 0);
        if let Some(recipe) = &recipe {
            let label = if recipe.what.is_empty() { recipe.phase.clone() } else { recipe.what.clone() };
            if recipe.total > 0 {
                let fraction = (recipe.done as f64 / recipe.total as f64).min(1.0);
                let _ = writeln!(out, "  progress  {} {:>3.0}%  {}/{} {}", bar(fraction), fraction * 100.0, recipe.done, recipe.total, label);
            } else {
                let _ = writeln!(out, "  progress  {} (no count; {} phase)", if label.is_empty() { "working".into() } else { label }, recipe.phase);
            }
            let invocation_start = timeline::parse_rfc3339(&recipe.started).unwrap_or(now);
            let _ = writeln!(out, "  recipe    {} up {}", recipe.recipe, duration((now - invocation_start) as f64));
        }
        let mut eta_done = false;
        if let Some(recipe) = counted {
            let history = histories.entry(state.distro.clone()).or_default();
            let at = timeline::parse_rfc3339(&recipe.updated).unwrap_or(now) as f64;
            history.add(&format!("{}/{}/{}", recipe.recipe, recipe.phase, recipe.what), at, recipe.done);
            let invocation_start = timeline::parse_rfc3339(&recipe.started).unwrap_or(started) as f64;
            let (rate, basis) = match history.rate() {
                Some(rate) => (Some(rate), "last 5 min"),
                None if recipe.done > 0 && at > invocation_start => (Some(recipe.done as f64 / (at - invocation_start)), "average since start"),
                None => (None, ""),
            };
            match rate {
                Some(rate) => {
                    let _ = writeln!(out, "  rate      {:.1} {}/min ({basis})", rate * 60.0, recipe.what);
                    let remaining = recipe.total.saturating_sub(recipe.done) as f64;
                    let seconds = remaining / rate;
                    let _ = writeln!(out, "  ETA       {} (at {}) for this step, from the rate", duration(seconds), timeline::clock_at(now + seconds as i64));
                    eta_done = true;
                }
                None => {
                    let _ = writeln!(out, "  rate      not enough progress yet");
                }
            }
        }
        if let Some(step) = state.steps.last().filter(|_| !eta_done) {
            {
                let step_started = timeline::parse_rfc3339(&step.started).unwrap_or(now);
                let spent = (now - step_started) as f64;
                match earlier_median(root, state, step) {
                    Some((median, runs)) => {
                        if spent <= median {
                            let _ = writeln!(out, "  ETA       ~ {} (at ~{}) ~ from earlier runs (median {} of {runs})", duration(median - spent), timeline::clock_at(now + (median - spent) as i64), duration(median));
                        } else {
                            let _ = writeln!(out, "  ETA       over the usual {} by {} ~ from earlier runs ({runs})", duration(median), duration(spent - median));
                        }
                    }
                    None => {
                        let _ = writeln!(out, "  ETA       unknown (no count, no earlier run of this step)");
                    }
                }
            }
        }
        let quiet = now - updated;
        let recipe_quiet = read_recipe(state).and_then(|r| timeline::parse_rfc3339(&r.updated)).map(|t| now - t);
        if quiet > STALE && recipe_quiet.is_none_or(|age| age > STALE) {
            let _ = writeln!(out, "  note      no update for {} (pid {} is alive; a long step without counts)", duration(quiet as f64), state.pid);
        }
    }
    let events = read_events(Path::new(&state.run_dir));
    let finished: Vec<&Event> = events.iter().filter(|event| event.event == "end" && event.kind != "run").collect();
    if !finished.is_empty() {
        let _ = writeln!(out, "  finished steps (last {FINISHED}):");
        for event in finished.iter().rev().take(FINISHED).rev() {
            let note = event.note.as_deref().map(|n| format!(" ({n})")).unwrap_or_default();
            let failed = if event.ok == Some(false) { "  FAILED" } else { "" };
            let _ = writeln!(out, "    {:>9}  {}{}{note}{failed}", duration(event.seconds.unwrap_or(0.0)), "  ".repeat(event.depth.saturating_sub(1).min(4)), event.step);
        }
    }
    out
}

fn read_state(root: &Path, distro: &str) -> Option<State> {
    let bytes = std::fs::read(timeline::state_path(root, distro)).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn read_recipe(state: &State) -> Option<RecipeProgress> {
    let active = state.recipe.as_ref()?;
    serde_json::from_slice(&std::fs::read(&active.progress_file).ok()?).ok()
}

fn read_events(run_dir: &Path) -> Vec<Event> {
    let Ok(text) = std::fs::read_to_string(run_dir.join("timeline.jsonl")) else {
        return Vec::new();
    };
    text.lines().filter_map(|line| serde_json::from_str(line).ok()).collect()
}

/// The median duration of the earlier successful runs of `step`, and how many
/// runs it is of (the other runs' `timeline.jsonl`, same distro, any product).
fn earlier_median(root: &Path, state: &State, step: &timeline::StepRef) -> Option<(f64, usize)> {
    let mut seconds = Vec::new();
    let releases = distro_contract::runs::distro_output_dir(root, &state.distro).join("releases");
    for product in std::fs::read_dir(releases).ok()?.flatten() {
        for run in std::fs::read_dir(product.path()).into_iter().flatten().flatten() {
            if run.path() == Path::new(&state.run_dir) {
                continue;
            }
            for event in read_events(&run.path()) {
                // A reused part took no work: it says nothing about building one.
                if event.event == "end" && event.step == step.name && event.kind == step.kind && event.ok == Some(true) && event.note.as_deref() != Some("reused") {
                    seconds.extend(event.seconds);
                }
            }
        }
    }
    if seconds.is_empty() {
        return None;
    }
    seconds.sort_by(f64::total_cmp);
    let middle = seconds.len() / 2;
    let median = if seconds.len() % 2 == 1 { seconds[middle] } else { (seconds[middle - 1] + seconds[middle]) / 2.0 };
    Some((median, seconds.len()))
}

/// `part system-desktop > preparing the system tree > recipe install x.rhai`
/// (without the build itself, the first step).
fn step_path(state: &State) -> String {
    let names: Vec<&str> = state.steps.iter().filter(|step| step.kind != "run").map(|step| step.name.as_str()).collect();
    if names.is_empty() { "starting".into() } else { names.join(" > ") }
}

fn bar(fraction: f64) -> String {
    let width = 30;
    let filled = ((fraction * width as f64).round() as usize).min(width);
    format!("[{}{}]", "#".repeat(filled), "-".repeat(width - filled))
}

fn duration(seconds: f64) -> String {
    timeline::format_duration(seconds)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).map(|p| p.display().to_string()).unwrap_or_else(|_| path.display().to_string())
}

/// Whether process `pid` exists (EPERM: it does, as another user's).
fn alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    // SAFETY: signal 0 only checks that the process exists.
    let result = unsafe { libc::kill(pid as libc::pid_t, 0) };
    result == 0 || std::io::Error::last_os_error().raw_os_error() == Some(libc::EPERM)
}
