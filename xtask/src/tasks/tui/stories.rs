//! `stories`: a local web page of live terminals, one per story, each running the real
//! app in a real PTY.
//!
//! The catalog comes from the app manifests (each app's start screen, its
//! `inspect.stories` and its pages) and from each variant's `[install_ux]` (the
//! composed install screen: the real split pane with the shell and the docs). A tile
//! on the page is xterm.js over a WebSocket: the server starts the story with the
//! same environment `inspect` and `check tui` use, types the story's keys as the
//! screen settles, then passes the browser's keys through. Nothing here draws UI of
//! its own.
//!
//! It listens on every address; each request needs the token in the URL it prints.

use anyhow::{Context, Result, bail};
use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};
use tungstenite::{Message, WebSocket};

use super::inspect::Launch;
use crate::util::pty_capture::parse_keys;
use distro_contract::InstallDocs;
use distro_contract::tui_apps;

const PAGE: &str = include_str!("stories.html");
/// How long the screen must be quiet before the next story key is typed.
const KEY_SETTLE: Duration = Duration::from_millis(300);

pub struct Options {
    pub port: u16,
    pub open: bool,
    /// Instead of serving, record every story and write one self-contained page here.
    pub export: Option<PathBuf>,
}

/// One entry in the catalog: a program to start in a PTY and the keys that reach the story.
struct Story {
    group: String,
    name: String,
    cwd: PathBuf,
    argv: Vec<String>,
    env: Vec<(String, String)>,
    keys: Vec<Vec<u8>>,
}

pub fn run(options: Options) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let catalog = Arc::new(catalog(&root)?);
    let xterm = root.join("node_modules/@xterm/xterm");
    if !xterm.join("lib/xterm.js").is_file() {
        bail!("xterm.js is missing from node_modules (bun install at the repo root)");
    }
    if let Some(path) = &options.export {
        return export(&catalog, &xterm, path);
    }
    // Any machine on the network can reach the port, and a tile can run a shell, so
    // every request must carry this token; the printed URL has it.
    let token: Arc<str> = random_token()?.into();
    let listener = TcpListener::bind(("0.0.0.0", options.port))
        .with_context(|| format!("listening on port {}", options.port))?;
    // The machine's IPv4 addresses: other machines may not resolve its hostname.
    let addresses = std::process::Command::new("hostname")
        .arg("-I")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).into_owned())
        .unwrap_or_default();
    let mut urls: Vec<String> = addresses
        .split_whitespace()
        .filter(|address| address.parse::<std::net::Ipv4Addr>().is_ok())
        .map(|address| format!("http://{address}:{}/?token={token}", options.port))
        .collect();
    if urls.is_empty() {
        urls.push(format!("http://127.0.0.1:{}/?token={token}", options.port));
    }
    println!(
        "stories: {} stories (Ctrl-C stops), open one of:",
        catalog.len()
    );
    for url in &urls {
        println!("  {url}");
    }
    let url = &urls[0];
    if options.open {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let catalog = Arc::clone(&catalog);
        let xterm = xterm.clone();
        let token = Arc::clone(&token);
        thread::spawn(move || {
            if let Err(error) = serve(stream, &catalog, &xterm, &token) {
                eprintln!("stories: {error:#}");
            }
        });
    }
    Ok(())
}

/// The size a story opens at: the install screen on a serial console, the rest roomy.
fn default_size(story: &Story) -> (u16, u16) {
    if story.group == "install screen" {
        (80, 24)
    } else {
        let size = super::inspect::DEFAULT_SIZE;
        (size.columns, size.rows)
    }
}

/// Records every story (the real app in a PTY, its keys typed as the screen settles) and
/// writes one page that replays the recordings in xterm.js, with nothing to serve.
fn export(catalog: &[Story], xterm: &Path, path: &Path) -> Result<()> {
    let wait = super::inspect::DEFAULT_WAIT;
    let mut recorded = Vec::new();
    for (index, story) in catalog.iter().enumerate() {
        eprintln!(
            "[stories] {}/{} {} / {}",
            index + 1,
            catalog.len(),
            story.group,
            story.name
        );
        let (cols, rows) = default_size(story);
        let mut session = crate::util::pty_capture::PtySession::spawn(
            &format!("story {}", story.name),
            &story.cwd,
            &story.argv,
            &story.env,
            cols,
            rows,
        )?;
        let mut note = None;
        for step in 0..=story.keys.len() {
            match session.settle(wait.settle, wait.timeout, step == 0) {
                Ok(crate::util::pty_capture::Settled::Quiet) => {}
                Ok(crate::util::pty_capture::Settled::Exited(status)) => {
                    note = Some(format!("the program exited ({status:?})"));
                    break;
                }
                Err(error) => {
                    note = Some(format!("{error:#}"));
                    break;
                }
            }
            if let Some(key) = story.keys.get(step) {
                session.send_key(key)?;
            }
        }
        recorded.push(serde_json::json!({
            "id": index,
            "group": story.group,
            "name": story.name,
            "cols": cols,
            "rows": rows,
            "data": base64(session.transcript()),
            "note": note,
        }));
        session.finish()?;
    }
    let js = std::fs::read_to_string(xterm.join("lib/xterm.js"))?.replace("</script", "<\\/script");
    let css = std::fs::read_to_string(xterm.join("css/xterm.css"))?;
    let data = serde_json::to_string(&recorded)?.replace("</", "<\\/");
    let page = PAGE
        .replace(
            r#"<link rel="stylesheet" href="/xterm.css">"#,
            &format!("<style>{css}</style>"),
        )
        .replace(
            r#"<script src="/xterm.js"></script>"#,
            &format!("<script>{js}</script>\n<script>window.RECORDED = {data};</script>"),
        );
    std::fs::write(path, page).with_context(|| format!("writing '{}'", path.display()))?;
    println!(
        "stories: recorded {} stories into {}",
        recorded.len(),
        path.display()
    );
    Ok(())
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn catalog(root: &Path) -> Result<Vec<Story>> {
    let mut stories = Vec::new();
    for app in tui_apps::discover(root)? {
        let launch = Launch::prepare(root, app)?;
        let name = launch.app.name.clone();
        let mut add = |story: &str, args: Vec<String>, env: Vec<(String, String)>, keys| {
            stories.push(Story {
                group: name.clone(),
                name: story.to_string(),
                cwd: launch.app.dir.clone(),
                argv: launch.argv(&args),
                env: [launch.env().to_vec(), env].concat(),
                keys,
            });
        };
        add("start", Vec::new(), Vec::new(), Vec::new());
        for story in &launch.app.inspect.stories {
            let mut keys = Vec::new();
            for step in &story.steps {
                keys.extend(parse_keys(&step.keys)?);
            }
            add(
                &story.name,
                story.args.clone(),
                story.env.clone().into_iter().collect(),
                keys,
            );
        }
        if launch.app.inspect.pages.is_some() {
            for (page, args) in launch.pages()? {
                add(&format!("page {page}"), args, Vec::new(), Vec::new());
            }
        }
    }
    stories.extend(install_screens(root)?);
    Ok(stories)
}

/// Each variant's install screen as it ships: the split pane, the variant's shell on the
/// left, its docs on the right.
fn install_screens(root: &Path) -> Result<Vec<Story>> {
    let split = build_split_pane(root)?;
    let mut stories = Vec::new();
    for distro in crate::cli::Distro::ALL {
        let contract = distro_contract::load_variant_contract_for_distro_from(root, distro.id())
            .with_context(|| format!("loading variant '{}'", distro.id()))?;
        let install_ux = &contract.product_config.install_ux;
        let shell = if Path::new(&install_ux.shell).is_file() {
            install_ux.shell.clone()
        } else {
            "/bin/sh".to_string()
        };
        let (right, env) = match &install_ux.docs {
            InstallDocs::TuiApp { app, page } => {
                let launch = Launch::prepare(root, tui_apps::find(root, app)?)?;
                let mut args = vec![
                    "--title".to_string(),
                    format!("{} install guide", contract.identity.os_name),
                ];
                if let (Some(page), Some(pages)) = (page, &launch.app.inspect.pages) {
                    args.extend([pages.arg.clone(), page.clone()]);
                }
                let mut argv = launch.argv(&args);
                argv[1] = launch.app.dir.join(&argv[1]).display().to_string();
                (argv, launch.env().to_vec())
            }
            InstallDocs::Text(guide) => {
                let script =
                    distro_builder::install_ux::text_docs_script(&guide.display().to_string())?;
                (
                    vec!["/bin/sh".to_string(), "-c".to_string(), script],
                    Vec::new(),
                )
            }
        };
        let mut argv = vec![
            split.display().to_string(),
            "--left".to_string(),
            shell,
            "--right".to_string(),
        ];
        argv.extend(right);
        stories.push(Story {
            group: "install screen".to_string(),
            name: contract.identity.os_name.clone(),
            cwd: root.to_path_buf(),
            argv,
            env,
            keys: Vec::new(),
        });
    }
    Ok(stories)
}

fn build_split_pane(root: &Path) -> Result<PathBuf> {
    let status = std::process::Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "-q",
            "-p",
            "install-split-pane",
            "--bin",
            "install-split",
        ])
        .status()
        .context("building install-split")?;
    if !status.success() {
        bail!("building install-split failed");
    }
    Ok(crate::util::repo::cargo_target_dir(root).join("debug/install-split"))
}

fn random_token() -> Result<String> {
    let mut bytes = [0u8; 16];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut random| random.read_exact(&mut bytes))
        .context("reading /dev/urandom")?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

fn query_param(query: &str, key: &str) -> Option<String> {
    query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
}

fn serve(stream: TcpStream, catalog: &[Story], xterm: &Path, token: &str) -> Result<()> {
    let mut head = [0u8; 4096];
    let n = stream.peek(&mut head)?;
    let request = String::from_utf8_lossy(&head[..n]).into_owned();
    let target = request.split_whitespace().nth(1).unwrap_or("/").to_string();
    let (path, query) = target.split_once('?').unwrap_or((&target, ""));
    // The page and xterm.js are public; what can run a program needs the token.
    let needs_token = matches!(path, "/pty" | "/catalog.json");
    if needs_token && query_param(query, "token").as_deref() != Some(token) {
        let refusal = b"open the URL cargo xtask stories printed (it has the access token)";
        return respond_status(stream, "403 Forbidden", "text/plain", refusal);
    }
    match path {
        "/pty" => terminal(accept_from_page(stream)?, catalog, query),
        "/" => respond(stream, "text/html; charset=utf-8", PAGE.as_bytes()),
        "/catalog.json" => {
            let entries: Vec<_> = catalog
                .iter()
                .enumerate()
                .map(|(id, s)| serde_json::json!({ "id": id, "group": s.group, "name": s.name }))
                .collect();
            respond(
                stream,
                "application/json",
                serde_json::to_string(&entries)?.as_bytes(),
            )
        }
        "/xterm.js" => respond(
            stream,
            "text/javascript",
            &std::fs::read(xterm.join("lib/xterm.js"))?,
        ),
        "/xterm.css" => respond(
            stream,
            "text/css",
            &std::fs::read(xterm.join("css/xterm.css"))?,
        ),
        _ => respond_status(stream, "404 Not Found", "text/plain", b"not found"),
    }
}

/// Accepts the WebSocket only from this page (same origin as the address it was loaded
/// from): a tile can run a shell, and another site's page must not reach it.
fn accept_from_page(stream: TcpStream) -> Result<WebSocket<TcpStream>> {
    use tungstenite::handshake::server::{ErrorResponse, Request, Response};
    let check = |request: &Request, response: Response| -> Result<Response, ErrorResponse> {
        let header = |name: &str| request.headers().get(name).and_then(|v| v.to_str().ok());
        let same_origin = matches!(
            (header("origin"), header("host")),
            (Some(origin), Some(host)) if origin == format!("http://{host}")
        );
        if same_origin {
            Ok(response)
        } else {
            let mut refusal = ErrorResponse::new(Some("only the stories page may connect".into()));
            *refusal.status_mut() = tungstenite::http::StatusCode::FORBIDDEN;
            Err(refusal)
        }
    };
    tungstenite::accept_hdr(stream, check)
        .map_err(|e| anyhow::anyhow!("refused a terminal connection: {e}"))
}

fn respond(stream: TcpStream, content_type: &str, body: &[u8]) -> Result<()> {
    respond_status(stream, "200 OK", content_type, body)
}

fn respond_status(
    mut stream: TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    // The request was only peeked; read it so the client sees a clean close.
    let mut discard = [0u8; 4096];
    stream.set_read_timeout(Some(Duration::from_millis(50)))?;
    let _ = stream.read(&mut discard);
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)?;
    Ok(())
}

/// Runs one story in a PTY and bridges it to the WebSocket until either side ends.
/// Query: `id`, `cols`, `rows`, and `nocolor=1` to set `NO_COLOR`.
fn terminal(mut ws: WebSocket<TcpStream>, catalog: &[Story], query: &str) -> Result<()> {
    let param = |key: &str| query_param(query, key);
    let story = param("id")
        .and_then(|id| id.parse::<usize>().ok())
        .and_then(|id| catalog.get(id))
        .context("no such story")?;
    let size = PtySize {
        cols: param("cols").and_then(|v| v.parse().ok()).unwrap_or(120),
        rows: param("rows").and_then(|v| v.parse().ok()).unwrap_or(36),
        pixel_width: 0,
        pixel_height: 0,
    };

    let pair = native_pty_system().openpty(size).context("open PTY")?;
    let mut cmd = CommandBuilder::new(&story.argv[0]);
    cmd.args(&story.argv[1..]);
    cmd.cwd(&story.cwd);
    // The terminal here is xterm.js, an xterm with 256 colours.
    cmd.env("TERM", "xterm-256color");
    for inherited in ["COLORTERM", "FORCE_COLOR", "NO_COLOR"] {
        cmd.env_remove(inherited);
    }
    for (key, value) in &story.env {
        cmd.env(key, value);
    }
    if param("nocolor").as_deref() == Some("1") {
        cmd.env("NO_COLOR", "1");
    }
    let mut child = pair.slave.spawn_command(cmd).context("start the story")?;
    drop(pair.slave);
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;
    let (tx, output) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 || tx.send(buf[..n].to_vec()).is_err() {
                break;
            }
        }
    });

    ws.get_ref()
        .set_read_timeout(Some(Duration::from_millis(15)))?;
    let mut pending = story.keys.iter();
    let mut drawn = false;
    let mut last_output = Instant::now();
    let result = loop {
        match output.recv_timeout(Duration::from_millis(15)) {
            Ok(bytes) => {
                drawn = true;
                last_output = Instant::now();
                if let Err(error) = ws.send(Message::binary(bytes)) {
                    break Err(error.into());
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                let _ = ws.send(Message::text("\r\n[the program exited]\r\n"));
                break Ok(());
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
        if drawn && last_output.elapsed() >= KEY_SETTLE {
            if let Some(key) = pending.next() {
                writer.write_all(key)?;
                writer.flush()?;
                last_output = Instant::now();
            }
        }
        match ws.read() {
            Ok(Message::Binary(bytes)) => {
                writer.write_all(&bytes)?;
                writer.flush()?;
            }
            Ok(Message::Text(text)) => {
                // {"cols":N,"rows":N} resizes; other text is typed.
                match serde_json::from_str::<serde_json::Value>(text.as_str()) {
                    Ok(value) if value.get("cols").is_some() => {
                        let get =
                            |k: &str| value.get(k).and_then(|v| v.as_u64()).unwrap_or(0) as u16;
                        let _ = pair.master.resize(PtySize {
                            cols: get("cols").max(1),
                            rows: get("rows").max(1),
                            pixel_width: 0,
                            pixel_height: 0,
                        });
                    }
                    _ => {
                        writer.write_all(text.as_bytes())?;
                        writer.flush()?;
                    }
                }
            }
            Ok(Message::Close(_)) => break Ok(()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            // A tab closed without a closing handshake is an ordinary end too.
            Err(
                tungstenite::Error::ConnectionClosed
                | tungstenite::Error::AlreadyClosed
                | tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                ),
            ) => break Ok(()),
            Err(error) => break Err(error.into()),
        }
    };
    let _ = child.kill();
    let _ = child.wait();
    result
}
