use anyhow::{Context, Result, anyhow, bail};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

pub struct ShellCapture<'a> {
    pub label: &'a str,
    pub cwd: &'a Path,
    pub command: &'a str,
    pub columns: u16,
    pub rows: u16,
    pub seconds: u64,
    pub transcript_path: &'a Path,
}

pub fn capture_shell_transcript(request: ShellCapture<'_>) -> Result<()> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: request.rows.max(1),
            cols: request.columns.max(1),
            pixel_width: 0,
            pixel_height: 0,
        })
        .with_context(|| format!("{}: open PTY", request.label))?;

    let mut cmd = CommandBuilder::new("/bin/sh");
    cmd.arg("-lc");
    cmd.arg(request.command);
    cmd.cwd(request.cwd);
    apply_terminal_env(&mut cmd);

    let mut child = pair
        .slave
        .spawn_command(cmd)
        .with_context(|| format!("{}: spawn PTY command", request.label))?;
    drop(pair.slave);

    let master = pair.master;
    let mut reader = master
        .try_clone_reader()
        .with_context(|| format!("{}: clone PTY reader", request.label))?;
    let reader_label = request.label.to_string();
    let reader_handle = thread::spawn(move || -> Result<Vec<u8>> {
        let mut transcript = Vec::new();
        reader
            .read_to_end(&mut transcript)
            .with_context(|| format!("{reader_label}: read PTY transcript"))?;
        Ok(transcript)
    });

    let deadline = Instant::now() + Duration::from_secs(request.seconds.max(1));
    let mut timed_out = false;
    let exit_status = loop {
        if let Some(status) = child
            .try_wait()
            .with_context(|| format!("{}: poll PTY child", request.label))?
        {
            break status;
        }

        if Instant::now() >= deadline {
            timed_out = true;
            kill_pty_process_group(master.as_ref());
            let _ = child.kill();
            break child
                .wait()
                .with_context(|| format!("{}: wait for killed PTY child", request.label))?;
        }

        thread::sleep(Duration::from_millis(50));
    };

    drop(master);

    let transcript = reader_handle
        .join()
        .map_err(|_| anyhow!("{}: PTY reader thread panicked", request.label))??;
    fs::write(request.transcript_path, &transcript).with_context(|| {
        format!(
            "{}: write transcript '{}'",
            request.label,
            request.transcript_path.display()
        )
    })?;

    if !timed_out && !exit_status.success() {
        bail!(
            "{}: command exited before capture window completed ({exit_status}). Transcript: '{}'",
            request.label,
            request.transcript_path.display()
        );
    }

    Ok(())
}

#[cfg(unix)]
fn kill_pty_process_group(master: &dyn portable_pty::MasterPty) {
    if let Some(pgid) = master.process_group_leader() {
        unsafe {
            libc::kill(-pgid, libc::SIGKILL);
        }
    }
}

#[cfg(not(unix))]
fn kill_pty_process_group(_: &dyn portable_pty::MasterPty) {}

fn apply_terminal_env(cmd: &mut CommandBuilder) {
    let mut term = env::var("TERM").unwrap_or_else(|_| "xterm-256color".to_string());
    let normalized = term.trim().to_ascii_lowercase();
    if normalized.is_empty()
        || normalized == "dumb"
        || normalized == "vt100"
        || normalized == "vt102"
        || normalized == "linux"
    {
        term = "xterm-256color".to_string();
    }

    cmd.env("TERM", term);
    let colorterm = env::var("COLORTERM").unwrap_or_else(|_| "truecolor".to_string());
    cmd.env("COLORTERM", colorterm);
    let force_color = env::var("FORCE_COLOR").unwrap_or_else(|_| "3".to_string());
    cmd.env("FORCE_COLOR", force_color);
    cmd.env_remove("NO_COLOR");
}
