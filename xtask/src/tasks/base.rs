//! `base lock`: lock the packages a variant takes from its distribution base.
//!
//! The variant contract (`sources.rootfs_source.base`) names the release, its
//! signing key and the base packages; the group recipes of the public
//! catalogue (`recipes/*.rhai`, their `PACKAGES` seeds, see
//! [`group_seeds`]) name what the software installed through Recipe adds. For
//! an RPM variant this resolves each package and its closure
//! (the requires and provides in Fedora's release and updates primary
//! metadata, newest build of each name, no weak dependencies), checks every
//! RPM against the repository checksum and the release's signing key; an APK
//! variant goes through `apk_lock` (the same closure over Alpine's indexes).
//! Either writes three locks: the base, which the rootfs recipes import; the
//! desktop lock (the group recipes' seeds' closure minus what the base already
//! has), which `base groups` shares out into the recipes' pins and whose
//! preseed fills the release medium's package cache; and the apps-build layer
//! (what compiling the desktop apps in `tools/gui/` needs beyond the base),
//! which only the root they are compiled in gets.
//! Rerunning it is how a build takes the distribution's updates.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::cli::Distro;
use distro_contract::profile::PackageBackend;

const MIRROR: &str = "https://dl.fedoraproject.org/pub/fedora/linux";
const KOJI: &str = "https://kojipkgs.fedoraproject.org/packages";
/// Short id of Fedora 45's signing key (distro.toml `signing_key`). koji's
/// plain file of a build is the unsigned one; the signed RPM the repositories
/// carry, with the same checksum, is under `data/signed/<key id>/`. Change it
/// with the release.
const KOJI_KEY: &str = "f577861e";

/// One RPM build as a repository's primary index lists it.
#[derive(Clone, Debug)]
pub(super) struct Rpm {
    pub(super) name: String,
    pub(super) epoch: String,
    pub(super) version: String,
    pub(super) release: String,
    pub(super) arch: String,
    pub(super) url: String,
    pub(super) sha256: String,
    /// Everything the package provides: capabilities, sonames and the files
    /// the index lists.
    pub(super) provides: Vec<String>,
    /// What it requires (strong dependencies only), rich expressions as written.
    pub(super) requires: Vec<String>,
}

impl Rpm {
    pub(super) fn nevra(&self) -> String {
        format!("{}-{}-{}.{}", self.name, self.version, self.release, self.arch)
    }

    fn file_name(&self) -> &str {
        self.url.rsplit('/').next().unwrap_or(&self.url)
    }
}

/// Which lock: the base system's, what the group recipes add to it (no image
/// carries it; Recipe installs it), or what compiling the desktop apps adds to
/// the base (never in an image).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    Base,
    Desktop,
    AppsBuild,
}

impl Layer {
    pub const ALL: [Layer; 3] = [Layer::Base, Layer::Desktop, Layer::AppsBuild];

    pub fn name(self) -> &'static str {
        match self {
            Layer::Base => "base",
            Layer::Desktop => "desktop",
            Layer::AppsBuild => "apps-build",
        }
    }

    /// What the lock holds, for its heading.
    pub fn holds(self) -> &'static str {
        match self {
            Layer::Base => "the variant's packages and everything they need",
            Layer::Desktop => "what the group recipes' seeds need beyond the base lock",
            Layer::AppsBuild => "what compiling the desktop apps needs beyond the base lock",
        }
    }
}

/// The seeds a recipe's `PACKAGES` declaration names (docs/recipe-format.md,
/// section 2.6). Read from the recipe's text as a literal, not by running it
/// (the Recipe library's declaration reader insists on the pins file this
/// generator is about to write).
#[derive(Clone, Debug, Default)]
pub struct PackageSeeds {
    pub glibc: Vec<String>,
    pub musl: Vec<String>,
    /// 32-bit seeds, glibc only (`lib32`).
    pub glibc_i686: Vec<String>,
    /// The declaration is a map from C library to seeds. A plain list (the
    /// same packages on every system, `desktop-runtime`) is pinned by its own
    /// generator, so its names are not read here.
    pub by_libc: bool,
}

/// The quoted strings of `text`.
fn quoted(text: &str) -> Vec<String> {
    text.split('"').skip(1).step_by(2).map(str::to_string).collect()
}

/// `PACKAGES` of a recipe, or `None` when it declares none.
pub fn declared_packages(text: &str) -> Result<Option<PackageSeeds>> {
    const OPENER: &str = "const PACKAGES = ";
    let start = if text.starts_with(OPENER) {
        0
    } else if let Some(i) = text.find(&format!("\n{OPENER}")) {
        i + 1
    } else {
        return Ok(None);
    };
    let rest = &text[start + OPENER.len()..];
    let (closer, by_libc) = if rest.starts_with("#{") {
        ("};", true)
    } else if rest.starts_with('[') {
        ("];", false)
    } else {
        bail!("PACKAGES is a literal list of package names, or a map from C library to such lists");
    };
    // The declaration without its comments, up to its closing line.
    let mut body = String::new();
    let mut closed = false;
    for line in rest.lines() {
        let code = line.split("//").next().unwrap_or("");
        body.push_str(code);
        body.push('\n');
        if code.contains(closer) {
            closed = true;
            break;
        }
    }
    if !closed {
        bail!("PACKAGES has no closing `{closer}`");
    }
    if !by_libc {
        return Ok(Some(PackageSeeds::default()));
    }
    let field = |key: &str| -> Vec<String> {
        let pattern = format!("{key}:");
        let mut from = 0;
        while let Some(i) = body[from..].find(&pattern).map(|i| i + from) {
            let boundary = body[..i].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
            if boundary {
                let after = &body[i + pattern.len()..];
                if let (Some(open), Some(close)) = (after.find('['), after.find(']')) {
                    return quoted(&after[open..close]);
                }
            }
            from = i + pattern.len();
        }
        Vec::new()
    };
    Ok(Some(PackageSeeds {
        glibc: field("glibc"),
        musl: field("musl"),
        glibc_i686: field("glibc_i686"),
        by_libc,
    }))
}

/// Every seed the group recipes of the public catalogue (`recipes/*.rhai`)
/// name in `PACKAGES` for `backend`'s C library (`glibc` for RPM, `musl` for
/// APK), each once, in file then list order: what the desktop lock resolves.
/// A plain-list `PACKAGES` (`desktop-runtime`, pinned by its own generator)
/// and the 32-bit seeds are not part of the locks.
pub fn group_seeds(root: &Path, backend: PackageBackend) -> Result<Vec<String>> {
    let dir = root.join("recipes");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<std::io::Result<_>>()?;
    files.retain(|f| f.extension().is_some_and(|e| e == "rhai"));
    files.sort();
    let mut seeds: Vec<String> = Vec::new();
    for file in files {
        let text = fs::read_to_string(&file).with_context(|| format!("reading {}", file.display()))?;
        let Some(declared) = declared_packages(&text).with_context(|| format!("{}", file.display()))? else {
            continue;
        };
        let list = match backend {
            PackageBackend::Rpm => declared.glibc,
            PackageBackend::Apk => declared.musl,
        };
        for seed in list {
            if !seeds.contains(&seed) {
                seeds.push(seed);
            }
        }
    }
    if seeds.is_empty() {
        bail!("no group recipe in {} names a seed in its PACKAGES for this C library", dir.display());
    }
    Ok(seeds)
}

/// The lock file the rootfs recipes import.
pub fn lock_path(root: &Path, distro: &Distro, backend: PackageBackend, layer: Layer) -> PathBuf {
    let suffix = match layer {
        Layer::Base => "lock",
        Layer::Desktop => "desktop-lock",
        Layer::AppsBuild => "apps-build-lock",
    };
    root.join("distro/builder/os-recipes/sources")
        .join(format!("{}-{backend}-{suffix}.rhai", distro.id()))
}

pub fn lock(distro: &Distro, check: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let bundle = distro_contract::load(&root, distro.id())?;
    distro_builder::process::check_host_tools(distro_builder::process::HostNeed::BaseLock)
        .map_err(|e| crate::util::exit::missing(format!("{e:#}"), "cargo xtask doctor"))?;
    match bundle.contract.profile.packages {
        PackageBackend::Rpm => lock_rpm(&root, distro, &bundle, check),
        PackageBackend::Apk => super::apk_lock::lock(&root, distro, &bundle, check),
    }
}

/// The Rocky 10 repositories the compiler image is built from, and so the
/// ones its kernel build dependencies come from.
const ROCKY: &str = "https://download.rockylinux.org/pub/rocky/10";

/// The packages `linux-deps.rhai` installs for a kernel build (its list
/// before it read the lock this writes).
const KERNEL_DEPS: &[&str] = &[
    "flex",
    "bison",
    "m4",
    "bc",
    "gcc",
    "cpp",
    "gcc-c++",
    "make",
    "binutils",
    "perl",
    "elfutils-libelf",
    "elfutils-libelf-devel",
    "openssl",
    "openssl-libs",
    "openssl-devel",
    "zlib-ng-compat",
    "diffutils",
    "findutils",
];

/// `base kernel-deps`: resolve [`KERNEL_DEPS`] and their closure from the
/// Rocky 10 BaseOS and AppStream primary indexes (newest build of each
/// name, no weak dependencies), check every RPM against its repository
/// checksum and the Rocky 10 signing key, and write the lock
/// `linux-deps.rhai` downloads from. Like the base locks, rerunning it is
/// how a build takes Rocky's updates.
pub fn kernel_deps_lock(check: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    distro_builder::process::check_host_tools(distro_builder::process::HostNeed::BaseLock)
        .map_err(|e| crate::util::exit::missing(format!("{e:#}"), "cargo xtask doctor"))?;

    let mut newest: BTreeMap<String, Rpm> = BTreeMap::new();
    for repo in [format!("{ROCKY}/BaseOS/x86_64/os/"), format!("{ROCKY}/AppStream/x86_64/os/")] {
        eprintln!("[step] Read {repo}");
        primary_index(&repo, &mut newest)?;
    }
    let index = Index::new(newest);
    // What the compiler image already has, from its own rpmdb in the pinned
    // rootfs tar: only what those packages do not provide is locked, as
    // `dnf download --resolve` against that rpmdb used to download only
    // what was missing.
    let tar = root.join(".artifacts/ginux-env/prepared-image-build-tools/rootfs.tar");
    if !tar.is_file() {
        bail!(
            "the pinned compiler image {} is not produced; run any OS build first so Recipe stages it",
            tar.display()
        );
    }
    let db = tempfile::tempdir()?;
    let status = Command::new("tar")
        .args(["-xf"])
        .arg(&tar)
        .arg("-C")
        .arg(db.path())
        .arg("usr/lib/sysimage/rpm")
        .status()
        .context("running tar")?;
    if !status.success() {
        bail!("extracting the image rpmdb from {} failed ({status})", tar.display());
    }
    let out = Command::new("rpm")
        .arg("--dbpath")
        .arg(db.path().join("usr/lib/sysimage/rpm"))
        .args(["-qa", "--queryformat", "%{NAME}\n"])
        .output()
        .context("running rpm -qa on the image rpmdb")?;
    if !out.status.success() {
        bail!("rpm -qa on the image rpmdb failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    let image_names: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|name| index.pkgs.contains_key(*name))
        .map(str::to_string)
        .collect();
    let installed = index.resolve(&image_names, BTreeSet::new(), &HashSet::new())?;
    let wanted: Vec<String> = KERNEL_DEPS.iter().map(|name| name.to_string()).collect();
    let mut names = index.resolve(&wanted, installed.clone(), &HashSet::new())?;
    names.retain(|n| !installed.contains(n));
    // A kept package whose requirement only an image package provides
    // still needs that provider's files in the tools tree: openssl-devel's
    // libcrypto.so is a symlink whose target only openssl-libs ships, and
    // a dangling symlink is worse than none to the linker (likewise
    // elfutils-libelf-devel's -lelf). Add those providers back, to a
    // fixpoint over requires.
    loop {
        let mut provided: HashSet<&str> = HashSet::new();
        for name in &names {
            provided.insert(name.as_str());
            provided.extend(index.pkgs[name].provides.iter().map(String::as_str));
        }
        let mut additions: Vec<String> = Vec::new();
        for name in names.clone() {
            for req in &index.pkgs[&name].requires {
                let cap = req.split_whitespace().next().unwrap_or(req);
                if provided.contains(cap) || cap.starts_with('/') {
                    continue;
                }
                let Some(cands) = index.providers.get(cap) else { continue };
                let Some(pick) = cands.iter().find(|p| {
                    installed.contains(*p)
                        && !names.contains(*p)
                        // Only library and package-name requirements can
                        // dangle across the image boundary; the rest (a
                        // `passwd` capability, a shell) the image serves
                        // at run time as it served dnf.
                        && (cap.contains(".so") || cap == p.as_str() || cap.starts_with(&format!("{p}(")))
                }) else {
                    continue;
                };
                additions.push(pick.clone());
            }
        }
        if additions.is_empty() {
            break;
        }
        names.extend(additions);
    }
    let set: BTreeMap<String, Rpm> = names.iter().map(|n| (n.clone(), index.pkgs[n].clone())).collect();

    let path = root.join("distro/builder/os-recipes/sources/kernel-deps-lock.rhai");
    let locked = read_locked(&path);
    let changes = report_changes(&locked, &set);
    if check {
        if changes == 0 {
            println!("{} is current with Rocky 10 updates", rel(&root, &path));
            return Ok(());
        }
        bail!("{changes} package(s) differ from the kernel-deps lock; run cargo xtask base kernel-deps");
    }
    let cache = root.join(".artifacts/rpm-cache").join("rocky-10");
    let files = download_verified(&cache, set.values())?;
    check_signatures(&path.with_file_name("RPM-GPG-KEY-Rocky-10"), &files)?;
    fs::write(&path, render_kernel_deps_lock(&set)?).with_context(|| format!("writing {}", path.display()))?;
    println!("wrote {} ({} packages, {changes} changed)", rel(&root, &path), set.len());
    Ok(())
}

fn render_kernel_deps_lock(rpms: &BTreeMap<String, Rpm>) -> Result<String> {
    let today = String::from_utf8(Command::new("date").arg("+%F").output()?.stdout)?;
    let mut text = format!(
        "// Generated by `cargo xtask base kernel-deps` from the Rocky 10 BaseOS and\n\
         // AppStream repositories: the kernel build dependencies and everything they\n\
         // need, as their requires and provides resolve (no weak dependencies). Every\n\
         // RPM was checked against its repository checksum and the Rocky 10 signing\n\
         // key (`RPM-GPG-KEY-Rocky-10` beside this file, pinned by SHA-256 in Recipe's\n\
         // `tools/recipe/bootstrap/product-ci.json`). Do not edit: rerun the command.\n\
         \n\
         fn locked_on() {{ \"{date}\" }}\n\
         \n\
         // [name, NEVRA, URL, SHA256]\n\
         fn packages() {{\n    [\n",
        date = today.trim(),
    );
    for rpm in rpms.values() {
        text.push_str(&format!(
            "        [\"{}\", \"{}\", \"{}\", \"{}\"],\n",
            rpm.name, rpm.nevra(), rpm.url, rpm.sha256
        ));
    }
    text.push_str("    ]\n}\n");
    Ok(text)
}

fn lock_rpm(root: &Path, distro: &Distro, bundle: &distro_contract::Variant, check: bool) -> Result<()> {
    let base = &bundle.contract.sources.rootfs_source.base;
    let release: u32 = base
        .release
        .parse()
        .with_context(|| format!("base.release {:?} is not a Fedora release number", base.release))?;

    let mut newest: BTreeMap<String, Rpm> = BTreeMap::new();
    for repo in [
        format!("{MIRROR}/releases/{release}/Everything/x86_64/os/"),
        format!("{MIRROR}/updates/{release}/Everything/x86_64/"),
    ] {
        eprintln!("[step] Read {repo}");
        primary_index(&repo, &mut newest)?;
    }
    let index = Index::new(newest);
    let desktop_packages = &group_seeds(root, PackageBackend::Rpm)?;
    let assumed: HashSet<&str> = base.provided_elsewhere.iter().map(String::as_str).collect();
    let base_set = index.resolve(&base.packages, BTreeSet::new(), &assumed)?;
    let mut desktop_set = index.resolve(desktop_packages, base_set.clone(), &assumed)?;
    desktop_set.retain(|n| !base_set.contains(n));
    // Resolved over the base alone, from the same index as the desktop: a
    // package both need (Qt) is the same build in both locks.
    let mut apps_build_set =
        index.resolve(&bundle.contract.desktop.apps_build_packages, base_set.clone(), &assumed)?;
    apps_build_set.retain(|n| !base_set.contains(n));

    let mut stale = 0;
    for (layer, names) in
        [(Layer::Base, &base_set), (Layer::Desktop, &desktop_set), (Layer::AppsBuild, &apps_build_set)]
    {
        let set: BTreeMap<String, Rpm> = names.iter().map(|n| (n.clone(), index.pkgs[n].clone())).collect();
        let path = lock_path(root, distro, PackageBackend::Rpm, layer);
        let locked = read_locked(&path);
        let changes = report_changes(&locked, &set);
        if check {
            if changes == 0 {
                println!("{} is current with Fedora {release} updates", rel(root, &path));
            }
            stale += changes;
            continue;
        }
        let cache = root.join(".artifacts/rpm-cache").join(format!("fedora-{release}"));
        let files = download_verified(&cache, set.values())?;
        check_signatures(&bundle.path(&base.signing_key), &files)?;
        fs::write(&path, render_lock(distro, release, layer, &set)?)
            .with_context(|| format!("writing {}", path.display()))?;
        println!("wrote {} ({} packages, {changes} changed)", rel(root, &path), set.len());
    }
    if stale > 0 {
        bail!("{stale} package(s) differ from the locks; run cargo xtask base lock {}", distro.id());
    }
    Ok(())
}

/// Every x86_64/noarch package of the release and updates repositories (the
/// newest build of each name) with what it provides and requires.
pub(super) struct Index {
    pkgs: BTreeMap<String, Rpm>,
    /// capability -> names of the packages providing it
    providers: HashMap<String, Vec<String>>,
}

impl Index {
    pub(super) fn new(pkgs: BTreeMap<String, Rpm>) -> Self {
        let mut providers: HashMap<String, Vec<String>> = HashMap::new();
        for rpm in pkgs.values() {
            for cap in rpm.provides.iter().chain(std::iter::once(&rpm.name)) {
                let list = providers.entry(cap.clone()).or_default();
                if list.last() != Some(&rpm.name) {
                    list.push(rpm.name.clone());
                }
            }
        }
        Index { pkgs, providers }
    }

    /// The closure of `wanted` over `chosen`: strong requires only, each
    /// resolved against what is already chosen first, else the newest build
    /// of the package that provides it.
    pub(super) fn resolve(&self, wanted: &[String], chosen: BTreeSet<String>, assumed: &HashSet<&str>) -> Result<BTreeSet<String>> {
        let mut st = Solver {
            index: self,
            assumed,
            provided: HashSet::new(),
            chosen: BTreeSet::new(),
            todo: Vec::new(),
            prefer: wanted.iter().cloned().collect(),
        };
        for name in chosen {
            st.install(&name);
        }
        st.todo.clear();
        let missing: Vec<&String> = wanted.iter().filter(|n| !self.pkgs.contains_key(*n)).collect();
        if !missing.is_empty() {
            bail!(
                "Fedora has no x86_64/noarch build of: {}",
                missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            );
        }
        for name in wanted {
            st.install(name);
        }
        let mut unresolved = Vec::new();
        while let Some(name) = st.todo.pop() {
            for req in &self.pkgs[&name].requires {
                if !st.sat(req) {
                    unresolved.push(format!("{name} requires {req}"));
                }
            }
        }
        if !unresolved.is_empty() {
            unresolved.sort();
            unresolved.dedup();
            bail!("unresolvable requires:\n  {}", unresolved.join("\n  "));
        }
        Ok(st.chosen)
    }
}

struct Solver<'a> {
    index: &'a Index,
    assumed: &'a HashSet<&'a str>,
    provided: HashSet<&'a str>,
    chosen: BTreeSet<String>,
    todo: Vec<String>,
    prefer: HashSet<String>,
}

impl<'a> Solver<'a> {
    fn install(&mut self, name: &str) {
        if self.chosen.insert(name.to_string()) {
            let rpm = &self.index.pkgs[name];
            self.provided.insert(&rpm.name);
            self.provided.extend(rpm.provides.iter().map(String::as_str));
            self.todo.push(name.to_string());
        }
    }

    /// Whether `expr` (a capability or a rich dependency) is, or can now be, met.
    fn sat(&mut self, expr: &str) -> bool {
        let e = expr.trim();
        let Some(inner) = e.strip_prefix('(').and_then(|r| r.strip_suffix(')')) else {
            let cap = e.split_whitespace().next().unwrap_or(e);
            if cap.starts_with("rpmlib(") || self.assumed.contains(cap) || self.provided.contains(cap) {
                return true;
            }
            let Some(cands) = self.index.providers.get(cap) else { return false };
            let pick = cands
                .iter()
                .min_by_key(|n| (n.as_str() != cap, !self.prefer.contains(*n), n.len(), n.as_str()))
                .expect("providers are never empty")
                .clone();
            self.install(&pick);
            return true;
        };
        if split_top(inner, " if ").len() > 1 || split_top(inner, " unless ").len() > 1 {
            return true; // conditional: only when the condition holds, which is not asked
        }
        let alts = split_top(inner, " or ");
        if alts.len() > 1 {
            return alts.iter().any(|a| self.already(a)) || alts.iter().any(|a| self.sat(a));
        }
        let all = split_top(inner, " and ");
        if all.len() > 1 {
            let mut satisfied = true;
            for dependency in &all {
                // Resolve every conjunct, including after an unsatisfied one.
                let accepted = self.sat(dependency);
                satisfied = satisfied && accepted;
            }
            return satisfied;
        }
        let first = split_top(inner, " with ").remove(0);
        let first = split_top(&first, " without ").remove(0);
        self.sat(&first)
    }

    fn already(&self, expr: &str) -> bool {
        let e = expr.trim();
        match e.strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
            Some(inner) => split_top(inner, " or ").iter().any(|a| self.already(a)),
            None => self.provided.contains(e.split_whitespace().next().unwrap_or(e)),
        }
    }
}

/// `expr` split at `sep` outside parentheses.
fn split_top(expr: &str, sep: &str) -> Vec<String> {
    let (mut depth, mut start, mut out) = (0i32, 0, Vec::new());
    let bytes = expr.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && expr[i..].starts_with(sep) => {
                out.push(expr[start..i].to_string());
                i += sep.len();
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(expr[start..].to_string());
    out
}

pub(super) fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).display().to_string()
}

pub(super) fn curl(url: &str, out: &Path) -> Result<()> {
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

pub(super) fn sha256_file(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut file = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// The URL a package is pinned at. The releases repository never changes, so
/// its packages keep the mirror URL. Fedora deletes a superseded build from
/// the updates repository, so those packages are pinned at koji, which keeps
/// every build: `packages/<source name>/<version>/<release>/data/signed/<key>/<arch>/<file>`,
/// all three of source name, version and release taken from the package's
/// sourcerpm.
pub(super) fn pinned_url(repo: &str, href: &str, sourcerpm: &str, arch: &str) -> String {
    let mirror_url = format!("{repo}{href}");
    if !repo.contains("/updates/") {
        return mirror_url;
    }
    let Some(stem) = sourcerpm.strip_suffix(".src.rpm") else { return mirror_url };
    // name-version-release of the source package, which is not the binary's
    // own version (nspr 4.39.0-10 is built from nss 3.133.0-1).
    let mut parts = stem.rsplitn(3, '-');
    let (Some(release), Some(version), Some(source)) = (parts.next(), parts.next(), parts.next()) else {
        return mirror_url;
    };
    let file = href.rsplit('/').next().unwrap_or(href);
    format!("{KOJI}/{source}/{version}/{release}/data/signed/{KOJI_KEY}/{arch}/{file}")
}

/// Fold a repository's x86_64 and noarch packages into `newest`, keeping the
/// newest build of each name.
fn primary_index(repo: &str, newest: &mut BTreeMap<String, Rpm>) -> Result<()> {
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
    parse_primary(BufReader::new(child.stdout.take().unwrap()), repo, newest)?;
    if !child.wait()?.success() {
        bail!("decompressing {href} failed");
    }
    Ok(())
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

fn parse_primary(reader: impl std::io::BufRead, repo: &str, newest: &mut BTreeMap<String, Rpm>) -> Result<()> {
    use quick_xml::events::Event;

    let mut xml = quick_xml::Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut current: Option<Rpm> = None;
    let mut href = String::new();
    let mut sourcerpm = String::new();
    let mut field: Option<&'static str> = None;
    // Which dependency list an `rpm:entry` belongs to; weak ones are skipped.
    let mut deps = "";

    loop {
        match xml.read_event_into(&mut buf)? {
            Event::Start(e) | Event::Empty(e) => match e.name().as_ref() {
                b"package" => {
                    href.clear();
                    sourcerpm.clear();
                    current = Some(Rpm {
                        name: String::new(),
                        epoch: "0".into(),
                        version: String::new(),
                        release: String::new(),
                        arch: String::new(),
                        url: String::new(),
                        sha256: String::new(),
                        provides: Vec::new(),
                        requires: Vec::new(),
                    })
                }
                b"rpm:provides" => deps = "provides",
                b"rpm:requires" => deps = "requires",
                b"rpm:recommends" | b"rpm:suggests" | b"rpm:supplements" | b"rpm:enhances" | b"rpm:conflicts"
                | b"rpm:obsoletes" => deps = "",
                b"rpm:entry" => {
                    if let Some(rpm) = current.as_mut() {
                        let name = e.attributes().flatten().find(|a| a.key.as_ref() == b"name").map(|a| {
                            String::from_utf8_lossy(&a.value)
                                .replace("&lt;", "<")
                                .replace("&gt;", ">")
                                .replace("&amp;", "&")
                        });
                        match (deps, name) {
                            ("provides", Some(n)) => rpm.provides.push(n),
                            ("requires", Some(n)) if !n.starts_with("rpmlib(") => rpm.requires.push(n),
                            _ => {}
                        }
                    }
                }
                b"file" => field = Some("file"),
                b"name" => field = Some("name"),
                b"arch" => field = Some("arch"),
                b"rpm:sourcerpm" => field = Some("sourcerpm"),
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
                    for a in e.attributes().flatten() {
                        if a.key.as_ref() == b"href" {
                            href = String::from_utf8_lossy(&a.value).into_owned();
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
                        "sourcerpm" => sourcerpm = text,
                        "file" => rpm.provides.push(text),
                        _ => {}
                    }
                }
            }
            Event::End(e) => {
                field = None;
                if matches!(e.name().as_ref(), b"rpm:provides" | b"rpm:requires") {
                    deps = "";
                }
                if e.name().as_ref() == b"package"
                    && let Some(mut rpm) = current.take()
                    && {
                        rpm.url = pinned_url(repo, &href, &sourcerpm, &rpm.arch);
                        true
                    }
                    && (rpm.arch == "x86_64" || rpm.arch == "noarch")
                    && !rpm.sha256.is_empty()
                    && newest.get(&rpm.name).is_none_or(|cur| evr_cmp(&rpm, cur) == Ordering::Greater)
                {
                    newest.insert(rpm.name.clone(), rpm);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(())
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
pub(super) fn read_locked(path: &Path) -> BTreeMap<String, String> {
    let Ok(text) = fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    text.lines()
        .skip_while(|line| !line.starts_with("fn packages()"))
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

fn render_lock(distro: &Distro, release: u32, layer: Layer, rpms: &BTreeMap<String, Rpm>) -> Result<String> {
    let today = String::from_utf8(Command::new("date").arg("+%F").output()?.stdout)?;
    let mut text = format!(
        "// Generated by `cargo xtask base lock {id}` from the Fedora {release} release and\n\
         // updates repositories: {what}, as their requires and provides\n\
         // resolve (no weak dependencies). Every RPM was checked against its\n\
         // repository checksum and the release signing key. Do not edit: rerun the\n\
         // command.\n\
         \n\
         fn release() {{ {release} }}\n\
         fn locked_on() {{ \"{date}\" }}\n\
         \n\
         // [name, NEVRA, URL, SHA256]\n\
         fn packages() {{\n    [\n",
        id = distro.id(),
        what = layer.holds(),
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
