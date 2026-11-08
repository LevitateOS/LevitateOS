use anyhow::{Context, Result, bail};
use recinit::live::{BOOT_INJECTION_DIR, BOOT_INJECTION_ENV};
use recqemu::libvirt::{runtime_dir, Domain, DomainSpec, SerialTarget, UefiVars};
use recqemu::serial::Console;
use install_tests::boot_injection::BootInjection;
use install_tests::scenarios::{ScenarioId, resolve_latest_install_runtime};
use crate::cli::{BootArgs, Distro, RunArgs, TestArgs, TestSelection};
use crate::util::exit::usage;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
use std::net::TcpListener;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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

    // A runnable scenario boots its ISO, or else the latest install's disk.
    let Some(product) = args.scenario.input().iso else {
        if args.ssh || args.no_shell {
            return Err(usage(format!(
                "{key} opens the installed system's console; to check it without one use cargo xtask test {key} {}",
                distro.id()
            )));
        }
        if args.boot.inject.is_some() || args.boot.inject_file.is_some() || args.boot.ssh_key.is_some() {
            return Err(usage(format!(
                "{key} takes no boot injection: the live init reads it, the installed system does not",
            )));
        }
        return boot_installed_disk(distro, key);
    };
    let iso = crate::tasks::build::require_iso(distro, product)?;

    // What the live init is told at boot.
    let mut pairs = Vec::new();
    let ssh_key = if args.ssh || args.boot.ssh_key.is_some() {
        let (key, public) = crate::util::ssh::resolve(&root, args.boot.ssh_key.as_deref())?;
        pairs.push(format!("SSH_AUTHORIZED_KEY={public}"));
        Some(key)
    } else {
        None
    };
    if !args.ssh && !args.no_shell {
        // An interactive serial console: the install UX runs on it.
        pairs.push(format!("{}=1", distro_builder::install_ux::SERIAL_KEY));
    }
    let payload = injection_payload(&pairs, &args.boot)?;
    let injection = match &payload {
        Some(file) => Some(BootInjection::from_file(file.path())?),
        None => None,
    };

    match ssh_key.filter(|_| args.ssh) {
        Some(key) => boot_live_iso_ssh(
            distro,
            args.scenario.key(),
            &iso,
            injection,
            args.ssh_port,
            args.ssh_timeout,
            args.no_shell,
            &key,
        ),
        None => boot_live_iso_serial(distro, key, &iso, injection, args.no_shell),
    }
}

/// `pairs`, then `--inject-file`, then `--inject`, as a KEY=VALUE payload file
/// (none when there is nothing to inject).
fn injection_payload(pairs: &[String], boot: &BootArgs) -> Result<Option<tempfile::NamedTempFile>> {
    let mut text = String::new();
    for pair in pairs {
        text.push_str(pair);
        text.push('\n');
    }
    if let Some(path) = &boot.inject_file {
        let extra = fs::read_to_string(path)
            .with_context(|| format!("reading --inject-file {}", path.display()))?;
        text.push_str(extra.trim_end());
        text.push('\n');
    }
    if let Some(raw) = &boot.inject {
        for item in raw.split(',').map(str::trim).filter(|item| !item.is_empty()) {
            text.push_str(item);
            text.push('\n');
        }
    }
    if text.trim().is_empty() {
        return Ok(None);
    }
    let mut file = tempfile::Builder::new()
        .prefix("levitate-boot-injection-")
        .suffix(".env")
        .tempfile()
        .context("creating the boot injection payload")?;
    file.write_all(text.as_bytes())?;
    Ok(Some(file))
}

/// What a scenario needs built or run first (`ScenarioId::input`).
fn require_inputs(distro: &Distro, scenario: ScenarioId) -> Result<()> {
    let input = scenario.input();
    if input.install_run {
        resolve_latest_install_runtime(distro.id()).map_err(|_| {
            crate::util::exit::missing(
                format!("{scenario} needs a disk from the install scenario, and there is none for {}", distro.id()),
                format!("cargo xtask test install {}", distro.id()),
            )
        })?;
    }
    if let Some(product) = input.iso {
        crate::tasks::build::require_iso(distro, product)?;
    }
    Ok(())
}

/// Runs the selected scenarios. Each runs on its own against the artifacts
/// that exist: one failing, or missing its inputs, does not stop the rest.
pub fn test(args: TestArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = &args.distro;
    let scenarios = match args.scenario {
        TestSelection::All => ScenarioId::ALL.to_vec(),
        TestSelection::One(scenario) => {
            require_inputs(distro, scenario)?;
            vec![scenario]
        }
    };
    crate::util::tools_env::apply_to_process(&root)?;
    let (key, public) = crate::util::ssh::resolve(&root, args.boot.ssh_key.as_deref())?;
    let payload = injection_payload(&[format!("SSH_AUTHORIZED_KEY={public}")], &args.boot)?
        .context("the payload always holds the SSH key")?;
    // The scenario runs read the injection and the login key from here.
    // SAFETY: single-threaded command entry point.
    unsafe {
        std::env::set_var(install_tests::boot_injection::ENV_INJECT_FILE, payload.path());
        std::env::set_var("LEVITATE_SSH_PRIVATE_KEY", &key);
    }

    let mut results = Vec::new();
    for scenario in scenarios {
        let result = require_inputs(distro, scenario)
            .and_then(|()| install_tests::scenarios::run_scenario(distro.id(), scenario, args.force));
        if let Err(err) = &result {
            // A scenario's own failure is already reported by the run.
            if crate::util::exit::code_for(err) == 3 {
                eprintln!("[MISSING] {scenario}: {err:#}");
            }
        }
        results.push((scenario, result));
    }

    if args.json {
        let summary: Vec<_> = results
            .iter()
            .map(|(scenario, result)| {
                serde_json::json!({
                    "scenario": scenario.key(),
                    "passed": result.is_ok(),
                    "error": result.as_ref().err().map(|err| format!("{err:#}")),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&summary)?);
    }
    let failed: Vec<&str> = results
        .iter()
        .filter(|(_, result)| result.is_err())
        .map(|(scenario, _)| scenario.key())
        .collect();
    let count = failed.len();
    let names = failed.join(", ");
    match results.into_iter().find_map(|(_, result)| result.err()) {
        None => Ok(()),
        Some(first) if count == 1 => Err(first),
        Some(first) => Err(first.context(format!(
            "{count} scenarios failed for {}: {names} (the first one's error follows)",
            distro.id()
        ))),
    }
}

pub fn status(distro: &Distro, json: bool) -> Result<()> {
    if !json {
        return install_tests::scenarios::print_status(distro.id());
    }
    let scenarios: Vec<_> = install_tests::scenarios::scenario_statuses(distro.id())?
        .into_iter()
        .map(|(scenario, status)| {
            serde_json::json!({ "scenario": scenario.key(), "status": status.label(), "error": status.error() })
        })
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({ "distro": distro.id(), "scenarios": scenarios }))?
    );
    Ok(())
}

pub fn reset(distro: &Distro) -> Result<()> {
    install_tests::scenarios::reset_state(distro.id())
}

fn boot_live_iso_serial(
    distro: &Distro,
    scenario_label: &str,
    iso_path: &Path,
    injection: Option<BootInjection>,
    no_shell: bool,
) -> Result<()> {
    let vm = start_vm(
        distro,
        scenario_label,
        Boot::Iso(iso_path),
        injection.as_ref(),
        None,
        !no_shell,
    )?;
    if no_shell {
        return monitor_live_iso_serial(&vm, &LiveBootPatterns::load(distro.id())?);
    }
    eprintln!("Booting {} {scenario_label}...", distro.name());
    attach_console(vm)
}

/// What the live-boot console output means: the live boot's derived
/// expectations (`install_tests::expect`) plus the serial test protocol's
/// readiness marker.
struct LiveBootPatterns {
    success: Vec<String>,
    fatal: Vec<String>,
}

impl LiveBootPatterns {
    fn load(distro_id: &str) -> Result<Self> {
        let root = crate::util::repo::repo_root()?;
        let live = install_tests::expect::live_boot(&distro_contract::load(&root, distro_id)?);
        let mut success = live.success;
        success.push(recqemu::serial::protocol::SHELL_READY_MARKER.to_string());
        Ok(Self { success, fatal: live.fatal })
    }

    fn first_in<'a>(list: &'a [String], log_path: &Path) -> Option<&'a str> {
        let content = fs::read_to_string(log_path).ok()?;
        list.iter()
            .find(|p| content.contains(p.as_str()))
            .map(String::as_str)
    }
}

fn monitor_live_iso_serial(vm: &RunVm, patterns: &LiveBootPatterns) -> Result<()> {
    let default_timeout = 120u64;
    let timeout_secs = std::env::var("LEVITATE_LIVE_BOOT_SERIAL_TIMEOUT")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(default_timeout);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs.max(1));
    let log_path = &vm.log;

    loop {
        if !vm.domain.is_running() {
            return bail_with_tail("The VM stopped before live boot completed", log_path, None::<&str>);
        }
        if let Some(pat) = detect_boot_regression(log_path, patterns)? {
            return bail_with_tail(
                &format!("Detected boot regression while waiting for live boot: {pat}"),
                log_path,
                None::<&str>,
            );
        }
        if LiveBootPatterns::first_in(&patterns.success, log_path).is_some() {
            return Ok(());
        }
        if Instant::now() > deadline {
            return bail_with_tail(
                &format!("Timed out waiting for live-boot serial handoff ({timeout_secs}s)"),
                log_path,
                Some("No live-boot success pattern observed."),
            );
        }
        sleep(Duration::from_secs(1));
    }
}

fn boot_live_iso_ssh(
    distro: &Distro,
    scenario_label: &str,
    iso_path: &Path,
    injection: Option<BootInjection>,
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    ssh_private_key: &Path,
) -> Result<()> {
    eprintln!(
        "Booting {} {} with SSH wait (port {}:{ssh_port})...",
        distro.name(),
        scenario_label,
        recqemu::HOSTFWD_ADDRESS
    );
    ensure_ssh_port_available(ssh_port)?;

    let mut vm = start_vm(
        distro,
        scenario_label,
        Boot::Iso(iso_path),
        injection.as_ref(),
        Some(ssh_port),
        false,
    )?;
    let log_path = vm.log.clone();
    let result = monitor_live_iso_ssh(
        &mut vm,
        &log_path,
        &LiveBootPatterns::load(distro.id())?,
        ssh_port,
        ssh_timeout,
        no_shell,
        ssh_private_key,
    );
    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            let report = maybe_append_log_fault(&log_path);
            if let Some(report) = report {
                bail!("{err}\n{report}");
            }
            bail!("{:#}", err);
        }
    }
}

fn monitor_live_iso_ssh(
    vm: &mut RunVm,
    log_path: &Path,
    patterns: &LiveBootPatterns,
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    key: &Path,
) -> Result<()> {
    let known_hosts = temp_file_path("levitate-live-boot-ssh-known-hosts");
    fs::write(&known_hosts, "").context("creating known-hosts scratch file")?;
    let deadline = Instant::now() + Duration::from_secs(ssh_timeout.max(1));
    let mut hook_seen = false;
    let mut lines_seen = 0usize;

    loop {
        let _ = emit_new_log_lines(log_path, &mut lines_seen)?;

        if !vm.domain.is_running() {
            let _ = fs::remove_file(&known_hosts);
            return bail_with_tail("The VM stopped before SSH became ready", log_path, None::<&str>);
        }

        if let Some(pat) = detect_boot_regression(log_path, patterns)? {
            let _ = fs::remove_file(&known_hosts);
            return bail_with_tail(
                &format!("Detected boot regression while waiting for SSH: {pat}"),
                log_path,
                None::<&str>,
            );
        }

        if !hook_seen {
            if let Some(pat) = LiveBootPatterns::first_in(&patterns.success, log_path) {
                hook_seen = true;
                eprintln!(
                    "Boot hook observed ({pat}); waiting for SSH readiness on {}:{ssh_port}...",
                    recqemu::HOSTFWD_ADDRESS
                );
            }
        } else if can_ssh_connect(ssh_port, &key, &known_hosts)? {
            if no_shell {
                let _ = fs::remove_file(&known_hosts);
                return Ok(());
            }

            let status = run_interactive_ssh(ssh_port, &key, &known_hosts);
            let _ = fs::remove_file(&known_hosts);
            return status;
        }

        if Instant::now() > deadline {
            if hook_seen {
                let _ = collect_guest_ssh_debug(vm);
            }
            let _ = fs::remove_file(&known_hosts);
            let mut extra = format!("No successful SSH handshake observed.");
            if !hook_seen {
                extra = format!("No boot hook observed yet after {ssh_timeout}s.");
            }
            return bail_with_tail(
                &format!("Timed out waiting for SSH readiness ({ssh_timeout}s)"),
                log_path,
                Some(&extra),
            );
        }
        sleep(Duration::from_secs(1));
    }
}

fn emit_new_log_lines(log_path: &Path, line_cursor: &mut usize) -> Result<()> {
    let mut lines = match fs::read_to_string(log_path) {
        Ok(raw) => raw.lines().map(str::to_string).collect::<Vec<_>>(),
        Err(_) => return Ok(()),
    };

    let total_lines = lines.len();
    if total_lines <= *line_cursor {
        return Ok(());
    }

    for line in lines.drain(..*line_cursor) {
        let _ = line;
    }
    for line in lines {
        println!("{}", recqemu::serial::strip_ansi_codes(&line));
    }
    *line_cursor = total_lines;
    Ok(())
}

fn run_interactive_ssh(ssh_port: u16, private_key: &Path, known_hosts: &Path) -> Result<()> {
    // Some interactive shells emit cursor-position queries (CSI 6n). If a reply
    // races with session teardown, bytes can leak into the next shell prompt.
    flush_tty_input_queue();

    let mut args = common_ssh_args(private_key, ssh_port, known_hosts);
    args.push("-tt".to_string());
    args.push("-o".to_string());
    args.push("BatchMode=no".to_string());
    args.push(format!("root@{}", recqemu::HOSTFWD_ADDRESS));
    let status = Command::new("ssh")
        .env("TERM", "vt100")
        .args(&args)
        .status()
        .context("launching interactive SSH session")?;

    flush_tty_input_queue();

    if status.success() {
        Ok(())
    } else {
        bail!("interactive SSH session exited with status {status}")
    }
}

fn flush_tty_input_queue() {
    let tty = match OpenOptions::new().read(true).open("/dev/tty") {
        Ok(file) => file,
        Err(_) => return,
    };
    let fd = tty.as_raw_fd();
    // SAFETY: tcflush only uses the provided valid file descriptor.
    let _ = unsafe { libc::tcflush(fd, libc::TCIFLUSH) };
}

fn can_ssh_connect(ssh_port: u16, private_key: &Path, known_hosts: &Path) -> Result<bool> {
    let mut args = common_ssh_args(private_key, ssh_port, known_hosts);
    args.push("-n".to_string());
    args.push("-o".to_string());
    args.push("BatchMode=yes".to_string());
    args.push(format!("root@{}", recqemu::HOSTFWD_ADDRESS));
    args.push("true".to_string());
    let status = Command::new("ssh")
        .args(&args)
        .status()
        .context("checking SSH readiness")?;
    Ok(status.success())
}

fn common_ssh_args(key: &Path, ssh_port: u16, known_hosts: &Path) -> Vec<String> {
    vec![
        "-o".to_string(),
        "ConnectTimeout=10".to_string(),
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-o".to_string(),
        format!("UserKnownHostsFile={}", known_hosts.display()),
        "-o".to_string(),
        "IdentitiesOnly=yes".to_string(),
        "-i".to_string(),
        key.display().to_string(),
        "-p".to_string(),
        ssh_port.to_string(),
    ]
}

fn boot_installed_disk(distro: &Distro, scenario_label: &str) -> Result<()> {
    let runtime = resolve_latest_install_runtime(distro.id())?;
    eprintln!(
        "Booting installed {} from install scenario run {}...",
        distro.name(),
        runtime.run_id
    );
    eprintln!("  disk: {}", runtime.disk_path.display());
    eprintln!("  ovmf vars: {}", runtime.ovmf_vars_path.display());
    let vm = start_vm(
        distro,
        scenario_label,
        Boot::Disk {
            disk: &runtime.disk_path,
            vars: &runtime.ovmf_vars_path,
        },
        None,
        None,
        true,
    )?;
    attach_console(vm)
}

/// What a run boots.
enum Boot<'a> {
    /// A live ISO, with a fresh UEFI variable store.
    Iso(&'a Path),
    /// An installed disk and its UEFI variable store.
    Disk { disk: &'a Path, vars: &'a Path },
}

/// A run's VM: its libvirt domain, the serial transcript, and the console
/// when this process reads the serial port (`--ssh`, `--no-shell`).
struct RunVm {
    domain: Domain,
    log: PathBuf,
    console: Option<Console>,
}

/// Start the run's VM as a libvirt domain named `<distro>-run-<scenario>`,
/// visible in Cockpit. `interactive` gives it a pty serial port for
/// `virsh console`; otherwise this process reads the serial port.
fn start_vm(
    distro: &Distro,
    scenario: &str,
    boot: Boot<'_>,
    injection: Option<&BootInjection>,
    ssh_port: Option<u16>,
    interactive: bool,
) -> Result<RunVm> {
    let dir = runtime_dir();
    let name = format!("{}-run-{scenario}", distro.id());
    let socket = dir.join(format!("{name}.serial"));
    let log = dir.join(format!("{name}.serial.log"));
    let _ = fs::remove_file(&log);
    let absolute = |path: &Path| fs::canonicalize(path).with_context(|| format!("resolving '{}'", path.display()));
    let (boot_iso, disk, uefi_vars) = match boot {
        Boot::Iso(iso) => (
            Some(absolute(iso)?),
            None,
            UefiVars::Fresh {
                template: recqemu::find_ovmf_vars()?,
                path: dir.join(format!("{name}.VARS.fd")),
            },
        ),
        Boot::Disk { disk, vars } => (None, Some(absolute(disk)?), UefiVars::Existing(absolute(vars)?)),
    };
    let mut spec = DomainSpec {
        name: name.clone(),
        title: format!("{} {scenario} (cargo xtask run)", distro.name()),
        // The host's OVMF, matching the variable stores and libvirt's QEMU.
        uefi_code: recqemu::find_ovmf()?,
        uefi_vars,
        disk,
        boot_iso,
        extra_cdroms: Vec::new(),
        fw_cfg: Vec::new(),
        ssh_forward: ssh_port,
        serial: if interactive { SerialTarget::Pty } else { SerialTarget::Socket(socket.clone()) },
        serial_log: log.clone(),
    };
    if let Some(injection) = injection {
        injection.attach(&mut spec);
    }
    let domain = Domain::create_paused(&spec)?;
    let console = if interactive {
        None
    } else {
        Some(Console::connect(&socket, Duration::from_secs(20))?)
    };
    domain.resume()?;
    eprintln!(
        "VM '{name}' runs on {}: watch it in Cockpit (Virtual Machines) or `virsh -c {} console {name}`.",
        domain.uri(),
        domain.uri()
    );
    Ok(RunVm { domain, log, console })
}

/// Hand the terminal to the VM's serial console (Ctrl+] leaves it). The VM
/// keeps running after you leave, for Cockpit; it stops when the guest powers off.
fn attach_console(vm: RunVm) -> Result<()> {
    eprintln!("Attaching the serial console (Ctrl+] to leave)...");
    let status = Command::new("virsh")
        .args(["-c", vm.domain.uri(), "console", "--force", vm.domain.name()])
        .status()
        .context("running virsh console")?;
    flush_tty_input_queue();
    if vm.domain.is_running() {
        eprintln!(
            "Left the console; '{}' keeps running. Stop it in Cockpit or with: virsh -c {} destroy {}",
            vm.domain.name(),
            vm.domain.uri(),
            vm.domain.name()
        );
        vm.domain.keep_running();
    }
    if !status.success() {
        bail!("virsh console exited with {status}");
    }
    Ok(())
}

/// Type a probe of the guest's network and sshd state into its serial
/// console; the output lands in the serial transcript.
fn collect_guest_ssh_debug(vm: &mut RunVm) -> Result<()> {
    let Some(console) = vm.console.as_mut() else {
        return Ok(());
    };
    let probe = format!(
        "\n\
         echo ___SSH_DEBUG_BEGIN___\n\
         ip -brief addr 2>/dev/null || ip addr 2>/dev/null || true\n\
         ip route 2>/dev/null || route -n 2>/dev/null || true\n\
         rc-service networking status 2>/dev/null || true\n\
         rc-service dhcpcd status 2>/dev/null || true\n\
         ss -ltnp | grep ':22' || true\n\
         ls -l /root/.ssh /root/.ssh/authorized_keys {BOOT_INJECTION_DIR} {BOOT_INJECTION_DIR}/* 2>/dev/null || true\n\
         cat {BOOT_INJECTION_DIR}/source {BOOT_INJECTION_ENV} 2>/dev/null || true\n\
         journalctl -b -u sshd.service --no-pager -n 120 || true\n\
         echo ___SSH_DEBUG_END___\n"
    );
    console.send(&probe).context("typing the SSH debug probe into the guest")?;
    sleep(Duration::from_secs(2));
    Ok(())
}

/// A fatal live-boot pattern.
fn detect_boot_regression(log_path: &Path, patterns: &LiveBootPatterns) -> Result<Option<String>> {
    Ok(LiveBootPatterns::first_in(&patterns.fatal, log_path).map(str::to_string))
}

fn ensure_ssh_port_available(ssh_port: u16) -> Result<()> {
    match TcpListener::bind((recqemu::HOSTFWD_ADDRESS, ssh_port)) {
        Ok(listener) => {
            drop(listener);
            Ok(())
        }
        Err(err) => bail!(
            "local SSH host port {ssh_port} is unavailable (bind error: {err}). \
            Use `--ssh-port` to choose a free port."
        ),
    }
}

fn temp_file_path(prefix: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur| dur.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("{prefix}-{ts}"))
}

fn dump_log_tail(log_path: &Path, lines: usize) -> String {
    match fs::read_to_string(log_path) {
        Ok(raw) => {
            let mut output = Vec::new();
            for line in raw
                .lines()
                .rev()
                .take(lines)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
            {
                output.push(line);
            }
            output.join("\n")
        }
        Err(_) => String::new(),
    }
}

fn maybe_append_log_fault(log_path: &Path) -> Option<String> {
    let tail = dump_log_tail(log_path, 120);
    if tail.is_empty() {
        None
    } else {
        Some(format!("Last log lines:\n{tail}"))
    }
}

fn bail_with_tail(message: &str, log_path: &Path, extra: Option<&str>) -> Result<()> {
    let tail = dump_log_tail(log_path, 120);
    let detail = if tail.is_empty() {
        String::new()
    } else {
        format!("\nLast log lines:\n{tail}")
    };
    let tail_extra = extra.unwrap_or("");
    if !tail_extra.is_empty() {
        bail!("{message}\n{tail_extra}{detail}");
    }
    bail!("{message}{detail}");
}

