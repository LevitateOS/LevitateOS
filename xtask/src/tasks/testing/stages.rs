use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{Read, Write};
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
    fn allocate() -> Result<Self> {
        let mode = detect_window_mode()?;
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
        let serial_log = temp_file_path("levitate-stage-window-serial").with_extension("log");
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
            "vnc={}:{display_index},to=99,id=stage-window",
            bind_host
        ))
    }
}

pub fn boot(
    n: u8,
    distro: crate::cli::BootDistro,
    inject: Option<String>,
    inject_file: Option<PathBuf>,
    ssh: bool,
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    window: bool,
    ssh_private_key: Option<PathBuf>,
) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let cfg = BootConfig::for_distro(&root, distro);

    if window && ssh {
        bail!(
            "`--window` cannot be combined with `--ssh`.\n\
             Use `just stage-window <n> <distro>` for local QEMU window mode,\n\
             `just stage-window-remote <n> <distro>` for foreground VNC remote mode,\n\
             or `just stage-ssh <n> <distro>` for SSH workflow."
        );
    }
    if window && no_shell {
        bail!(
            "`--window` cannot be combined with `--no-shell`.\n\
             Window mode runs in foreground and should be stopped with Ctrl-C."
        );
    }

    let window_cfg = if window {
        Some(WindowConfig::allocate()?)
    } else {
        None
    };

    match n {
        1 => {
            let iso_path = resolve_stage_iso(
                "01Boot",
                &cfg.stage01_root,
                cfg.stage01_iso_filename,
                cfg.harness_distro,
            )?;
            boot_live_iso(
                &root,
                &cfg,
                "Stage 01 live ISO",
                &iso_path,
                inject,
                inject_file,
                ssh,
                ssh_port,
                ssh_timeout,
                no_shell,
                window_cfg.as_ref(),
                ssh_private_key,
            )
        }
        2 => {
            let iso_path = resolve_stage_iso(
                "02LiveTools",
                &cfg.stage02_root,
                cfg.stage02_iso_filename,
                cfg.harness_distro,
            )?;
            boot_live_iso(
                &root,
                &cfg,
                "Stage 02 live tools ISO",
                &iso_path,
                inject,
                inject_file,
                ssh,
                ssh_port,
                ssh_timeout,
                no_shell,
                window_cfg.as_ref(),
                ssh_private_key,
            )
        }
        4 => {
            if ssh {
                bail!(
                    "`--ssh` is only supported for Stage 01; use `cargo xtask stages boot 1 --ssh`."
                );
            }
            boot_installed_disk(&root, &cfg, inject, inject_file, window_cfg.as_ref())
        }
        _ => bail!(
            "Stage {n} is automated. Interactive stages: 01 (live), 02 (live tools), 04 (installed)."
        ),
    }
}

pub fn test(
    n: u8,
    distro: crate::cli::HarnessDistro,
    inject: Option<String>,
    inject_file: Option<PathBuf>,
    force: bool,
) -> Result<()> {
    let mut args = vec![
        "--distro".to_string(),
        distro.id().to_string(),
        "--stage".to_string(),
        n.to_string(),
    ];
    if force {
        args.push("--force".to_string());
    }
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_install_tests(&arg_refs, inject, inject_file)
}

pub fn test_up_to(
    n: u8,
    distro: crate::cli::HarnessDistro,
    inject: Option<String>,
    inject_file: Option<PathBuf>,
) -> Result<()> {
    run_install_tests(
        &["--distro", distro.id(), "--up-to", &n.to_string()],
        inject,
        inject_file,
    )
}

pub fn status(distro: crate::cli::HarnessDistro) -> Result<()> {
    run_install_tests(&["--distro", distro.id(), "--status"], None, None)
}

pub fn reset(distro: crate::cli::HarnessDistro) -> Result<()> {
    run_install_tests(&["--distro", distro.id(), "--reset"], None, None)
}

struct BootConfig {
    stage01_root: PathBuf,
    stage01_iso_filename: &'static str,
    stage02_root: PathBuf,
    stage02_iso_filename: &'static str,
    stage03_root: PathBuf,
    pretty_name: &'static str,
    harness_distro: crate::cli::HarnessDistro,
}

impl BootConfig {
    fn for_distro(root: &Path, distro: crate::cli::BootDistro) -> Self {
        match distro {
            crate::cli::BootDistro::Levitate => Self {
                stage01_root: root.join(".artifacts/out/levitate/s01-boot"),
                stage01_iso_filename: "levitateos-x86_64-s01_boot.iso",
                stage02_root: root.join(".artifacts/out/levitate/s02-live-tools"),
                stage02_iso_filename: "levitateos-x86_64-s02_live_tools.iso",
                stage03_root: root.join(".artifacts/out/levitate/s03-install"),
                pretty_name: "LevitateOS",
                harness_distro: crate::cli::HarnessDistro::Levitate,
            },
            crate::cli::BootDistro::Acorn => Self {
                stage01_root: root.join(".artifacts/out/acorn/s01-boot"),
                stage01_iso_filename: "acornos-s01_boot.iso",
                stage02_root: root.join(".artifacts/out/acorn/s02-live-tools"),
                stage02_iso_filename: "acornos-s02_live_tools.iso",
                stage03_root: root.join(".artifacts/out/acorn/s03-install"),
                pretty_name: "AcornOS",
                harness_distro: crate::cli::HarnessDistro::Acorn,
            },
            crate::cli::BootDistro::Iuppiter => Self {
                stage01_root: root.join(".artifacts/out/iuppiter/s01-boot"),
                stage01_iso_filename: "iuppiter-x86_64-s01_boot.iso",
                stage02_root: root.join(".artifacts/out/iuppiter/s02-live-tools"),
                stage02_iso_filename: "iuppiter-x86_64-s02_live_tools.iso",
                stage03_root: root.join(".artifacts/out/iuppiter/s03-install"),
                pretty_name: "IuppiterOS",
                harness_distro: crate::cli::HarnessDistro::Iuppiter,
            },
            crate::cli::BootDistro::Ralph => Self {
                stage01_root: root.join(".artifacts/out/ralph/s01-boot"),
                stage01_iso_filename: "ralphos-x86_64-s01_boot.iso",
                stage02_root: root.join(".artifacts/out/ralph/s02-live-tools"),
                stage02_iso_filename: "ralphos-x86_64-s02_live_tools.iso",
                stage03_root: root.join(".artifacts/out/ralph/s03-install"),
                pretty_name: "RalphOS",
                harness_distro: crate::cli::HarnessDistro::Ralph,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct StageRunManifest {
    run_id: Option<String>,
    status: String,
    created_at_utc: String,
    finished_at_utc: Option<String>,
    iso_path: Option<String>,
    disk_path: Option<String>,
    ovmf_vars_path: Option<String>,
}

struct Stage03RuntimeArtifacts {
    run_id: String,
    disk_path: PathBuf,
    ovmf_vars_path: PathBuf,
    disk_format: String,
}

fn resolve_stage_iso(
    stage_label: &str,
    stage_root: &Path,
    stage_iso_filename: &str,
    harness_distro: crate::cli::HarnessDistro,
) -> Result<PathBuf> {
    let mut candidates: Vec<(String, PathBuf)> = Vec::new();
    if stage_root.is_dir() {
        for entry in fs::read_dir(stage_root).with_context(|| {
            format!(
                "reading {} output directory '{}'",
                stage_label,
                stage_root.display()
            )
        })? {
            let entry = entry.with_context(|| {
                format!(
                    "iterating {} output directory '{}'",
                    stage_label,
                    stage_root.display()
                )
            })?;
            let run_dir = entry.path();
            if !run_dir.is_dir() {
                continue;
            }
            let manifest_path = run_dir.join("run-manifest.json");
            if !manifest_path.is_file() {
                continue;
            }
            let raw = fs::read(&manifest_path).with_context(|| {
                format!("reading stage run manifest '{}'", manifest_path.display())
            })?;
            let manifest: StageRunManifest = serde_json::from_slice(&raw).with_context(|| {
                format!("parsing stage run manifest '{}'", manifest_path.display())
            })?;
            if manifest.status != "success" {
                continue;
            }
            let sort_key = manifest
                .finished_at_utc
                .clone()
                .unwrap_or(manifest.created_at_utc.clone());
            let iso_candidate = manifest
                .iso_path
                .as_ref()
                .map(PathBuf::from)
                .filter(|path| path.is_file())
                .unwrap_or_else(|| run_dir.join(stage_iso_filename));
            if iso_candidate.is_file() {
                candidates.push((sort_key, iso_candidate));
            }
        }
    }

    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    if let Some((_, iso_path)) = candidates.into_iter().next() {
        return Ok(iso_path);
    }

    let expected_path = stage_root.join(stage_iso_filename);

    bail!(
        "Missing {} ISO: no successful run-manifest ISO found under '{}'.\n\
         Expected ISO path: {}\n\
         Build it first, e.g. `just build {} {}`.",
        stage_label,
        stage_root.display(),
        expected_path.display(),
        harness_distro.id(),
        stage_label,
    )
}

struct BootInjection {
    path: PathBuf,
    cleanup: bool,
    media_iso: Option<PathBuf>,
}

impl Drop for BootInjection {
    fn drop(&mut self) {
        if self.cleanup {
            let _ = fs::remove_file(&self.path);
        }
        if let Some(path) = &self.media_iso {
            let _ = fs::remove_file(path);
        }
    }
}

fn boot_injection_payload(
    inject: Option<String>,
    inject_file: Option<PathBuf>,
) -> Result<Option<BootInjection>> {
    if let Some(path) = inject_file {
        if !path.is_file() {
            bail!("--inject-file is not a readable file: {}", path.display());
        }
        return Ok(Some(BootInjection {
            path,
            cleanup: false,
            media_iso: None,
        }));
    }

    let inject = match inject {
        Some(raw) => raw,
        None => return Ok(None),
    };
    let raw = inject.trim();
    if raw.is_empty() {
        return Ok(None);
    }

    let mut lines = Vec::new();
    for entry in raw
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
    {
        match entry.split_once('=') {
            Some((key, _value)) if !key.trim().is_empty() => {
                lines.push(entry.to_string());
            }
            _ => {
                bail!(
                    "invalid --inject payload '{}'; expected KEY=VALUE pairs separated by commas",
                    entry
                );
            }
        }
    }
    if lines.is_empty() {
        return Ok(None);
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock before UNIX_EPOCH")?
        .as_nanos();
    let path = std::env::temp_dir().join(format!("levitate-boot-injection-{ts}.env"));
    fs::write(&path, format!("{}\n", lines.join("\n")))
        .with_context(|| format!("writing boot injection payload '{}'", path.display()))?;

    Ok(Some(BootInjection {
        path,
        cleanup: true,
        media_iso: None,
    }))
}

fn boot_live_iso(
    root: &Path,
    cfg: &BootConfig,
    stage_label: &'static str,
    iso_path: &Path,
    inject: Option<String>,
    inject_file: Option<PathBuf>,
    ssh: bool,
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    window: Option<&WindowConfig>,
    ssh_private_key: Option<PathBuf>,
) -> Result<()> {
    let mut injection = boot_injection_payload(inject, inject_file)?;
    if let Some(inj) = injection.as_mut() {
        inj.media_iso = Some(create_boot_injection_iso(&inj.path)?);
    }
    if ssh {
        boot_live_iso_ssh(
            root,
            cfg,
            stage_label,
            iso_path,
            injection,
            ssh_port,
            ssh_timeout,
            no_shell,
            window,
            ssh_private_key,
        )
    } else {
        boot_live_iso_serial(root, cfg, iso_path, injection, no_shell, window)
    }
}

fn boot_live_iso_serial(
    root: &Path,
    cfg: &BootConfig,
    iso_path: &Path,
    injection: Option<BootInjection>,
    no_shell: bool,
    window: Option<&WindowConfig>,
) -> Result<()> {
    if no_shell {
        let log_path = temp_log_path("levitate-stage01-serial-boot");
        let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), None, window)?;
        let child = spawn_qemu_with_log(&mut cmd, &log_path, false)?;
        monitor_live_iso_serial(child, &log_path)?;
        let _ = fs::remove_file(&log_path);
        return Ok(());
    }

    if window.is_some() {
        eprintln!(
            "Booting {} live ISO in window mode... (Ctrl-C to stop)",
            cfg.pretty_name
        );
    } else {
        eprintln!("Booting {} live ISO... (Ctrl-A X to exit)", cfg.pretty_name);
    }
    let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), None, window)?;
    if let Some(window_cfg) = window {
        run_window_mode_foreground(&mut cmd, window_cfg)
    } else {
        run_checked(&mut cmd)
    }
}

fn monitor_live_iso_serial(mut child: Child, log_path: &Path) -> Result<()> {
    let booted_message = "switching root to live system";
    let default_timeout = 120u64;
    let timeout_secs = std::env::var("LEVITATE_STAGE01_SERIAL_TIMEOUT")
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

        if let Some(pat) = detect_boot_regression(log_path)? {
            let _ = child.kill();
            let _ = child.wait();
            return bail_with_tail(
                &format!("Detected boot regression while waiting for live boot handoff: {pat}"),
                log_path,
                None::<&str>,
            );
        }

        if detect_live_boot_success(log_path, booted_message) {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(());
        }

        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            return bail_with_tail(
                &format!("Timed out waiting for Stage 01 serial boot handoff ({timeout_secs}s)"),
                log_path,
                Some("No root-switch handoff marker observed."),
            );
        }

        sleep(Duration::from_secs(1));
    }
}

fn detect_live_boot_success(log_path: &Path, pattern: &str) -> bool {
    let content = match fs::read_to_string(log_path) {
        Ok(raw) => raw,
        Err(_) => return false,
    };

    let needle = pattern.to_lowercase();
    content
        .lines()
        .any(|line| line.to_lowercase().contains(&needle))
}

fn boot_live_iso_ssh(
    root: &Path,
    cfg: &BootConfig,
    stage_label: &'static str,
    iso_path: &Path,
    injection: Option<BootInjection>,
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    window: Option<&WindowConfig>,
    ssh_private_key: Option<PathBuf>,
) -> Result<()> {
    eprintln!(
        "Booting {} {} with SSH wait (port 127.0.0.1:{ssh_port})...",
        cfg.pretty_name, stage_label
    );
    ensure_ssh_port_available(ssh_port)?;

    let mut cmd = qemu_base_command(root, iso_path, injection.as_ref(), Some(ssh_port), window)?;
    let log_path = temp_log_path("levitate-stage01-ssh-boot");
    let child = spawn_qemu_with_log(&mut cmd, &log_path, true)?;
    let result = monitor_live_iso_ssh(
        child,
        &log_path,
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
    ssh_port: u16,
    ssh_timeout: u64,
    no_shell: bool,
    ssh_private_key: Option<PathBuf>,
) -> Result<()> {
    let known_hosts = temp_file_path("levitate-stage01-ssh-known-hosts");
    fs::write(&known_hosts, "").context("creating known-hosts scratch file")?;
    let deadline = Instant::now() + Duration::from_secs(ssh_timeout.max(1));
    let key = resolve_ssh_private_key(ssh_private_key)?;
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

        if let Some(pat) = detect_boot_regression(log_path)? {
            let _ = child.kill();
            let _ = child.wait();
            let _ = fs::remove_file(&known_hosts);
            return bail_with_tail(
                &format!(
                    "Detected boot regression while waiting for SSH (sshd failure or locale warning): {pat}"
                ),
                log_path,
                None::<&str>,
            );
        }

        if !hook_seen {
            if let Some(pat) = detect_stage01_boot_hook(log_path)? {
                hook_seen = true;
                eprintln!(
                    "Boot hook observed ({pat}); waiting for SSH readiness on 127.0.0.1:{ssh_port}..."
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

fn detect_stage01_boot_hook(log_path: &Path) -> Result<Option<String>> {
    let raw = match fs::read_to_string(log_path) {
        Ok(raw) => raw,
        Err(_) => return Ok(None),
    };

    for pat in ["___SHELL_READY___"] {
        if raw.contains(pat) {
            return Ok(Some(pat.to_string()));
        }
    }
    Ok(None)
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
        println!("{}", strip_ansi_escapes(&line));
    }
    *line_cursor = total_lines;
    Ok(())
}

fn strip_ansi_escapes(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] != b'\x1b' {
            out.push(bytes[i] as char);
            i += 1;
            continue;
        }

        if i + 1 >= bytes.len() {
            break;
        }

        match bytes[i + 1] {
            b'[' => {
                i += 2;
                while i < bytes.len() {
                    let b = bytes[i];
                    if (0x40..=0x7e).contains(&b) {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
            }
            b']' => {
                i += 2;
                while i < bytes.len() {
                    if bytes[i] == 0x07 {
                        i += 1;
                        break;
                    }
                    if bytes[i] == b'\x1b' && i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            }
            _ => {
                i += 2;
            }
        }
    }

    out
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
    args.push("root@127.0.0.1".to_string());
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
    args.push("root@127.0.0.1".to_string());
    args.push("true".to_string());
    let status = Command::new("ssh")
        .args(&args)
        .status()
        .context("checking SSH readiness")?;
    Ok(status.success())
}

fn resolve_ssh_private_key(arg: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = arg {
        if !path.is_file() {
            bail!("SSH private key does not exist: {}", path.display());
        }
        return Ok(path);
    }

    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set; pass --ssh-private-key")?;
    let fallback = home.join(".ssh").join("id_ed25519");
    if !fallback.is_file() {
        bail!(
            "--ssh-private-key was not provided and {} does not exist",
            fallback.display()
        );
    }
    Ok(fallback)
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
    cfg: &BootConfig,
    _inject: Option<String>,
    _inject_file: Option<PathBuf>,
    window: Option<&WindowConfig>,
) -> Result<()> {
    let runtime = resolve_stage03_runtime("03Install", &cfg.stage03_root, cfg.harness_distro)?;
    let disk = runtime.disk_path;
    let vars = runtime.ovmf_vars_path;
    let disk_format = runtime.disk_format;
    let ovmf = crate::util::repo::ovmf_path(root)?;

    let mut cmd = qemu_command_for_window(window)?;
    cmd.args([
        "-enable-kvm",
        "-cpu",
        "host",
        "-smp",
        "4",
        "-m",
        "4G",
        "-drive",
        &format!(
            "file={},format={},if=virtio",
            disk.display(),
            disk_format.as_str()
        ),
        "-drive",
        &format!("if=pflash,format=raw,readonly=on,file={}", ovmf.display()),
        "-drive",
        &format!("if=pflash,format=raw,file={}", vars.display()),
        "-boot",
        "c",
        "-netdev",
        "user,id=net0",
        "-device",
        "virtio-net-pci,netdev=net0",
    ]);
    if window.is_some() {
        eprintln!(
            "Booting installed {} from Stage 03 run {} in window mode... (Ctrl-C to stop)",
            cfg.pretty_name, runtime.run_id
        );
    } else {
        eprintln!(
            "Booting installed {} from Stage 03 run {}... (Ctrl-A X to exit)",
            cfg.pretty_name, runtime.run_id
        );
    }
    eprintln!("  disk: {}", disk.display());
    eprintln!("  disk format: {}", disk_format);
    eprintln!("  ovmf vars: {}", vars.display());
    if disk_format != "qcow2" {
        eprintln!(
            "  warning: Stage 03 produced non-qcow2 disk format '{}'; investigate qemu-img availability for strict qcow2 parity.",
            disk_format
        );
    }
    apply_qemu_console_mode(&mut cmd, window);
    cmd.arg("-no-reboot");

    apply_qemu_runtime_env(&mut cmd, root, window)?;
    if let Some(window_cfg) = window {
        run_window_mode_foreground(&mut cmd, window_cfg)
    } else {
        run_checked(&mut cmd)
    }
}

fn resolve_stage03_runtime(
    stage_label: &str,
    stage_root: &Path,
    harness_distro: crate::cli::HarnessDistro,
) -> Result<Stage03RuntimeArtifacts> {
    let mut candidates: Vec<(String, Stage03RuntimeArtifacts)> = Vec::new();
    if stage_root.is_dir() {
        for entry in fs::read_dir(stage_root).with_context(|| {
            format!(
                "reading {} output directory '{}'",
                stage_label,
                stage_root.display()
            )
        })? {
            let entry = entry.with_context(|| {
                format!(
                    "iterating {} output directory '{}'",
                    stage_label,
                    stage_root.display()
                )
            })?;
            let run_dir = entry.path();
            if !run_dir.is_dir() {
                continue;
            }
            let manifest_path = run_dir.join("run-manifest.json");
            if !manifest_path.is_file() {
                continue;
            }
            let raw = fs::read(&manifest_path).with_context(|| {
                format!("reading stage run manifest '{}'", manifest_path.display())
            })?;
            let manifest: StageRunManifest = serde_json::from_slice(&raw).with_context(|| {
                format!("parsing stage run manifest '{}'", manifest_path.display())
            })?;
            if manifest.status != "success" {
                continue;
            }

            let sort_key = manifest
                .finished_at_utc
                .clone()
                .unwrap_or(manifest.created_at_utc.clone());
            let run_id = manifest
                .run_id
                .clone()
                .or_else(|| {
                    run_dir
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(ToOwned::to_owned)
                })
                .unwrap_or_else(|| "<unknown>".to_string());

            let disk_candidate = manifest
                .disk_path
                .as_ref()
                .map(PathBuf::from)
                .unwrap_or_else(|| run_dir.join("stage-disk.qcow2"));
            let vars_candidate = manifest
                .ovmf_vars_path
                .as_ref()
                .map(PathBuf::from)
                .unwrap_or_else(|| run_dir.join("stage-ovmf-vars.fd"));
            if disk_candidate.is_file() && vars_candidate.is_file() {
                let disk_format = detect_disk_image_format(&disk_candidate).with_context(|| {
                    format!("detecting disk format for '{}'", disk_candidate.display())
                })?;
                candidates.push((
                    sort_key,
                    Stage03RuntimeArtifacts {
                        run_id,
                        disk_path: disk_candidate,
                        ovmf_vars_path: vars_candidate,
                        disk_format,
                    },
                ));
            }
        }
    }

    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    if let Some((_, artifacts)) = candidates.into_iter().next() {
        return Ok(artifacts);
    }

    bail!(
        "Missing Stage 03 install runtime artifacts under '{}'.\n\
         Run Stage 03 first (example: `just test 3 {}`), then retry Stage 04 boot.\n\
         Expected latest successful run with files: stage-disk.qcow2 + stage-ovmf-vars.fd",
        stage_root.display(),
        harness_distro.id(),
    )
}

fn detect_disk_image_format(path: &Path) -> Result<String> {
    // qcow2 images begin with ASCII 'QFI' + 0xfb.
    const QCOW2_MAGIC: [u8; 4] = [0x51, 0x46, 0x49, 0xFB];

    let mut file = File::open(path).with_context(|| {
        format!(
            "opening disk image for format detection '{}'",
            path.display()
        )
    })?;
    let mut header = [0u8; 4];
    file.read_exact(&mut header).with_context(|| {
        format!(
            "reading disk image header for format detection '{}'",
            path.display()
        )
    })?;
    if header == QCOW2_MAGIC {
        Ok("qcow2".to_string())
    } else {
        Ok("raw".to_string())
    }
}

fn run_install_tests(
    args: &[&str],
    inject: Option<String>,
    inject_file: Option<PathBuf>,
) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    run_install_tests_in_dir(&root, args, inject, inject_file)
}

fn run_install_tests_in_dir(
    root: &Path,
    args: &[&str],
    inject: Option<String>,
    inject_file: Option<PathBuf>,
) -> Result<()> {
    let install_tests_dir = root.join("testing/install-tests");
    if !install_tests_dir.is_dir() {
        bail!(
            "Missing {} (submodule not initialized? try `git submodule update --init --recursive`)",
            install_tests_dir.display()
        );
    }

    let mut cmd = Command::new("cargo");
    cmd.current_dir(&install_tests_dir)
        .args(["run", "--bin", "stages", "--"])
        .args(args);
    if let Some(path) = &inject_file {
        let path = path.to_string_lossy();
        cmd.args(["--inject-file", path.as_ref()]);
    }
    if let Some(payload) = inject {
        cmd.args(["--inject", &payload]);
    }

    crate::util::tools_env::apply_to_command(&mut cmd, root)?;
    run_checked(&mut cmd).with_context(|| {
        format!(
            "Running install-tests stages in {}",
            install_tests_dir.display()
        )
    })
}

fn qemu_base_command(
    root: &Path,
    iso_path: &Path,
    injection: Option<&BootInjection>,
    ssh_port: Option<u16>,
    window: Option<&WindowConfig>,
) -> Result<Command> {
    let ovmf = crate::util::repo::ovmf_path(root)?;
    let mut cmd = qemu_command_for_window(window)?;
    cmd.args([
        "-enable-kvm",
        "-cpu",
        "host",
        "-smp",
        "4",
        "-m",
        "4G",
        "-device",
        "virtio-scsi-pci,id=scsi0",
        "-device",
        "scsi-cd,drive=cdrom0,bus=scsi0.0",
        "-drive",
        &format!(
            "id=cdrom0,if=none,format=raw,readonly=on,file={}",
            iso_path.display()
        ),
        "-drive",
        &format!("if=pflash,format=raw,readonly=on,file={}", ovmf.display()),
    ]);
    apply_qemu_console_mode(&mut cmd, window);
    cmd.arg("-no-reboot");
    if let Some(injection) = injection {
        let fw_cfg = format!(
            "name=opt/levitate/boot-injection,file={}",
            injection.path.display()
        );
        cmd.args(["-fw_cfg", &fw_cfg]);
        if let Some(media_iso) = &injection.media_iso {
            cmd.args([
                "-device",
                "virtio-scsi-pci,id=scsi2",
                "-device",
                "scsi-cd,drive=inject0,bus=scsi2.0",
                "-drive",
                &format!(
                    "id=inject0,if=none,format=raw,readonly=on,file={}",
                    media_iso.display()
                ),
            ]);
        }
    }
    if let Some(ssh_port) = ssh_port {
        cmd.args([
            "-netdev",
            &format!("user,id=net0,hostfwd=tcp:127.0.0.1:{ssh_port}-:22"),
            "-device",
            "virtio-net-pci,netdev=net0",
        ]);
    } else {
        cmd.args(["-netdev", "user,id=net0"]);
        cmd.args(["-device", "virtio-net-pci,netdev=net0"]);
    }

    apply_qemu_runtime_env(&mut cmd, root, window)?;
    Ok(cmd)
}

fn apply_qemu_console_mode(cmd: &mut Command, window: Option<&WindowConfig>) {
    if let Some(window_cfg) = window {
        match window_cfg.mode {
            WindowMode::RemoteVnc => {
                let display_arg = window_cfg
                    .qemu_display_arg()
                    .expect("internal error: remote window mode missing VNC display configuration");
                cmd.args(["-vga", "none", "-device", "secondary-vga"]);
                cmd.arg("-display").arg(display_arg);
                cmd.arg("-serial")
                    .arg(format!("file:{}", window_cfg.serial_log.display()));
                cmd.args(["-monitor", "none"]);
            }
            WindowMode::LocalGui => {
                let display_backend = window_cfg
                    .local_display_backend
                    .expect("internal error: local window mode missing local display backend");
                cmd.args(["-display", display_backend.qemu_arg(), "-vga", "virtio"]);
                cmd.arg("-serial")
                    .arg(format!("file:{}", window_cfg.serial_log.display()));
                cmd.args(["-monitor", "none"]);
            }
        }
    } else {
        cmd.args(["-vga", "none", "-nographic", "-serial", "mon:stdio"]);
    }
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

fn detect_window_mode() -> Result<WindowMode> {
    let raw = std::env::var("LEVITATE_STAGE_WINDOW_MODE").unwrap_or_else(|_| "remote".to_string());
    match raw.trim().to_ascii_lowercase().as_str() {
        "remote" | "vnc" | "" => Ok(WindowMode::RemoteVnc),
        "local" | "gtk" => Ok(WindowMode::LocalGui),
        other => bail!(
            "Unsupported LEVITATE_STAGE_WINDOW_MODE value '{}'. Expected one of: remote, vnc, local, gtk.",
            other
        ),
    }
}

fn qemu_command_for_window(window: Option<&WindowConfig>) -> Result<Command> {
    if let Some(window_cfg) = window {
        if window_cfg.mode == WindowMode::LocalGui {
            let qemu_bin = window_cfg.local_qemu_bin.as_ref().ok_or_else(|| {
                anyhow::anyhow!("local window mode missing detected qemu binary path")
            })?;
            return Ok(Command::new(qemu_bin));
        }
    }
    Ok(Command::new("qemu-system-x86_64"))
}

fn apply_qemu_runtime_env(
    cmd: &mut Command,
    root: &Path,
    window: Option<&WindowConfig>,
) -> Result<()> {
    if matches!(window.map(|cfg| cfg.mode), Some(WindowMode::LocalGui)) {
        // Local GUI mode intentionally uses the detected system QEMU binary.
        // Avoid forcing tools PATH/LD_LIBRARY_PATH, which can shadow host GUI-enabled builds.
        cmd.env_remove("LD_LIBRARY_PATH");
        return Ok(());
    }
    crate::util::tools_env::apply_to_command(cmd, root)
}

fn detect_local_qemu_runtime() -> Result<(PathBuf, LocalDisplayBackend)> {
    if let Ok(raw) = std::env::var("LEVITATE_STAGE_WINDOW_QEMU_BIN") {
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
         or use `just stage-window-remote <n> <distro>`."
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
    if let Ok(raw) = std::env::var("LEVITATE_STAGE_WINDOW_BIND_HOST") {
        let parsed: Ipv4Addr = raw.parse().with_context(|| {
            format!("Parsing LEVITATE_STAGE_WINDOW_BIND_HOST as IPv4 failed: {raw}")
        })?;
        return Ok(parsed);
    }
    Ok(Ipv4Addr::UNSPECIFIED)
}

fn detect_vnc_advertise_host() -> Result<Ipv4Addr> {
    if let Ok(raw) = std::env::var("LEVITATE_STAGE_WINDOW_HOST") {
        let parsed: Ipv4Addr = raw.parse().with_context(|| {
            format!("Parsing LEVITATE_STAGE_WINDOW_HOST as IPv4 address failed: {raw}")
        })?;
        if parsed.is_unspecified() {
            bail!("LEVITATE_STAGE_WINDOW_HOST must not be unspecified; got {parsed}");
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

fn create_boot_injection_iso(payload_path: &Path) -> Result<PathBuf> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock before UNIX_EPOCH")?
        .as_nanos();
    let iso_path = std::env::temp_dir().join(format!("levitate-boot-injection-{ts}.iso"));

    let mut tried = Vec::new();
    for (tool, mut args) in [
        (
            "xorriso",
            vec![
                "-as".to_string(),
                "mkisofs".to_string(),
                "-quiet".to_string(),
                "-V".to_string(),
                "LEVITATE_INJECT".to_string(),
                "-o".to_string(),
                iso_path.display().to_string(),
                "-graft-points".to_string(),
            ],
        ),
        (
            "genisoimage",
            vec![
                "-quiet".to_string(),
                "-V".to_string(),
                "LEVITATE_INJECT".to_string(),
                "-o".to_string(),
                iso_path.display().to_string(),
                "-graft-points".to_string(),
            ],
        ),
        (
            "mkisofs",
            vec![
                "-quiet".to_string(),
                "-V".to_string(),
                "LEVITATE_INJECT".to_string(),
                "-o".to_string(),
                iso_path.display().to_string(),
                "-graft-points".to_string(),
            ],
        ),
    ] {
        args.push(format!("boot-injection.env={}", payload_path.display()));
        match Command::new(tool).args(&args).status() {
            Ok(status) if status.success() => return Ok(iso_path),
            Ok(status) => {
                tried.push(format!("{tool} exited with status {status}"));
            }
            Err(err) => {
                tried.push(format!("{tool} unavailable: {err}"));
            }
        }
    }

    bail!(
        "failed to build boot-injection ISO from '{}': {}",
        payload_path.display(),
        tried.join("; ")
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn resolve_stage03_runtime_prefers_latest_successful_run() {
        let root = test_temp_dir("stage03-resolve-latest");
        let stage_root = root.join("s03-install");
        fs::create_dir_all(&stage_root).expect("create stage root");

        let run_old = stage_root.join("run-old");
        fs::create_dir_all(&run_old).expect("create old run dir");
        fs::write(run_old.join("stage-disk.qcow2"), b"old-disk").expect("write old disk");
        fs::write(run_old.join("stage-ovmf-vars.fd"), b"old-vars").expect("write old vars");
        fs::write(
            run_old.join("run-manifest.json"),
            r#"{
  "run_id": "run-old",
  "status": "success",
  "created_at_utc": "20260101T000000Z",
  "finished_at_utc": "20260101T000100Z"
}"#,
        )
        .expect("write old manifest");

        let run_new = stage_root.join("run-new");
        fs::create_dir_all(&run_new).expect("create new run dir");
        fs::write(run_new.join("stage-disk.qcow2"), b"new-disk").expect("write new disk");
        fs::write(run_new.join("stage-ovmf-vars.fd"), b"new-vars").expect("write new vars");
        fs::write(
            run_new.join("run-manifest.json"),
            r#"{
  "run_id": "run-new",
  "status": "success",
  "created_at_utc": "20260101T010000Z",
  "finished_at_utc": "20260101T010100Z"
}"#,
        )
        .expect("write new manifest");

        let resolved = resolve_stage03_runtime(
            "03Install",
            &stage_root,
            crate::cli::HarnessDistro::Iuppiter,
        )
        .expect("resolve runtime");

        assert_eq!(resolved.run_id, "run-new");
        assert_eq!(resolved.disk_path, run_new.join("stage-disk.qcow2"));
        assert_eq!(resolved.ovmf_vars_path, run_new.join("stage-ovmf-vars.fd"));
        assert_eq!(resolved.disk_format, "raw");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_stage03_runtime_ignores_failed_or_incomplete_runs() {
        let root = test_temp_dir("stage03-resolve-filter");
        let stage_root = root.join("s03-install");
        fs::create_dir_all(&stage_root).expect("create stage root");

        let run_failed = stage_root.join("run-failed");
        fs::create_dir_all(&run_failed).expect("create failed run dir");
        fs::write(run_failed.join("stage-disk.qcow2"), b"failed-disk").expect("write failed disk");
        fs::write(run_failed.join("stage-ovmf-vars.fd"), b"failed-vars")
            .expect("write failed vars");
        fs::write(
            run_failed.join("run-manifest.json"),
            r#"{
  "run_id": "run-failed",
  "status": "failed",
  "created_at_utc": "20260101T020000Z",
  "finished_at_utc": "20260101T020100Z"
}"#,
        )
        .expect("write failed manifest");

        let run_success = stage_root.join("run-success");
        fs::create_dir_all(&run_success).expect("create success run dir");
        fs::write(run_success.join("stage-disk.qcow2"), b"ok-disk").expect("write success disk");
        fs::write(run_success.join("stage-ovmf-vars.fd"), b"ok-vars").expect("write success vars");
        fs::write(
            run_success.join("run-manifest.json"),
            r#"{
  "run_id": "run-success",
  "status": "success",
  "created_at_utc": "20260101T021000Z",
  "finished_at_utc": "20260101T021100Z"
}"#,
        )
        .expect("write success manifest");

        let resolved = resolve_stage03_runtime(
            "03Install",
            &stage_root,
            crate::cli::HarnessDistro::Iuppiter,
        )
        .expect("resolve runtime");
        assert_eq!(resolved.run_id, "run-success");
        assert_eq!(resolved.disk_format, "raw");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn detect_disk_image_format_recognizes_qcow2_magic() {
        let root = test_temp_dir("stage03-detect-qcow2");
        fs::create_dir_all(&root).expect("create temp dir");
        let disk = root.join("disk.qcow2");
        fs::write(&disk, [0x51_u8, 0x46, 0x49, 0xFB, 0, 0, 0, 0]).expect("write disk");

        let format = detect_disk_image_format(&disk).expect("detect format");
        assert_eq!(format, "qcow2");

        let _ = fs::remove_dir_all(root);
    }

    fn test_temp_dir(tag: &str) -> PathBuf {
        let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        std::env::temp_dir().join(format!("levitate-xtask-{tag}-{nanos}-{id}"))
    }
}

fn collect_guest_ssh_debug(child: &mut Child) -> Result<()> {
    let Some(stdin) = child.stdin.as_mut() else {
        return Ok(());
    };

    let probe = concat!(
        "\n",
        "echo ___SSH_DEBUG_BEGIN___\n",
        "ip -brief addr 2>/dev/null || ip addr 2>/dev/null || true\n",
        "ip route 2>/dev/null || route -n 2>/dev/null || true\n",
        "rc-service networking status 2>/dev/null || true\n",
        "rc-service dhcpcd status 2>/dev/null || true\n",
        "ss -ltnp | grep ':22' || true\n",
        "ls -l /root/.ssh /root/.ssh/authorized_keys /run/boot-injection /run/boot-injection/* 2>/dev/null || true\n",
        "cat /run/boot-injection/source /run/boot-injection/payload.env 2>/dev/null || true\n",
        "journalctl -b -u sshd.service --no-pager -n 120 || true\n",
        "echo ___SSH_DEBUG_END___\n",
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

fn detect_boot_regression(log_path: &Path) -> Result<Option<String>> {
    if !log_path.is_file() {
        return Ok(None);
    }

    let content = fs::read_to_string(log_path).unwrap_or_default();
    if content.is_empty() {
        return Ok(None);
    }

    for line in content.lines() {
        let lower = line.to_lowercase();
        if lower.contains("could not set up host forwarding rule") {
            return Ok(Some(format!("hostfwd setup failed: {line}")));
        }
        if lower.contains("warning") && lower.contains("locale") {
            return Ok(Some(format!("locale warning: {line}")));
        }
        if lower.contains("failed to start sshd.service")
            || lower.contains("sshd.service: failed with result")
            || lower.contains("start request repeated too quickly")
            || lower.contains("failed to start ssh.service")
        {
            return Ok(Some(format!("sshd failure: {line}")));
        }
    }

    Ok(None)
}

fn ensure_ssh_port_available(ssh_port: u16) -> Result<()> {
    match TcpListener::bind(("127.0.0.1", ssh_port)) {
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
