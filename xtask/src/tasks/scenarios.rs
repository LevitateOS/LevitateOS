use anyhow::{Context, Result, bail};
use distro_spec::shared::live_boot::{BOOT_INJECTION_DIR, BOOT_INJECTION_ENV};
use install_tests::boot_injection::BootInjection;
use install_tests::scenarios::{ScenarioId, resolve_latest_install_runtime};
use crate::cli::{BootArgs, Distro, Product, RunArgs, RunScenario, TestArgs, TestScenario};
use crate::util::exit::usage;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, TcpListener, UdpSocket};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindowMode {
    RemoteVnc,
    LocalGui,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalDisplayBackend {
    Gtk,
    Sdl,
}

impl LocalDisplayBackend {
    fn qemu_arg(self) -> &'static str {
        match self {
            Self::Gtk => "gtk",
            Self::Sdl => "sdl",
        }
    }
}

#[derive(Clone, Debug)]
struct WindowConfig {
    mode: WindowMode,
    local_qemu_bin: Option<PathBuf>,
    local_display_backend: Option<LocalDisplayBackend>,
    vnc_bind_host: Option<Ipv4Addr>,
    vnc_host: Option<Ipv4Addr>,
    vnc_port: Option<u16>,
    serial_log: PathBuf,
}

impl WindowConfig {
    fn allocate(mode: WindowMode) -> Result<Self> {
        let (local_qemu_bin, local_display_backend, vnc_bind_host, vnc_host, vnc_port) = match mode
        {
            WindowMode::RemoteVnc => {
                let vnc_bind_host = detect_vnc_bind_host()?;
                let vnc_host = detect_vnc_advertise_host()?;
                let vnc_port = allocate_local_port(vnc_bind_host, 5900, 5999)?;
                (
                    None,
                    None,
                    Some(vnc_bind_host),
                    Some(vnc_host),
                    Some(vnc_port),
                )
            }
            WindowMode::LocalGui => {
                let (qemu_bin, display_backend) = detect_local_qemu_runtime()?;
                (Some(qemu_bin), Some(display_backend), None, None, None)
            }
        };
        let serial_log = temp_file_path("levitate-scenario-window-serial").with_extension("log");
        Ok(Self {
            mode,
            local_qemu_bin,
            local_display_backend,
            vnc_bind_host,
            vnc_host,
            vnc_port,
            serial_log,
        })
    }

    fn vnc_endpoint(&self) -> Option<String> {
        let (host, port) = match (self.vnc_host, self.vnc_port) {
            (Some(host), Some(port)) => (host, port),
            _ => return None,
        };
        Some(format!("vnc://{}:{}", host, port))
    }

    fn qemu_display_arg(&self) -> Option<String> {
        let (bind_host, vnc_port) = match (self.vnc_bind_host, self.vnc_port) {
            (Some(bind_host), Some(vnc_port)) => (bind_host, vnc_port),
            _ => return None,
        };
        let display_index = vnc_port.saturating_sub(5900);
        Some(format!(
            "vnc={}:{display_index},to=99,id=scenario-window",
            bind_host
        ))
    }
}

pub fn run(args: RunArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = args.distro;
    let key = match args.scenario {
        RunScenario::LiveBoot => "live-boot",
        RunScenario::LiveTools => "live-tools",
        RunScenario::InstalledBoot => "installed-boot",
    };
    if !args.no_shell {
        crate::util::exit::require_terminal(
            "`cargo xtask run` (it hands you the guest's console)",
            &format!(
                "cargo xtask run {key} {0} --no-shell to boot and check it, or cargo xtask test {key} {0}",
                distro.id()
            ),
        )?;
    }
    let window = if args.window {
        Some(WindowConfig::allocate(WindowMode::LocalGui)?)
    } else if args.vnc {
        Some(WindowConfig::allocate(WindowMode::RemoteVnc)?)
    } else {
        None
    };

    let product = match args.scenario {
        RunScenario::LiveBoot => Product::LiveBoot,
        RunScenario::LiveTools => Product::LiveTools,
        RunScenario::InstalledBoot => {
            if args.ssh || args.no_shell {
                return Err(usage(format!(
                    "installed-boot opens the installed system's console; to check it without one use cargo xtask test installed-boot {}",
                    distro.id()
                )));
            }
            if args.boot.inject.is_some() || args.boot.inject_file.is_some() || args.boot.ssh_key.is_some() {
                return Err(usage(
                    "installed-boot takes no boot injection: the live init reads it, the installed system does not",
                ));
            }
            return boot_installed_disk(&root, distro, window.as_ref());
        }
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
    if !args.ssh && !args.no_shell && window.is_none() {
        // An interactive serial console: the install UX runs on it.
        pairs.push("LEVITATE_INSTALL_SERIAL_UX=1".to_string());
        if args.verbose {
            pairs.push("LEVITATE_INSTALL_SERIAL_VERBOSE=1".to_string());
        }
    }
    let payload = injection_payload(&pairs, &args.boot)?;
    let injection = match &payload {
        Some(file) => Some(BootInjection::from_file(file.path())?),
        None => None,
    };

    match ssh_key.filter(|_| args.ssh) {
        Some(key) => boot_live_iso_ssh(
            &root,
            distro,
            product.id(),
            &iso,
            injection,
            args.ssh_port,
            args.ssh_timeout,
            args.no_shell,
            &key,
        ),
        None => boot_live_iso_serial(&root, distro, &iso, injection, args.no_shell, window.as_ref()),
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

/// What a scenario needs built or run first.
fn require_inputs(distro: Distro, scenario: ScenarioId) -> Result<()> {
    let product = match scenario {
        ScenarioId::BuildPreflight => Product::BaseRootfs,
        ScenarioId::LiveBoot => Product::LiveBoot,
        ScenarioId::LiveTools | ScenarioId::Install => Product::LiveTools,
        ScenarioId::InstalledBoot | ScenarioId::AutomatedLogin | ScenarioId::Runtime => {
            return resolve_latest_install_runtime(distro.id()).map(|_| ()).map_err(|_| {
                crate::util::exit::missing(
                    format!("{} needs a disk from the install scenario, and there is none for {}", scenario.key(), distro.id()),
                    format!("cargo xtask test install {}", distro.id()),
                )
            });
        }
    };
    crate::tasks::build::require_iso(distro, product).map(|_| ())
}

fn scenarios_for(scenario: TestScenario) -> Vec<ScenarioId> {
    match scenario {
        TestScenario::All => vec![
            ScenarioId::LiveBoot,
            ScenarioId::LiveTools,
            ScenarioId::Install,
            ScenarioId::InstalledBoot,
            ScenarioId::AutomatedLogin,
            ScenarioId::Runtime,
        ],
        TestScenario::BuildPreflight => vec![ScenarioId::BuildPreflight],
        TestScenario::LiveBoot => vec![ScenarioId::LiveBoot],
        TestScenario::LiveTools => vec![ScenarioId::LiveTools],
        TestScenario::Install => vec![ScenarioId::Install],
        TestScenario::InstalledBoot => vec![ScenarioId::InstalledBoot],
        TestScenario::AutomatedLogin => vec![ScenarioId::AutomatedLogin],
        TestScenario::Runtime => vec![ScenarioId::Runtime],
    }
}

pub fn test(args: TestArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = args.distro;
    let scenarios = scenarios_for(args.scenario);
    require_inputs(distro, scenarios[0])?;
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
        require_inputs(distro, scenario)?;
        let passed = if args.force {
            install_tests::scenarios::run_scenario_forced(distro.id(), scenario)?
        } else {
            install_tests::scenarios::run_scenario(distro.id(), scenario)?
        };
        results.push((scenario, passed));
        if !passed {
            break;
        }
    }

    if args.json {
        let summary: Vec<_> = results
            .iter()
            .map(|(scenario, passed)| serde_json::json!({ "scenario": scenario.key(), "passed": passed }))
            .collect();
        println!("{}", serde_json::to_string_pretty(&summary)?);
    }
    if let Some((scenario, _)) = results.iter().find(|(_, passed)| !passed) {
        bail!("scenario {} failed for {}", scenario.key(), distro.id());
    }
    Ok(())
}

pub fn status(distro: Distro, json: bool) -> Result<()> {
    if !json {
        return install_tests::scenarios::print_status(distro.id());
    }
    let scenarios: Vec<_> = install_tests::scenarios::scenario_statuses(distro.id())?
        .into_iter()
        .map(|(scenario, status)| serde_json::json!({ "scenario": scenario.key(), "status": status }))
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({ "distro": distro.id(), "scenarios": scenarios }))?
    );
    Ok(())
}

pub fn reset(distro: Distro) -> Result<()> {
    install_tests::scenarios::reset_state(distro.id())
}

fn boot_live_iso_serial(
    root: &Path,
    distro: crate::cli::Distro,
    iso_path: &Path,
    injection: Option<BootInjection>,
    no_shell: bool,
    window: Option<&WindowConfig>,
) -> Result<()> {
    if no_shell {
        let log_path = temp_log_path("levitate-live-boot-serial");
        let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), None, window)?;
        let child = spawn_qemu_with_log(&mut cmd, &log_path, false)?;
        monitor_live_iso_serial(child, &log_path, &LiveBootPatterns::load(distro.id())?)?;
        let _ = fs::remove_file(&log_path);
        return Ok(());
    }

    if window.is_some() {
        eprintln!(
            "Booting {} live ISO in window mode... (Ctrl-C to stop)",
            distro.pretty_name()
        );
    } else {
        eprintln!("Booting {} live ISO... (Ctrl-A X to exit)", distro.pretty_name());
    }
    let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), None, window)?;
    if let Some(window_cfg) = window {
        run_window_mode_foreground(&mut cmd, window_cfg)
    } else {
        run_checked(&mut cmd)
    }
}

/// What the live-boot console output means, from the variant's scenario
/// contract (plus the serial test protocol's marker and the live init's
/// rootfs-missing message).
struct LiveBootPatterns {
    success: Vec<String>,
    fatal: Vec<String>,
}

impl LiveBootPatterns {
    fn load(distro_id: &str) -> Result<Self> {
        let live = install_tests::distro::load_scenarios(distro_id)?.live_boot;
        let fatal = install_tests::qemu::serial::live_fatal(&live);
        let mut success = live.success_patterns;
        success.push(recqemu::serial::protocol::SHELL_READY_MARKER.to_string());
        Ok(Self { success, fatal })
    }

    fn first_in<'a>(list: &'a [String], log_path: &Path) -> Option<&'a str> {
        let content = fs::read_to_string(log_path).ok()?;
        list.iter()
            .find(|p| content.contains(p.as_str()))
            .map(String::as_str)
    }
}

fn monitor_live_iso_serial(mut child: Child, log_path: &Path, patterns: &LiveBootPatterns) -> Result<()> {
    let default_timeout = 120u64;
    let timeout_secs = std::env::var("LEVITATE_LIVE_BOOT_SERIAL_TIMEOUT")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(default_timeout);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs.max(1));

    loop {
        if let Some(exit_status) = child.try_wait()? {
            let reason = match exit_status.code() {
                Some(code) => format!("QEMU exited with code {code}"),
                None => "QEMU exited by signal".to_string(),
            };
            return bail_with_tail(
                &format!("{reason} before live boot completed"),
                log_path,
                None::<&str>,
            );
        }

        if let Some(pat) = detect_boot_regression(log_path, patterns)? {
            let _ = child.kill();
            let _ = child.wait();
            return bail_with_tail(
                &format!("Detected boot regression while waiting for live boot: {pat}"),
                log_path,
                None::<&str>,
            );
        }

        if LiveBootPatterns::first_in(&patterns.success, log_path).is_some() {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(());
        }

        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
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
    root: &Path,
    distro: crate::cli::Distro,
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
        distro.pretty_name(),
        scenario_label,
        recqemu::HOSTFWD_ADDRESS
    );
    ensure_ssh_port_available(ssh_port)?;

    let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), Some(ssh_port), None)?;
    let log_path = temp_log_path("levitate-live-boot-ssh");
    let child = spawn_qemu_with_log(&mut cmd, &log_path, true)?;
    let result = monitor_live_iso_ssh(
        child,
        &log_path,
        &LiveBootPatterns::load(distro.id())?,
        ssh_port,
        ssh_timeout,
        no_shell,
        ssh_private_key,
    );
    let result = match result {
        Ok(()) => Ok(()),
        Err(err) => {
            let report = maybe_append_log_fault(&log_path);
            if let Some(report) = report {
                bail!("{err}\n{report}");
            }
            bail!("{:#}", err);
        }
    };
    let _ = fs::remove_file(&log_path);
    result
}

fn monitor_live_iso_ssh(
    mut child: Child,
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

        if let Some(exit_status) = child.try_wait()? {
            let reason = match exit_status.code() {
                Some(code) => format!("QEMU exited with code {code}"),
                None => "QEMU exited by signal".to_string(),
            };
            let _ = fs::remove_file(&known_hosts);
            return bail_with_tail(
                &format!("{reason} before SSH became ready"),
                log_path,
                None::<&str>,
            );
        }

        if let Some(pat) = detect_boot_regression(log_path, patterns)? {
            let _ = child.kill();
            let _ = child.wait();
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
                let _ = child.kill();
                let _ = child.wait();
                let _ = fs::remove_file(&known_hosts);
                return Ok(());
            }

            let status = run_interactive_ssh(ssh_port, &key, &known_hosts, &mut child);
            let _ = fs::remove_file(&known_hosts);
            return status;
        }

        if Instant::now() > deadline {
            if hook_seen {
                let _ = collect_guest_ssh_debug(&mut child);
            }
            let _ = child.kill();
            let _ = child.wait();
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

fn run_interactive_ssh(
    ssh_port: u16,
    private_key: &Path,
    known_hosts: &Path,
    qemu: &mut Child,
) -> Result<()> {
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

    let _ = qemu.kill();
    let _ = qemu.wait();
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

fn boot_installed_disk(
    root: &Path,
    distro: crate::cli::Distro,
    window: Option<&WindowConfig>,
) -> Result<()> {
    let runtime = resolve_latest_install_runtime(distro.id())?;
    let disk = runtime.disk_path;
    let vars = runtime.ovmf_vars_path;
    let disk_format = recqemu::disk_image_format(&disk);
    let ovmf = crate::util::repo::ovmf_path(root)?;

    let mut cmd = qemu_builder_for_window(window)?
        .disk(&disk)
        .uefi(ovmf)
        .uefi_vars(&vars)
        .boot_order("c")
        .user_network()
        .build();
    if window.is_some() {
        eprintln!(
            "Booting installed {} from install scenario run {} in window mode... (Ctrl-C to stop)",
            distro.pretty_name(), runtime.run_id
        );
    } else {
        eprintln!(
            "Booting installed {} from install scenario run {}... (Ctrl-A X to exit)",
            distro.pretty_name(), runtime.run_id
        );
    }
    eprintln!("  disk: {}", disk.display());
    eprintln!("  disk format: {}", disk_format);
    eprintln!("  ovmf vars: {}", vars.display());

    apply_qemu_runtime_env(&mut cmd, root, window)?;
    if let Some(window_cfg) = window {
        run_window_mode_foreground(&mut cmd, window_cfg)
    } else {
        run_checked(&mut cmd)
    }
}

fn qemu_base_command(
    root: &Path,
    iso_path: &Path,
    injection: Option<&BootInjection>,
    ssh_port: Option<u16>,
    window: Option<&WindowConfig>,
) -> Result<Command> {
    let mut builder = qemu_builder_for_window(window)?
        .cdrom(iso_path)
        .cdrom_as_disk()
        .uefi(crate::util::repo::ovmf_path(root)?);
    if let Some(injection) = injection {
        builder = injection.attach(builder);
    }
    builder = match ssh_port {
        Some(port) => builder.user_network_with_hostfwd(port, recqemu::GUEST_SSH_PORT),
        None => builder.user_network(),
    };
    let mut cmd = builder.build();
    apply_qemu_runtime_env(&mut cmd, root, window)?;
    Ok(cmd)
}

/// The QEMU console for a boot: serial on stdio, or a window (VNC or local
/// GUI) with serial logged to a file.
fn qemu_builder_for_window(window: Option<&WindowConfig>) -> Result<recqemu::QemuBuilder> {
    let builder = recqemu::QemuBuilder::new().no_reboot();
    let Some(window_cfg) = window else {
        return Ok(builder.nographic().serial_stdio());
    };
    let builder = builder.serial_file(&window_cfg.serial_log).no_monitor();
    Ok(match window_cfg.mode {
        WindowMode::RemoteVnc => {
            let display_arg = window_cfg
                .qemu_display_arg()
                .context("remote window mode has no VNC display configuration")?;
            builder
                .vga("none")
                .device("secondary-vga")
                .display(&display_arg)
        }
        WindowMode::LocalGui => {
            let display_backend = window_cfg
                .local_display_backend
                .context("local window mode has no local display backend")?;
            let qemu_bin = window_cfg
                .local_qemu_bin
                .as_ref()
                .context("local window mode has no detected qemu binary")?;
            builder
                .program(qemu_bin)
                .display(display_backend.qemu_arg())
                .vga("virtio")
        }
    })
}

fn run_window_mode_foreground(cmd: &mut Command, window_cfg: &WindowConfig) -> Result<()> {
    let mut child = cmd
        .spawn()
        .context("Spawning QEMU for foreground window mode")?;
    print_window_mode_details(window_cfg, child.id());
    let status = child
        .wait()
        .context("Waiting for QEMU foreground window mode")?;
    if !status.success() {
        bail!("Command failed with status {status}");
    }
    Ok(())
}

fn print_window_mode_details(window_cfg: &WindowConfig, pid: u32) {
    eprintln!("Window mode: PID {pid}");
    eprintln!(
        "Window mode: serial log {}",
        window_cfg.serial_log.display()
    );
    match window_cfg.mode {
        WindowMode::LocalGui => {
            let qemu_bin = window_cfg
                .local_qemu_bin
                .as_ref()
                .expect("internal error: missing local qemu binary in local mode");
            let display_backend = window_cfg
                .local_display_backend
                .expect("internal error: missing local display backend in local mode");
            eprintln!(
                "Window mode: local QEMU window requested via '{}' with display backend '{}'.",
                qemu_bin.display(),
                display_backend.qemu_arg()
            );
        }
        WindowMode::RemoteVnc => {
            let bind_host = window_cfg
                .vnc_bind_host
                .expect("internal error: missing VNC bind host in remote mode");
            let vnc_port = window_cfg
                .vnc_port
                .expect("internal error: missing VNC port in remote mode");
            let endpoint = window_cfg
                .vnc_endpoint()
                .expect("internal error: missing VNC endpoint in remote mode");
            eprintln!("Window mode: VNC bind {}:{}", bind_host, vnc_port);
            eprintln!("Window mode: VNC endpoint {endpoint}");
            eprintln!(
                "Window mode: SSH tunnel (run on your local machine):\n  ssh -N -L {port}:127.0.0.1:{port} <user>@<remote-host>",
                port = vnc_port
            );
            eprintln!("Window mode: open viewer locally (direct):\n  remote-viewer {endpoint}");
            eprintln!(
                "Window mode: open viewer locally (via tunnel):\n  remote-viewer vnc://127.0.0.1:{}",
                vnc_port
            );
        }
    }
}

fn apply_qemu_runtime_env(
    cmd: &mut Command,
    root: &Path,
    window: Option<&WindowConfig>,
) -> Result<()> {
    if matches!(window.map(|distro| distro.mode), Some(WindowMode::LocalGui)) {
        // Local GUI mode intentionally uses the detected system QEMU binary.
        // Avoid forcing tools PATH/LD_LIBRARY_PATH, which can shadow host GUI-enabled builds.
        cmd.env_remove("LD_LIBRARY_PATH");
        return Ok(());
    }
    crate::util::tools_env::apply_to_command(cmd, root)
}

fn detect_local_qemu_runtime() -> Result<(PathBuf, LocalDisplayBackend)> {
    if let Ok(raw) = std::env::var("LEVITATE_SCENARIO_WINDOW_QEMU_BIN") {
        let candidate = PathBuf::from(raw.trim());
        return pick_local_display_backend(&candidate).map(|backend| (candidate, backend));
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(path_os) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_os) {
            let candidate = dir.join("qemu-system-x86_64");
            if !candidate.is_file() {
                continue;
            }
            let path_str = candidate.to_string_lossy();
            if path_str.contains("/.artifacts/tools/.tools/") {
                continue;
            }
            if !candidates.iter().any(|existing| existing == &candidate) {
                candidates.push(candidate);
            }
        }
    }
    for candidate in [
        PathBuf::from("/usr/bin/qemu-system-x86_64"),
        PathBuf::from("/usr/local/bin/qemu-system-x86_64"),
    ] {
        if candidate.is_file() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    }

    for candidate in candidates {
        if let Ok(backend) = pick_local_display_backend(&candidate) {
            return Ok((candidate, backend));
        }
    }

    bail!(
        "Local window mode requires a system qemu-system-x86_64 with either gtk or sdl display backend.\n\
         The bundled tools QEMU is headless-only.\n\
         Remediation: install host QEMU GUI support (for example `sudo dnf install qemu-system-x86 qemu-ui-gtk`) and rerun,\n\
         or rerun without `--window` and use the serial/SSH workflow."
    )
}

fn pick_local_display_backend(qemu_bin: &Path) -> Result<LocalDisplayBackend> {
    if !qemu_bin.is_file() {
        bail!("qemu binary not found at '{}'", qemu_bin.display());
    }
    let output = Command::new(qemu_bin)
        .env_remove("LD_LIBRARY_PATH")
        .args(["-display", "help"])
        .output()
        .with_context(|| {
            format!(
                "querying display backends for local qemu binary '{}'",
                qemu_bin.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "qemu binary '{}' failed display backend probe",
            qemu_bin.display()
        );
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let has_gtk = text.lines().any(|line| line.trim() == "gtk");
    if has_gtk {
        return Ok(LocalDisplayBackend::Gtk);
    }
    let has_sdl = text.lines().any(|line| line.trim() == "sdl");
    if has_sdl {
        return Ok(LocalDisplayBackend::Sdl);
    }
    bail!(
        "qemu binary '{}' does not expose gtk/sdl display backends",
        qemu_bin.display()
    )
}

fn detect_vnc_bind_host() -> Result<Ipv4Addr> {
    if let Ok(raw) = std::env::var("LEVITATE_SCENARIO_WINDOW_BIND_HOST") {
        let parsed: Ipv4Addr = raw.parse().with_context(|| {
            format!("Parsing LEVITATE_SCENARIO_WINDOW_BIND_HOST as IPv4 failed: {raw}")
        })?;
        return Ok(parsed);
    }
    Ok(Ipv4Addr::UNSPECIFIED)
}

fn detect_vnc_advertise_host() -> Result<Ipv4Addr> {
    if let Ok(raw) = std::env::var("LEVITATE_SCENARIO_WINDOW_HOST") {
        let parsed: Ipv4Addr = raw.parse().with_context(|| {
            format!("Parsing LEVITATE_SCENARIO_WINDOW_HOST as IPv4 address failed: {raw}")
        })?;
        if parsed.is_unspecified() {
            bail!("LEVITATE_SCENARIO_WINDOW_HOST must not be unspecified; got {parsed}");
        }
        return Ok(parsed);
    }
    if let Some(ip) = detect_ssh_server_ipv4() {
        return Ok(ip);
    }
    if let Some(ip) = detect_default_route_ipv4() {
        return Ok(ip);
    }
    Ok(Ipv4Addr::LOCALHOST)
}

fn detect_default_route_ipv4() -> Option<Ipv4Addr> {
    // UDP connect does not send packets; it asks kernel routing for the outbound interface address.
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    socket.connect((Ipv4Addr::new(1, 1, 1, 1), 80)).ok()?;
    let local_addr = socket.local_addr().ok()?;
    match local_addr.ip() {
        IpAddr::V4(ip) if !ip.is_loopback() && !ip.is_unspecified() => Some(ip),
        _ => None,
    }
}

fn detect_ssh_server_ipv4() -> Option<Ipv4Addr> {
    let raw = std::env::var("SSH_CONNECTION").ok()?;
    let mut parts = raw.split_whitespace();
    let _client_ip = parts.next()?;
    let _client_port = parts.next()?;
    let server_ip = parts.next()?;
    let parsed: Ipv4Addr = server_ip.parse().ok()?;
    if parsed.is_unspecified() {
        return None;
    }
    Some(parsed)
}

fn allocate_local_port(host: Ipv4Addr, start: u16, end: u16) -> Result<u16> {
    for port in start..=end {
        if let Ok(listener) = TcpListener::bind((host, port)) {
            drop(listener);
            return Ok(port);
        }
    }
    bail!("No free local TCP port on {host} in range {start}..={end}")
}

fn spawn_qemu_with_log(cmd: &mut Command, log_path: &Path, allow_stdin: bool) -> Result<Child> {
    let log_out = File::create(log_path)
        .with_context(|| format!("creating QEMU log file '{}'", log_path.display()))?;
    let log_err = log_out
        .try_clone()
        .with_context(|| format!("duplicating QEMU log file '{}'", log_path.display()))?;
    cmd.stdout(Stdio::from(log_out));
    cmd.stderr(Stdio::from(log_err));
    if allow_stdin {
        cmd.stdin(Stdio::piped());
    } else {
        cmd.stdin(Stdio::null());
    }
    let child = cmd.spawn().context("Spawning QEMU for SSH boot")?;
    Ok(child)
}

fn collect_guest_ssh_debug(child: &mut Child) -> Result<()> {
    let Some(stdin) = child.stdin.as_mut() else {
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
    stdin
        .write_all(probe.as_bytes())
        .context("writing guest SSH debug probe to QEMU stdin")?;
    stdin
        .flush()
        .context("flushing guest SSH debug probe to QEMU stdin")?;
    sleep(Duration::from_secs(2));
    Ok(())
}

/// A fatal live-boot pattern from the contract, or QEMU failing to set up
/// the SSH host forward.
fn detect_boot_regression(log_path: &Path, patterns: &LiveBootPatterns) -> Result<Option<String>> {
    let content = fs::read_to_string(log_path).unwrap_or_default();
    if let Some(line) = content
        .lines()
        .find(|l| l.to_lowercase().contains("could not set up host forwarding rule"))
    {
        return Ok(Some(format!("hostfwd setup failed: {line}")));
    }
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

fn temp_log_path(prefix: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur| dur.as_nanos())
        .unwrap_or(0);
    std::env::temp_dir().join(format!("{prefix}-{ts}.log"))
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

fn run_checked(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().with_context(|| "Spawning command")?;
    if !status.success() {
        bail!("Command failed with status {status}");
    }
    Ok(())
}
