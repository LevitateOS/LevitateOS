//! `kernel publish <distro>`: the whole manual kernel publish process in the
//! foreground: fetch the pinned source, configure, build, package, boot-test
//! and (with --upload) upload it and repoint `linux-prebuilt.rhai`. Every
//! long command logs to a file, prints a heartbeat, and has a stall watchdog
//! and a hard limit, so a run cannot hang silently.

use anyhow::{Context, Result, bail};
use distro_builder::kernel::{self, KernelPin};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::cli::PublishArgs;

const STEPS: [&str; 6] = ["fetch", "config", "build", "package", "boot", "upload"];
const TAG: &str = "0.0.0-pre-alpha";
const RECIPE: &str = "distro/builder/recipes/linux-prebuilt.rhai";
const HEARTBEAT: Duration = Duration::from_secs(60);
const BUILD_STALL: Duration = Duration::from_secs(10 * 60);
const BUILD_LIMIT: Duration = Duration::from_secs(120 * 60);
const BOOT_LIMIT: Duration = Duration::from_secs(180);
const LIBVIRT: &str = "qemu:///session";

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

struct Ctx {
    root: PathBuf,
    distro: String,
    pin: KernelPin,
    /// `.artifacts/kernel/<distro>/<release>`
    dir: PathBuf,
    /// `.artifacts/kernel/src`
    src_cache: PathBuf,
    args: PublishArgs,
}

impl Ctx {
    fn src(&self) -> PathBuf {
        self.src_cache.join(format!("linux-{}", self.pin.version))
    }
    fn build_dir(&self) -> PathBuf {
        self.dir.join("build")
    }
    fn dist(&self) -> PathBuf {
        self.dir.join("dist")
    }
    fn pkg(&self) -> PathBuf {
        self.dir.join("pkg")
    }
    fn log(&self, step: &str) -> PathBuf {
        self.dir.join(format!("{}.log", if step == "boot" { "console" } else { step }))
    }
    fn runtime_name(&self) -> String {
        format!("{}-{}-runtime.tar.xz", self.distro, self.pin.release)
    }
    fn debug_name(&self) -> String {
        format!("{}-{}-debug.tar.xz", self.distro, self.pin.release)
    }
    fn sums_name(&self) -> String {
        format!("SHA256SUMS-{}-{}.txt", self.distro, self.pin.release)
    }
}

pub fn publish(args: PublishArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let distro = args.distro.id().to_owned();
    let variant = distro_contract::load(&root, &distro)?;
    let pin = kernel::pin(&variant)?;
    if args.from.as_deref() == Some("upload") && !args.upload {
        return Err(crate::util::exit::usage("--from upload needs --upload"));
    }
    let dir = root.join(".artifacts/kernel").join(&distro).join(&pin.release);
    fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    // SAFETY: the handler only stores into an atomic.
    unsafe {
        libc::signal(libc::SIGINT, on_signal as *const () as usize as libc::sighandler_t);
        libc::signal(libc::SIGTERM, on_signal as *const () as usize as libc::sighandler_t);
    }
    let ctx = Ctx { src_cache: root.join(".artifacts/kernel/src"), root, distro, pin, dir, args };
    eprintln!("[info] {} kernel {} (state: {})", ctx.distro, ctx.pin.release, ctx.dir.join("state.json").display());

    let mut state = State::load(ctx.dir.join("state.json"));
    let from = ctx.args.from.as_deref().and_then(|f| STEPS.iter().position(|s| *s == f));
    let wanted: Vec<&str> = STEPS.iter().copied().filter(|s| *s != "upload" || ctx.args.upload).collect();
    let todo: Vec<&str> = wanted
        .iter()
        .copied()
        .filter(|s| match from {
            Some(from) => STEPS.iter().position(|x| x == s).unwrap() >= from,
            None => !state.is_ok(s),
        })
        .collect();

    if let Err(error) = prerequisites(&ctx, &todo) {
        return fail(&ctx, "prereq", None, &error);
    }
    for step in &wanted {
        if !todo.contains(step) {
            eprintln!("[skip] {step} (done earlier; --from {step} to redo)");
            continue;
        }
        eprintln!("[step] {step}");
        let started = Instant::now();
        state.start(step, &ctx.log(step));
        let result = match *step {
            "fetch" => fetch(&ctx),
            "config" => config(&ctx),
            "build" => build(&ctx),
            "package" => package(&ctx),
            "boot" => boot(&ctx),
            _ => upload(&ctx),
        };
        match result {
            Ok(()) => {
                state.finish(step, None);
                eprintln!("[ok] {step} ({})", fmt_duration(started.elapsed()));
            }
            Err(error) => {
                state.finish(step, Some(format!("{error:#}")));
                return fail(&ctx, step, Some(&ctx.log(step)), &error);
            }
        }
    }
    if !ctx.args.upload {
        print_upload_plan(&ctx)?;
    }
    Ok(())
}

/// Print the failure and exit nonzero, with the `[fail]` line last.
fn fail(ctx: &Ctx, step: &str, log: Option<&Path>, error: &anyhow::Error) -> Result<()> {
    let text = format!("{error:#}");
    let first = text.lines().next().unwrap_or("failed").to_owned();
    let log = log.map(Path::to_path_buf).unwrap_or_else(|| ctx.dir.clone());
    if matches!(step, "build" | "package") {
        eprintln!("--- last 40 lines of {} ---", log.display());
        for line in tail_lines(&log, 40) {
            eprintln!("{line}");
        }
        eprintln!("---");
    }
    for line in text.lines().skip(1) {
        eprintln!("{line}");
    }
    eprintln!("[fail] {step}: {first} (log: {})", log.display());
    std::process::exit(1);
}

// ---------------------------------------------------------------- state

struct State {
    path: PathBuf,
    value: Value,
}

impl State {
    fn load(path: PathBuf) -> Self {
        let value = fs::read_to_string(&path)
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or_else(|| json!({ "steps": {} }));
        Self { path, value }
    }
    fn is_ok(&self, step: &str) -> bool {
        self.value["steps"][step]["status"] == "ok"
    }
    fn start(&mut self, step: &str, log: &Path) {
        // Later steps used this step's output; they are stale now.
        if let Some(i) = STEPS.iter().position(|s| *s == step) {
            for later in &STEPS[i + 1..] {
                if let Some(steps) = self.value["steps"].as_object_mut() {
                    steps.remove(*later);
                }
            }
        }
        self.value["steps"][step] =
            json!({ "status": "running", "started": now_secs(), "finished": null, "log": log, "error": null });
        self.save();
    }
    fn finish(&mut self, step: &str, error: Option<String>) {
        let entry = &mut self.value["steps"][step];
        entry["status"] = if error.is_none() { "ok".into() } else { "failed".into() };
        entry["finished"] = now_secs().into();
        entry["error"] = error.map_or(Value::Null, Value::from);
        self.save();
    }
    fn save(&self) {
        let text = serde_json::to_string_pretty(&self.value).unwrap_or_default();
        if let Err(error) = fs::write(&self.path, text) {
            eprintln!("[warn] writing {}: {error}", self.path.display());
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn fmt_duration(d: Duration) -> String {
    let s = d.as_secs();
    if s >= 3600 {
        format!("{}h{:02}m{:02}s", s / 3600, s / 60 % 60, s % 60)
    } else {
        format!("{}m{:02}s", s / 60, s % 60)
    }
}

// ---------------------------------------------------------- prerequisites

fn find_tool(name: &str) -> bool {
    which::which(name).is_ok() || ["/usr/sbin", "/sbin"].iter().any(|d| Path::new(d).join(name).is_file())
}

/// Check the host tools the steps that will run need; list every missing one.
fn prerequisites(ctx: &Ctx, todo: &[&str]) -> Result<()> {
    if todo.is_empty() {
        return Ok(());
    }
    eprintln!("[step] prereq");
    let mut missing: Vec<String> = Vec::new();
    let mut tool = |name: &str, fix: &str| {
        if !find_tool(name) {
            missing.push(format!("{name}: sudo dnf install {fix}"));
        }
    };
    let has = |s: &str| todo.contains(&s);
    if has("fetch") {
        tool("curl", "curl");
        tool("tar", "tar");
        tool("xz", "xz");
    }
    if has("config") || has("build") || has("package") {
        tool("make", "make");
        tool("gcc", "gcc");
        tool("flex", "flex");
        tool("bison", "bison");
        tool("bc", "bc");
        tool("perl", "perl");
        tool("xz", "xz");
        tool("tar", "tar");
        tool("depmod", "kmod");
    }
    if has("boot") {
        tool("virsh", "libvirt-client");
        tool("virt-install", "virt-install");
        tool("cpio", "cpio");
        tool("gzip", "gzip");
        tool("timeout", "coreutils");
    }
    if has("upload") {
        tool("gh", "gh");
    }
    if has("config") || has("build") {
        let any = |paths: &[&str]| paths.iter().any(|p| Path::new(p).is_file());
        if !any(&["/usr/include/libelf.h", "/usr/include/elfutils/libelf.h"]) {
            missing.push("libelf headers: sudo dnf install elfutils-libelf-devel".into());
        }
        if !any(&["/usr/include/openssl/ssl.h"]) {
            missing.push("openssl headers: sudo dnf install openssl-devel".into());
        }
    }
    let _ = ctx;
    if missing.is_empty() {
        eprintln!("[ok] prereq");
        return Ok(());
    }
    bail!("{} host prerequisite(s) missing\n  {}", missing.len(), missing.join("\n  "))
}

// ------------------------------------------------------- running commands

struct Limits<'a> {
    label: &'a str,
    log: &'a Path,
    /// A file whose growth also counts as progress and whose size is shown.
    watch: Option<&'a Path>,
    stall: Option<Duration>,
    hard: Duration,
}

fn size_of(p: &Path) -> u64 {
    fs::metadata(p).map(|m| m.len()).unwrap_or(0)
}

fn kill_group(child: &mut std::process::Child) {
    // SAFETY: signals the child's own process group (it leads one).
    unsafe { libc::kill(-(child.id() as i32), libc::SIGKILL) };
    let _ = child.wait();
}

fn tail_lines(path: &Path, n: usize) -> Vec<String> {
    let Ok(mut file) = File::open(path) else { return vec![] };
    let len = file.metadata().map(|m| m.len()).unwrap_or(0);
    let start = len.saturating_sub(64 * 1024);
    let mut bytes = Vec::new();
    let _ = std::io::Seek::seek(&mut file, std::io::SeekFrom::Start(start));
    let _ = file.read_to_end(&mut bytes);
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].iter().map(|l| (*l).to_owned()).collect()
}

fn last_line(path: &Path) -> String {
    let line = tail_lines(path, 5).into_iter().rev().find(|l| !l.trim().is_empty()).unwrap_or_default();
    line.chars().take(100).collect()
}

/// Run `cmd` in its own process group with output appended to `limits.log`;
/// heartbeat every 60 s, kill it on a stall, the hard limit or Ctrl-C.
fn run_logged(mut cmd: Command, limits: Limits) -> Result<()> {
    let mut log = fs::OpenOptions::new().create(true).append(true).open(limits.log)?;
    writeln!(log, "$ {cmd:?}")?;
    cmd.stdin(Stdio::null())
        .stdout(log.try_clone()?)
        .stderr(log)
        .process_group(0);
    let mut child = cmd.spawn().with_context(|| format!("starting {}", limits.label))?;
    let (start, mut beat, mut grew) = (Instant::now(), Instant::now(), Instant::now());
    let size = |l: &Limits| size_of(l.log) + l.watch.map_or(0, size_of);
    let mut last = size(&limits);
    loop {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            }
            bail!("{} failed ({status})", limits.label);
        }
        if INTERRUPTED.load(Ordering::SeqCst) {
            kill_group(&mut child);
            bail!("{} interrupted", limits.label);
        }
        let now = Instant::now();
        let current = size(&limits);
        if current != last {
            last = current;
            grew = now;
        }
        if now - beat >= HEARTBEAT {
            beat = now;
            eprintln!(
                "[..] {} {} elapsed, {} KiB output, last: {}",
                limits.label,
                fmt_duration(start.elapsed()),
                current / 1024,
                last_line(limits.log)
            );
        }
        if let Some(stall) = limits.stall
            && now - grew >= stall
        {
            kill_group(&mut child);
            bail!("{} stalled: no output for {} min", limits.label, stall.as_secs() / 60);
        }
        if now - start >= limits.hard {
            kill_group(&mut child);
            bail!("{} exceeded the {} min limit", limits.label, limits.hard.as_secs() / 60);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn quick(ctx: &Ctx, step: &str, label: &str, cmd: Command, hard_min: u64) -> Result<()> {
    run_logged(cmd, Limits { label, log: &ctx.log(step), watch: None, stall: None, hard: Duration::from_secs(hard_min * 60) })
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

fn make(ctx: &Ctx) -> Command {
    let mut cmd = Command::new("make");
    cmd.arg("-C").arg(ctx.src()).arg(format!("O={}", ctx.build_dir().display()));
    cmd
}

// ------------------------------------------------------------------ steps

fn fetch(ctx: &Ctx) -> Result<()> {
    let version = &ctx.pin.version;
    let major = version.split('.').next().unwrap_or("");
    let url = format!("https://cdn.kernel.org/pub/linux/kernel/v{major}.x/linux-{version}.tar.xz");
    fs::create_dir_all(&ctx.src_cache)?;
    let tarball = ctx.src_cache.join(format!("linux-{version}.tar.xz"));
    let log = ctx.log("fetch");
    let valid = tarball.is_file() && sha256_file(&tarball)? == ctx.pin.sha256;
    if valid {
        eprintln!("[info] using the cached {}", tarball.display());
    } else {
        let part = ctx.src_cache.join(format!("linux-{version}.tar.xz.part"));
        let mut cmd = Command::new("curl");
        cmd.args(["-fsSL", "--retry", "3", "--connect-timeout", "30", "--speed-time", "60", "--speed-limit", "10000", "-o"])
            .arg(&part)
            .arg(&url);
        run_logged(
            cmd,
            Limits { label: "download", log: &log, watch: Some(&part), stall: Some(Duration::from_secs(5 * 60)), hard: Duration::from_secs(30 * 60) },
        )
        .with_context(|| format!("downloading {url}"))?;
        let got = sha256_file(&part)?;
        if got != ctx.pin.sha256 {
            let _ = fs::remove_file(&part);
            bail!("sha256 of {url} is {got}, kernel/metadata.rhai pins {}", ctx.pin.sha256);
        }
        fs::rename(&part, &tarball)?;
    }
    if ctx.src().join("Makefile").is_file() {
        eprintln!("[info] using the extracted {}", ctx.src().display());
        return Ok(());
    }
    let tmp = ctx.src_cache.join(format!(".linux-{version}.extracting"));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp)?;
    let mut cmd = Command::new("tar");
    cmd.arg("-xJf").arg(&tarball).arg("-C").arg(&tmp);
    quick(ctx, "fetch", "extract", cmd, 20)?;
    let _ = fs::remove_dir_all(ctx.src());
    fs::rename(tmp.join(format!("linux-{version}")), ctx.src()).context("moving the extracted source into place")?;
    let _ = fs::remove_dir_all(&tmp);
    Ok(())
}

fn parse_config(text: &str) -> HashMap<String, String> {
    text.lines()
        .filter(|l| l.starts_with("CONFIG_"))
        .filter_map(|l| l.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned())))
        .collect()
}

fn config(ctx: &Ctx) -> Result<()> {
    let variant = distro_contract::load(&ctx.root, &ctx.distro)?;
    let kconfig = kernel::kconfig_text(&variant)?;
    let build = ctx.build_dir();
    let _ = fs::remove_dir_all(&build);
    fs::create_dir_all(&build)?;
    let mut cmd = make(ctx);
    cmd.args(["-s", "x86_64_defconfig"]);
    quick(ctx, "config", "defconfig", cmd, 10)?;
    let mut file = fs::OpenOptions::new().append(true).open(build.join(".config"))?;
    writeln!(file, "{kconfig}")?;
    writeln!(file, "CONFIG_LOCALVERSION=\"{}\"", ctx.pin.localversion)?;
    drop(file);
    let mut cmd = make(ctx);
    cmd.args(["-s", "olddefconfig"]);
    quick(ctx, "config", "olddefconfig", cmd, 10)?;
    let have = parse_config(&fs::read_to_string(build.join(".config"))?);
    let mut dropped = Vec::new();
    for line in kconfig.lines().map(str::trim).filter(|l| l.starts_with("CONFIG_")) {
        let Some((name, want)) = line.split_once('=') else {
            bail!("kconfig line is not CONFIG_X=value: '{line}'");
        };
        let ok = match have.get(name) {
            Some(got) => got == want,
            None => want == "n",
        };
        if !ok {
            dropped.push(format!("{line} (got {})", have.get(name).map_or("unset", String::as_str)));
        }
    }
    if !dropped.is_empty() {
        bail!(
            "{} kconfig option(s) did not stick in {}: {}",
            dropped.len(),
            build.join(".config").display(),
            dropped.join(", ")
        );
    }
    Ok(())
}

fn build(ctx: &Ctx) -> Result<()> {
    super::enforce_build_hours(ctx.args.now)?;
    let jobs = ctx
        .args
        .jobs
        .unwrap_or_else(|| (std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2) / 2).max(1));
    let log = ctx.log("build");
    let _ = fs::remove_file(&log);
    eprintln!("[info] make -j{jobs}; heartbeat every 60 s, stall limit 10 min, hard limit 120 min");
    let mut cmd = make(ctx);
    cmd.arg(format!("-j{jobs}"));
    run_logged(cmd, Limits { label: "make", log: &log, watch: None, stall: Some(BUILD_STALL), hard: BUILD_LIMIT })?;
    let image = ctx.build_dir().join("arch/x86/boot/bzImage");
    if !image.is_file() {
        bail!("make succeeded but {} is missing", image.display());
    }
    Ok(())
}

fn package(ctx: &Ctx) -> Result<()> {
    let build = ctx.build_dir();
    let release_file = build.join("include/config/kernel.release");
    let release = fs::read_to_string(&release_file)
        .with_context(|| format!("reading {}", release_file.display()))?
        .trim()
        .to_owned();
    if release != ctx.pin.release {
        bail!("built kernel.release is '{release}', pinned '{}'", ctx.pin.release);
    }
    let _ = fs::remove_file(ctx.log("package"));
    let root = ctx.pkg();
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("staging/boot"))?;
    fs::create_dir_all(root.join("kernel-build/include/config"))?;
    fs::copy(build.join("arch/x86/boot/bzImage"), root.join("staging/boot/vmlinuz"))?;
    let mut cmd = make(ctx);
    cmd.arg(format!("INSTALL_MOD_PATH={}", root.join("staging").display())).arg("modules_install");
    quick(ctx, "package", "modules_install", cmd, 30)?;
    let modules = root.join("staging/usr/lib/modules");
    fs::create_dir_all(&modules)?;
    for entry in fs::read_dir(root.join("staging/lib/modules"))? {
        let entry = entry?;
        fs::rename(entry.path(), modules.join(entry.file_name()))?;
    }
    fs::remove_dir_all(root.join("staging/lib"))?;
    for link in ["build", "source"] {
        let _ = fs::remove_file(modules.join(&release).join(link));
    }
    fs::copy(&release_file, root.join(kernel::KERNEL_RELEASE_PATH))?;
    fs::copy(build.join(".config"), root.join("kernel-build/.config"))?;

    let dist = ctx.dist();
    let _ = fs::remove_dir_all(&dist);
    fs::create_dir_all(&dist)?;
    let tar = |from: &Path, out: &str, files: &[&str]| {
        let mut cmd = Command::new("tar");
        cmd.args(["--owner=0", "--group=0", "--numeric-owner", "-cJf"])
            .arg(dist.join(out))
            .current_dir(from)
            .args(files);
        quick(ctx, "package", "tar", cmd, 30)
    };
    tar(
        &root,
        &ctx.runtime_name(),
        &["kernel-build/include/config/kernel.release", "kernel-build/.config", "staging/boot/vmlinuz", "staging/usr/lib/modules"],
    )?;
    tar(&build, &ctx.debug_name(), &["System.map", ".config"])?;
    let mut sums = String::new();
    for name in [ctx.runtime_name(), ctx.debug_name()] {
        sums += &format!("{}  {name}\n", sha256_file(&dist.join(&name))?);
    }
    fs::write(dist.join(ctx.sums_name()), &sums)?;
    eprint!("{sums}");

    // The bundle rule of kernel/metadata.rhai: release, vmlinuz, modules dir.
    let found = fs::read_to_string(root.join(kernel::KERNEL_RELEASE_PATH))?;
    if found.trim() != ctx.pin.release {
        bail!("bundle kernel.release is '{}', pinned '{}'", found.trim(), ctx.pin.release);
    }
    if !root.join(kernel::KERNEL_IMAGE_PATH).is_file() {
        bail!("bundle kernel image is missing at {}", root.join(kernel::KERNEL_IMAGE_PATH).display());
    }
    let dir = root.join(kernel::KERNEL_MODULES_DIR).join(&ctx.pin.release);
    if !dir.is_dir() {
        bail!("bundle kernel modules are missing at {}", dir.display());
    }
    Ok(())
}

const INIT: &str = r#"#!/bin/sh
mount -t proc proc /proc; mount -t sysfs sys /sys; mount -t devtmpfs dev /dev
echo "BOOTTEST release=$(uname -r)"
for m in bridge 8021q tun veth nbd; do modprobe $m 2>&1 || echo "BOOTTEST modprobe-failed=$m"; done
echo "BOOTTEST modules=$(cat /proc/modules | cut -d' ' -f1 | tr '\n' ' ')"
echo "BOOTTEST preempt=$(cat /sys/kernel/debug/sched/preempt 2>/dev/null || uname -v)"
echo "BOOTTEST done"
poweroff -f
"#;

/// Destroys and undefines the libvirt domain when dropped.
struct Domain(String);

impl Domain {
    fn virsh(&self, args: &[&str]) -> bool {
        Command::new("timeout")
            .args(["60", "virsh", "-q", "-c", LIBVIRT])
            .args(args)
            .arg(&self.0)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    }
    fn cleanup(&self) {
        self.virsh(&["destroy"]);
        self.virsh(&["undefine"]);
    }
}

impl Drop for Domain {
    fn drop(&mut self) {
        self.cleanup();
    }
}

fn boot(ctx: &Ctx) -> Result<()> {
    let busybox = match &ctx.args.busybox {
        Some(path) => path.clone(),
        None => {
            let variant = distro_contract::load(&ctx.root, &ctx.distro)?;
            distro_builder::release::busybox(&variant).context(
                "no static busybox for the boot test: Recipe's busybox-static could not provide it; pass --busybox <path to a static busybox>",
            )?
        }
    };
    let w = ctx.dir.join("boot");
    let _ = fs::remove_dir_all(&w);
    let root = w.join("root");
    for d in ["bin", "proc", "sys", "dev", "lib"] {
        fs::create_dir_all(root.join(d))?;
    }
    fs::copy(&busybox, root.join("bin/busybox")).with_context(|| format!("copying busybox {}", busybox.display()))?;
    for applet in ["sh", "mount", "uname", "modprobe", "lsmod", "poweroff", "cat", "sleep", "echo", "cut", "tr"] {
        std::os::unix::fs::symlink("busybox", root.join("bin").join(applet))?;
    }
    let mut cp = Command::new("cp");
    cp.arg("-a").arg(ctx.pkg().join("staging/usr/lib/modules")).arg(root.join("lib/modules"));
    quick(ctx, "boot", "copy modules", cp, 5)?;
    fs::write(root.join("init"), INIT)?;
    fs::set_permissions(root.join("init"), std::os::unix::fs::PermissionsExt::from_mode(0o755))?;
    let initramfs = w.join("initramfs.gz");
    let mut pack = Command::new("sh");
    pack.args(["-c", r#"cd "$1" && find . | cpio -o -H newc --quiet | gzip -1 > "$2""#, "sh"])
        .arg(&root)
        .arg(&initramfs);
    quick(ctx, "boot", "initramfs", pack, 5)?;
    fs::copy(ctx.pkg().join(kernel::KERNEL_IMAGE_PATH), w.join("vmlinuz"))?;

    let name = format!("boottest-{}", ctx.distro);
    let domain = Domain(name.clone());
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| format!("/run/user/{}", unsafe { libc::getuid() }));
    let sock = Path::new(&runtime).join("levitate-vm").join(format!("{name}.sock"));
    fs::create_dir_all(sock.parent().unwrap())?;
    let _ = fs::remove_file(&sock);
    domain.cleanup();
    let xml = Command::new("timeout")
        .args(["120", "virt-install", "--print-xml", "--connect", LIBVIRT, "--name", &name, "--memory", "1024", "--vcpus", "2", "--boot"])
        .arg(format!(
            "kernel={},initrd={},kernel_args=console=ttyS0 panic=-1",
            w.join("vmlinuz").display(),
            initramfs.display()
        ))
        .args(["--disk", "none", "--network", "none", "--graphics", "none", "--osinfo", "linux2024", "--serial"])
        .arg(format!("unix,path={},mode=bind", sock.display()))
        .arg("--noreboot")
        .output()
        .context("running virt-install")?;
    if !xml.status.success() {
        bail!("virt-install --print-xml failed: {}", String::from_utf8_lossy(&xml.stderr).trim());
    }
    let xml_path = w.join("domain.xml");
    fs::write(&xml_path, &xml.stdout)?;
    let created = Command::new("timeout")
        .args(["60", "virsh", "-q", "-c", LIBVIRT, "create"])
        .arg(&xml_path)
        .arg("--paused")
        .output()
        .context("running virsh create")?;
    if !created.status.success() {
        bail!("virsh create failed: {}", String::from_utf8_lossy(&created.stderr).trim());
    }
    eprintln!("[info] domain {name} created paused; console socket {}", sock.display());

    let mut stream = None;
    let connect_by = Instant::now() + Duration::from_secs(15);
    while Instant::now() < connect_by {
        if let Ok(s) = UnixStream::connect(&sock) {
            stream = Some(s);
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let mut stream = stream.with_context(|| format!("could not connect to the console socket {}", sock.display()))?;
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    if !domain.virsh(&["resume"]) {
        bail!("virsh resume {name} failed");
    }
    let log_path = ctx.log("boot");
    let mut log = File::create(&log_path)?;
    let mut seen = Vec::new();
    let mut buf = [0u8; 65536];
    let (deadline, mut beat) = (Instant::now() + BOOT_LIMIT, Instant::now());
    let why;
    loop {
        if INTERRUPTED.load(Ordering::SeqCst) {
            why = Some("interrupted");
            break;
        }
        if Instant::now() >= deadline {
            why = Some("no 'BOOTTEST done' within 180 s");
            break;
        }
        if beat.elapsed() >= HEARTBEAT {
            beat = Instant::now();
            eprintln!("[..] boot {} KiB console so far", seen.len() / 1024);
        }
        match stream.read(&mut buf) {
            Ok(0) => {
                why = Some("the console closed before 'BOOTTEST done'");
                break;
            }
            Ok(n) => {
                log.write_all(&buf[..n])?;
                log.flush()?;
                seen.extend_from_slice(&buf[..n]);
            }
            Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {}
            Err(e) => bail!("reading the console: {e}"),
        }
        let text = String::from_utf8_lossy(&seen);
        if text.contains("BOOTTEST done") {
            why = None;
            break;
        }
        if text.contains("Kernel panic") {
            why = Some("kernel panic");
            break;
        }
    }
    drop(domain);
    let text = String::from_utf8_lossy(&seen).into_owned();
    for line in text.lines().filter(|l| l.contains("BOOTTEST")) {
        eprintln!("{}", line.trim_end());
    }
    let wanted = format!("BOOTTEST release={}", ctx.pin.release);
    if let Some(why) = why {
        bail!("boot test failed: {why}; last console: {}", tail_lines(&log_path, 3).join(" | "));
    }
    if !text.lines().any(|l| l.trim_end() == wanted) {
        bail!("boot test failed: the console has no '{wanted}' line");
    }
    let failed: Vec<&str> = text.lines().filter_map(|l| l.trim_end().strip_prefix("BOOTTEST modprobe-failed=")).collect();
    if !failed.is_empty() {
        bail!("boot test failed: modprobe failed for {}", failed.join(", "));
    }
    Ok(())
}

// ----------------------------------------------------------------- upload

struct Plan {
    repo: String,
    assets: Vec<PathBuf>,
    old_digest: String,
    new_digest: String,
    recipe: PathBuf,
}

/// The `distro` entry of `_asset` in linux-prebuilt.rhai: the text range of
/// its sha256 string, plus its repo and current digest.
fn asset_entry(text: &str, distro: &str) -> Result<(String, std::ops::Range<usize>)> {
    let from = text.find("fn _asset").context("linux-prebuilt.rhai has no _asset")?;
    let start = text[from..]
        .find(&format!("{distro}: #{{"))
        .map(|i| i + from)
        .with_context(|| format!("linux-prebuilt.rhai has no asset entry for {distro}"))?;
    let end = text[start..].find('}').map(|i| i + start).context("unterminated asset entry")?;
    let string_after = |key: &str| -> Result<std::ops::Range<usize>> {
        let k = text[start..end].find(key).map(|i| i + start + key.len()).with_context(|| format!("no {key} for {distro}"))?;
        let close = text[k..].find('"').map(|i| i + k).context("unterminated string")?;
        Ok(k..close)
    };
    let repo = string_after("repo: \"")?;
    Ok((text[repo].to_owned(), string_after("sha256: \"")?))
}

fn plan(ctx: &Ctx) -> Result<Plan> {
    let recipe = ctx.root.join(RECIPE);
    let text = fs::read_to_string(&recipe).with_context(|| format!("reading {}", recipe.display()))?;
    let (repo, digest) = asset_entry(&text, &ctx.distro)?;
    let dist = ctx.dist();
    let runtime = dist.join(ctx.runtime_name());
    if !runtime.is_file() {
        bail!("{} does not exist (run the package step)", runtime.display());
    }
    Ok(Plan {
        repo,
        assets: vec![runtime.clone(), dist.join(ctx.debug_name()), dist.join(ctx.sums_name())],
        old_digest: text[digest].to_owned(),
        new_digest: sha256_file(&runtime)?,
        recipe,
    })
}

fn print_upload_plan(ctx: &Ctx) -> Result<()> {
    let p = plan(ctx)?;
    eprintln!("\nRe-run with --upload (--from upload) to publish. It would:");
    eprintln!("  1. gh release download {TAG} -R {} the existing same-name assets into {}", p.repo, ctx.dir.join("old-assets").display());
    eprintln!("  2. gh release upload {TAG} --clobber -R {} with:", p.repo);
    for asset in &p.assets {
        eprintln!("       {}", asset.file_name().unwrap().to_string_lossy());
    }
    eprintln!("  3. edit {} ({}: sha256)", RECIPE, ctx.distro);
    eprintln!("       {} -> {}", p.old_digest, p.new_digest);
    Ok(())
}

fn upload(ctx: &Ctx) -> Result<()> {
    let p = plan(ctx)?;
    let old = ctx.dir.join("old-assets");
    fs::create_dir_all(&old)?;
    for asset in &p.assets {
        let name = asset.file_name().unwrap().to_string_lossy().into_owned();
        let mut cmd = Command::new("gh");
        cmd.args(["release", "download", TAG, "-R", &p.repo, "-p", &name, "--clobber", "-D"]).arg(&old);
        if quick(ctx, "upload", "gh download", cmd, 10).is_err() {
            eprintln!("[info] no existing {name} to keep in old-assets/");
        }
    }
    let mut cmd = Command::new("gh");
    cmd.args(["release", "upload", TAG, "--clobber", "-R", &p.repo]).args(&p.assets);
    quick(ctx, "upload", "gh upload", cmd, 30)?;
    let text = fs::read_to_string(&p.recipe)?;
    let (_, digest) = asset_entry(&text, &ctx.distro)?;
    let mut edited = text.clone();
    edited.replace_range(digest, &p.new_digest);
    fs::write(&p.recipe, edited)?;
    eprintln!("[info] {RECIPE}: {} sha256 {} -> {}", ctx.distro, p.old_digest, p.new_digest);
    eprintln!("\nNot done here (yours):");
    eprintln!("  - tools/recipe/tests/e2e/kernel_acquisition.rs hard-codes the AcornOS digest; update it.");
    eprintln!("Commit (no branches; submodule first):");
    eprintln!(
        "  git -C distro/builder commit -m 'Publish the {} {} kernel' -- recipes/linux-prebuilt.rhai && git -C distro/builder push origin main",
        ctx.distro, ctx.pin.release
    );
    eprintln!("  git commit -m 'Point distro/builder at the {} {} kernel' -- distro/builder && git push origin master", ctx.distro, ctx.pin.release);
    Ok(())
}
