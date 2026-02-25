use anyhow::{Context, Result, bail};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Options {
    pub slugs: Vec<String>,
    pub columns: u16,
    pub rows: u16,
    pub seconds: u64,
    pub out_dir: Option<PathBuf>,
    pub stdout: bool,
    pub ansi: bool,
    pub keep_transcript: bool,
}

pub fn run(options: Options) -> Result<()> {
    ensure_required_tools()?;

    let root = crate::util::repo::repo_root()?;
    let docs_tui_dir = root.join("tui/apps/s02-live-tools/install-docs");
    if !docs_tui_dir.is_dir() {
        bail!(
            "docs inspect: missing install-docs directory at '{}'",
            docs_tui_dir.display()
        );
    }

    let slugs = resolve_slugs(&root, &options)?;
    if slugs.is_empty() {
        bail!("docs inspect: no slugs resolved");
    }
    validate_slugs(&slugs)?;

    let seconds = if options.seconds == 0 {
        1
    } else {
        options.seconds
    };
    let columns = options.columns.max(1);
    let rows = options.rows.max(1);
    let run_dir = create_run_dir(&root, options.out_dir.as_deref())?;

    let mut manifest = String::new();
    manifest.push_str(&format!("run_dir={}\n", run_dir.display()));
    manifest.push_str(&format!(
        "viewport={}x{} duration={}s\n",
        columns, rows, seconds
    ));
    manifest.push_str(&format!("total_slugs={}\n", slugs.len()));

    for (index, slug) in slugs.iter().enumerate() {
        eprintln!("[docs.inspect] {}/{} slug={}", index + 1, slugs.len(), slug);

        let transcript_path = transcript_path(
            &run_dir,
            index + 1,
            slug,
            options.keep_transcript,
            options.columns,
            options.rows,
        );

        render_slug_transcript(
            &docs_tui_dir,
            slug,
            columns,
            rows,
            seconds,
            &transcript_path,
        )?;
        let transcript = fs::read(&transcript_path)
            .with_context(|| format!("docs inspect: reading '{}'", transcript_path.display()))?;
        let plain = parse_plain_screen(&transcript, rows, columns);

        let plain_path = run_dir.join(format!("{:03}-{slug}.txt", index + 1));
        fs::write(&plain_path, plain)
            .with_context(|| format!("docs inspect: writing '{}'", plain_path.display()))?;

        let ansi_path = run_dir.join(format!("{:03}-{slug}.ansi", index + 1));
        if options.ansi {
            let formatted = parse_formatted_screen(&transcript, rows, columns);
            fs::write(&ansi_path, formatted)
                .with_context(|| format!("docs inspect: writing '{}'", ansi_path.display()))?;
        }

        if !options.keep_transcript {
            let _ = fs::remove_file(&transcript_path);
        }

        manifest.push_str(&format!(
            "{} => {}\n",
            slug,
            plain_path
                .strip_prefix(&root)
                .unwrap_or(&plain_path)
                .display()
        ));
        if options.ansi {
            manifest.push_str(&format!(
                "{} (ansi) => {}\n",
                slug,
                ansi_path
                    .strip_prefix(&root)
                    .unwrap_or(&ansi_path)
                    .display()
            ));
        }

        if options.stdout {
            println!("===== {} ({}/{}) =====", slug, index + 1, slugs.len());
            let rendered = fs::read_to_string(&plain_path)
                .with_context(|| format!("docs inspect: reading '{}'", plain_path.display()))?;
            println!("{rendered}");
        }
    }

    let manifest_path = run_dir.join("manifest.txt");
    fs::write(&manifest_path, manifest)
        .with_context(|| format!("docs inspect: writing '{}'", manifest_path.display()))?;

    println!(
        "docs inspect: wrote {} page snapshot(s) to {}",
        slugs.len(),
        run_dir.display()
    );
    Ok(())
}

fn ensure_required_tools() -> Result<()> {
    for tool in ["bun", "script", "timeout"] {
        if which::which(tool).is_err() {
            bail!(
                "docs inspect: required tool '{}' not found in PATH. Install it and retry.",
                tool
            );
        }
    }
    Ok(())
}

fn resolve_slugs(root: &Path, options: &Options) -> Result<Vec<String>> {
    if !options.slugs.is_empty() {
        return Ok(options.slugs.clone());
    }

    let generated = root.join("docs/content/src/generated/index.ts");
    let raw = fs::read_to_string(&generated).with_context(|| {
        format!(
            "docs inspect: reading docs nav from '{}'",
            generated.display()
        )
    })?;
    let href_re = Regex::new(r#""href"\s*:\s*"/docs/([a-z0-9][a-z0-9\-]*)""#)
        .context("docs inspect: compiling docs href regex")?;

    let mut slugs = Vec::new();
    let mut seen = HashSet::new();
    for captures in href_re.captures_iter(&raw) {
        if let Some(found) = captures.get(1).map(|m| m.as_str()) {
            let slug = found.to_string();
            if seen.insert(slug.clone()) {
                slugs.push(slug);
            }
        }
    }
    Ok(slugs)
}

fn validate_slugs(slugs: &[String]) -> Result<()> {
    let slug_re =
        Regex::new(r"^[a-z0-9][a-z0-9-]*$").context("docs inspect: compiling slug regex")?;
    for slug in slugs {
        if !slug_re.is_match(slug) {
            bail!(
                "docs inspect: invalid slug '{}'. Allowed: lowercase letters, digits, and '-'",
                slug
            );
        }
    }
    Ok(())
}

fn create_run_dir(root: &Path, out_dir: Option<&Path>) -> Result<PathBuf> {
    let base = out_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| root.join(".artifacts/out/tui/install-docs-inspect"));
    let run_dir = base.join(format!("run-{}", unix_timestamp()));
    fs::create_dir_all(&run_dir)
        .with_context(|| format!("docs inspect: creating output dir '{}'", run_dir.display()))?;
    Ok(run_dir)
}

fn transcript_path(
    run_dir: &Path,
    index: usize,
    slug: &str,
    keep_transcript: bool,
    columns: u16,
    rows: u16,
) -> PathBuf {
    if keep_transcript {
        return run_dir.join(format!("{index:03}-{slug}.transcript"));
    }

    std::env::temp_dir().join(format!(
        "levitate-docs-inspect-{}-{}-{}x{}-{slug}.log",
        std::process::id(),
        index,
        columns,
        rows
    ))
}

fn render_slug_transcript(
    docs_tui_dir: &Path,
    slug: &str,
    columns: u16,
    rows: u16,
    seconds: u64,
    transcript_path: &Path,
) -> Result<()> {
    let command = format!(
        "unset NO_COLOR; export FORCE_COLOR=3; export TERM=xterm-256color; stty rows {rows} cols {columns}; timeout {seconds} bun src/main.ts --slug {slug}"
    );

    let status = Command::new("script")
        .arg("-q")
        .arg("-c")
        .arg(&command)
        .arg(transcript_path)
        .current_dir(docs_tui_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| {
            format!(
                "docs inspect: running script capture for slug '{}' in '{}'",
                slug,
                docs_tui_dir.display()
            )
        })?;
    if !status.success() {
        bail!(
            "docs inspect: script capture failed for slug '{}', exit={status}",
            slug
        );
    }
    Ok(())
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
    use super::{
        find_subslice, parse_formatted_screen, strip_script_bookends, trim_line_trailing_spaces,
    };

    #[test]
    fn strip_script_bookends_removes_script_headers_and_footers() {
        let input = b"Script started on 2026-02-23 [COMMAND=\"foo\"]\n\
\x1b[?25lrow-1\r\nrow-2\r\n\
Script done on 2026-02-23 [COMMAND_EXIT_CODE=\"0\"]\n";

        let cleaned = strip_script_bookends(input);
        assert_eq!(cleaned, b"\x1b[?25lrow-1\r\nrow-2\r");
    }

    #[test]
    fn strip_script_bookends_preserves_carriage_returns_without_footer() {
        let input = b"Script started on 2026-02-23\n\x1b[Hline-a\rline-b\r\n";
        let cleaned = strip_script_bookends(input);
        assert_eq!(cleaned, b"\x1b[Hline-a\rline-b\r\n");
    }

    #[test]
    fn find_subslice_matches_expected_index() {
        let haystack = b"alpha\nScript done on omega";
        let needle = b"\nScript done on ";
        assert_eq!(find_subslice(haystack, needle), Some(5));
    }

    #[test]
    fn parse_formatted_screen_preserves_ansi_styles() {
        let input = b"\x1b[41mA\x1b[0m\r\n";
        let formatted = parse_formatted_screen(input, 2, 2);
        assert!(formatted.windows(2).any(|window| window == b"\x1b["));
        assert!(formatted.contains(&b'A'));
    }

    #[test]
    fn trim_line_trailing_spaces_removes_only_right_padding() {
        let input = "A  \n| x |   \n";
        let trimmed = trim_line_trailing_spaces(input);
        assert_eq!(trimmed, "A\n| x |");
    }
}
