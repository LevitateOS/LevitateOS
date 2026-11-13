//! `release recipe`: the standalone Recipe release (package name `recipe-pm`).
//!
//! Takes explicit inputs (the static `recipe`, the Ginux launcher, the runtime reference, the
//! recipes catalog), lets Recipe stage its own runtime tree, and packs that tree into every
//! release format. Nothing here needs rpmbuild, dpkg or abuild: RPM, deb and apk are written
//! in-process so the release builds on any host. The design is docs/recipe-standalone-release.md.
use anyhow::{Context, Result, bail, ensure};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const PKG: &str = "recipe-pm";
const HOMEPAGE: &str = "https://github.com/LevitateOS/recipe";
const RELEASE_REPO: &str = "LevitateOS/recipe";
const PREBUILT_REPO: &str = "LevitateOS/prebuilt";
const MAINTAINER: &str = "LevitateOS <noreply@levitateos.org>";
const LICENSE: &str = "MIT OR Apache-2.0";
const SUMMARY: &str = "Recipe, the package manager that builds software inside Ginux";
const DESCRIPTION: &str = "Recipe acquires, builds, installs, updates and removes software from \
recipes, running every build inside the Ginux sandbox. The package holds the static recipe \
command, the Ginux launcher and the official recipe source. The runtime image (about 330 MB) \
is downloaded on first use.";
const CONFFILES: [&str; 2] = ["etc/profile.d/recipe.sh", "etc/apparmor.d/recipe-ginux"];
const PROFILE: &str = "/etc/apparmor.d/recipe-ginux";
const MANIFEST: &str = "/usr/local/share/recipe/install-manifest";

const USAGE: &str = "usage: cargo xtask release recipe [--version V] [--out DIR] [--recipe-bin FILE] [--launcher FILE] [--publish]

Builds the standalone Recipe release (package recipe-pm): tarball with install.sh, a
packager tarball, RPM, deb and apk, the AUR and Void build files, SHA256SUMS,
provenance.json and RELEASE_NOTES.md, into .artifacts/recipe-release/<version>/.

  --version V        release version (default: version in tools/recipe/Cargo.toml)
  --out DIR          output folder (default .artifacts/recipe-release/<version>; must be empty)
  --recipe-bin FILE  use this recipe instead of building the static one with distro-builder
                     (not static: the release is marked unofficial and cannot be published)
  --launcher FILE    use this Ginux launcher instead of building tools/ginux
  --publish          create the GitHub release in LevitateOS/recipe (official builds only)";

struct Args {
    version: Option<String>,
    out: Option<PathBuf>,
    recipe_bin: Option<PathBuf>,
    launcher: Option<PathBuf>,
    publish: bool,
}

fn parse(args: &[String]) -> Result<Args> {
    let mut a = Args { version: None, out: None, recipe_bin: None, launcher: None, publish: false };
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut value = |name: &str| {
            it.next().cloned().with_context(|| format!("{name} needs a value\n\n{USAGE}"))
        };
        match arg.as_str() {
            "--version" => a.version = Some(value("--version")?),
            "--out" => a.out = Some(value("--out")?.into()),
            "--recipe-bin" => a.recipe_bin = Some(value("--recipe-bin")?.into()),
            "--launcher" => a.launcher = Some(value("--launcher")?.into()),
            "--publish" => a.publish = true,
            other => bail!("unknown argument {other}\n\n{USAGE}"),
        }
    }
    Ok(a)
}

pub fn run(args: Vec<String>) -> Result<()> {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    let args = parse(&args)?;
    let root = crate::util::repo::repo_root()?;
    let recipe_dir = root.join("tools/recipe");
    let ginux_dir = root.join("tools/ginux");
    let catalog = root.join("recipes");

    let version = match args.version.clone() {
        Some(v) => v,
        None => cargo_version(&recipe_dir.join("Cargo.toml"))?,
    };
    ensure!(
        !version.is_empty() && version.chars().all(|c| c.is_ascii_digit() || c == '.'),
        "version {version:?} must be plain numbers and dots (the same string goes into RPM, deb, apk, AUR and Void); pass --version"
    );
    let out = args
        .out
        .clone()
        .unwrap_or_else(|| root.join(".artifacts/recipe-release").join(&version));
    if out.exists() && fs::read_dir(&out)?.next().is_some() {
        bail!("{} is not empty; remove it or pass --out DIR", out.display());
    }

    // Source identities.
    let sources = [
        ("tools/recipe", recipe_dir.clone()),
        ("tools/ginux", ginux_dir.clone()),
        ("recipes", catalog.clone()),
        ("superproject", root.clone()),
    ];
    let mut identities = serde_json::Map::new();
    let mut official = true;
    let mut dirty_names = vec![];
    for (name, dir) in &sources {
        let commit = git(dir, &["rev-parse", "HEAD"])?;
        // Submodule pointers are covered by their own entries.
        let dirty = !git(dir, &["status", "--porcelain", "--ignore-submodules=all"])?.is_empty();
        if dirty {
            official = false;
            dirty_names.push(*name);
        }
        println!("source   {name:<13} {commit}{}", if dirty { "  (dirty)" } else { "" });
        identities.insert((*name).into(), json!({ "commit": commit, "dirty": dirty }));
    }
    let recipe_commit = git(&recipe_dir, &["rev-parse", "HEAD"])?;
    let mtime: u64 = git(&recipe_dir, &["log", "-1", "--format=%ct"])?.parse()?;

    // Inputs.
    let mut inputs = serde_json::Map::new();
    let mut recipe_key = None;
    let (recipe_bin, recipe_origin) = match &args.recipe_bin {
        Some(p) => (p.clone(), "--recipe-bin"),
        None => {
            let (key, dir) = build_recipe(&root)?;
            recipe_key = Some(key);
            (dir.join("recipe"), "distro-builder prebuilt")
        }
    };
    ensure!(recipe_bin.is_file(), "recipe binary {} does not exist", recipe_bin.display());
    if !is_static_x86_64_elf(&recipe_bin)? {
        let msg = format!("{} is not a static x86_64 ELF (it would not run on any x86_64 Linux)", recipe_bin.display());
        if args.recipe_bin.is_none() {
            bail!("{msg}; `distro-builder prebuilt build tools levitate-recipe` should produce a static musl binary, check its output");
        }
        eprintln!("WARNING: {msg}. This release is marked \"official\": false and cannot be published.");
        official = false;
    }
    let launcher = match &args.launcher {
        Some(p) => p.clone(),
        None => build_launcher(&root, &ginux_dir)?,
    };
    ensure!(launcher.is_file(), "launcher {} does not exist", launcher.display());
    if !is_static_x86_64_elf(&launcher)? {
        let msg = format!("{} is not a static x86_64 ELF", launcher.display());
        if args.launcher.is_none() {
            bail!("{msg}; the Ginux build in tools/ginux should be freestanding, check tools/ginux/.cargo/config.toml");
        }
        eprintln!("WARNING: {msg}. This release is marked \"official\": false and cannot be published.");
        official = false;
    }
    let (reference, image_sha) = runtime_reference(&recipe_dir)?;
    let short = &image_sha[..16];
    check_runtime_release(short)?;
    ensure!(catalog.join("index.json").is_file(), "recipes/ has no index.json; run `git submodule update --init recipes`");
    for (name, path) in [("recipe", &recipe_bin), ("launcher", &launcher), ("catalog index", &catalog.join("index.json"))] {
        let hash = sha256_file(path)?;
        println!("input    {name:<13} {hash}  {}", path.display());
        inputs.insert(name.into(), json!({ "path": path.display().to_string(), "sha256": hash }));
    }
    println!("input    reference     {reference}");

    if args.publish {
        ensure!(
            official,
            "refusing --publish: this build is not official ({}). Build from clean sources with the static recipe from distro-builder and the launcher from tools/ginux",
            if dirty_names.is_empty() { "an input is not a static ELF".to_string() } else { format!("dirty: {}", dirty_names.join(", ")) }
        );
        check_publishable(&recipe_dir, &recipe_commit, &version)?;
    }

    // Stage.
    fs::create_dir_all(&out)?;
    let out = out.canonicalize()?;
    let stage_usr = out.join("stage-usr");
    let stage_local = out.join("stage-local");
    for (dir, prefix) in [(&stage_usr, "/usr"), (&stage_local, "/usr/local")] {
        stage(&recipe_bin, dir, prefix, &launcher, &reference, &catalog, &recipe_dir, &ginux_dir)?;
        println!("staged   {} (prefix {prefix})", dir.display());
    }
    let usr = collect(&stage_usr)?;
    let local = collect(&stage_local)?;

    // Assets.
    let v = &version;
    let sha_of = |name: &str| sha256_file(&out.join(name));
    let mut assets: Vec<String> = vec![];

    let tarball = format!("{PKG}-{v}-x86_64.tar.zst");
    let top = format!("{PKG}-{v}");
    let mut items = prefixed(&local, &format!("{top}/root/"));
    items.push(dir_item(&top, mtime));
    items.push(dir_item(&format!("{top}/root"), mtime));
    for (name, body, exec) in [
        ("install.sh", install_sh(), true),
        ("uninstall.sh", uninstall_sh(), true),
        ("README.txt", readme_txt(v), false),
    ] {
        items.push(bytes_item(&format!("{top}/{name}"), body.into_bytes(), exec, mtime));
    }
    write_zst(&out.join(&tarball), &tar_bytes(&mut items, "", false, true, mtime)?)?;
    assets.push(tarball);

    let usr_tarball = format!("{PKG}-{v}-x86_64-usr.tar.zst");
    let mut items = usr.clone();
    write_zst(&out.join(&usr_tarball), &tar_bytes(&mut items, "", false, true, mtime)?)?;
    assets.push(usr_tarball.clone());

    let rpm_name = format!("{PKG}-{v}-1.x86_64.rpm");
    build_rpm(&usr, v, mtime, &out.join(&rpm_name))?;
    assets.push(rpm_name);

    let deb_name = format!("{PKG}_{v}-1_amd64.deb");
    build_deb(&usr, v, mtime, &out.join(&deb_name))?;
    assets.push(deb_name);

    let apk_name = format!("{PKG}-{v}-r0.x86_64.apk");
    build_apk(&usr, v, mtime, &out.join(&apk_name))?;
    assets.push(apk_name);

    let usr_sha = sha_of(&usr_tarball)?;
    let url = format!("{HOMEPAGE}/releases/download/{v}/{usr_tarball}");
    fs::create_dir_all(out.join("aur"))?;
    fs::write(out.join("aur/PKGBUILD"), pkgbuild(v, &usr_tarball, &url, &usr_sha))?;
    fs::write(out.join("aur/.SRCINFO"), srcinfo(v, &usr_tarball, &url, &usr_sha))?;
    fs::write(out.join("aur/recipe-pm.install"), aur_install())?;
    fs::create_dir_all(out.join("void/srcpkgs/recipe-pm"))?;
    fs::write(out.join("void/srcpkgs/recipe-pm/template"), void_template(v, &url, &usr_sha))?;

    let mut sums = String::new();
    let mut asset_info = serde_json::Map::new();
    let mut sorted = assets.clone();
    sorted.sort();
    for name in &sorted {
        let hash = sha_of(name)?;
        let size = fs::metadata(out.join(name))?.len();
        sums.push_str(&format!("{hash}  {name}\n"));
        asset_info.insert(name.clone(), json!({ "sha256": hash, "size": size }));
    }
    fs::write(out.join("SHA256SUMS"), &sums)?;

    let built = git(&root, &["show", "-s", "--format=%cI", "HEAD"]).ok();
    let build_time = Command::new("date").args(["-u", "+%Y-%m-%dT%H:%M:%SZ"]).output()?;
    let provenance = json!({
        "package": PKG,
        "version": version,
        "official": official,
        "source_date_epoch": mtime,
        "sources": identities,
        "runtime_reference": reference,
        "runtime_release": format!("recipe-runtime-{short}"),
        "recipe_origin": recipe_origin,
        "recipe_prebuilt_key": recipe_key,
        "inputs": inputs,
        "assets": asset_info,
        "superproject_commit_time": built,
        "build_time": String::from_utf8_lossy(&build_time.stdout).trim(),
    });
    fs::write(out.join("provenance.json"), serde_json::to_string_pretty(&provenance)? + "\n")?;
    fs::write(out.join("RELEASE_NOTES.md"), release_notes(v, official))?;

    for name in &sorted {
        println!("asset    {:>10} B  {name}", fs::metadata(out.join(name))?.len());
    }
    println!("wrote    {}", out.display());
    if !official {
        println!("NOTE     this release is not official (dirty sources or a non-static input); --publish will refuse it");
    }

    if args.publish {
        publish(&out, v, &recipe_commit, &sorted)?;
    }
    println!(
        "\nNot published by this command:
  AUR:  copy {0}/aur/* into a clone of ssh://aur@aur.archlinux.org/recipe-pm.git, then `git add . && git commit && git push` (needs the GitHub release to exist first).
  Void: copy {0}/void/srcpkgs/recipe-pm into a void-packages checkout and open a pull request to void-linux/void-packages.",
        out.display()
    );
    Ok(())
}

// ---- inputs -------------------------------------------------------------------------------

fn cargo_version(manifest: &Path) -> Result<String> {
    let text = fs::read_to_string(manifest).with_context(|| format!("reading {}", manifest.display()))?;
    let mut in_package = false;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
        } else if in_package
            && let Some(rest) = line.strip_prefix("version")
            && let Some(v) = rest.trim().strip_prefix('=')
        {
            return Ok(v.trim().trim_matches('"').to_string());
        }
    }
    bail!("no [package] version in {}; pass --version", manifest.display())
}

fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git").arg("-C").arg(dir).args(args).output().context("running git")?;
    ensure!(out.status.success(), "git -C {} {} failed: {}", dir.display(), args.join(" "), String::from_utf8_lossy(&out.stderr).trim());
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    std::io::copy(&mut fs::File::open(path).with_context(|| format!("opening {}", path.display()))?, &mut hasher)?;
    Ok(hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// x86-64 ELF without a PT_INTERP program header (no dynamic loader, so it runs anywhere).
fn is_static_x86_64_elf(path: &Path) -> Result<bool> {
    let data = fs::read(path)?;
    ensure!(data.len() > 64 && data.starts_with(b"\x7fELF") && data[4] == 2 && data[5] == 1, "{} is not a 64-bit little-endian ELF file", path.display());
    let u16_at = |o: usize| u16::from_le_bytes([data[o], data[o + 1]]) as usize;
    let u64_at = |o: usize| u64::from_le_bytes(data[o..o + 8].try_into().unwrap()) as usize;
    ensure!(u16_at(0x12) == 0x3e, "{} is not an x86-64 ELF file", path.display());
    let (phoff, phentsize, phnum) = (u64_at(0x20), u16_at(0x36), u16_at(0x38));
    for i in 0..phnum {
        let at = phoff + i * phentsize;
        ensure!(at + 4 <= data.len(), "{} has a damaged program header table", path.display());
        if u32::from_le_bytes(data[at..at + 4].try_into().unwrap()) == 3 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn build_recipe(root: &Path) -> Result<(String, PathBuf)> {
    println!("building the static recipe: distro-builder prebuilt build tools levitate-recipe");
    let builder = crate::util::repo::cargo_bin(root, "distro-builder", "distro-builder")?;
    let out = Command::new(builder)
        .args(["prebuilt", "build", "tools", "levitate-recipe"])
        .current_dir(root)
        .stderr(std::process::Stdio::inherit())
        .output()
        .context("running distro-builder")?;
    ensure!(out.status.success(), "distro-builder prebuilt build tools levitate-recipe failed ({}); fix that, or pass --recipe-bin FILE for an unofficial release", out.status);
    let text = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    ensure!(lines.len() >= 2, "distro-builder printed no key and cache directory (got: {text:?})");
    Ok((lines[lines.len() - 2].to_string(), PathBuf::from(lines[lines.len() - 1])))
}

fn build_launcher(root: &Path, ginux_dir: &Path) -> Result<PathBuf> {
    let target = root.join(".artifacts/recipe-release/ginux-target");
    println!("building the Ginux launcher from tools/ginux into {}", target.display());
    let triple = "x86_64-unknown-linux-gnu";
    let status = Command::new("cargo")
        .current_dir(ginux_dir)
        .env("CARGO_TARGET_DIR", &target)
        .args(["build", "-p", "ginux-init", "--target", triple, "--release"])
        .status()
        .context("running cargo")?;
    ensure!(status.success(), "building the Ginux launcher failed; the same command is `make build` in tools/ginux, or pass --launcher FILE");
    Ok(target.join(triple).join("release/ginux"))
}

fn runtime_reference(recipe_dir: &Path) -> Result<(String, String)> {
    let path = recipe_dir.join("bootstrap/product-ci.json");
    let doc: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?)?;
    let sha = doc["image_sha256"].as_str().with_context(|| format!("{} has no image_sha256", path.display()))?;
    ensure!(sha.len() == 64 && sha.chars().all(|c| c.is_ascii_hexdigit()), "image_sha256 in {} is not a sha256", path.display());
    Ok((format!("oci:recipe-compiler@sha256:{sha}"), sha.to_string()))
}

fn gh(args: &[&str]) -> Result<std::process::Output> {
    Command::new("gh").args(args).output().context("running gh (install the GitHub CLI and run `gh auth login`)")
}

fn check_runtime_release(short: &str) -> Result<()> {
    let tag = format!("recipe-runtime-{short}");
    let out = gh(&["release", "view", &tag, "-R", PREBUILT_REPO, "--json", "assets"])?;
    ensure!(out.status.success(), "release {tag} does not exist in {PREBUILT_REPO}; `cargo xtask prebuilt publish-runtime` publishes it ({})", String::from_utf8_lossy(&out.stderr).trim());
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout)?;
    let has = doc["assets"].as_array().is_some_and(|a| a.iter().any(|x| x["name"] == "rootfs.tar.zst"));
    ensure!(has, "release {tag} in {PREBUILT_REPO} has no rootfs.tar.zst; `cargo xtask prebuilt publish-runtime` publishes it");
    println!("runtime  {tag} holds rootfs.tar.zst");
    Ok(())
}

fn check_publishable(recipe_dir: &Path, commit: &str, version: &str) -> Result<()> {
    let on_main = ["origin/master", "origin/main"].iter().any(|b| {
        Command::new("git")
            .arg("-C")
            .arg(recipe_dir)
            .args(["merge-base", "--is-ancestor", commit, b])
            .status()
            .is_ok_and(|s| s.success())
    });
    ensure!(on_main, "tools/recipe commit {commit} is not on origin/master; push tools/recipe first (`git -C tools/recipe push`), then rerun");
    let exists = gh(&["release", "view", version, "-R", RELEASE_REPO])?;
    ensure!(!exists.status.success(), "release {version} already exists in {RELEASE_REPO}; pick a new --version");
    Ok(())
}

// ---- staging ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn stage(recipe: &Path, dir: &Path, prefix: &str, launcher: &Path, reference: &str, catalog: &Path, recipe_dir: &Path, ginux_dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)?;
    let out = Command::new(recipe)
        .args(["runtime", "stage", "--root"])
        .arg(dir)
        .args(["--prefix", prefix, "--launcher"])
        .arg(launcher)
        .args(["--reference", reference, "--catalog"])
        .arg(catalog)
        .arg("--apparmor")
        .output()
        .with_context(|| format!("running {}", recipe.display()))?;
    ensure!(out.status.success(), "`recipe runtime stage` failed ({}): {}; the recipe must be the version that has `runtime stage --apparmor` (tools/recipe commit edee1b9 or later)", out.status, String::from_utf8_lossy(&out.stderr).trim());

    let p = dir.join(prefix.trim_start_matches('/'));
    fs::create_dir_all(p.join("bin"))?;
    fs::copy(recipe, p.join("bin/recipe"))?;
    for (file, section) in [("recipe.1", "man1"), ("recipe-recipe.5", "man5"), ("recipe-helpers.7", "man7")] {
        copy_into(&recipe_dir.join("docs/man").join(file), &p.join("share/man").join(section).join(file))?;
    }
    for file in ["README.md", "WRITING_RECIPES.md"] {
        copy_into(&recipe_dir.join(file), &p.join("share/doc/recipe-pm").join(file))?;
    }
    for file in ["LICENSE-MIT", "LICENSE-APACHE"] {
        copy_into(&recipe_dir.join(file), &p.join("share/licenses/recipe-pm").join(file))?;
    }
    for file in ["LICENSE", "NOTICE"] {
        copy_into(&ginux_dir.join(file), &p.join("share/licenses/recipe-pm/ginux").join(file))?;
    }
    Ok(())
}

fn copy_into(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to.parent().unwrap())?;
    fs::copy(from, to).with_context(|| format!("copying {} (is the submodule checked out?)", from.display()))?;
    Ok(())
}

// ---- tree and archive plumbing --------------------------------------------------------------

#[derive(Clone)]
enum Kind {
    Dir,
    File(Vec<u8>),
    Link(String),
}

/// One archive entry. Owners are always root; modes are normalised (dirs 0755, files 0644,
/// executables 0755); `mtime` is the source commit time.
#[derive(Clone)]
struct Item {
    path: String,
    kind: Kind,
    mode: u32,
    mtime: u64,
}

fn dir_item(path: &str, mtime: u64) -> Item {
    Item { path: path.into(), kind: Kind::Dir, mode: 0o755, mtime }
}

fn bytes_item(path: &str, data: Vec<u8>, exec: bool, mtime: u64) -> Item {
    Item { path: path.into(), kind: Kind::File(data), mode: if exec { 0o755 } else { 0o644 }, mtime }
}

/// Every entry under `dir`, sorted by path (mtime 0 means "the source commit time", set by `tar_bytes`).
fn collect(dir: &Path) -> Result<Vec<Item>> {
    fn walk(base: &Path, rel: &Path, out: &mut Vec<Item>) -> Result<()> {
        let mut names: Vec<_> = fs::read_dir(base.join(rel))?.collect::<std::io::Result<Vec<_>>>()?;
        names.sort_by_key(|e| e.file_name());
        for entry in names {
            let rel = rel.join(entry.file_name());
            let meta = fs::symlink_metadata(entry.path())?;
            let path = rel.to_string_lossy().to_string();
            if meta.is_dir() {
                out.push(Item { path, kind: Kind::Dir, mode: 0o755, mtime: 0 });
                walk(base, &rel, out)?;
            } else if meta.is_symlink() {
                let target = fs::read_link(entry.path())?.to_string_lossy().to_string();
                out.push(Item { path, kind: Kind::Link(target), mode: 0o777, mtime: 0 });
            } else {
                use std::os::unix::fs::PermissionsExt;
                let exec = meta.permissions().mode() & 0o111 != 0;
                out.push(Item { path, kind: Kind::File(fs::read(entry.path())?), mode: if exec { 0o755 } else { 0o644 }, mtime: 0 });
            }
        }
        Ok(())
    }
    let mut items = vec![];
    walk(dir, Path::new(""), &mut items)?;
    Ok(items)
}

fn prefixed(items: &[Item], prefix: &str) -> Vec<Item> {
    items.iter().map(|i| Item { path: format!("{prefix}{}", i.path), ..i.clone() }).collect()
}

/// A tar of `items` (sorted by path). `prefix` goes before each path ("./" for deb);
/// `sha1_pax` adds abuild's `APK-TOOLS.checksum.SHA1` header; `end` keeps the two zero
/// end-of-archive blocks (abuild cuts them from the apk control tar).
fn tar_bytes(items: &mut [Item], prefix: &str, sha1_pax: bool, end: bool, mtime: u64) -> Result<Vec<u8>> {
    use sha1::Digest as _;
    items.sort_by(|a, b| a.path.cmp(&b.path));
    let mut builder = tar::Builder::new(Vec::new());
    for item in items.iter() {
        let mut name = format!("{prefix}{}", item.path);
        let mut header = tar::Header::new_gnu();
        header.set_uid(0);
        header.set_gid(0);
        header.set_username("root")?;
        header.set_groupname("root")?;
        header.set_mtime(if item.mtime == 0 { mtime } else { item.mtime });
        header.set_mode(item.mode);
        match &item.kind {
            Kind::Dir => {
                if !name.ends_with('/') {
                    name.push('/');
                }
                header.set_entry_type(tar::EntryType::Directory);
                header.set_size(0);
                append(&mut builder, &mut header, &name, None, std::io::empty())?;
            }
            Kind::File(data) => {
                if sha1_pax {
                    let digest = sha1::Sha1::digest(data);
                    builder.append_pax_extensions([("APK-TOOLS.checksum.SHA1", hex(&digest).as_bytes())])?;
                }
                header.set_entry_type(tar::EntryType::Regular);
                header.set_size(data.len() as u64);
                append(&mut builder, &mut header, &name, None, &data[..])?;
            }
            Kind::Link(target) => {
                if sha1_pax {
                    let digest = sha1::Sha1::digest(target.as_bytes());
                    builder.append_pax_extensions([("APK-TOOLS.checksum.SHA1", hex(&digest).as_bytes())])?;
                }
                header.set_entry_type(tar::EntryType::Symlink);
                header.set_size(0);
                append(&mut builder, &mut header, &name, Some(target), std::io::empty())?;
            }
        }
    }
    let mut bytes = builder.into_inner()?;
    if !end {
        ensure!(bytes.len() >= 1024 && bytes[bytes.len() - 1024..].iter().all(|b| *b == 0), "tar writer did not end with two zero blocks");
        bytes.truncate(bytes.len() - 1024);
    }
    Ok(bytes)
}

/// Appends one entry under exactly `name`. The tar crate's path setter drops a leading "./",
/// but deb archives are conventionally `./usr/...`, so short names are written verbatim;
/// long ones fall back to the crate's GNU long-name extension.
fn append(builder: &mut tar::Builder<Vec<u8>>, header: &mut tar::Header, name: &str, link: Option<&str>, data: impl std::io::Read) -> Result<()> {
    if name.len() < 100 && link.is_none_or(|l| l.len() < 100) {
        let old = header.as_old_mut();
        old.name = [0; 100];
        old.name[..name.len()].copy_from_slice(name.as_bytes());
        if let Some(target) = link {
            header.set_link_name(target)?;
        }
        header.set_cksum();
        builder.append(header, data)?;
    } else if let Some(target) = link {
        builder.append_link(header, name, target)?;
    } else {
        builder.append_data(header, name, data)?;
    }
    Ok(())
}

fn write_zst(path: &Path, tar: &[u8]) -> Result<()> {
    fs::write(path, zstd::stream::encode_all(tar, 19)?)?;
    Ok(())
}

fn gzip(data: &[u8]) -> Result<Vec<u8>> {
    let mut enc = flate2::GzBuilder::new().mtime(0).write(Vec::new(), flate2::Compression::best());
    enc.write_all(data)?;
    Ok(enc.finish()?)
}

fn xz(data: &[u8]) -> Result<Vec<u8>> {
    let mut enc = xz2::write::XzEncoder::new(Vec::new(), 6);
    enc.write_all(data)?;
    Ok(enc.finish()?)
}

fn load_profile_sh(indent: &str) -> String {
    format!(
        "{indent}if [ -d /sys/kernel/security/apparmor ] && command -v apparmor_parser >/dev/null 2>&1; then\n{indent}    apparmor_parser -r {PROFILE} || true\n{indent}fi\n"
    )
}

fn unload_profile_sh(indent: &str) -> String {
    format!(
        "{indent}if [ -d /sys/kernel/security/apparmor ] && command -v apparmor_parser >/dev/null 2>&1 && [ -f {PROFILE} ]; then\n{indent}    apparmor_parser -R {PROFILE} || true\n{indent}fi\n"
    )
}

fn is_recipe_dir(path: &str) -> bool {
    path.split('/').any(|c| c.starts_with("recipe"))
}

// ---- RPM ----------------------------------------------------------------------------------

fn build_rpm(usr: &[Item], v: &str, mtime: u64, out: &Path) -> Result<()> {
    let mut b = rpm::PackageBuilder::new(PKG, v, LICENSE, "x86_64", SUMMARY);
    b.using_config(rpm::BuildConfig::v4().source_date(u32::try_from(mtime)?).compression(rpm::CompressionType::Gzip))
        .release("1")
        .url(HOMEPAGE)
        .description(DESCRIPTION)
        .packager(MAINTAINER)
        .vendor("LevitateOS");
    let mut items = usr.to_vec();
    items.sort_by(|a, b| a.path.cmp(&b.path));
    for item in &items {
        let dest = format!("/{}", item.path);
        match &item.kind {
            Kind::Dir if is_recipe_dir(&item.path) => {
                b.with_dir_entry(rpm::FileOptions::dir(dest).permissions(0o755).user("root").group("root"))?;
            }
            Kind::Dir => {}
            Kind::File(data) => {
                let mut o = rpm::FileOptions::new(dest).permissions(item.mode as u16).user("root").group("root");
                if CONFFILES.contains(&item.path.as_str()) {
                    o = o.config().noreplace();
                } else if item.path.contains("/share/doc/") || item.path.contains("/share/man/") {
                    o = o.doc();
                }
                b.with_file_contents(data.clone(), o)?;
            }
            Kind::Link(target) => {
                b.with_symlink(rpm::FileOptions::symlink(dest, target.clone()))?;
            }
        }
    }
    b.post_install_script(rpm::Scriptlet::new(format!("{}exit 0\n", load_profile_sh(""))));
    b.pre_uninstall_script(rpm::Scriptlet::new(format!("if [ \"$1\" = 0 ]; then\n{}fi\nexit 0\n", unload_profile_sh("    "))));
    b.build()?.write_file(out)?;
    Ok(())
}

// ---- deb ----------------------------------------------------------------------------------

fn build_deb(usr: &[Item], v: &str, mtime: u64, out: &Path) -> Result<()> {
    use md5::Digest as _;
    let mut data_items = usr.to_vec();
    data_items.push(dir_item("", mtime));
    let installed_kib: usize = usr.iter().map(|i| if let Kind::File(d) = &i.kind { d.len() } else { 0 }).sum::<usize>().div_ceil(1024);
    let mut md5sums = String::new();
    let mut files = usr.to_vec();
    files.sort_by(|a, b| a.path.cmp(&b.path));
    for i in &files {
        if let Kind::File(d) = &i.kind {
            md5sums.push_str(&format!("{}  {}\n", hex(&md5::Md5::digest(d)), i.path));
        }
    }
    let long: String = DESCRIPTION.split(". ").collect::<Vec<_>>().join(".\n").lines().map(|l| format!(" {l}\n")).collect();
    let control = format!(
        "Package: {PKG}\nVersion: {v}-1\nArchitecture: amd64\nMaintainer: {MAINTAINER}\nInstalled-Size: {installed_kib}\nSection: admin\nPriority: optional\nHomepage: {HOMEPAGE}\nDescription: {SUMMARY}\n{long}"
    );
    let conffiles: String = CONFFILES.iter().map(|c| format!("/{c}\n")).collect();
    let postinst = format!("#!/bin/sh\nset -e\ncase \"$1\" in\n    configure)\n{}        ;;\nesac\nexit 0\n", load_profile_sh("        "));
    let prerm = format!("#!/bin/sh\nset -e\ncase \"$1\" in\n    remove)\n{}        ;;\nesac\nexit 0\n", unload_profile_sh("        "));
    let mut control_items = vec![
        dir_item("", mtime),
        bytes_item("control", control.into_bytes(), false, mtime),
        bytes_item("md5sums", md5sums.into_bytes(), false, mtime),
        bytes_item("conffiles", conffiles.into_bytes(), false, mtime),
        bytes_item("postinst", postinst.into_bytes(), true, mtime),
        bytes_item("prerm", prerm.into_bytes(), true, mtime),
    ];
    let control_tar = xz(&tar_bytes(&mut control_items, "./", false, true, mtime)?)?;
    let data_tar = xz(&tar_bytes(&mut data_items, "./", false, true, mtime)?)?;
    let mut ar = b"!<arch>\n".to_vec();
    for (name, body) in [("debian-binary", &b"2.0\n"[..]), ("control.tar.xz", &control_tar), ("data.tar.xz", &data_tar)] {
        ar.extend(format!("{name:<16}{mtime:<12}{:<6}{:<6}{:<8}{:<10}`\n", 0, 0, "100644", body.len()).as_bytes());
        ar.extend(body);
        if body.len() % 2 == 1 {
            ar.push(b'\n');
        }
    }
    fs::write(out, ar)?;
    Ok(())
}

// ---- apk ----------------------------------------------------------------------------------

fn build_apk(usr: &[Item], v: &str, mtime: u64, out: &Path) -> Result<()> {
    let mut data_items = usr.to_vec();
    let size: usize = usr.iter().map(|i| if let Kind::File(d) = &i.kind { d.len() } else { 0 }).sum();
    let data_gz = gzip(&tar_bytes(&mut data_items, "", true, true, mtime)?)?;
    let datahash = hex(&Sha256::digest(&data_gz));
    let pkginfo = format!(
        "# Generated by cargo xtask release recipe\npkgname = {PKG}\npkgver = {v}-r0\npkgdesc = {SUMMARY}\nurl = {HOMEPAGE}\nbuilddate = {mtime}\npackager = {MAINTAINER}\nsize = {size}\narch = x86_64\norigin = {PKG}\nlicense = {LICENSE}\ndatahash = {datahash}\n"
    );
    let post = format!("#!/bin/sh\n{}exit 0\n", load_profile_sh(""));
    let pre = format!("#!/bin/sh\n{}exit 0\n", unload_profile_sh(""));
    let mut control_items = vec![
        bytes_item(".PKGINFO", pkginfo.into_bytes(), false, mtime),
        bytes_item(".post-install", post.clone().into_bytes(), true, mtime),
        bytes_item(".post-upgrade", post.into_bytes(), true, mtime),
        bytes_item(".pre-deinstall", pre.into_bytes(), true, mtime),
    ];
    // Sorted order puts .PKGINFO first; abuild cuts the end-of-archive blocks of this tar only.
    let control_tar = tar_bytes(&mut control_items, "", false, false, mtime)?;
    let mut apk = gzip(&control_tar)?;
    apk.extend(&data_gz);
    fs::write(out, apk)?;
    Ok(())
}

// ---- scripts and build files ------------------------------------------------------------------

fn fill(template: &str, pairs: &[(&str, &str)]) -> String {
    pairs.iter().fold(template.to_string(), |text, (key, value)| text.replace(key, value))
}

fn install_sh() -> String {
    fill(
        r##"#!/bin/sh
# Installs recipe-pm from this folder's root/ into ${DESTDIR:-/}.
# Ordinary users cannot write to /usr/local: run it as root (sudo sh install.sh).
# Set DESTDIR=DIR to install into a staging folder instead (no root needed).
set -eu

here=$(cd "$(dirname "$0")" && pwd)
dest=${DESTDIR:-}
dest=${dest%/}
if [ -z "$dest" ] && [ "$(id -u)" != 0 ]; then
    echo "install.sh: installing into / needs root. Run: sudo sh $0" >&2
    echo "            or set DESTDIR=DIR to install into a folder." >&2
    exit 1
fi
if [ ! -d "$here/root" ]; then
    echo "install.sh: $here/root is missing; run this script from the unpacked recipe-pm folder." >&2
    exit 1
fi
if [ -n "$dest" ]; then
    mkdir -p "$dest"
    dest=$(cd "$dest" && pwd)
fi

manifest=@MANIFEST@
list=$(mktemp)
dirs=$(mktemp)
files=$(mktemp)
trap 'rm -f "$list" "$dirs" "$files"' EXIT
cd "$here/root"

# Directories that do not exist yet are created 0755; existing ones are left as they are.
find . -mindepth 1 -type d | LC_ALL=C sort > "$dirs"
while IFS= read -r d; do
    p=${d#.}
    if [ ! -d "$dest$p" ]; then
        mkdir "$dest$p"
        chmod 0755 "$dest$p"
        printf 'd %s\n' "$p"
    fi
done < "$dirs" > "$list"

find . \( -type f -o -type l \) | LC_ALL=C sort > "$files"
while IFS= read -r f; do
    p=${f#.}
    if [ -L "$f" ]; then
        rm -f "$dest$p"
        cp -P "$f" "$dest$p"
    elif [ -x "$f" ]; then
        install -m 0755 "$f" "$dest$p"
    else
        install -m 0644 "$f" "$dest$p"
    fi
    printf 'f %s\n' "$p"
done < "$files" >> "$list"

# Keep the directories an earlier install created, so uninstall.sh can remove them.
if [ -f "$dest$manifest" ]; then
    grep '^d ' "$dest$manifest" >> "$list" || true
fi
printf 'f %s\n' "$manifest" >> "$list"
LC_ALL=C sort -u "$list" > "$dest$manifest"
echo "installed $(grep -c '^f ' "$dest$manifest") files; manifest: $dest$manifest"

if [ -z "$dest" ] && [ -d /sys/kernel/security/apparmor ] && command -v apparmor_parser >/dev/null 2>&1; then
    if apparmor_parser -r @PROFILE@; then
        echo "loaded the AppArmor profile @PROFILE@"
    else
        echo "WARNING: could not load the AppArmor profile @PROFILE@." >&2
        echo "         Ordinary users then need 'sysctl kernel.apparmor_restrict_unprivileged_userns=0'" >&2
        echo "         (Ubuntu 23.10 and later), or must run recipe as root." >&2
    fi
fi

cat <<'EOF'

Recipe is installed. To use it in this shell now:
    . /etc/profile.d/recipe.sh
or open a new login shell. Then try: recipe list

The first recipe run downloads the runtime image (about 330 MB) into
$XDG_CACHE_HOME/recipe/images (default ~/.cache/recipe/images; root: /root/.cache/recipe).
EOF
"##,
        &[("@MANIFEST@", MANIFEST), ("@PROFILE@", PROFILE)],
    )
}

fn uninstall_sh() -> String {
    fill(
        r##"#!/bin/sh
# Removes what install.sh installed, as listed in its manifest.
# Set DESTDIR=DIR if you installed into a folder.
set -eu

dest=${DESTDIR:-}
dest=${dest%/}
if [ -z "$dest" ] && [ "$(id -u)" != 0 ]; then
    echo "uninstall.sh: removing from / needs root. Run: sudo sh $0" >&2
    exit 1
fi
[ -z "$dest" ] || dest=$(cd "$dest" && pwd)
manifest=@MANIFEST@
if [ ! -f "$dest$manifest" ]; then
    echo "uninstall.sh: $dest$manifest is missing; recipe-pm was not installed by install.sh here." >&2
    exit 1
fi

if [ -z "$dest" ] && [ -d /sys/kernel/security/apparmor ] && command -v apparmor_parser >/dev/null 2>&1 && [ -f @PROFILE@ ]; then
    apparmor_parser -R @PROFILE@ || true
fi

list=$(mktemp)
trap 'rm -f "$list"' EXIT
cp "$dest$manifest" "$list"
while IFS= read -r line; do
    case $line in
        "f "*) rm -f "$dest${line#f }" ;;
    esac
done < "$list"
# Only folders install.sh created, and only when they are empty now (deepest first).
LC_ALL=C sort -r "$list" > "$list.rev"
while IFS= read -r line; do
    case $line in
        "d "*) rmdir "$dest${line#d }" 2>/dev/null || true ;;
    esac
done < "$list.rev"
rm -f "$list.rev"

cat <<'EOF'
recipe-pm is removed. Left in place on purpose:
    software installed with recipe: /var/lib/recipe (for everyone) and ~/.local/share/recipe (per user)
    downloaded runtime images:      ~/.cache/recipe (root: /root/.cache/recipe)
Delete those folders yourself if you want them gone.
EOF
"##,
        &[("@MANIFEST@", MANIFEST), ("@PROFILE@", PROFILE)],
    )
}

fn readme_txt(v: &str) -> String {
    format!(
        "recipe-pm {v}

Recipe, the package manager that builds software inside Ginux, for any x86_64 Linux.

Install (as root):
    sudo sh install.sh
It copies root/ into /usr/local (and /etc/profile.d/recipe.sh), writes the list of
installed files to {MANIFEST}, and on hosts with AppArmor loads {PROFILE}.
Remove it with: sudo sh uninstall.sh

The first recipe run downloads the runtime image (about 330 MB) to
$XDG_CACHE_HOME/recipe/images. Nothing else is fetched at install time.

Ordinary users can install software for themselves; root installs for everyone.
Ubuntu 23.10 and later restrict user namespaces: install.sh loads an AppArmor profile
for the launcher. Without it, ordinary users need
kernel.apparmor_restrict_unprivileged_userns=0 or must run recipe as root.

Homepage: {HOMEPAGE}
Licenses: MIT OR Apache-2.0 (recipe); Ginux's own licenses are in root/usr/local/share/licenses/recipe-pm/ginux.
"
    )
}

fn pkgbuild(v: &str, file: &str, url: &str, sha: &str) -> String {
    fill(
        r##"# Maintainer: @MAINTAINER@
pkgname=recipe-pm
pkgver=@V@
pkgrel=1
pkgdesc='@SUMMARY@'
arch=('x86_64')
url='@HOMEPAGE@'
license=('MIT' 'Apache-2.0')
install=recipe-pm.install
backup=('etc/profile.d/recipe.sh' 'etc/apparmor.d/recipe-ginux')
options=(!strip !debug)
source=('@FILE@::@URL@')
sha256sums=('@SHA@')

package() {
    cd "$srcdir"
    cp -a usr etc "$pkgdir/"
}
"##,
        &[("@MAINTAINER@", MAINTAINER), ("@V@", v), ("@SUMMARY@", SUMMARY), ("@HOMEPAGE@", HOMEPAGE), ("@FILE@", file), ("@URL@", url), ("@SHA@", sha)],
    )
}

/// The text `makepkg --printsrcinfo` prints for `pkgbuild`.
fn srcinfo(v: &str, file: &str, url: &str, sha: &str) -> String {
    fill(
        "pkgbase = recipe-pm
\tpkgdesc = @SUMMARY@
\tpkgver = @V@
\tpkgrel = 1
\turl = @HOMEPAGE@
\tinstall = recipe-pm.install
\tarch = x86_64
\tlicense = MIT
\tlicense = Apache-2.0
\toptions = !strip
\toptions = !debug
\tbackup = etc/profile.d/recipe.sh
\tbackup = etc/apparmor.d/recipe-ginux
\tsource = @FILE@::@URL@
\tsha256sums = @SHA@

pkgname = recipe-pm
",
        &[("@SUMMARY@", SUMMARY), ("@V@", v), ("@HOMEPAGE@", HOMEPAGE), ("@FILE@", file), ("@URL@", url), ("@SHA@", sha)],
    )
}

fn aur_install() -> String {
    format!(
        "post_install() {{\n{load}}}\n\npost_upgrade() {{\n    post_install\n}}\n\npre_remove() {{\n{unload}}}\n",
        load = load_profile_sh("    "),
        unload = unload_profile_sh("    "),
    )
}

fn void_template(v: &str, url: &str, sha: &str) -> String {
    fill(
        r##"# Template file for 'recipe-pm'
pkgname=recipe-pm
version=@V@
revision=1
archs="x86_64 x86_64-musl"
create_wrksrc=yes
short_desc="@SUMMARY@"
maintainer="@MAINTAINER@"
license="MIT, Apache-2.0"
homepage="@HOMEPAGE@"
distfiles="@URL@"
checksum=@SHA@
nostrip=yes
conf_files="/etc/profile.d/recipe.sh /etc/apparmor.d/recipe-ginux"

do_install() {
	vmkdir usr
	vmkdir etc
	cp -a usr/. ${DESTDIR}/usr/
	cp -a etc/. ${DESTDIR}/etc/
}
"##,
        &[("@V@", v), ("@SUMMARY@", SUMMARY), ("@MAINTAINER@", MAINTAINER), ("@HOMEPAGE@", HOMEPAGE), ("@URL@", url), ("@SHA@", sha)],
    )
}

fn release_notes(v: &str, official: bool) -> String {
    let unofficial = if official {
        ""
    } else {
        "\n> This build is NOT official (dirty sources or a non-static input). Do not publish it.\n"
    };
    fill(
        r##"# recipe-pm @V@
@UNOFFICIAL@
Recipe is the package manager that builds software inside Ginux, for any x86_64 Linux.
The package holds the static `recipe` command, the Ginux launcher, the official recipe
source and the manual pages. The command stays `recipe`; the package is `recipe-pm`.

The runtime image (about 330 MB) is downloaded on the first recipe run into
`$XDG_CACHE_HOME/recipe/images` (default `~/.cache/recipe/images`). No package contains it.
Ordinary users install software for themselves (in `~/.local/share/recipe`); root installs
for everyone (in `/var/lib/recipe`).

## Install

| Format | Command |
|---|---|
| Tarball | `tar --zstd -xf recipe-pm-@V@-x86_64.tar.zst && cd recipe-pm-@V@ && sudo sh install.sh` (installs to `/usr/local`; `sudo sh uninstall.sh` removes it) |
| RPM (Fedora, RHEL family, openSUSE) | `sudo rpm -i recipe-pm-@V@-1.x86_64.rpm` (or `dnf install ./recipe-pm-@V@-1.x86_64.rpm`) |
| deb (Debian, Ubuntu) | `sudo apt install ./recipe-pm_@V@-1_amd64.deb` |
| apk (Alpine) | `sudo apk add --allow-untrusted recipe-pm-@V@-r0.x86_64.apk` (unsigned until package signing exists) |
| AUR (Arch) | the `recipe-pm` package (`PKGBUILD` and `.SRCINFO` built from this release's `-usr` tarball) |
| Void | the `recipe-pm` xbps-src template (built from this release's `-usr` tarball) |
| Packagers | `recipe-pm-@V@-x86_64-usr.tar.zst` holds `usr/` and `etc/` as installed to `/usr` |

Then open a new login shell (or `. /etc/profile.d/recipe.sh`) and run `recipe list`.

## Ubuntu 23.10 and later (AppArmor)

These restrict unprivileged user namespaces, which Ginux uses for ordinary users. The
packages and `install.sh` load the profile `/etc/apparmor.d/recipe-ginux` for the launcher.
If it cannot be loaded, ordinary users need `kernel.apparmor_restrict_unprivileged_userns=0`
or must run `recipe` as root.

## Files

`SHA256SUMS` lists every asset; `provenance.json` records the source commits, the runtime
reference and the sha256 of every input.
"##,
        &[("@V@", v), ("@UNOFFICIAL@", unofficial)],
    )
}

// ---- publish --------------------------------------------------------------------------------

fn publish(out: &Path, v: &str, commit: &str, assets: &[String]) -> Result<()> {
    println!("publishing release {v} to {RELEASE_REPO}");
    let title = format!("{PKG} {v}");
    let notes = out.join("RELEASE_NOTES.md");
    let mut create = Command::new("gh");
    create
        .args(["release", "create", v, "-R", RELEASE_REPO, "--target", commit, "--title", &title, "--notes-file"])
        .arg(&notes);
    for name in assets.iter().map(String::as_str).chain(["SHA256SUMS", "provenance.json"]) {
        create.arg(out.join(name));
    }
    let status = create.status().context("running gh")?;
    ensure!(status.success(), "gh release create failed ({status}); if the release was created partly, delete it with `gh release delete {v} -R {RELEASE_REPO}` and rerun");
    println!("published https://github.com/{RELEASE_REPO}/releases/tag/{v}");
    Ok(())
}
