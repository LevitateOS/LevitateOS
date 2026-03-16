use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Options {
    pub app: Option<crate::cli::TuiInspectApp>,
    pub cwd: Option<PathBuf>,
    pub command: Option<String>,
    pub input: Option<String>,
    pub input_delay_seconds: u64,
    pub columns: u16,
    pub rows: u16,
    pub seconds: u64,
    pub out_dir: Option<PathBuf>,
    pub stdout: bool,
    pub ansi: bool,
    pub keep_transcript: bool,
}

#[derive(Debug, Clone)]
struct Target {
    name: String,
    cwd: PathBuf,
    command: String,
}

pub fn run(options: Options) -> Result<()> {
    ensure_required_tools()?;

    let root = crate::util::repo::repo_root()?;
    let target = resolve_target(&root, &options)?;
    if !target.cwd.is_dir() {
        bail!(
            "tui inspect: target cwd does not exist: '{}'",
            target.cwd.display()
        );
    }

    let seconds = if options.seconds == 0 {
        1
    } else {
        options.seconds
    };
    let columns = options.columns.max(1);
    let rows = options.rows.max(1);
    let run_dir = create_run_dir(&root, options.out_dir.as_deref())?;
    let transcript_path = transcript_path(
        &run_dir,
        &target.name,
        options.keep_transcript,
        options.columns,
        options.rows,
    );

    eprintln!(
        "[tui.inspect] target={} cwd={}",
        target.name,
        target.cwd.display()
    );
    render_transcript(
        &target,
        &options.input,
        options.input_delay_seconds,
        columns,
        rows,
        seconds,
        &transcript_path,
    )?;

    let transcript = fs::read(&transcript_path)
        .with_context(|| format!("tui inspect: reading '{}'", transcript_path.display()))?;
    let plain = parse_plain_screen(&transcript, rows, columns);
    let plain_path = run_dir.join(format!("001-{}.txt", target.name));
    fs::write(&plain_path, plain)
        .with_context(|| format!("tui inspect: writing '{}'", plain_path.display()))?;

    let ansi_path = run_dir.join(format!("001-{}.ansi", target.name));
    if options.ansi {
        let formatted = parse_formatted_screen(&transcript, rows, columns);
        fs::write(&ansi_path, formatted)
            .with_context(|| format!("tui inspect: writing '{}'", ansi_path.display()))?;
    }

    if !options.keep_transcript {
        let _ = fs::remove_file(&transcript_path);
    }

    let manifest = build_manifest(
        &root,
        &run_dir,
        &target,
        columns,
        rows,
        seconds,
        &options,
        &plain_path,
        if options.ansi { Some(&ansi_path) } else { None },
    );
    let manifest_path = run_dir.join("manifest.txt");
    fs::write(&manifest_path, manifest)
        .with_context(|| format!("tui inspect: writing '{}'", manifest_path.display()))?;

    if options.stdout {
        let rendered = fs::read_to_string(&plain_path)
            .with_context(|| format!("tui inspect: reading '{}'", plain_path.display()))?;
        println!("{rendered}");
    }

    println!("tui inspect: wrote snapshot(s) to {}", run_dir.display());
    Ok(())
}

fn resolve_target(root: &Path, options: &Options) -> Result<Target> {
    if let Some(app) = options.app {
        if options.cwd.is_some() || options.command.is_some() {
            bail!("tui inspect: --app cannot be combined with --cwd/--command");
        }
        return Ok(match app {
            crate::cli::TuiInspectApp::InstallDocs => Target {
                name: "install-docs".to_string(),
                cwd: root.join("tui/apps/live-tools/install-docs"),
                command: "bun src/main.ts".to_string(),
            },
            crate::cli::TuiInspectApp::S03DiskPlan => Target {
                name: "s03-disk-plan".to_string(),
                cwd: root.join("tui/apps/s03-install/disk-plan"),
                command: "bun run start -- --disk /dev/sda".to_string(),
            },
        });
    }

    let cwd = options
        .cwd
        .as_ref()
        .map(|path| {
            if path.is_absolute() {
                path.clone()
            } else {
                root.join(path)
            }
        })
        .context("tui inspect: missing --cwd when --app is not used")?;
    let command = options
        .command
        .as_ref()
        .cloned()
        .context("tui inspect: missing --command when --app is not used")?;

    Ok(Target {
        name: "custom".to_string(),
        cwd,
        command,
    })
}

fn build_manifest(
    root: &Path,
    run_dir: &Path,
    target: &Target,
    columns: u16,
    rows: u16,
    seconds: u64,
    options: &Options,
    plain_path: &Path,
    ansi_path: Option<&Path>,
) -> String {
    let mut manifest = String::new();
    manifest.push_str(&format!("run_dir={}\n", run_dir.display()));
    manifest.push_str(&format!("target={}\n", target.name));
    manifest.push_str(&format!("cwd={}\n", target.cwd.display()));
    manifest.push_str(&format!("command={}\n", target.command));
    manifest.push_str(&format!(
        "viewport={}x{} duration={}s\n",
        columns, rows, seconds
    ));
    manifest.push_str(&format!(
        "input={}\n",
        options.input.as_deref().unwrap_or("(none)")
    ));
    manifest.push_str(&format!(
        "input_delay_seconds={}\n",
        options.input_delay_seconds
    ));
    manifest.push_str(&format!(
        "plain={}\n",
        plain_path
            .strip_prefix(root)
            .unwrap_or(plain_path)
            .display()
    ));
    if let Some(path) = ansi_path {
        manifest.push_str(&format!(
            "ansi={}\n",
            path.strip_prefix(root).unwrap_or(path).display()
        ));
    }
    manifest
}

fn ensure_required_tools() -> Result<()> {
    for tool in ["script", "timeout"] {
        if which::which(tool).is_err() {
            bail!(
                "tui inspect: required tool '{}' not found in PATH. Install it and retry.",
                tool
            );
        }
    }
    Ok(())
}

fn create_run_dir(root: &Path, out_dir: Option<&Path>) -> Result<PathBuf> {
    let base = out_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| root.join(".artifacts/out/tui/inspect"));
    let run_dir = base.join(format!("run-{}", unix_timestamp()));
    fs::create_dir_all(&run_dir)
        .with_context(|| format!("tui inspect: creating output dir '{}'", run_dir.display()))?;
    Ok(run_dir)
}

fn transcript_path(
    run_dir: &Path,
    name: &str,
    keep_transcript: bool,
    columns: u16,
    rows: u16,
) -> PathBuf {
    if keep_transcript {
        return run_dir.join(format!("001-{name}.transcript"));
    }

    std::env::temp_dir().join(format!(
        "levitate-tui-inspect-{}-{}x{}-{name}.log",
        std::process::id(),
        columns,
        rows
    ))
}

fn render_transcript(
    target: &Target,
    input: &Option<String>,
    input_delay_seconds: u64,
    columns: u16,
    rows: u16,
    seconds: u64,
    transcript_path: &Path,
) -> Result<()> {
    let base = format!(
        "unset NO_COLOR; export FORCE_COLOR=3; export TERM=xterm-256color; stty rows {rows} cols {columns}; timeout {seconds} {}",
        target.command
    );
    let command = if let Some(sequence) = input {
        let escaped = sh_single_quote(sequence);
        format!(
            "( sleep {input_delay_seconds}; printf '%b' '{escaped}' > /dev/tty; sleep 1; printf '%b' '{escaped}' > /dev/tty ) & unset NO_COLOR; export FORCE_COLOR=3; export TERM=xterm-256color; stty rows {rows} cols {columns}; timeout {seconds} {}",
            target.command
        )
    } else {
        base
    };

    let status = Command::new("script")
        .arg("-q")
        .arg("-c")
        .arg(&command)
        .arg(transcript_path)
        .current_dir(&target.cwd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| {
            format!(
                "tui inspect: running script capture for '{}' in '{}'",
                target.name,
                target.cwd.display()
            )
        })?;
    if !status.success() {
        bail!(
            "tui inspect: script capture failed for '{}' (exit={status})",
            target.name
        );
    }
    Ok(())
}

fn sh_single_quote(input: &str) -> String {
    input.replace('\'', r#"'\''"#)
}

fn parse_plain_screen(transcript: &[u8], rows: u16, columns: u16) -> String {
    let parser = parse_screen(transcript, rows, columns);
    trim_line_trailing_spaces(&parser.screen().contents())
}

fn parse_formatted_screen(transcript: &[u8], rows: u16, columns: u16) -> Vec<u8> {
    let parser = parse_screen(transcript, rows, columns);
    parser.screen().contents_formatted()
}

fn parse_screen(transcript: &[u8], rows: u16, columns: u16) -> vt100::Parser {
    let sanitized = strip_script_bookends(transcript);
    let mut parser = vt100::Parser::new(rows, columns, 0);
    parser.process(&sanitized);
    parser
}

fn trim_line_trailing_spaces(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end_matches(' '))
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_script_bookends(transcript: &[u8]) -> Vec<u8> {
    let mut start = 0usize;
    let mut end = transcript.len();

    if transcript.starts_with(b"Script started on ") {
        if let Some(idx) = transcript.iter().position(|byte| *byte == b'\n') {
            start = idx + 1;
        } else {
            return Vec::new();
        }
    }

    if let Some(done_idx) = find_subslice(&transcript[start..end], b"\nScript done on ") {
        end = start + done_idx;
    } else if transcript[start..end].starts_with(b"Script done on ") {
        end = start;
    }

    transcript[start..end].to_vec()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{sh_single_quote, strip_script_bookends, trim_line_trailing_spaces};

    #[test]
    fn strip_script_bookends_removes_headers() {
        let input =
            b"Script started on 2026-02-25\n\x1b[Halpha\r\nbeta\r\nScript done on 2026-02-25\n";
        let cleaned = strip_script_bookends(input);
        assert_eq!(cleaned, b"\x1b[Halpha\r\nbeta\r");
    }

    #[test]
    fn trim_line_trailing_spaces_removes_only_right_padding() {
        let input = "A  \n| x |   \n";
        let trimmed = trim_line_trailing_spaces(input);
        assert_eq!(trimmed, "A\n| x |");
    }

    #[test]
    fn sh_single_quote_escapes_single_quotes() {
        assert_eq!(sh_single_quote("a'b"), "a'\\''b");
    }
}
