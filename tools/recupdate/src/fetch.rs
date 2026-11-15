//! `recupdate fetch`: download and verify the release's files.

use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::release::{Archive, Verified};

pub const DEFAULT_CACHE_DIR: &str = "/var/cache/recupdate";

pub fn cache_dir() -> PathBuf {
    crate::util::env_path("RECUPDATE_CACHE_DIR", DEFAULT_CACHE_DIR)
}

/// How a fetch reports: each archive as it starts (`step`), and its bytes (`progress`).
pub struct Report<'a> {
    pub step: &'a dyn Fn(u32, &str),
    pub progress: &'a dyn Fn(u64, u64),
}

pub fn run(json: bool) -> Result<()> {
    match fetch(json) {
        Ok(()) => Ok(()),
        Err(error) => {
            if json {
                let line = format!("{error:#}")
                    .lines()
                    .next()
                    .unwrap_or("failed")
                    .to_owned();
                println!("{}", json!({"event":"failed","error":line}));
            }
            Err(error)
        }
    }
}

fn fetch(json: bool) -> Result<()> {
    let step = |n: u32, text: &str| {
        if json {
            println!("{}", json!({"event":"step","n":n,"of":4,"text":text}));
        } else {
            eprintln!("[{n}/4] {text}");
        }
    };
    let progress = |done: u64, total: u64| {
        if json {
            println!("{}", json!({"event":"progress","done":done,"total":total}));
        }
    };
    step(1, "Checking the release channel");
    let channel = crate::release::load_channel()?;
    let report = Report {
        step: &|n, text| step(n + 1, text),
        progress: &progress,
    };
    let dir = into_cache(&channel, &report)?;
    let m = &channel.manifest;
    if json {
        println!(
            "{}",
            json!({"event":"done","restart_needed":false,"version":m.version,"serial":m.serial})
        );
    } else {
        eprintln!(
            "fetched {} {} (serial {}) into {}",
            m.os,
            m.version,
            m.serial,
            dir.display()
        );
    }
    Ok(())
}

/// Download and verify the verified channel's release into `<cache>/<serial>/`; return that
/// folder. `report.step` is called with 1, 2, 3 for the three archives. Other serial folders
/// are removed afterwards, except the one an interrupted `apply` still needs.
pub fn into_cache(channel: &Verified, report: &Report) -> Result<PathBuf> {
    let m = &channel.manifest;
    let cache = cache_dir();
    let dir = cache.join(m.serial.to_string());
    fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    fs::set_permissions(&cache, fs::Permissions::from_mode(0o755))?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o755))?;

    let parts: [(&str, &Archive, &str); 3] = [
        ("base.tar.zst", &m.base, "Downloading the base"),
        ("boot.tar.zst", &m.boot, "Downloading the boot files"),
        ("recipes.tar.zst", &m.recipes, "Downloading the recipes"),
    ];
    for (i, (name, archive, text)) in parts.iter().enumerate() {
        (report.step)(i as u32 + 1, text);
        get_archive(&dir, name, archive, report)?;
    }

    // The verified manifest and its signature go last: their presence marks a complete fetch.
    crate::util::write_atomic(&dir.join("release.json"), &channel.bytes, 0o644)?;
    crate::util::write_atomic(&dir.join("release.json.sig"), &channel.sig, 0o644)?;

    let keep = crate::apply::read_intent()
        .ok()
        .flatten()
        .map(|intent| intent.folder);
    for entry in fs::read_dir(&cache)?.flatten() {
        let name = entry.file_name();
        let is_serial = name.to_string_lossy().chars().all(|c| c.is_ascii_digit());
        let path = entry.path();
        if is_serial && path != dir && keep.as_deref() != Some(path.as_path()) && path.is_dir() {
            let _ = fs::remove_dir_all(&path);
        }
    }
    Ok(dir)
}

fn get_archive(dir: &Path, name: &str, archive: &Archive, report: &Report) -> Result<()> {
    ensure!(
        archive.url.starts_with("https://"),
        "the manifest's address for {name} is not https: {}",
        archive.url
    );
    ensure!(
        archive.sha256.len() == 64 && archive.sha256.bytes().all(|b| b.is_ascii_hexdigit()),
        "the manifest's sha256 for {name} is not a SHA-256"
    );
    let want = archive.sha256.to_ascii_lowercase();
    let file = dir.join(name);
    if fs::metadata(&file)
        .map(|m| m.len() == archive.size)
        .unwrap_or(false)
        && crate::net::sha256_file(&file)? == want
    {
        (report.progress)(archive.size, archive.size);
        return Ok(());
    }
    let part = dir.join(format!("{name}.part"));
    crate::net::download(&archive.url, &part, archive.size, &mut |done| {
        (report.progress)(done, archive.size)
    })?;
    let got = crate::net::sha256_file(&part)?;
    if got != want {
        let _ = fs::remove_file(&part);
        anyhow::bail!(
            "{name} has SHA-256 {got}, the signed manifest says {want}; the download was removed"
        );
    }
    fs::set_permissions(&part, fs::Permissions::from_mode(0o644))?;
    fs::rename(&part, &file).with_context(|| format!("renaming {}", part.display()))?;
    Ok(())
}
