//! `run`, `test`, `status` and `reset`: install-tests does the work (VMs,
//! SSH, serial, inputs, caching, results); xtask turns flags into its typed
//! options and renders what it returns.

use anyhow::{Context, Result};
use install_tests::interactive::Mode;
use install_tests::scenarios::{Outcome, RunOptions, ScenarioId, ScenarioStatus};
use std::fs::OpenOptions;
use std::os::fd::AsRawFd;

use crate::cli::{BootArgs, Distro, RunArgs, TestArgs, TestSelection};
use crate::util::exit::usage;

/// The options every scenario run shares, from the boot flags.
fn run_options(boot: &BootArgs, force: bool) -> Result<RunOptions> {
    let mut injection = Vec::new();
    if let Some(path) = &boot.inject_file {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading --inject-file {}", path.display()))?;
        injection.extend(text.lines().map(str::trim).filter(|l| !l.is_empty()).map(str::to_string));
    }
    if let Some(raw) = &boot.inject {
        injection.extend(install_tests::boot_injection::parse_pairs(raw).map_err(|e| usage(format!("--inject: {e:#}")))?);
    }
    if let Some(key) = &boot.ssh_key
        && !key.is_file() {
            return Err(usage(format!("--ssh-key {} is not a file", key.display())));
        }
    Ok(RunOptions {
        force,
        injection,
        login_key: boot.ssh_key.clone(),
        install_mode: Default::default(),
    })
}

pub fn run(args: RunArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = &args.distro;
    let key = args.scenario.key();
    if !args.no_shell {
        crate::util::exit::require_terminal(
            "`cargo xtask run` (it hands you the guest's console)",
            &format!(
                "cargo xtask run {key} {0} --no-shell to boot and check it, or cargo xtask test {key} {0}",
                distro.id()
            ),
        )?;
    }
    let installed = args.scenario.input().iso.is_none();
    if installed && (args.ssh || args.no_shell) {
        return Err(usage(format!(
            "{key} opens the installed system's console; to check it without one use cargo xtask test {key} {}",
            distro.id()
        )));
    }
    if installed && (args.boot.inject.is_some() || args.boot.inject_file.is_some() || args.boot.ssh_key.is_some()) {
        return Err(usage(format!(
            "{key} takes no boot injection: the live init reads it, the installed system does not",
        )));
    }
    let mode = match (args.ssh, args.no_shell) {
        (true, no_shell) => Mode::Ssh { port: args.ssh_port, shell: !no_shell },
        (false, true) => Mode::Check,
        (false, false) => Mode::Console,
    };
    let options = run_options(&args.boot, false)?;
    // Some shells send cursor-position queries; their replies must not leak
    // into the guest's console or into the shell after it.
    flush_tty_input_queue();
    let result = install_tests::interactive::open(&root, distro.id(), args.scenario, mode, &options);
    flush_tty_input_queue();
    result
}

/// Runs the selected scenarios. Each runs on its own against the artifacts
/// that exist: one failing, or missing its inputs, does not stop the rest.
pub fn test(args: TestArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = &args.distro;
    let scenarios = match args.scenario {
        TestSelection::All => ScenarioId::ALL
            .into_iter()
            .filter(|scenario| {
                // ab-update updates an A/B install; `all` leaves it out of other modes.
                let skip = *scenario == ScenarioId::AbUpdate && args.mode != recpart::InstallMode::Ab;
                if skip {
                    eprintln!("skipping {}: it needs --mode {}", scenario.key(), recpart::InstallMode::Ab);
                }
                !skip
            })
            .collect(),
        TestSelection::One(scenario) => vec![scenario],
    };
    let options = RunOptions {
        install_mode: args.mode,
        ..run_options(&args.boot, args.force)?
    };
    let outcomes = install_tests::scenarios::run(&root, distro.id(), &scenarios, &options)?;

    let label = |outcome: &Outcome| match outcome {
        Outcome::Passed(_) => "pass",
        Outcome::Cached => "cached",
        Outcome::Failed(_) => "fail",
        Outcome::Missing(_) => "missing",
        Outcome::NotRun(_) => "not-run",
    };
    let detail = |outcome: &Outcome| match outcome {
        Outcome::Passed(evidence) => Some(evidence.clone()),
        Outcome::Cached => None,
        Outcome::Failed(err) => Some(format!("{err:#}")),
        Outcome::Missing(missing) => Some(format!("{missing}; fix: {}", crate::util::exit::fix_for(missing))),
        Outcome::NotRun(why) => Some(why.clone()),
    };
    if args.json {
        let summary: Vec<_> = outcomes
            .iter()
            .map(|(scenario, outcome)| {
                serde_json::json!({
                    "scenario": scenario.key(),
                    "result": label(outcome),
                    "passed": matches!(outcome, Outcome::Passed(_) | Outcome::Cached),
                    "detail": detail(outcome),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else if outcomes.len() > 1 {
        println!();
        for (scenario, outcome) in &outcomes {
            let line = detail(outcome).map(|d| d.lines().next().unwrap_or_default().to_string());
            println!("  {:<8} {:<15} {}", label(outcome), scenario.key(), line.unwrap_or_default());
        }
    }

    let failed: Vec<&str> = outcomes
        .iter()
        .filter(|(_, outcome)| !matches!(outcome, Outcome::Passed(_) | Outcome::Cached))
        .map(|(scenario, _)| scenario.key())
        .collect();
    let count = failed.len();
    let names = failed.join(", ");
    let first = outcomes.into_iter().find_map(|(_, outcome)| match outcome {
        Outcome::Passed(_) | Outcome::Cached => None,
        Outcome::Failed(err) => Some(err),
        Outcome::Missing(missing) => Some(anyhow::Error::new(missing)),
        Outcome::NotRun(why) => Some(anyhow::anyhow!(why)),
    });
    match first {
        None => Ok(()),
        Some(first) if count == 1 => Err(first),
        Some(first) => Err(first.context(format!(
            "{count} scenarios did not pass for {}: {names} (the first one's error follows)",
            distro.id()
        ))),
    }
}

pub fn status(distro: &Distro, json: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let statuses = install_tests::scenarios::scenario_statuses(&root, distro.id())?;
    if !json {
        println!("{} Scenario Status", distro.name());
        println!();
        for (scenario, status) in &statuses {
            let label = match status {
                ScenarioStatus::Stale => "[STALE]",
                ScenarioStatus::Error(_) => "[ERROR]",
                ScenarioStatus::Pass => "[PASS] ",
                ScenarioStatus::Fail => "[FAIL] ",
                ScenarioStatus::NotRun => "[     ]",
            };
            println!("  {label} {:<15} {}", scenario.key(), scenario.title());
            if let Some(why) = status.error() {
                println!("          {}", why.lines().next().unwrap_or_default());
            }
        }
        println!();
        return Ok(());
    }
    let scenarios: Vec<_> = statuses
        .iter()
        .map(|(scenario, status)| {
            serde_json::json!({ "scenario": scenario.key(), "status": status.label(), "error": status.error() })
        })
        .collect();
    // The latest ISO of each release product, for scripts (CI uploads them).
    let mut artifacts = serde_json::Map::new();
    for product in distro_contract::ProductId::releases() {
        let iso = distro_contract::runs::latest_release_iso(&root, distro.id(), product)?;
        artifacts.insert(product.name().to_string(), serde_json::json!(iso));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({ "distro": distro.id(), "scenarios": scenarios, "artifacts": artifacts })
        )?
    );
    Ok(())
}

pub fn reset(distro: &Distro) -> Result<()> {
    install_tests::scenarios::reset(&crate::util::repo::repo_root()?, distro.id())?;
    println!("Scenario runs removed for {}.", distro.id());
    Ok(())
}

/// Drop pending input on the controlling terminal, if there is one.
fn flush_tty_input_queue() {
    let Ok(tty) = OpenOptions::new().read(true).open("/dev/tty") else {
        return;
    };
    // SAFETY: tcflush only uses the provided valid file descriptor.
    unsafe { libc::tcflush(tty.as_raw_fd(), libc::TCIFLUSH) };
}
