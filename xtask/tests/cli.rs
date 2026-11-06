//! Public CLI checks: scenario state handling and the exit-code contract.
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use tempfile::TempDir;

fn xtask(state_dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_levitate-xtask"));
    command.env("LEVITATE_SCENARIO_STATE_DIR", state_dir);
    command.env("NO_COLOR", "1");
    command.stdin(Stdio::null());
    command
}

fn diagnostic(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

#[test]
fn status_never_reports_stale_cached_success_as_a_pass() {
    let temp = TempDir::new().unwrap();
    let state = temp.path().join("levitate.json");
    let cached = r#"{
        "input_fingerprints": {"live-tools": "obsolete-input", "runtime": "obsolete-input"},
        "results": {
            "live-tools": {"passed": true, "timestamp": "old", "evidence": "old run"},
            "runtime": {"passed": true, "timestamp": "old", "evidence": "old run"}
        }
    }"#;
    fs::write(&state, cached).unwrap();

    let output = xtask(temp.path()).args(["status", "levitate"]).output().unwrap();
    assert!(output.status.success(), "{}", diagnostic(&output));
    let text = String::from_utf8(output.stdout).unwrap();
    for scenario in ["live-tools", "runtime"] {
        let row = text.lines().find(|line| line.contains(scenario)).unwrap();
        assert!(row.contains("[STALE]"), "{text}");
    }
    assert!(!text.contains("[PASS]"), "{text}");

    let output = xtask(temp.path())
        .args(["status", "levitate", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", diagnostic(&output));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let statuses: Vec<(&str, &str)> = json["scenarios"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| (s["scenario"].as_str().unwrap(), s["status"].as_str().unwrap()))
        .collect();
    assert!(statuses.contains(&("live-tools", "stale")), "{json}");
    assert!(statuses.contains(&("install", "not-run")), "{json}");
    assert_eq!(fs::read_to_string(state).unwrap(), cached);
}

#[test]
fn corrupt_state_is_reported_and_preserved_until_explicit_reset() {
    let temp = TempDir::new().unwrap();
    let state = temp.path().join("acorn.json");
    fs::write(&state, "{broken cache").unwrap();
    let output = xtask(temp.path()).args(["status", "acorn"]).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "{}", diagnostic(&output));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("Invalid scenario state"), "{error}");
    assert!(error.contains("cargo xtask reset acorn"), "{error}");
    assert_eq!(fs::read_to_string(&state).unwrap(), "{broken cache");

    let reset = xtask(temp.path()).args(["reset", "acorn"]).output().unwrap();
    assert!(reset.status.success(), "{}", diagnostic(&reset));
    assert!(!state.exists());
}

#[test]
fn bad_usage_exits_2_without_touching_state() {
    let temp = TempDir::new().unwrap();
    let state = temp.path().join("levitate.json");
    fs::write(&state, "sentinel").unwrap();
    for args in [
        vec!["test", "runtime", "levitate", "--up-to-scenario"],
        vec!["status", "levitate", "--force"],
        vec!["reset", "../other"],
        vec!["build", "kernel"],
        vec!["run", "live-boot", "levitate", "--no-such-option", "--no-shell"],
    ] {
        let output = xtask(temp.path()).args(&args).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}\n{}", diagnostic(&output));
        assert_eq!(fs::read_to_string(&state).unwrap(), "sentinel");
    }
}

#[test]
fn interactive_commands_without_a_terminal_exit_2_and_name_the_alternative() {
    let temp = TempDir::new().unwrap();
    for args in [vec!["run", "live-tools", "acorn"], vec!["tui", "install-docs"]] {
        let output = xtask(temp.path())
            .args(&args)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}\n{}", diagnostic(&output));
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("needs a terminal"), "{error}");
        assert!(error.contains("instead: cargo xtask"), "{error}");
    }
}

#[test]
fn doctor_json_lists_every_check_with_a_fix_for_each_gap() {
    let temp = TempDir::new().unwrap();
    let output = xtask(temp.path()).args(["doctor", "--json"]).output().unwrap();
    assert!(
        matches!(output.status.code(), Some(0) | Some(3)),
        "{}",
        diagnostic(&output)
    );
    let checks: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
    assert!(checks.iter().any(|c| c["check"] == "OVMF firmware"), "{checks:?}");
    for check in &checks {
        if check["ok"] == false {
            assert!(check["fix"].is_string(), "{check}");
        }
    }
}
