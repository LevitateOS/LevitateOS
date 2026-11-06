//! Run a program in a pseudo-terminal, type into it and read its screen.
//!
//! xtask is the terminal here: it decides the size and `TERM`, writes keys to the
//! PTY master (the program's stdin) and feeds everything the program prints into a
//! vt100 screen. Waits are on the screen going quiet, not on fixed timers.

use anyhow::{Context, Result, anyhow};
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use std::env;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread;
use std::time::{Duration, Instant};

pub struct PtySession {
    label: String,
    master: Box<dyn MasterPty + Send>,
    writer: Box<dyn Write + Send>,
    child: Box<dyn Child + Send + Sync>,
    output: Receiver<Vec<u8>>,
    parser: vt100::Parser,
    transcript: Vec<u8>,
    exit: Option<portable_pty::ExitStatus>,
}

/// How a wait ended.
pub enum Settled {
    /// No output for the quiet period.
    Quiet,
    /// The program exited.
    Exited(portable_pty::ExitStatus),
}

impl PtySession {
    /// Starts `argv` in `cwd` on a `columns`×`rows` terminal, with `env` added.
    pub fn spawn(
        label: &str,
        cwd: &Path,
        argv: &[String],
        env_vars: &[(String, String)],
        columns: u16,
        rows: u16,
    ) -> Result<Self> {
        let pair = native_pty_system()
            .openpty(PtySize {
                rows,
                cols: columns,
                pixel_width: 0,
                pixel_height: 0,
            })
            .with_context(|| format!("{label}: open PTY"))?;
        let (program, args) = argv
            .split_first()
            .ok_or_else(|| anyhow!("{label}: empty command"))?;
        let mut cmd = CommandBuilder::new(program);
        cmd.args(args);
        cmd.cwd(cwd);
        apply_terminal_env(&mut cmd);
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        let child = pair
            .slave
            .spawn_command(cmd)
            .with_context(|| format!("{label}: spawn {program}"))?;
        drop(pair.slave);

        let mut reader = pair
            .master
            .try_clone_reader()
            .with_context(|| format!("{label}: clone PTY reader"))?;
        let writer = pair
            .master
            .take_writer()
            .with_context(|| format!("{label}: take PTY writer"))?;
        let (sender, output) = channel();
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            while let Ok(n) = reader.read(&mut buf) {
                if n == 0 || sender.send(buf[..n].to_vec()).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            label: label.to_string(),
            master: pair.master,
            writer,
            child,
            output,
            parser: vt100::Parser::new(rows, columns, 0),
            transcript: Vec::new(),
            exit: None,
        })
    }

    /// Reads output until nothing arrives for `quiet`, the program exits, or `timeout` passes.
    pub fn settle(&mut self, quiet: Duration, timeout: Duration) -> Result<Settled> {
        let deadline = Instant::now() + timeout;
        let mut seen_output = false;
        loop {
            if let Some(status) = &self.exit {
                return Ok(Settled::Exited(status.clone()));
            }
            let now = Instant::now();
            if now >= deadline {
                return Err(anyhow!(
                    "{}: still drawing after {}s; the screen never went quiet",
                    self.label,
                    timeout.as_secs()
                ));
            }
            match self.output.recv_timeout(quiet.min(deadline - now)) {
                Ok(chunk) => {
                    seen_output = true;
                    self.parser.process(&chunk);
                    self.transcript.extend_from_slice(&chunk);
                }
                Err(RecvTimeoutError::Timeout) => {
                    if let Some(status) = self.poll_exit()? {
                        return Ok(Settled::Exited(status));
                    }
                    if seen_output {
                        return Ok(Settled::Quiet);
                    }
                }
                Err(RecvTimeoutError::Disconnected) => {
                    let status = self
                        .child
                        .wait()
                        .with_context(|| format!("{}: wait for exit", self.label))?;
                    self.exit = Some(status.clone());
                    return Ok(Settled::Exited(status));
                }
            }
        }
    }

    fn poll_exit(&mut self) -> Result<Option<portable_pty::ExitStatus>> {
        if self.exit.is_none() {
            self.exit = self
                .child
                .try_wait()
                .with_context(|| format!("{}: poll child", self.label))?;
        }
        Ok(self.exit.clone())
    }

    /// Types one key (one write, as a terminal sends it).
    pub fn send_key(&mut self, key: &[u8]) -> Result<()> {
        self.writer
            .write_all(key)
            .and_then(|()| self.writer.flush())
            .with_context(|| format!("{}: write key", self.label))
    }

    pub fn screen(&self) -> &vt100::Screen {
        self.parser.screen()
    }

    pub fn transcript(&self) -> &[u8] {
        &self.transcript
    }

    /// Kills the program's process group if it is still running.
    pub fn finish(mut self) -> Result<()> {
        if self.poll_exit()?.is_none() {
            kill_process_group(self.master.as_ref());
            self.child
                .kill()
                .or_else(|e| match self.child.try_wait() {
                    Ok(Some(_)) => Ok(()),
                    _ => Err(e),
                })
                .with_context(|| format!("{}: kill", self.label))?;
            self.child
                .wait()
                .with_context(|| format!("{}: wait after kill", self.label))?;
        }
        Ok(())
    }
}

/// The screen as plain text, trailing spaces trimmed per line.
pub fn plain_text(screen: &vt100::Screen) -> String {
    screen
        .contents()
        .lines()
        .map(|line| line.trim_end_matches(' '))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Splits printf-style input (`\n`, `\r`, `\t`, `\e`, `\x1b`, `\033`, `\\`) into keys:
/// one escape sequence or one character per key.
pub fn parse_keys(spec: &str) -> Result<Vec<Vec<u8>>> {
    let mut bytes = Vec::new();
    let mut chars = spec.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        match chars.next() {
            Some('n') => bytes.push(b'\n'),
            Some('r') => bytes.push(b'\r'),
            Some('t') => bytes.push(b'\t'),
            Some('e') => bytes.push(0x1b),
            Some('\\') => bytes.push(b'\\'),
            Some('x') => {
                let hex: String = (0..2).filter_map(|_| chars.next()).collect();
                bytes.push(u8::from_str_radix(&hex, 16).with_context(|| format!("bad \\x{hex}"))?);
            }
            Some('0') => {
                let oct: String = (0..2).filter_map(|_| chars.next()).collect();
                bytes.push(u8::from_str_radix(&oct, 8).with_context(|| format!("bad \\0{oct}"))?);
            }
            other => return Err(anyhow!("unsupported escape \\{}", other.unwrap_or(' '))),
        }
    }

    let mut keys = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        if bytes[i] == 0x1b && matches!(bytes.get(i + 1), Some(b'[') | Some(b'O')) {
            i += 2;
            while i < bytes.len() && !(0x40..=0x7e).contains(&bytes[i]) {
                i += 1;
            }
            i += 1;
        } else {
            let len = match bytes[i] {
                b if b >= 0xf0 => 4,
                b if b >= 0xe0 => 3,
                b if b >= 0xc0 => 2,
                _ => 1,
            };
            i += len;
        }
        keys.push(bytes[start..i.min(bytes.len())].to_vec());
    }
    Ok(keys)
}

#[cfg(unix)]
fn kill_process_group(master: &dyn MasterPty) {
    if let Some(pgid) = master.process_group_leader() {
        // SAFETY: kill(2) with a negative pid signals that process group; no memory is touched.
        unsafe {
            libc::kill(-pgid, libc::SIGKILL);
        }
    }
}

#[cfg(not(unix))]
fn kill_process_group(_: &dyn MasterPty) {}

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
