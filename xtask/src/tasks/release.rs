//! `release keygen`, `release sign` and `release publish`: signing and publishing a release.
//!
//! Design: docs/release-engineering.md (sections 2 to 4 and 10). The build writes an unsigned
//! `release.json` next to the ISO; `publish` fills the download facts, signs, uploads the files
//! to a GitHub release `<os>-<serial>` in `LevitateOS/releases` (resumable: assets already
//! uploaded with the same size are skipped) and moves the channel pointer last.
use anyhow::{Context, Result, bail, ensure};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "LevitateOS/releases";
const KEY_ENV: &str = "LEVITATE_RELEASE_KEY";
/// The archives the build writes next to the ISO, and the manifest field of each.
const ARCHIVES: [(&str, &str); 3] = [
    ("base", "base.tar.zst"),
    ("boot", "boot.tar.zst"),
    ("recipes", "recipes.tar.zst"),
];

pub const USAGE: &str = "usage:
  cargo xtask release keygen --out FILE
  cargo xtask release sign RELEASE.JSON [--key FILE]
  cargo xtask release publish DISTRO [--key FILE] [--dir DIR]

keygen writes the private seed (base64, mode 0600) and prints the public key line for
/usr/share/levitate/release-keys/. The key path comes from --key or LEVITATE_RELEASE_KEY.
publish reads the unsigned release.json and base.tar.zst, boot.tar.zst, recipes.tar.zst and the
.iso next to it (--dir, default: the newest .artifacts/out/<distro>/releases/release/* folder with
a release.json), fills url, sha256 and size, signs, uploads to LevitateOS/releases as release
<os>-<serial> and last commits channels/<os>/stable.json (+ .sig). A rerun resumes.";

pub fn run(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("keygen") => keygen(&args[1..]),
        Some("sign") => sign(&args[1..]),
        Some("publish") => publish(&args[1..]),
        _ => bail!("{USAGE}"),
    }
}

/// Split `args` into positionals and `--name value` options.
type Options = Vec<(String, String)>;

fn parse(args: &[String], names: &[&str]) -> Result<(Vec<String>, Options)> {
    let mut positional = Vec::new();
    let mut options = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        if names.contains(&arg.as_str()) {
            let value = it
                .next()
                .with_context(|| format!("{arg} needs a value\n\n{USAGE}"))?;
            options.push((arg.clone(), value.clone()));
        } else if arg.starts_with("--") {
            bail!("unknown option {arg}\n\n{USAGE}");
        } else {
            positional.push(arg.clone());
        }
    }
    Ok((positional, options))
}

fn option<'a>(options: &'a [(String, String)], name: &str) -> Option<&'a str> {
    options
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.as_str())
}

fn signing_key(path: Option<&str>) -> Result<SigningKey> {
    let path = match path {
        Some(p) => PathBuf::from(p),
        None => std::env::var_os(KEY_ENV)
            .map(PathBuf::from)
            .with_context(|| format!("no signing key: pass --key FILE or set {KEY_ENV}"))?,
    };
    let text =
        fs::read_to_string(&path).with_context(|| format!("reading the key {}", path.display()))?;
    let seed = STANDARD
        .decode(text.trim())
        .with_context(|| format!("{} is not base64", path.display()))?;
    let seed: [u8; 32] = seed
        .try_into()
        .map_err(|_| anyhow::anyhow!("{} is not a 32-byte seed", path.display()))?;
    Ok(SigningKey::from_bytes(&seed))
}

// ---------------------------------------------------------------- keygen and sign

fn keygen(args: &[String]) -> Result<()> {
    let (positional, options) = parse(args, &["--out"])?;
    ensure!(positional.is_empty(), "{USAGE}");
    let out = PathBuf::from(
        option(&options, "--out").with_context(|| format!("keygen needs --out FILE\n\n{USAGE}"))?,
    );
    let mut seed = [0u8; 32];
    fs::File::open("/dev/urandom")?.read_exact(&mut seed)?;
    let key = SigningKey::from_bytes(&seed);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&out)
        .with_context(|| format!("creating {} (it must not exist)", out.display()))?;
    std::io::Write::write_all(&mut file, format!("{}\n", STANDARD.encode(seed)).as_bytes())?;
    eprintln!(
        "wrote the private key {} (0600); keep it on this machine only",
        out.display()
    );
    eprintln!("the public key line, for /usr/share/levitate/release-keys/<name>.pub:");
    println!("{}", STANDARD.encode(key.verifying_key().to_bytes()));
    Ok(())
}

/// The `.sig` text for `bytes`: the base64 signature and a newline.
fn signature_line(key: &SigningKey, bytes: &[u8]) -> String {
    format!("{}\n", STANDARD.encode(key.sign(bytes).to_bytes()))
}

fn sign(args: &[String]) -> Result<()> {
    let (positional, options) = parse(args, &["--key"])?;
    ensure!(positional.len() == 1, "{USAGE}");
    let file = PathBuf::from(&positional[0]);
    let key = signing_key(option(&options, "--key"))?;
    let bytes = fs::read(&file).with_context(|| format!("reading {}", file.display()))?;
    let sig = PathBuf::from(format!("{}.sig", file.display()));
    fs::write(&sig, signature_line(&key, &bytes))?;
    println!("wrote {}", sig.display());
    Ok(())
}

// ---------------------------------------------------------------- publish

struct Ctx {
    dir: PathBuf,
    os: String,
    tag: String,
}

fn publish(args: &[String]) -> Result<()> {
    let (positional, options) = parse(args, &["--key", "--dir"])?;
    ensure!(positional.len() == 1, "{USAGE}");
    let distro = positional[0].clone();
    let step = |name: &'static str, result: Result<()>| -> Result<()> {
        match result {
            Ok(()) => {
                eprintln!("[ok] {name}");
                Ok(())
            }
            Err(error) => {
                let text = format!("{error:#}");
                let first = text.lines().next().unwrap_or("failed").to_owned();
                for line in text.lines().skip(1) {
                    eprintln!("{line}");
                }
                eprintln!("[fail] {name}: {first}");
                std::process::exit(1);
            }
        }
    };

    let mut ctx = None;
    let mut manifest = Value::Null;
    let mut files: Vec<PathBuf> = Vec::new();
    let key = signing_key(option(&options, "--key"));
    let key = match key {
        Ok(k) => k,
        Err(error) => {
            eprintln!("[fail] key: {error:#}");
            std::process::exit(1);
        }
    };
    step(
        "manifest",
        (|| {
            let root = crate::util::repo::repo_root()?;
            let dir = match option(&options, "--dir") {
                Some(d) => PathBuf::from(d),
                None => newest_release_dir(&root, &distro)?,
            };
            let (c, m, f) = fill_manifest(&dir)?;
            ctx = Some(c);
            manifest = m;
            files = f;
            Ok(())
        })(),
    )?;
    let ctx = ctx.unwrap();

    step(
        "sign",
        (|| {
            let mut text = serde_json::to_string_pretty(&manifest)?;
            text.push('\n');
            fs::write(ctx.dir.join("release.json"), &text)?;
            fs::write(
                ctx.dir.join("release.json.sig"),
                signature_line(&key, text.as_bytes()),
            )?;
            files.push(ctx.dir.join("release.json"));
            files.push(ctx.dir.join("release.json.sig"));
            Ok(())
        })(),
    )?;
    step("upload", upload(&ctx, &files))?;
    step("channel", channel(&ctx, &manifest))?;
    println!(
        "published {} {} (serial {}) to {REPO}",
        ctx.os,
        manifest["version"].as_str().unwrap_or(""),
        manifest["serial"]
    );
    Ok(())
}

/// The newest folder under `.artifacts/out/<distro>/releases/release/` holding a release.json.
fn newest_release_dir(root: &Path, distro: &str) -> Result<PathBuf> {
    let base = root
        .join(".artifacts/out")
        .join(distro)
        .join("releases/release");
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in fs::read_dir(&base)
        .with_context(|| format!("reading {}", base.display()))?
        .flatten()
    {
        let manifest = entry.path().join("release.json");
        if let Ok(modified) = fs::metadata(&manifest).and_then(|m| m.modified())
            && best.as_ref().is_none_or(|(t, _)| modified > *t)
        {
            best = Some((modified, entry.path()));
        }
    }
    best.map(|(_, p)| p).with_context(|| {
        format!(
            "no release.json under {}; the release build writes it next to the ISO (or pass --dir)",
            base.display()
        )
    })
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// Read the unsigned manifest and fill url, sha256 and size for base, boot, recipes and iso.
/// Returns the context, the filled manifest and the data files to upload.
fn fill_manifest(dir: &Path) -> Result<(Ctx, Value, Vec<PathBuf>)> {
    let path = dir.join("release.json");
    let mut manifest: Value = serde_json::from_str(
        &fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?,
    )
    .with_context(|| format!("{} is not JSON", path.display()))?;
    ensure!(
        manifest["schema"] == 1,
        "{} must have \"schema\": 1",
        path.display()
    );
    let os = manifest["os"]
        .as_str()
        .context("release.json has no os")?
        .to_owned();
    ensure!(
        manifest["version"].as_str().is_some_and(|v| !v.is_empty()),
        "release.json has no version"
    );
    let serial = manifest["serial"]
        .as_u64()
        .context("release.json has no serial")?;
    let tag = format!("{os}-{serial}");

    let mut iso = None;
    for entry in fs::read_dir(dir)?.flatten() {
        if entry.path().extension().is_some_and(|e| e == "iso") {
            ensure!(iso.is_none(), "more than one .iso in {}", dir.display());
            iso = Some(entry.path());
        }
    }
    let iso = iso.with_context(|| format!("no .iso in {}", dir.display()))?;
    let mut parts: Vec<(&str, PathBuf)> = ARCHIVES
        .iter()
        .map(|(field, name)| (*field, dir.join(name)))
        .collect();
    parts.push(("iso", iso));

    let mut files = Vec::new();
    for (field, file) in parts {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        ensure!(
            file.is_file(),
            "{} is missing; the release build must write it next to release.json",
            file.display()
        );
        eprintln!("[info] hashing {name}");
        let entry = manifest
            .as_object_mut()
            .unwrap()
            .entry(field)
            .or_insert_with(|| json!({}));
        ensure!(
            entry.is_object() || entry.is_null(),
            "release.json: {field} is not an object"
        );
        let object = entry
            .as_object_mut()
            .context("release.json field is not an object")?;
        object.insert(
            "url".into(),
            json!(format!(
                "https://github.com/{REPO}/releases/download/{tag}/{name}"
            )),
        );
        object.insert("sha256".into(), json!(sha256_file(&file)?));
        object.insert("size".into(), json!(fs::metadata(&file)?.len()));
        files.push(file);
    }
    ensure!(
        manifest["recipes"]["revision"]
            .as_str()
            .is_some_and(|r| !r.is_empty()),
        "release.json: recipes.revision is empty"
    );
    Ok((
        Ctx {
            dir: dir.to_owned(),
            os,
            tag,
        },
        manifest,
        files,
    ))
}

/// Run `gh` (or git) under `timeout`, failing with its output on a nonzero exit.
fn exec(program: &str, args: &[&str], cwd: Option<&Path>, seconds: u64) -> Result<String> {
    let mut cmd = Command::new("timeout");
    cmd.arg(seconds.to_string()).arg(program).args(args);
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    let output = cmd.output().with_context(|| format!("running {program}"))?;
    if !output.status.success() {
        bail!(
            "{program} {} failed ({}){}\n{}",
            args.first().copied().unwrap_or(""),
            output.status,
            if output.status.code() == Some(124) {
                ": timed out"
            } else {
                ""
            },
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn remote_assets(tag: &str) -> Result<Option<Vec<(String, u64)>>> {
    let cmd = Command::new("timeout")
        .args([
            "30", "gh", "release", "view", tag, "-R", REPO, "--json", "assets",
        ])
        .output()
        .context("running gh")?;
    if !cmd.status.success() {
        return Ok(None);
    }
    let value: Value = serde_json::from_slice(&cmd.stdout)?;
    Ok(Some(
        value["assets"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|a| Some((a["name"].as_str()?.to_owned(), a["size"].as_u64()?)))
                    .collect()
            })
            .unwrap_or_default(),
    ))
}

fn upload(ctx: &Ctx, files: &[PathBuf]) -> Result<()> {
    let assets = match remote_assets(&ctx.tag)? {
        Some(a) => a,
        None => {
            eprintln!("[info] creating release {} in {REPO}", ctx.tag);
            let title = ctx.tag.clone();
            exec(
                "gh",
                &[
                    "release",
                    "create",
                    &ctx.tag,
                    "-R",
                    REPO,
                    "--title",
                    &title,
                    "--notes",
                    &format!("Release {}", ctx.tag),
                ],
                None,
                60,
            )?;
            Vec::new()
        }
    };
    for file in files {
        let name = file.file_name().unwrap().to_string_lossy().into_owned();
        let size = fs::metadata(file)?.len();
        // release.json and its signature change whenever the manifest does; always replace them.
        let always = name.starts_with("release.json");
        if !always && assets.iter().any(|(n, s)| *n == name && *s == size) {
            eprintln!("[info] {name} is already uploaded ({size} bytes)");
            continue;
        }
        eprintln!("[info] uploading {name} ({size} bytes)");
        // 10 minutes plus a floor speed of 500 kB/s.
        let seconds = 600 + size / 500_000;
        exec(
            "gh",
            &[
                "release",
                "upload",
                &ctx.tag,
                "--clobber",
                "-R",
                REPO,
                &file.to_string_lossy(),
            ],
            None,
            seconds,
        )?;
    }
    Ok(())
}

/// Commit and push `channels/<os>/stable.json` (+ `.sig`) through a temporary clone.
fn channel(ctx: &Ctx, manifest: &Value) -> Result<()> {
    let scratch = tempfile::tempdir()?;
    let clone = scratch.path().join("releases");
    exec(
        "gh",
        &[
            "repo",
            "clone",
            REPO,
            &clone.to_string_lossy(),
            "--",
            "--depth",
            "1",
        ],
        None,
        120,
    )?;
    let folder = clone.join("channels").join(&ctx.os);
    let target = folder.join("stable.json");
    if let Ok(text) = fs::read_to_string(&target)
        && let Ok(old) = serde_json::from_str::<Value>(&text)
        && let (Some(old), Some(new)) = (old["serial"].as_u64(), manifest["serial"].as_u64())
    {
        ensure!(
            old <= new,
            "the channel already points at serial {old}, higher than {new}; serials only go up"
        );
    }
    fs::create_dir_all(&folder)?;
    fs::copy(ctx.dir.join("release.json"), &target)?;
    fs::copy(
        ctx.dir.join("release.json.sig"),
        folder.join("stable.json.sig"),
    )?;
    let status = exec("git", &["status", "--porcelain"], Some(&clone), 30)?;
    if status.trim().is_empty() {
        eprintln!("[info] the channel already points at this release");
        return Ok(());
    }
    exec("git", &["add", "channels"], Some(&clone), 30)?;
    let message = format!(
        "Channel stable: {} {} (serial {})",
        ctx.os,
        manifest["version"].as_str().unwrap_or(""),
        manifest["serial"]
    );
    exec("git", &["commit", "-s", "-m", &message], Some(&clone), 30)?;
    exec("git", &["push", "origin", "HEAD:main"], Some(&clone), 120)?;
    Ok(())
}
