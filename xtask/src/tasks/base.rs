//! `base lock`: lock the RPMs a variant takes from its Fedora base.
//!
//! The variant contract (`rpm_base`) names the Fedora release and the
//! packages; this resolves the newest build of each from the release and
//! updates repositories, checks every RPM against the repository checksum and
//! the release's signing key, and writes the lock the rootfs recipes import.
//! Rerunning it is how a build takes Fedora's updates.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::cli::Distro;

const MIRROR: &str = "https://dl.fedoraproject.org/pub/fedora/linux";

/// One RPM build as a repository's primary index lists it.
#[derive(Clone, Debug)]
struct Rpm {
    name: String,
    epoch: String,
    version: String,
    release: String,
    arch: String,
    url: String,
    sha256: String,
}

impl Rpm {
    fn nevra(&self) -> String {
        format!("{}-{}-{}.{}", self.name, self.version, self.release, self.arch)
    }

    fn file_name(&self) -> &str {
        self.url.rsplit('/').next().unwrap_or(&self.url)
    }
}

/// The lock file the rootfs recipes import.
pub fn lock_path(root: &Path, distro: &Distro) -> PathBuf {
    root.join("distro/builder/recipes/sources")
        .join(format!("{}-rpm-lock.rhai", distro.id()))
}

pub fn lock(distro: &Distro, check: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let bundle = distro_contract::load(&root, distro.id())?;
    let Some(base) = bundle.contract.sources.rootfs_source.rpm_base.clone() else {
        return Err(crate::util::exit::usage(format!(
            "{} does not take its rootfs from an RPM base (no rpm_base in its contract)",
            distro.id()
        )));
    };
    distro_builder::process::check_host_tools(distro_builder::process::HostNeed::BaseLock)
        .map_err(|e| crate::util::exit::missing(format!("{e:#}"), "cargo xtask doctor"))?;
    let release = base.fedora_release;
    let wanted: HashSet<String> = base.packages.iter().cloned().collect();

    let mut newest: BTreeMap<String, Rpm> = BTreeMap::new();
    for repo in [
        format!("{MIRROR}/releases/{release}/Everything/x86_64/os/"),
        format!("{MIRROR}/updates/{release}/Everything/x86_64/"),
    ] {
        eprintln!("[step] Read {repo}");
        for rpm in primary_index(&repo, &wanted)? {
            let keep = newest
                .get(&rpm.name)
                .is_none_or(|current| evr_cmp(&rpm, current) == Ordering::Greater);
            if keep {
                newest.insert(rpm.name.clone(), rpm);
            }
        }
    }
    let missing: Vec<&String> = base.packages.iter().filter(|n| !newest.contains_key(*n)).collect();
    if !missing.is_empty() {
        bail!(
            "Fedora {release} has no x86_64/noarch build of: {}",
            missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
        );
    }

    let path = lock_path(&root, distro);
    let locked = read_locked(&path);
    let changes = report_changes(&locked, &newest);

    if check {
        if changes == 0 {
            println!("{} is current with Fedora {release} updates", rel(&root, &path));
            return Ok(());
        }
        bail!("{changes} package(s) have newer builds; run cargo xtask base lock {}", distro.id());
    }

    let cache = root.join(".artifacts/rpm-cache").join(format!("fedora-{release}"));
    let files = download_verified(&cache, newest.values())?;
    let key = bundle.path(&base.signing_key);
    check_signatures(&key, &files)?;

    fs::write(&path, render_lock(distro, release, &newest)?)
        .with_context(|| format!("writing {}", path.display()))?;
    println!("wrote {} ({} packages, {changes} changed)", rel(&root, &path), newest.len());
    Ok(())
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).display().to_string()
}

fn curl(url: &str, out: &Path) -> Result<()> {
    let status = Command::new("curl")
        .args(["-fsSL", "--retry", "3", "-o"])
        .arg(out)
        .arg(url)
        .status()
        .context("running curl")?;
    if !status.success() {
        bail!("downloading {url} failed ({status})");
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// The `wanted` packages (x86_64 or noarch) in a repository's primary index.
fn primary_index(repo: &str, wanted: &HashSet<String>) -> Result<Vec<Rpm>> {
    let tmp = tempfile::tempdir()?;
    let repomd_path = tmp.path().join("repomd.xml");
    curl(&format!("{repo}repodata/repomd.xml"), &repomd_path)?;
    let repomd = fs::read_to_string(&repomd_path)?;

    let section = repomd
        .split("<data ")
        .find(|s| s.starts_with("type=\"primary\""))
        .context("repomd.xml lists no primary index")?;
    let href = attr_in(section, "<location href=\"").context("primary index has no location")?;
    let sha = between(section, "<checksum type=\"sha256\">", "</checksum>")
        .context("primary index has no sha256 checksum")?;

    let index = tmp.path().join("primary");
    curl(&format!("{repo}{href}"), &index)?;
    if sha256_file(&index)? != sha {
        bail!("{repo}{href} does not match the checksum in repomd.xml");
    }
    let decompress = match href.rsplit('.').next() {
        Some("zst") => "zstd",
        Some("gz") => "gzip",
        Some("xz") => "xz",
        other => bail!("unexpected primary index compression {other:?}"),
    };
    let mut child = Command::new(decompress)
        .arg("-dc")
        .arg(&index)
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| format!("running {decompress}"))?;
    let rpms = parse_primary(BufReader::new(child.stdout.take().unwrap()), repo, wanted)?;
    if !child.wait()?.success() {
        bail!("decompressing {href} failed");
    }
    Ok(rpms)
}

fn attr_in<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let start = text.find(prefix)? + prefix.len();
    let end = text[start..].find('"')? + start;
    Some(&text[start..end])
}

fn between<'a>(text: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = text.find(open)? + open.len();
    let end = text[start..].find(close)? + start;
    Some(&text[start..end])
}

fn parse_primary(reader: impl std::io::BufRead, repo: &str, wanted: &HashSet<String>) -> Result<Vec<Rpm>> {
    use quick_xml::events::Event;

    let mut xml = quick_xml::Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut out = Vec::new();
    let mut current: Option<Rpm> = None;
    let mut field: Option<&'static str> = None;

    loop {
        match xml.read_event_into(&mut buf)? {
            Event::Start(e) | Event::Empty(e) => match e.name().as_ref() {
                b"package" => {
                    current = Some(Rpm {
                        name: String::new(),
                        epoch: "0".into(),
                        version: String::new(),
                        release: String::new(),
                        arch: String::new(),
                        url: String::new(),
                        sha256: String::new(),
                    })
                }
                b"name" => field = Some("name"),
                b"arch" => field = Some("arch"),
                b"checksum" => {
                    let is_sha256 = e
                        .attributes()
                        .flatten()
                        .any(|a| a.key.as_ref() == b"type" && a.value.as_ref() == b"sha256");
                    field = is_sha256.then_some("sha256");
                }
                b"version" => {
                    if let Some(rpm) = current.as_mut() {
                        for a in e.attributes().flatten() {
                            let value = String::from_utf8_lossy(&a.value).into_owned();
                            match a.key.as_ref() {
                                b"epoch" => rpm.epoch = value,
                                b"ver" => rpm.version = value,
                                b"rel" => rpm.release = value,
                                _ => {}
                            }
                        }
                    }
                }
                b"location" => {
                    if let Some(rpm) = current.as_mut() {
                        for a in e.attributes().flatten() {
                            if a.key.as_ref() == b"href" {
                                rpm.url = format!("{repo}{}", String::from_utf8_lossy(&a.value));
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::Text(t) => {
                if let (Some(f), Some(rpm)) = (field, current.as_mut()) {
                    let text = String::from_utf8_lossy(&t).into_owned();
                    match f {
                        "name" => rpm.name = text,
                        "arch" => rpm.arch = text,
                        "sha256" => rpm.sha256 = text,
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                field = None;
                if e.name().as_ref() == b"package" {
                    if let Some(rpm) = current.take() {
                        if wanted.contains(&rpm.name)
                            && (rpm.arch == "x86_64" || rpm.arch == "noarch")
                            && !rpm.sha256.is_empty()
                        {
                            out.push(rpm);
                        }
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(out)
}

/// RPM's epoch:version-release ordering.
fn evr_cmp(a: &Rpm, b: &Rpm) -> Ordering {
    let epoch = |e: &str| e.parse::<u64>().unwrap_or(0);
    epoch(&a.epoch)
        .cmp(&epoch(&b.epoch))
        .then_with(|| rpmvercmp(&a.version, &b.version))
        .then_with(|| rpmvercmp(&a.release, &b.release))
}

/// rpm's `rpmvercmp`: alternating numeric and alphabetic segments, `~` sorts
/// before everything and `^` after the base version.
fn rpmvercmp(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a.as_bytes(), b.as_bytes());
    loop {
        a = skip_separators(a);
        b = skip_separators(b);
        match (a.first(), b.first()) {
            (Some(b'~'), Some(b'~')) => {
                a = &a[1..];
                b = &b[1..];
                continue;
            }
            (Some(b'~'), _) => return Ordering::Less,
            (_, Some(b'~')) => return Ordering::Greater,
            (Some(b'^'), Some(b'^')) => {
                a = &a[1..];
                b = &b[1..];
                continue;
            }
            (Some(b'^'), None) => return Ordering::Greater,
            (None, Some(b'^')) => return Ordering::Less,
            (Some(b'^'), _) => return Ordering::Less,
            (_, Some(b'^')) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            _ => {}
        }
        let numeric = a[0].is_ascii_digit();
        let take = |s: &[u8]| {
            s.iter()
                .take_while(|c| if numeric { c.is_ascii_digit() } else { c.is_ascii_alphabetic() })
                .count()
        };
        let (na, nb) = (take(a), take(b));
        if nb == 0 {
            // Numeric segments are newer than alphabetic ones.
            return if numeric { Ordering::Greater } else { Ordering::Less };
        }
        let (sa, sb) = (&a[..na], &b[..nb]);
        let ord = if numeric {
            let trim = |s: &[u8]| {
                let z = s.iter().take_while(|c| **c == b'0').count();
                s[z..].to_vec()
            };
            let (ta, tb) = (trim(sa), trim(sb));
            ta.len().cmp(&tb.len()).then_with(|| ta.cmp(&tb))
        } else {
            sa.cmp(sb)
        };
        if ord != Ordering::Equal {
            return ord;
        }
        a = &a[na..];
        b = &b[nb..];
    }
}

fn skip_separators(s: &[u8]) -> &[u8] {
    let n = s
        .iter()
        .take_while(|c| !c.is_ascii_alphanumeric() && **c != b'~' && **c != b'^')
        .count();
    &s[n..]
}

/// name -> NEVRA of the current lock, if there is one.
fn read_locked(path: &Path) -> BTreeMap<String, String> {
    let Ok(text) = fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    text.lines()
        .filter_map(|line| {
            let line = line.trim().strip_prefix("[\"")?;
            let mut parts = line.split("\", \"");
            Some((parts.next()?.to_string(), parts.next()?.to_string()))
        })
        .collect()
}

fn report_changes(locked: &BTreeMap<String, String>, newest: &BTreeMap<String, Rpm>) -> usize {
    let mut changes = 0;
    for (name, rpm) in newest {
        match locked.get(name) {
            Some(old) if *old == rpm.nevra() => {}
            Some(old) => {
                changes += 1;
                println!("  {name}: {old} -> {}", rpm.nevra());
            }
            None => {
                changes += 1;
                println!("  {name}: (new) {}", rpm.nevra());
            }
        }
    }
    for name in locked.keys().filter(|n| !newest.contains_key(*n)) {
        changes += 1;
        println!("  {name}: removed");
    }
    changes
}

/// Download each RPM into the cache (once) and check it against the index checksum.
fn download_verified<'a>(cache: &Path, rpms: impl Iterator<Item = &'a Rpm>) -> Result<Vec<PathBuf>> {
    fs::create_dir_all(cache)?;
    let mut files = Vec::new();
    for rpm in rpms {
        let file = cache.join(rpm.file_name());
        if !(file.is_file() && sha256_file(&file)? == rpm.sha256) {
            let partial = file.with_extension("part");
            curl(&rpm.url, &partial)?;
            if sha256_file(&partial)? != rpm.sha256 {
                let _ = fs::remove_file(&partial);
                bail!("{} does not match its repository checksum", rpm.url);
            }
            fs::rename(&partial, &file)?;
        }
        files.push(file);
    }
    Ok(files)
}

/// Every RPM must carry a valid signature from the release key.
fn check_signatures(key: &Path, files: &[PathBuf]) -> Result<()> {
    if !key.is_file() {
        bail!("signing key {} does not exist", key.display());
    }
    let db = tempfile::tempdir()?;
    let rpmkeys = |args: &[&std::ffi::OsStr]| -> Result<std::process::Output> {
        Command::new("rpmkeys")
            .arg("--dbpath")
            .arg(db.path())
            .args(args)
            .output()
            .context("running rpmkeys")
    };
    let import = rpmkeys(&["--import".as_ref(), key.as_os_str()])?;
    if !import.status.success() {
        bail!("importing {} failed: {}", key.display(), String::from_utf8_lossy(&import.stderr).trim());
    }
    let mut args: Vec<&std::ffi::OsStr> = vec!["--checksig".as_ref()];
    args.extend(files.iter().map(|f| f.as_os_str()));
    let out = rpmkeys(&args)?;
    let text = String::from_utf8_lossy(&out.stdout);
    let bad: Vec<&str> = text.lines().filter(|l| !l.ends_with("signatures OK")).collect();
    if !out.status.success() || !bad.is_empty() || text.lines().count() != files.len() {
        bail!(
            "RPM signature check failed:\n{}{}",
            bad.join("\n"),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    eprintln!("[ok] {} RPMs signed by {}", files.len(), key.file_name().unwrap_or_default().to_string_lossy());
    Ok(())
}

fn render_lock(distro: &Distro, release: u32, rpms: &BTreeMap<String, Rpm>) -> Result<String> {
    let today = String::from_utf8(Command::new("date").arg("+%F").output()?.stdout)?;
    let mut text = format!(
        "// Generated by `cargo xtask base lock {id}` from the Fedora {release} release and\n\
         // updates repositories; every RPM was checked against its repository\n\
         // checksum and the release signing key. Do not edit: rerun the command.\n\
         \n\
         fn release() {{ {release} }}\n\
         fn locked_on() {{ \"{date}\" }}\n\
         \n\
         // [name, NEVRA, URL, SHA256]\n\
         fn packages() {{\n    [\n",
        id = distro.id(),
        date = today.trim(),
    );
    for rpm in rpms.values() {
        text.push_str(&format!(
            "        [\"{}\", \"{}\", \"{}\", \"{}\"],\n",
            rpm.name,
            rpm.nevra(),
            rpm.url,
            rpm.sha256
        ));
    }
    text.push_str("    ]\n}\n");
    Ok(text)
}
