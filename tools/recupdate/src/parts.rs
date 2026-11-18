//! `recupdate fetch-parts`: download the category parts of the release this medium was built
//! from and unpack them into Recipe's offline package cache. Design:
//! docs/release-engineering.md section 11.
//!
//! A part is `parts-<category>.tar.zst`, a tar of package-cache files with paths relative to
//! the recipe cache root, so unpacking it into that folder is the whole install of the part.

use anyhow::{bail, ensure, Context, Result};
use serde_json::json;
use std::fs;
use std::io::{BufReader, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Component, Path, PathBuf};

use crate::fetch::{get_archive, Report};
use crate::release::Part;

/// Recipe's offline package cache on the medium: mirrors `install_tools::RECIPE_CACHE_DIR`
/// (tools/recstrap/src/lib.rs), which recupdate does not depend on. Keep the two equal.
pub const DEFAULT_RECIPE_CACHE_DIR: &str = "/usr/share/recipe/cache";

pub struct Args {
    pub json: bool,
    pub categories: Vec<String>,
    pub from: Option<PathBuf>,
    pub list: bool,
}

fn recipe_cache_dir() -> PathBuf {
    crate::util::env_path("RECUPDATE_RECIPE_CACHE_DIR", DEFAULT_RECIPE_CACHE_DIR)
}

pub fn run(args: Args) -> Result<()> {
    let json = args.json;
    match go(args) {
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

/// A name that is one plain file name: it becomes `<name>.tar.zst` and `<name>.done`.
fn check_name(what: &str, name: &str) -> Result<()> {
    ensure!(
        !name.is_empty()
            && name != "."
            && name != ".."
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
        "the manifest's part {what} {name:?} is not a plain name"
    );
    Ok(())
}

fn sha_ok(sha: &str) -> bool {
    sha.len() == 64 && sha.bytes().all(|b| b.is_ascii_hexdigit())
}

fn done_file(dir: &Path, part: &Part) -> PathBuf {
    dir.join(format!("{}.done", part.name))
}

/// The part is unpacked already: its record holds the manifest's hash.
fn is_present(dir: &Path, part: &Part) -> bool {
    sha_ok(&part.sha256)
        && fs::read_to_string(done_file(dir, part))
            .map(|text| text.trim().eq_ignore_ascii_case(&part.sha256))
            .unwrap_or(false)
}

fn go(args: Args) -> Result<()> {
    let json = args.json && !args.list;
    let steps_total = std::cell::Cell::new(1u32);
    let step = |n: u32, text: &str| {
        let of = steps_total.get();
        if json {
            println!("{}", json!({"event":"step","n":n,"of":of,"text":text}));
        } else if !args.list {
            eprintln!("[{n}/{of}] {text}");
        }
    };
    let release = crate::release::load_medium_release(args.from.as_deref())?;
    let m = &release.manifest;

    for part in &m.parts {
        check_name("name", &part.name)?;
        check_name("category", &part.category)?;
    }
    for wanted in &args.categories {
        ensure!(
            m.parts.iter().any(|p| &p.category == wanted),
            "the release has no part for the category {wanted}; its categories are: {}",
            m.parts
                .iter()
                .map(|p| p.category.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let wanted: Vec<&Part> = m
        .parts
        .iter()
        .filter(|p| args.categories.is_empty() || args.categories.contains(&p.category))
        .collect();

    let dir = crate::fetch::cache_dir()
        .join("parts")
        .join(m.serial.to_string());

    if args.list {
        let parts: Vec<_> = wanted
            .iter()
            .map(|p| {
                json!({"name":p.name,"category":p.category,"size":p.size,"present":is_present(&dir,p)})
            })
            .collect();
        println!("{}", json!({"parts": parts}));
        return Ok(());
    }

    steps_total.set(1 + wanted.len() as u32);
    step(1, "Verified the release's signed manifest");
    let recipe_cache = recipe_cache_dir();
    for (i, part) in wanted.iter().enumerate() {
        let n = i as u32 + 2;
        let progress = |done: u64, total: u64| {
            if json {
                println!("{}", json!({"event":"progress","done":done,"total":total}));
            }
        };
        if is_present(&dir, part) {
            step(
                n,
                &format!("The {} part is already unpacked", part.category),
            );
            progress(part.size, part.size);
            continue;
        }
        step(n, &format!("Fetching the {} part", part.category));
        ensure!(
            sha_ok(&part.sha256) && !part.url.is_empty(),
            "the manifest does not publish the part {} (no address or SHA-256 yet)",
            part.name
        );
        let want = part.sha256.to_ascii_lowercase();
        let file_name = format!("{}.tar.zst", part.name);
        let (archive, downloaded) = match &args.from {
            Some(from) => {
                let path = from.join(&file_name);
                let meta = fs::metadata(&path).with_context(|| {
                    format!(
                        "cannot read {} (the folder must hold {file_name})",
                        path.display()
                    )
                })?;
                ensure!(
                    meta.is_file() && meta.len() == part.size,
                    "{} is {} bytes, the signed manifest says {}",
                    path.display(),
                    meta.len(),
                    part.size
                );
                let got = crate::net::sha256_file(&path)?;
                ensure!(
                    got == want,
                    "{} has SHA-256 {got}, the signed manifest says {want}",
                    path.display()
                );
                progress(part.size, part.size);
                (path, false)
            }
            None => {
                fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
                let report = Report {
                    step: &|_, _| {},
                    progress: &progress,
                };
                get_archive(
                    &dir,
                    &file_name,
                    &part.archive(),
                    release.plain_http,
                    &report,
                )?;
                (dir.join(&file_name), true)
            }
        };
        unpack(&archive, &recipe_cache)?;
        // The record goes after the unpacking: its presence says the whole part is in place.
        crate::util::write_atomic(&done_file(&dir, part), want.as_bytes(), 0o644)?;
        if downloaded {
            // The files live in the recipe cache now; the archive would only take space.
            let _ = fs::remove_file(&archive);
        }
    }

    if json {
        println!(
            "{}",
            json!({"event":"done","restart_needed":false,"version":m.version,"serial":m.serial})
        );
    } else {
        eprintln!(
            "the parts of {} {} (serial {}) are in {}",
            m.os,
            m.version,
            m.serial,
            recipe_cache.display()
        );
    }
    Ok(())
}

/// The archive path as a clean relative path, or `None` when it is absolute, climbs out with
/// `..`, or names nothing. `.` components are dropped.
fn clean_relative(path: &Path) -> Option<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(name) => clean.push(name),
            Component::CurDir => {}
            _ => return None,
        }
    }
    (!clean.as_os_str().is_empty()).then_some(clean)
}

/// Create the folders of `rel` under `root` one at a time, refusing any that exists as a
/// symbolic link (never followed), and return the full path.
fn make_dirs(root: &Path, rel: &Path) -> Result<PathBuf> {
    let mut path = root.to_path_buf();
    for component in rel.components() {
        path.push(component);
        match fs::symlink_metadata(&path) {
            Ok(meta) if meta.is_dir() => {}
            Ok(_) => bail!(
                "{} exists and is not a folder; the part would be written through it",
                path.display()
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir(&path).with_context(|| format!("creating {}", path.display()))?;
                fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
            }
            Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
        }
    }
    Ok(path)
}

/// Whether a symbolic link at `at` pointing to `target` stays inside the archive's root:
/// relative, and never above the root when resolved from `at`'s folder.
fn link_stays_inside(at: &Path, target: &Path) -> bool {
    if target.is_absolute() {
        return false;
    }
    let mut depth = at.components().count() as i64 - 1;
    for component in target.components() {
        match component {
            Component::Normal(_) => depth += 1,
            Component::ParentDir => depth -= 1,
            Component::CurDir => {}
            _ => return false,
        }
        if depth < 0 {
            return false;
        }
    }
    true
}

/// Unpack the tar.zst `archive` into `dest`: regular files, folders and links that stay
/// inside; nothing is written through a symbolic link, and an absolute path, a `..`, a hard
/// link or a special file ends the unpacking with an error.
fn unpack(archive: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest).with_context(|| format!("creating {}", dest.display()))?;
    let decoder = zstd::stream::read::Decoder::new(BufReader::new(
        fs::File::open(archive).with_context(|| format!("opening {}", archive.display()))?,
    ))
    .with_context(|| format!("reading {}", archive.display()))?;
    let mut tar = tar::Archive::new(decoder);
    for entry in tar
        .entries()
        .with_context(|| format!("reading {}", archive.display()))?
    {
        let mut entry = entry.with_context(|| format!("reading {}", archive.display()))?;
        let kind = entry.header().entry_type();
        if kind.is_pax_global_extensions() || kind.is_pax_local_extensions() {
            continue;
        }
        let raw = entry.path()?.into_owned();
        let Some(rel) = clean_relative(&raw) else {
            if kind.is_dir() && clean_relative(&raw.join("x")).is_some() {
                continue; // the root folder itself ("./")
            }
            bail!(
                "{} holds the path {:?}, which is absolute or leaves the cache folder; part archives hold paths relative to it",
                archive.display(),
                raw
            );
        };
        if kind.is_dir() {
            make_dirs(dest, &rel)?;
            continue;
        }
        let parent = rel.parent().unwrap_or(Path::new(""));
        let folder = make_dirs(dest, parent)?;
        let target = folder.join(rel.file_name().context("entry has no name")?);
        match fs::symlink_metadata(&target) {
            Ok(meta) if meta.is_dir() => bail!(
                "{} is a folder where {} holds a file",
                target.display(),
                archive.display()
            ),
            Ok(_) => fs::remove_file(&target)
                .with_context(|| format!("replacing {}", target.display()))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e).with_context(|| format!("reading {}", target.display())),
        }
        if kind.is_file() {
            let mode = entry.header().mode().unwrap_or(0o644) & 0o777;
            let mut out = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(mode)
                .custom_flags(libc::O_NOFOLLOW)
                .open(&target)
                .with_context(|| format!("creating {}", target.display()))?;
            let size = entry.header().size()?;
            let copied = std::io::copy(&mut entry, &mut out).with_context(|| {
                format!("unpacking {} from {}", rel.display(), archive.display())
            })?;
            ensure!(
                copied == size,
                "{} ends inside {}",
                archive.display(),
                rel.display()
            );
            out.flush()?;
        } else if kind.is_symlink() {
            let link = entry
                .link_name()?
                .context("a link entry has no target")?
                .into_owned();
            ensure!(
                link_stays_inside(&rel, &link),
                "{} holds the link {} -> {}, which points outside the cache folder",
                archive.display(),
                rel.display(),
                link.display()
            );
            std::os::unix::fs::symlink(&link, &target)
                .with_context(|| format!("creating {}", target.display()))?;
        } else {
            bail!(
                "{} holds {} of a kind part archives do not carry (files, folders and links only)",
                archive.display(),
                rel.display()
            );
        }
    }
    Ok(())
}
