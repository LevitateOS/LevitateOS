//! Public CLI checks: status output and the exit-code contract.
use std::process::{Command, Output, Stdio};

fn xtask() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_levitate-xtask"));
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
fn status_json_reports_every_scenario_and_the_latest_isos_for_both_distros() {
    for distro in ["levitate", "acorn"] {
        let output = xtask().args(["status", distro, "--json"]).output().unwrap();
        assert!(output.status.success(), "{}", diagnostic(&output));
        let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let scenarios = json["scenarios"].as_array().unwrap();
        assert_eq!(scenarios.len(), install_tests::scenarios::ScenarioId::ALL.len(), "{json}");
        for scenario in scenarios {
            let status = scenario["status"].as_str().unwrap();
            assert!(["not-run", "pass", "fail", "stale", "error"].contains(&status), "{json}");
            assert_eq!(status == "error", scenario["error"].is_string(), "{json}");
        }
        for product in ["base-rootfs", "live-boot", "live-tools"] {
            let iso = &json["artifacts"][product];
            assert!(iso.is_null() || iso.is_string(), "{json}");
        }
    }
}

#[test]
fn bad_usage_exits_2() {
    for args in [
        vec!["test", "runtime", "acorn", "--no-such-option"],
        vec!["status", "levitate", "--force"],
        vec!["reset", "../other"],
        vec!["build", "kernel"],
        vec!["run", "live-boot", "levitate", "--no-such-option", "--no-shell"],
        vec!["run", "live-boot", "acorn", "--ssh-port", "2222", "--no-shell"],
        vec!["test", "live-boot", "levitate", "--inject", "NOEQUALS"],
        vec!["test", "live-boot", "acorn", "--ssh-key", "/nonexistent/key"],
    ] {
        let output = xtask().args(&args).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}\n{}", diagnostic(&output));
    }
}

#[test]
fn interactive_commands_without_a_terminal_exit_2_and_name_the_alternative() {
    for args in [
        vec!["run", "live-tools", "acorn"],
        vec!["run", "live-boot", "levitate"],
        vec!["tui", "install-docs"],
    ] {
        let output = xtask()
            .args(&args)
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}\n{}", diagnostic(&output));
        // The error names the command that works without a terminal.
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(error.contains("cargo xtask"), "{error}");
    }
}

#[test]
fn doctor_json_lists_every_check_with_a_fix_for_each_gap() {
    let output = xtask().args(["doctor", "--json"]).output().unwrap();
    assert!(
        matches!(output.status.code(), Some(0) | Some(3)),
        "{}",
        diagnostic(&output)
    );
    let checks: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
    let required_gap = checks.iter().any(|c| c["required"] == true && c["ok"] == false);
    // Exit 3 exactly when a required check failed.
    assert_eq!(output.status.code() == Some(3), required_gap, "{}", diagnostic(&output));
    for check in &checks {
        if check["ok"] == false {
            assert!(check["fix"].is_string(), "{check}");
        }
    }
}
