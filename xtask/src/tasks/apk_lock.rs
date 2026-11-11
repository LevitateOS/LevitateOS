//! `base lock` for an Alpine variant: the closure of `base.packages` over the
//! release's main and community repositories, as apk resolves it
//! (dependencies, provides, install_if), written as the lock the APK producer
//! imports. Every downloaded file is then checked by apk itself against the
//! variant's release key, which also checks the repository indexes.

use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::base::{Layer, curl, read_locked, rel, sha256_file};
use crate::cli::Distro;
use distro_contract::profile::PackageBackend;

const MIRROR: &str = "https://dl-cdn.alpinelinux.org/alpine";
const REPOS: [&str; 2] = ["main", "community"];

/// One package as an APKINDEX stanza lists it.
#[derive(Debug, Default)]
struct Pkg {
    name: String,
    version: String,
    repo: &'static str,
    provides: Vec<String>,
    depends: Vec<String>,
    install_if: Vec<String>,
    priority: u32,
}

/// `name` out of `name`, `name=1`, `name>=1`, `so:libc.musl-x86_64.so.1`.
fn name_of(token: &str) -> &str {
    token.split(['=', '<', '>', '~']).next().unwrap_or(token)
}

/// apk's version order, close enough to pick a repository's newest build:
/// digit runs compare as numbers, letter runs as text.
fn version_key(v: &str) -> Vec<(u8, u64, String)> {
    let mut out = Vec::new();
    let mut rest = v.replacen("-r", ".", 1);
    rest.push('.');
    let bytes: Vec<char> = rest.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let run = |pred: fn(&char) -> bool| bytes[i..].iter().take_while(|c| pred(c)).count();
        if c.is_ascii_digit() {
            let n = run(char::is_ascii_digit);
            out.push((1, bytes[i..i + n].iter().collect::<String>().parse().unwrap_or(0), String::new()));
            i += n;
        } else if c.is_ascii_alphabetic() {
            let n = run(char::is_ascii_alphabetic);
            out.push((0, 0, bytes[i..i + n].iter().collect()));
            i += n;
        } else {
            i += 1;
        }
    }
    out
}

fn parse_index(text: &str, repo: &'static str, out: &mut Vec<Pkg>) {
    let words = |v: &str| v.split_whitespace().map(str::to_string).collect::<Vec<_>>();
    for stanza in text.split("\n\n").filter(|s| !s.trim().is_empty()) {
        let mut p = Pkg { repo, ..Pkg::default() };
        for line in stanza.lines() {
            let Some((k, v)) = line.split_once(':') else { continue };
            match k {
                "P" => p.name = v.to_string(),
                "V" => p.version = v.to_string(),
                "p" => p.provides = words(v),
                "D" => p.depends = words(v),
                "i" => p.install_if = words(v),
                "k" => p.priority = v.parse().unwrap_or(0),
                _ => {}
            }
        }
        out.push(p);
    }
}

struct Index {
    pkgs: Vec<Pkg>,
    by_name: HashMap<String, Vec<usize>>,
    provided: HashMap<String, Vec<usize>>,
}

impl Index {
    fn new(pkgs: Vec<Pkg>) -> Self {
        let (mut by_name, mut provided): (HashMap<_, Vec<usize>>, HashMap<_, Vec<usize>>) = Default::default();
        for (i, p) in pkgs.iter().enumerate() {
            by_name.entry(p.name.clone()).or_default().push(i);
            for prov in &p.provides {
                provided.entry(name_of(prov).to_string()).or_default().push(i);
            }
        }
        Index { pkgs, by_name, provided }
    }

    fn latest(&self, cands: &[usize]) -> usize {
        *cands
            .iter()
            .max_by_key(|&&i| (self.pkgs[i].priority, version_key(&self.pkgs[i].version)))
            .expect("candidates are never empty")
    }

    /// The closure of `wanted` over `chosen`, as package indexes by name.
    fn resolve(&self, wanted: &[String], mut chosen: BTreeMap<String, usize>) -> Result<BTreeMap<String, usize>> {
        let satisfied = |chosen: &BTreeMap<String, usize>, n: &str| {
            chosen.contains_key(n)
                || chosen
                    .values()
                    .any(|&i| self.pkgs[i].provides.iter().any(|p| name_of(p) == n))
        };
        let add = |chosen: &mut BTreeMap<String, usize>, start: &str| -> Result<()> {
            let mut stack = vec![start.to_string()];
            while let Some(n) = stack.pop() {
                if n.starts_with('!') || satisfied(chosen, &n) {
                    continue;
                }
                let pick = match self.by_name.get(&n) {
                    Some(c) => self.latest(c),
                    None => {
                        let c = self.provided.get(&n).with_context(|| format!("nothing provides {n}"))?;
                        c.iter().copied().find(|&i| chosen.contains_key(&self.pkgs[i].name)).unwrap_or_else(|| self.latest(c))
                    }
                };
                let p = &self.pkgs[pick];
                if chosen.insert(p.name.clone(), pick).is_none() {
                    stack.extend(p.depends.iter().filter(|d| !d.starts_with('!')).map(|d| name_of(d).to_string()));
                }
            }
            Ok(())
        };
        for w in wanted {
            add(&mut chosen, w)?;
        }
        // install_if packages join once all their conditions are installed.
        // Growth is counted by the chosen set, not by attempts: a package whose
        // name another chosen package already provides adds nothing, and
        // counting it would loop forever.
        loop {
            let mut grew = false;
            for (i, p) in self.pkgs.iter().enumerate() {
                if !p.install_if.is_empty()
                    && !chosen.contains_key(&p.name)
                    && p.install_if.iter().all(|t| satisfied(&chosen, name_of(t)))
                    && self.latest(&self.by_name[&p.name]) == i
                {
                    let before = chosen.len();
                    add(&mut chosen, &p.name)?;
                    grew |= chosen.len() > before;
                }
            }
            if !grew {
                return Ok(chosen);
            }
        }
    }
}

fn fetch_index(tmp: &Path, mirror: &str) -> Result<Index> {
    let mut pkgs = Vec::new();
    for repo in REPOS {
        let url = format!("{mirror}/{repo}/x86_64/APKINDEX.tar.gz");
        eprintln!("[step] Read {url}");
        let file = tmp.join(format!("APKINDEX-{repo}.tar.gz"));
        curl(&url, &file)?;
        let out = Command::new("tar").arg("-xzOf").arg(&file).arg("APKINDEX").output().context("running tar")?;
        if !out.status.success() {
            bail!("{url} holds no APKINDEX");
        }
        parse_index(&String::from_utf8_lossy(&out.stdout), repo, &mut pkgs);
    }
    Ok(Index::new(pkgs))
}

pub fn lock(root: &Path, distro: &Distro, bundle: &distro_contract::Variant, check: bool) -> Result<()> {
    let base = &bundle.contract.sources.rootfs_source.base;
    let release = &base.release;
    let mirror = format!("{MIRROR}/{release}");
    let tmp = tempfile::tempdir()?;
    let index = fetch_index(tmp.path(), &mirror)?;
    let base_set = index.resolve(&base.packages, BTreeMap::new())?;
    let mut desktop_set = index.resolve(&bundle.contract.desktop.packages, base_set.clone())?;
    desktop_set.retain(|n, _| !base_set.contains_key(n));
    // Resolved over the base alone, from the same index as the desktop: a
    // package both need (Qt) is the same build in both locks.
    let mut apps_build_set = index.resolve(&bundle.contract.desktop.apps_build_packages, base_set.clone())?;
    apps_build_set.retain(|n, _| !base_set.contains_key(n));
    let apk_tools = index
        .by_name
        .get("apk-tools-static")
        .map(|c| index.latest(c))
        .with_context(|| format!("{release} has no apk-tools-static"))?;
    let (tools_nv, tools_url) = row(&index.pkgs[apk_tools], &mirror)?;

    let mut stale = 0;
    for (layer, set) in
        [(Layer::Base, &base_set), (Layer::Desktop, &desktop_set), (Layer::AppsBuild, &apps_build_set)]
    {
        let rows: Vec<(&str, String, String)> = set
            .iter()
            .map(|(name, &i)| row(&index.pkgs[i], &mirror).map(|(nv, url)| (name.as_str(), nv, url)))
            .collect::<Result<_>>()?;
        let path = super::base::lock_path(root, distro, PackageBackend::Apk, layer);
        let locked = read_locked(&path);
        let changes = report_changes(&locked, &rows);

        if check {
            if changes == 0 {
                println!("{} is current with Alpine {release}", rel(root, &path));
            }
            stale += changes;
            continue;
        }

        let cache = root.join(".artifacts/apk-cache").join(release);
        fs::create_dir_all(&cache)?;
        let mut downloads = Vec::new();
        for url in std::iter::once(&tools_url).chain(rows.iter().map(|r| &r.2)) {
            let file = cache.join(url.rsplit('/').next().unwrap_or(url));
            if !file.is_file() {
                let partial = file.with_extension("part");
                curl(url, &partial)?;
                fs::rename(&partial, &file)?;
            }
            downloads.push((url.clone(), sha256_file(&file)?, file));
        }

        let key = bundle.path(&base.signing_key);
        let keys_dir = key.parent().context("signing key has no directory")?.canonicalize()?;
        verify(&keys_dir, tmp.path(), &mirror, &downloads)?;

        let pem = fs::read_to_string(&key).with_context(|| format!("reading {}", key.display()))?;
        let key_name = key.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let sha_of = |url: &str| downloads.iter().find(|d| d.0 == url).map(|d| d.1.clone()).unwrap_or_default();
        let rows = rows.into_iter().map(|(name, version, url)| {
            let digest = sha_of(&url);
            (name, version, url, digest)
        }).collect::<Vec<_>>();
        fs::write(
            &path,
            render_lock(distro, layer, release, &mirror, (&key_name, &pem), (&tools_nv, &tools_url, &sha_of(&tools_url)), &rows)?,
        )
        .with_context(|| format!("writing {}", path.display()))?;
        println!("wrote {} ({} packages, {changes} changed)", rel(root, &path), rows.len());
    }
    if stale > 0 {
        bail!("{stale} package(s) differ from the locks; run cargo xtask base lock {}", distro.id());
    }
    Ok(())
}

fn row(p: &Pkg, mirror: &str) -> Result<(String, String)> {
    let nv = format!("{}-{}", p.name, p.version);
    let url = format!("{mirror}/{}/x86_64/{nv}.apk", p.repo);
    Ok((nv, url))
}

fn report_changes(locked: &BTreeMap<String, String>, rows: &[(&str, String, String)]) -> usize {
    let mut changes = 0;
    for (name, nv, _) in rows {
        match locked.get(*name) {
            Some(old) if old == nv => {}
            Some(old) => {
                changes += 1;
                println!("  {name}: {old} -> {nv}");
            }
            None => {
                changes += 1;
                println!("  {name}: (new) {nv}");
            }
        }
    }
    for name in locked.keys().filter(|n| !rows.iter().any(|r| r.0 == n.as_str())) {
        changes += 1;
        println!("  {name}: removed");
    }
    changes
}

/// apk checks every package's signature, and the repository indexes, against
/// only the variant's key. The first download is apk-tools-static, whose
/// binary does the checking (and checks itself too).
fn verify(keys_dir: &Path, tmp: &Path, mirror: &str, downloads: &[(String, String, PathBuf)]) -> Result<()> {
    let tools = &downloads[0].2;
    let status = Command::new("tar")
        .arg("-xzf")
        .arg(tools)
        .arg("-C")
        .arg(tmp)
        .arg("sbin/apk.static")
        .stderr(std::process::Stdio::null())
        .status()
        .context("running tar")?;
    if !status.success() {
        bail!("{} holds no sbin/apk.static", tools.display());
    }
    let apk = tmp.join("sbin/apk.static");
    let mut verify = Command::new(&apk);
    verify.arg("--keys-dir").arg(keys_dir).arg("verify");
    verify.args(downloads.iter().map(|d| &d.2));
    let out = verify.output().context("running apk.static")?;
    if !out.status.success() {
        bail!("APK signature check failed:\n{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }
    let root = tmp.join("root");
    fs::create_dir_all(root.join("lib/apk/db"))?;
    fs::create_dir_all(root.join("etc/apk"))?;
    fs::write(root.join("lib/apk/db/installed"), "")?;
    fs::write(root.join("etc/apk/world"), "")?;
    let mut update = Command::new(&apk);
    update
        .args(["--no-cache", "--root"])
        .arg(&root)
        .arg("--keys-dir")
        .arg(keys_dir);
    for repo in REPOS {
        update.arg("-X").arg(format!("{mirror}/{repo}"));
    }
    let out = update.arg("update").output().context("running apk.static update")?;
    if !out.status.success() {
        bail!("APK index signature check failed:\n{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }
    eprintln!("[ok] {} APKs and both indexes signed by the release key", downloads.len());
    Ok(())
}

fn render_lock(
    distro: &Distro,
    layer: Layer,
    release: &str,
    mirror: &str,
    key: (&str, &str),
    tools: (&str, &str, &str),
    rows: &[(&str, String, String, String)],
) -> Result<String> {
    let today = String::from_utf8(Command::new("date").arg("+%F").output()?.stdout)?;
    let mut text = format!(
        "// Generated by `cargo xtask base lock {id}` from the Alpine {release} main and\n\
         // community repositories: {what},\n\
         // as apk resolves them (dependencies, provides, install_if). Every APK\n\
         // verified against the release signing key below. Do not edit: rerun the\n\
         // command.\n\
         \n\
         fn release() {{ \"{release}\" }}\n\
         fn locked_on() {{ \"{date}\" }}\n\
         fn mirror() {{ \"{mirror}\" }}\n\
         \n\
         // The key Alpine signs the release's packages with: [file name, PEM].\n\
         fn signing_key() {{\n    [\"{key_name}\", \"{pem}\"]\n}}\n\
         \n\
         // apk itself, which installs the packages: [name, name-version, URL, SHA256].\n\
         fn apk_tools() {{\n    [\"apk-tools-static\", \"{tools_nv}\", \"{tools_url}\", \"{tools_sha}\"]\n}}\n\
         \n\
         // [name, name-version, URL, SHA256]\n\
         fn packages() {{\n    [\n",
        id = distro.id(),
        what = layer.holds(),
        date = today.trim(),
        key_name = key.0,
        pem = key.1.replace('\n', "\\n"),
        tools_nv = tools.0,
        tools_url = tools.1,
        tools_sha = tools.2,
    );
    for (name, nv, url, sha) in rows {
        text.push_str(&format!("        [\"{name}\", \"{nv}\", \"{url}\", \"{}\"],\n", sha));
    }
    text.push_str("    ]\n}\n");
    Ok(text)
}
