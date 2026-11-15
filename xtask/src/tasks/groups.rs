//! `base groups`: share a variant's locked packages out among the group
//! recipes of the public catalogue (`recipes/`), one owner per package, and
//! write each group's pins into its recipe.
//!
//! A group recipe names its seeds per C library (`const SEEDS_GLIBC`, the
//! LevitateOS build from Fedora RPMs; `const SEEDS_MUSL`, the AcornOS build
//! from Alpine APKs) and the groups it needs (`let deps`). This resolves every
//! group's seeds over the base and over the closures of the groups it
//! declares, from the metadata of the packages the base and desktop locks
//! already hold (each read from the lock command's own download cache and
//! checked against the lock's SHA-256), so it works offline and only ever
//! picks a locked build. A package one group's own closure needs belongs to
//! that group; a package two or more groups' own closures need belongs to
//! `desktop-libs`, unless it needs one group's packages and every group that
//! needs it depends on that group (then it goes with that group, so
//! desktop-libs stands on the base alone). A package that only joins once
//! desktop-libs is installed too (APK install_if) goes to the group whose
//! world completes it. The base is the variant's base list, minus every
//! package a group names as a seed, plus [`base_adds`]. A lock row nothing in
//! distro.toml's lists needs any more is named and left out.
//!
//! Our own Qt apps (a `rec-apps` group) and the XLibre server itself (into
//! `xlibre.rhai`, which holds only the server's run-time packages today) join
//! in the later step that moves the desktop out of the builder.
//!
//! The pins go between `// BEGIN GENERATED PINS <libc>` and
//! `// END GENERATED PINS <libc>`; the recipe's `version:` line and the
//! `network_max_bytes:` line in its NEEDS follow from them. Nothing else in a
//! recipe is touched. `--check` writes nothing and fails when a pin block is
//! stale, when a group's closure holds another group's seed (or a
//! `desktop-libs` package) without the group declaring that dependency, or
//! when the base and the groups together differ from today's base and desktop
//! locks in any package or version.

use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::base::{self, Layer, curl, rel, sha256_file};
use crate::cli::Distro;
use distro_contract::profile::PackageBackend;

/// The group that owns what more than one group needs.
const SHARED: &str = "desktop-libs";

/// Packages the base takes beyond the variant's base list, until the builder
/// step that moves the desktop into the groups writes them there: what the
/// login session needs from the system itself whatever desktop is installed.
fn base_adds(distro: &Distro, backend: PackageBackend) -> &'static [&'static str] {
    match (backend, distro.id()) {
        // pam_systemd.so: without it logind registers no session.
        (PackageBackend::Rpm, _) => &["systemd-pam"],
        // The session and seat manager and the policy daemon on OpenRC.
        (PackageBackend::Apk, _) => &["elogind", "polkit-elogind"],
    }
}

/// Which C library a backend's builds are for, as recipes name it.
fn libc_of(backend: PackageBackend) -> &'static str {
    match backend {
        PackageBackend::Rpm => "glibc",
        PackageBackend::Apk => "musl",
    }
}

/// The constants a block names, by C library.
fn prefix_of(libc: &str) -> &'static str {
    if libc == "glibc" { "RPM" } else { "APK" }
}

/// One row of a lock: name, name-version, URL, SHA-256.
#[derive(Clone, Debug)]
struct Row {
    name: String,
    nv: String,
    url: String,
    sha256: String,
}

impl Row {
    fn file_name(&self) -> &str {
        self.url.rsplit('/').next().unwrap_or(&self.url)
    }
}

/// A locked package with what the generator learns from its file.
#[derive(Clone, Debug)]
struct Locked {
    row: Row,
    bytes: u64,
    license: String,
    /// Its `var/` entries are kept as state seeds: it has some, and no
    /// tmpfiles.d of its own makes them at boot (AcornOS runs no tmpfiles).
    seed: bool,
}

/// A group recipe as its hand-written part declares it.
struct Group {
    name: String,
    path: PathBuf,
    text: String,
    /// Seeds for the C library being generated.
    seeds: Vec<String>,
    /// Seeds per C library, for the libc check.
    seeds_by_libc: BTreeMap<&'static str, Vec<String>>,
    deps: Vec<String>,
    about_libc: Vec<String>,
}

pub fn groups(distro: &Distro, check: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let bundle = distro_contract::load(&root, distro.id())?;
    let backend = bundle.contract.profile.packages;
    let libc = libc_of(backend);
    let base_contract = &bundle.contract.sources.rootfs_source.base;

    // Today's locks: what the base and the desktop together hold.
    let base_lock = base::lock_path(&root, distro, backend, Layer::Base);
    let desktop_lock = base::lock_path(&root, distro, backend, Layer::Desktop);
    let mut rows = read_rows(&base_lock)?;
    rows.extend(read_rows(&desktop_lock)?);
    let pinned_on = [&base_lock, &desktop_lock]
        .iter()
        .map(|p| locked_on(p))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .max()
        .unwrap_or_default();
    let cache = match backend {
        PackageBackend::Rpm => root.join(".artifacts/rpm-cache").join(format!("fedora-{}", base_contract.release)),
        PackageBackend::Apk => root.join(".artifacts/apk-cache").join(&base_contract.release),
    };
    eprintln!("[step] Check {} locked packages against {}", rows.len(), rel(&root, &cache));
    let files = cached_files(&cache, &rows)?;
    let (universe, meta) = match backend {
        PackageBackend::Rpm => rpm_universe(&rows, &files, &base_contract.provided_elsewhere)?,
        PackageBackend::Apk => apk_universe(&rows, &files)?,
    };
    let locked: BTreeMap<String, Locked> = rows
        .iter()
        .map(|row| {
            let (bytes, license, seed) = meta[&row.name].clone();
            (row.name.clone(), Locked { row: row.clone(), bytes, license, seed })
        })
        .collect();

    let groups = read_groups(&root.join("recipes"), libc)?;
    let mut problems: Vec<String> = Vec::new();
    let present: BTreeMap<&str, &Group> = groups
        .iter()
        .filter(|g| g.name == SHARED || !g.seeds.is_empty())
        .map(|g| (g.name.as_str(), g))
        .collect();
    if !present.contains_key(SHARED) {
        bail!("recipes/{SHARED}.rhai is missing; it owns what more than one group needs");
    }

    // ABOUT.libc says where a group has a build: where it has seeds.
    for g in &groups {
        let mut want: Vec<&str> = g
            .seeds_by_libc
            .iter()
            .filter(|(_, s)| !s.is_empty() || g.name == SHARED)
            .map(|(l, _)| *l)
            .collect();
        want.sort();
        let mut have: Vec<&str> = g.about_libc.iter().map(String::as_str).collect();
        have.sort();
        if want != have {
            problems.push(format!(
                "{}: ABOUT.libc is {have:?} but the recipe has seeds for {want:?}",
                g.name
            ));
        }
    }

    // Every seed is a locked package; no seed is named by two groups.
    let mut seed_owner: BTreeMap<&str, &str> = BTreeMap::new();
    for g in present.values() {
        for s in &g.seeds {
            if !locked.contains_key(s) {
                problems.push(format!(
                    "{}: seed {s} is not in today's {} or {}",
                    g.name,
                    rel(&root, &base_lock),
                    rel(&root, &desktop_lock)
                ));
            }
            if let Some(other) = seed_owner.insert(s, &g.name) {
                problems.push(format!("seed {s} is named by both {other} and {}", g.name));
            }
        }
        for d in &g.deps {
            if !present.contains_key(d.as_str()) {
                problems.push(format!(
                    "{}: depends on {d}, which has no {libc} build (no seeds for {libc} in recipes/{d}.rhai)",
                    g.name
                ));
            }
        }
    }
    if !problems.is_empty() {
        bail!("the group recipes do not fit today's locks:\n  {}", problems.join("\n  "));
    }

    // The base: the variant's list without what a group names, plus base_adds.
    let mut base_seeds: Vec<String> = base_contract
        .packages
        .iter()
        .filter(|p| !seed_owner.contains_key(p.as_str()))
        .cloned()
        .collect();
    let moved: Vec<&String> = base_contract.packages.iter().filter(|p| seed_owner.contains_key(p.as_str())).collect();
    base_seeds.extend(base_adds(distro, backend).iter().map(|s| s.to_string()));
    let base_set = universe.resolve(&base_seeds, &BTreeSet::new())?;
    for (seed, owner) in &seed_owner {
        if base_set.contains(*seed) {
            problems.push(format!("{owner}: seed {seed} is pulled into the base by the base's own packages"));
        }
    }

    // Each group over the base and the full closures of what it declares, in
    // dependency order.
    let order = topo_order(&present)?;
    let mut full: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut own: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for name in &order {
        if *name == SHARED {
            continue;
        }
        let g = present[name];
        let mut under = base_set.clone();
        for d in transitive_deps(&present, name) {
            if let Some(set) = full.get(d) {
                under.extend(set.iter().cloned());
            }
        }
        let closure = universe.resolve(&g.seeds, &under)?;
        let mine: BTreeSet<String> = closure.difference(&under).cloned().collect();
        let mut whole = under.clone();
        whole.extend(mine.iter().cloned());
        whole.retain(|p| !base_set.contains(p));
        full.insert(name, whole);
        own.insert(name, mine);
    }

    // One owner per package.
    let mut needed_by: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (g, set) in &own {
        for p in set {
            needed_by.entry(p.as_str()).or_default().push(g);
        }
    }
    let mut assigned: BTreeMap<&str, BTreeSet<String>> = present.keys().map(|n| (*n, BTreeSet::new())).collect();
    for (p, gs) in &needed_by {
        let owner = if gs.len() == 1 { gs[0] } else { SHARED };
        assigned.get_mut(owner).unwrap().insert(p.to_string());
    }
    // What joins only once desktop-libs is there too (APK install_if: a
    // package that comes when all its conditions are installed, such as a
    // plug-in of a shared library for a group's own program): each group
    // again, now over desktop-libs as well, until nothing new joins.
    loop {
        let mut joined: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        for name in &order {
            if *name == SHARED {
                continue;
            }
            let declared = transitive_deps(&present, name);
            let mut under = base_set.clone();
            if declared.contains(SHARED) {
                under.extend(assigned[SHARED].iter().cloned());
            }
            for d in &declared {
                if let Some(set) = full.get(d) {
                    under.extend(set.iter().cloned());
                }
            }
            under.extend(full[name].iter().cloned());
            let closure = universe.resolve(&present[name].seeds, &under)?;
            for p in closure.difference(&under) {
                if !assigned.values().any(|s| s.contains(p)) {
                    joined.entry(p.clone()).or_default().push(name);
                    own.get_mut(name).unwrap().insert(p.clone());
                    full.get_mut(name).unwrap().insert(p.clone());
                }
            }
        }
        if joined.is_empty() {
            break;
        }
        // Joined by several groups: it goes with the one group that owns
        // what it needs, when every group that joined it depends on that one
        // (networkmanager-wwan, needing libnm, goes to network); to
        // desktop-libs when the base and desktop-libs hold all it needs.
        let mut settled: Vec<(String, &str)> = Vec::new();
        for (p, gs) in &joined {
            if gs.len() == 1 {
                settled.push((p.clone(), gs[0]));
                continue;
            }
            let mut beneath = base_set.clone();
            beneath.extend(assigned[SHARED].iter().cloned());
            let needs = universe.resolve(std::slice::from_ref(p), &beneath)?;
            let owners: BTreeSet<&str> = needs
                .iter()
                .filter(|n| *n != p && !beneath.contains(*n))
                .filter_map(|n| assigned.iter().find(|(_, s)| s.contains(n)).map(|(g, _)| *g))
                .collect();
            let owner = match owners.iter().next() {
                None => SHARED,
                Some(o)
                    if owners.len() == 1
                        && gs.iter().all(|g| g == o || transitive_deps(&present, g).contains(o)) =>
                {
                    *o
                }
                _ => {
                    problems.push(format!(
                        "{p} joins {} once their dependencies are installed, and needs packages of {owners:?}; \
                         no one of them depends on the others",
                        gs.join(" and ")
                    ));
                    SHARED
                }
            };
            settled.push((p.clone(), owner));
        }
        for (p, owner) in settled {
            for (g, set) in own.iter_mut() {
                if *g != owner && owner != SHARED {
                    set.remove(&p);
                }
            }
            assigned.get_mut(owner).unwrap().insert(p);
        }
    }
    // A shared package that needs one group's packages (and every group
    // needing it depends on that group) goes with that group instead:
    // desktop-libs stands on the base alone.
    loop {
        // One resolve says whether anything needs moving at all.
        let shared_now: Vec<String> = assigned[SHARED].iter().cloned().collect();
        let mut beneath_all = base_set.clone();
        beneath_all.extend(shared_now.iter().cloned());
        if universe.resolve(&shared_now, &base_set)?.is_subset(&beneath_all) {
            break;
        }
        let mut moves: Vec<(String, &str)> = Vec::new();
        for p in &assigned[SHARED] {
            let mut beneath = base_set.clone();
            beneath.extend(assigned[SHARED].iter().filter(|q| *q != p).cloned());
            let needs = universe.resolve(std::slice::from_ref(p), &beneath)?;
            let owners: BTreeSet<&str> = needs
                .iter()
                .filter(|n| *n != p && !beneath.contains(*n))
                .filter_map(|n| assigned.iter().find(|(_, s)| s.contains(n)).map(|(g, _)| *g))
                .collect();
            if owners.len() != 1 {
                continue;
            }
            let o = *owners.iter().next().unwrap();
            let needers: Vec<&str> = own.iter().filter(|(_, s)| s.contains(p)).map(|(g, _)| *g).collect();
            if needers.iter().all(|g| *g == o || transitive_deps(&present, g).contains(o)) {
                moves.push((p.clone(), o));
            }
        }
        if moves.is_empty() {
            break;
        }
        for (p, o) in moves {
            assigned.get_mut(SHARED).unwrap().remove(&p);
            for (g, set) in own.iter_mut() {
                if *g != o {
                    set.remove(&p);
                }
            }
            own.get_mut(o).unwrap().insert(p.clone());
            assigned.get_mut(o).unwrap().insert(p);
        }
    }

    // A group whose closure holds another group's seed declares that group.
    for (g, set) in &own {
        let declared = transitive_deps(&present, g);
        for p in set {
            if let Some(owner) = seed_owner.get(p.as_str())
                && owner != g
                && !declared.contains(owner)
            {
                problems.push(format!("{g}: needs {p}, a seed of {owner}, without declaring {owner} in deps"));
            }
        }
        if !declared.contains(&SHARED) && set.iter().any(|p| assigned[SHARED].contains(p)) {
            let shared: Vec<&String> = set.iter().filter(|p| assigned[SHARED].contains(*p)).take(5).collect();
            problems.push(format!(
                "{g}: needs packages of {SHARED} ({}) without declaring {SHARED} in deps",
                shared.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
    }
    // desktop-libs itself stands on the base alone.
    let shared_seeds: Vec<String> = assigned[SHARED].iter().cloned().collect();
    let shared_closure = universe.resolve(&shared_seeds, &base_set)?;
    let outside: Vec<&String> =
        shared_closure.iter().filter(|p| !base_set.contains(*p) && !assigned[SHARED].contains(*p)).collect();
    if !outside.is_empty() {
        let owners: BTreeSet<&str> = outside
            .iter()
            .filter_map(|p| assigned.iter().find(|(_, s)| s.contains(*p)).map(|(g, _)| *g))
            .collect();
        problems.push(format!(
            "{SHARED}: its packages need {} outside the base, owned by {owners:?}",
            outside.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
        ));
    }

    // The base and the groups together against today's locks.
    let mut union: BTreeMap<String, String> = BTreeMap::new();
    for p in &base_set {
        union.insert(p.clone(), locked[p].row.nv.clone());
    }
    for set in assigned.values() {
        for p in set {
            union.insert(p.clone(), locked[p].row.nv.clone());
        }
    }
    // A lock row that the variant's own lists no longer need (a package taken
    // out of [desktop] since the lock was written) is not part of today's
    // set: it is named, and `base lock` drops it.
    let mut listed: Vec<String> = base_contract.packages.clone();
    listed.extend(bundle.contract.desktop.packages.iter().cloned());
    let needed = universe.resolve(&listed, &BTreeSet::new())?;
    let orphans: Vec<&Row> = rows.iter().filter(|r| !needed.contains(&r.name)).collect();
    let today: BTreeMap<String, String> =
        rows.iter().filter(|r| needed.contains(&r.name)).map(|r| (r.name.clone(), r.nv.clone())).collect();
    let mut differences = Vec::new();
    for (name, nv) in &today {
        match union.get(name) {
            None => differences.push(format!("{nv}: in today's locks, in no group and not in the base")),
            Some(v) if v != nv => differences.push(format!("{name}: today's lock has {nv}, the groups {v}")),
            Some(_) => {}
        }
    }
    for (name, nv) in &union {
        if !today.contains_key(name) {
            differences.push(format!("{nv}: in the groups, not in today's locks"));
        }
    }

    // Nothing proprietary enters a group.
    for (g, set) in &assigned {
        for p in set {
            let l = locked[p].license.to_ascii_lowercase();
            if l.contains("proprietary") || l.contains("nonfree") || l.contains("non-free") {
                problems.push(format!("{g}: {p} is licensed {:?}", locked[p].license));
            }
        }
    }

    // The OS facts a group's configuration needs (its name, website, desktop
    // and palettes), from the variant contract: a public recipe cannot read
    // the root it is installed into, so they are written in.
    let os_facts = render_os_facts(distro, &bundle.contract);

    // Write (or compare) each group recipe's block for this C library.
    let release = &base_contract.release;
    let mut stale = Vec::new();
    for g in &groups {
        let set = assigned.get(g.name.as_str());
        let block = render_block(distro, backend, release, &pinned_on, &base_lock, &desktop_lock, set, &locked);
        let mut text = rewrite(&g.text, libc, &block)
            .with_context(|| format!("rewriting {}", rel(&root, &g.path)))?;
        text = replace_block(&text, &format!("GENERATED OS FACTS {libc}"), &os_facts)?;
        if text != g.text {
            if check {
                stale.push(rel(&root, &g.path));
            } else {
                fs::write(&g.path, &text).with_context(|| format!("writing {}", g.path.display()))?;
                println!("wrote {}", rel(&root, &g.path));
            }
        }
    }

    // Declared groups nothing in a group's closure comes from (said, not failed).
    let mut unneeded: Vec<String> = Vec::new();
    for name in &order {
        let g = present[name];
        if g.deps.is_empty() {
            continue;
        }
        let closure = universe.resolve(&g.seeds, &base_set)?;
        let used: BTreeSet<&str> = closure
            .iter()
            .filter(|p| !base_set.contains(*p))
            .filter_map(|p| assigned.iter().find(|(_, s)| s.contains(p)).map(|(o, _)| *o))
            .filter(|o| o != name)
            .collect();
        let reach: BTreeSet<&str> = used.iter().flat_map(|u| transitive_deps(&present, u)).collect();
        for d in &g.deps {
            if !used.contains(d.as_str()) && !reach.contains(d.as_str()) {
                unneeded.push(format!("{name} -> {d}"));
            }
        }
    }

    // The partition, for the person running it.
    println!();
    println!("{} ({libc}) from {} and {}:", distro.id(), rel(&root, &base_lock), rel(&root, &desktop_lock));
    let mib = |set: &BTreeSet<String>| set.iter().map(|p| locked[p].bytes).sum::<u64>() as f64 / 1048576.0;
    println!("  {:<22} {:>5} {:>9}  deps", "group", "pkgs", "MiB");
    println!("  {:<22} {:>5} {:>9.1}  (stays in the image)", "base", base_set.len(), mib(&base_set));
    let mut total = base_set.len();
    for name in &order {
        let set = &assigned[name];
        total += set.len();
        println!("  {:<22} {:>5} {:>9.1}  {}", name, set.len(), mib(set), present[name].deps.join(" "));
    }
    let lacking: Vec<&str> = groups.iter().filter(|g| !present.contains_key(g.name.as_str())).map(|g| g.name.as_str()).collect();
    if !lacking.is_empty() {
        println!("  no {libc} build: {}", lacking.join(" "));
    }
    if !moved.is_empty() {
        println!(
            "  moved out of the base list into groups: {}",
            moved.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ")
        );
    }
    println!("  added to the base: {}", base_adds(distro, backend).join(" "));
    if !unneeded.is_empty() {
        println!("  declared, nothing on {libc} comes from it (install_if joins aside): {}", unneeded.join(", "));
    }
    if !orphans.is_empty() {
        println!(
            "  left out: {} row(s) of today's locks that nothing in distro.toml's lists needs any more \
             (cargo xtask base lock {} drops them): {}",
            orphans.len(),
            distro.id(),
            orphans.iter().map(|r| r.nv.as_str()).collect::<Vec<_>>().join(" ")
        );
    }
    println!(
        "  base + groups: {total} packages; today's locks: {} packages ({} rows, {} left out)",
        today.len(),
        rows.len(),
        orphans.len()
    );
    println!("  union equals today's lock: {}", if differences.is_empty() { "yes" } else { "no" });

    let mut failures = Vec::new();
    if !differences.is_empty() {
        failures.push(format!(
            "the base and the groups differ from today's locks:\n  {}",
            differences.join("\n  ")
        ));
    }
    if !problems.is_empty() {
        failures.push(format!("the groups' dependencies do not hold:\n  {}", problems.join("\n  ")));
    }
    if !stale.is_empty() {
        failures.push(format!(
            "stale pins in {}; run cargo xtask base groups {}",
            stale.join(", "),
            distro.id()
        ));
    }
    if !failures.is_empty() {
        bail!("{}", failures.join("\n"));
    }
    if check {
        println!("every group recipe's {libc} pins are current");
    }
    Ok(())
}

/// The packages the resolver may pick: exactly today's locked builds.
enum Universe {
    Rpm { index: base::Index, assumed: Vec<String> },
    Apk(super::apk_lock::Index),
}

impl Universe {
    /// The closure of `seeds` over `under`, `under` included.
    fn resolve(&self, seeds: &[String], under: &BTreeSet<String>) -> Result<BTreeSet<String>> {
        match self {
            Universe::Rpm { index, assumed } => {
                let assumed: HashSet<&str> = assumed.iter().map(String::as_str).collect();
                index.resolve(seeds, under.clone(), &assumed)
            }
            Universe::Apk(index) => {
                let chosen = under.iter().map(|n| (n.clone(), index.by_name[n][0])).collect();
                Ok(index.resolve(seeds, chosen)?.into_keys().collect())
            }
        }
    }
}

/// Each row's file in the lock command's cache, downloaded there when it is
/// missing, and checked against the lock's SHA-256.
fn cached_files(cache: &Path, rows: &[Row]) -> Result<Vec<PathBuf>> {
    fs::create_dir_all(cache)?;
    let mut files = Vec::new();
    for row in rows {
        let file = cache.join(row.file_name());
        if !file.is_file() || sha256_file(&file)? != row.sha256 {
            let partial = file.with_extension("part");
            curl(&row.url, &partial)?;
            if sha256_file(&partial)? != row.sha256 {
                let _ = fs::remove_file(&partial);
                bail!("{} does not match the SHA-256 its lock names", row.url);
            }
            fs::rename(&partial, &file)?;
        }
        files.push(file);
    }
    Ok(files)
}

/// name -> (bytes, licence, keeps its var/ as seeds)
type Meta = BTreeMap<String, (u64, String, bool)>;

/// The RPMs' own requires, provides and files, read from their headers.
fn rpm_universe(rows: &[Row], files: &[PathBuf], assumed: &[String]) -> Result<(Universe, Meta)> {
    let out = Command::new("rpm")
        .args(["-qp", "--nosignature", "--nodigest", "--qf"])
        .arg("@@%{NAME}\t%{EPOCHNUM}\t%{VERSION}\t%{RELEASE}\t%{ARCH}\t%{LICENSE}\n[R %{REQUIRENAME}\n][P %{PROVIDENAME}\n][F %{FILENAMES}\n]")
        .args(files)
        .output()
        .context("running rpm -qp on the locked RPMs")?;
    if !out.status.success() {
        bail!("rpm -qp failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    let by_name: BTreeMap<&str, (&Row, &PathBuf)> = rows.iter().zip(files).map(|(r, f)| (r.name.as_str(), (r, f))).collect();
    let mut pkgs: BTreeMap<String, base::Rpm> = BTreeMap::new();
    let mut meta = Meta::new();
    let text = String::from_utf8_lossy(&out.stdout);
    let mut current: Option<base::Rpm> = None;
    // name -> (has var/ entries, ships a tmpfiles.d file)
    let mut files_of: BTreeMap<String, (bool, bool)> = BTreeMap::new();
    let finish = |rpm: Option<base::Rpm>, pkgs: &mut BTreeMap<String, base::Rpm>| {
        if let Some(rpm) = rpm {
            pkgs.insert(rpm.name.clone(), rpm);
        }
    };
    for line in text.lines() {
        if let Some(head) = line.strip_prefix("@@") {
            finish(current.take(), &mut pkgs);
            let f: Vec<&str> = head.split('\t').collect();
            if f.len() != 6 {
                bail!("unexpected rpm -qp header line {line:?}");
            }
            let (row, file) = by_name
                .get(f[0])
                .with_context(|| format!("rpm -qp read {}, which no lock row names", f[0]))?;
            meta.insert(f[0].to_string(), (fs::metadata(file)?.len(), f[5].to_string(), false));
            files_of.insert(f[0].to_string(), (false, false));
            current = Some(base::Rpm {
                name: f[0].into(),
                epoch: f[1].into(),
                version: f[2].into(),
                release: f[3].into(),
                arch: f[4].into(),
                url: row.url.clone(),
                sha256: row.sha256.clone(),
                provides: Vec::new(),
                requires: Vec::new(),
            });
        } else if let Some(rpm) = current.as_mut() {
            if let Some(r) = line.strip_prefix("R ") {
                if !r.starts_with("rpmlib(") {
                    rpm.requires.push(r.to_string());
                }
            } else if let Some(p) = line.strip_prefix("P ") {
                rpm.provides.push(p.to_string());
            } else if let Some(p) = line.strip_prefix("F ") {
                let flags = files_of.get_mut(&rpm.name).unwrap();
                flags.0 |= p.starts_with("/var/");
                flags.1 |= p.starts_with("/usr/lib/tmpfiles.d/");
                rpm.provides.push(p.to_string());
            }
        }
    }
    finish(current.take(), &mut pkgs);
    for (name, (var, tmpfiles)) in files_of {
        meta.get_mut(&name).unwrap().2 = var && !tmpfiles;
    }
    for row in rows {
        let rpm = pkgs.get(&row.name).with_context(|| format!("rpm -qp did not read {}", row.file_name()))?;
        if rpm.nevra() != row.nv {
            bail!("{} holds {}, its lock row says {}", row.file_name(), rpm.nevra(), row.nv);
        }
    }
    Ok((Universe::Rpm { index: base::Index::new(pkgs), assumed: assumed.to_vec() }, meta))
}

/// The APKs' own dependencies, provides and install_if, from their .PKGINFO.
fn apk_universe(rows: &[Row], files: &[PathBuf]) -> Result<(Universe, Meta)> {
    let mut pkgs = Vec::new();
    let mut meta = Meta::new();
    for (row, file) in rows.iter().zip(files) {
        let out = Command::new("tar")
            .arg("-xzOf")
            .arg(file)
            .arg(".PKGINFO")
            .stderr(std::process::Stdio::null())
            .output()
            .context("running tar")?;
        let info = String::from_utf8_lossy(&out.stdout);
        if !info.contains("pkgname = ") {
            bail!("{} holds no .PKGINFO", file.display());
        }
        let mut p = super::apk_lock::Pkg { repo: "main", ..Default::default() };
        let mut license = String::new();
        for line in info.lines() {
            let Some((k, v)) = line.split_once(" = ") else { continue };
            match k {
                "pkgname" => p.name = v.to_string(),
                "pkgver" => p.version = v.to_string(),
                "license" => license = v.to_string(),
                "depend" => p.depends.push(v.to_string()),
                "provides" => p.provides.push(v.to_string()),
                "install_if" => p.install_if.extend(v.split_whitespace().map(str::to_string)),
                "provider_priority" => p.priority = v.parse().unwrap_or(0),
                _ => {}
            }
        }
        if format!("{}-{}", p.name, p.version) != row.nv {
            bail!("{} holds {}-{}, its lock row says {}", file.display(), p.name, p.version, row.nv);
        }
        let list = Command::new("tar")
            .arg("-tzf")
            .arg(file)
            .stderr(std::process::Stdio::null())
            .output()
            .context("running tar")?;
        let var = String::from_utf8_lossy(&list.stdout).lines().any(|l| l.starts_with("var/"));
        meta.insert(row.name.clone(), (fs::metadata(file)?.len(), license, var));
        pkgs.push(p);
    }
    Ok((Universe::Apk(super::apk_lock::Index::new(pkgs)), meta))
}

/// The `[name, name-version, URL, SHA256]` rows of a lock's `packages()`.
fn read_rows(path: &Path) -> Result<Vec<Row>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let rows: Vec<Row> = text
        .lines()
        .skip_while(|line| !line.starts_with("fn packages()"))
        .filter_map(|line| {
            let line = line.trim().strip_prefix("[\"")?.strip_suffix("\"],")?;
            let parts: Vec<&str> = line.split("\", \"").collect();
            (parts.len() == 4).then(|| Row {
                name: parts[0].into(),
                nv: parts[1].into(),
                url: parts[2].into(),
                sha256: parts[3].into(),
            })
        })
        .collect();
    if rows.is_empty() {
        bail!("{} lists no packages", path.display());
    }
    Ok(rows)
}

fn locked_on(path: &Path) -> Result<String> {
    let text = fs::read_to_string(path)?;
    text.lines()
        .find_map(|l| l.strip_prefix("fn locked_on() { \"")?.strip_suffix("\" }").map(str::to_string))
        .with_context(|| format!("{} has no locked_on()", path.display()))
}

/// The quoted strings of the literal list that starts at `opener`
/// (`const SEEDS_GLIBC = [` …), up to its closing `]`; comments skipped.
fn literal_list(text: &str, opener: &str) -> Option<Vec<String>> {
    let start = text.find(opener)? + opener.len();
    let mut out = Vec::new();
    let mut rest = &text[start..];
    loop {
        let line_end = rest.find('\n').unwrap_or(rest.len());
        let line = &rest[..line_end];
        let code = line.split("//").next().unwrap_or("");
        let (code, done) = match code.find(']') {
            Some(i) => (&code[..i], true),
            None => (code, false),
        };
        out.extend(code.split('"').skip(1).step_by(2).map(str::to_string));
        if done {
            return Some(out);
        }
        if line_end >= rest.len() {
            return None;
        }
        rest = &rest[line_end + 1..];
    }
}

fn read_groups(dir: &Path, libc: &str) -> Result<Vec<Group>> {
    let mut groups = Vec::new();
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "rhai"))
        .collect();
    entries.sort();
    for path in entries {
        let text = fs::read_to_string(&path)?;
        if !text.contains("// BEGIN GENERATED PINS glibc") {
            continue;
        }
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let need = |opener: &str| {
            literal_list(&text, opener).with_context(|| format!("{}: no literal `{opener} … ]`", path.display()))
        };
        let glibc = need("const SEEDS_GLIBC = [")?;
        let musl = need("const SEEDS_MUSL = [")?;
        let deps = need("let deps = [")?;
        let about_libc = need("    libc: [")?;
        let seeds = if libc == "glibc" { glibc.clone() } else { musl.clone() };
        groups.push(Group {
            name,
            path,
            text,
            seeds,
            seeds_by_libc: BTreeMap::from([("glibc", glibc), ("musl", musl)]),
            deps,
            about_libc,
        });
    }
    if groups.is_empty() {
        bail!("{} holds no group recipe (none has a `// BEGIN GENERATED PINS glibc` block)", dir.display());
    }
    Ok(groups)
}

/// Every group `name` depends on, directly or through another.
fn transitive_deps<'a>(present: &BTreeMap<&'a str, &'a Group>, name: &str) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    let mut todo: Vec<&str> = present.get(name).map(|g| g.deps.iter().map(String::as_str).collect()).unwrap_or_default();
    while let Some(d) = todo.pop() {
        if let Some((key, g)) = present.get_key_value(d)
            && seen.insert(*key)
        {
            todo.extend(g.deps.iter().map(String::as_str));
        }
    }
    seen
}

/// The groups with every group before the groups that depend on it.
fn topo_order<'a>(present: &BTreeMap<&'a str, &'a Group>) -> Result<Vec<&'a str>> {
    let mut order: Vec<&str> = Vec::new();
    let mut placed: BTreeSet<&str> = BTreeSet::new();
    while order.len() < present.len() {
        let before = order.len();
        for (name, g) in present {
            if !placed.contains(name) && g.deps.iter().all(|d| placed.contains(d.as_str())) {
                order.push(name);
                placed.insert(name);
            }
        }
        if order.len() == before {
            let left: Vec<&str> = present.keys().filter(|n| !placed.contains(*n)).copied().collect();
            bail!("the group recipes' deps form a cycle among: {}", left.join(" "));
        }
    }
    Ok(order)
}

#[allow(clippy::too_many_arguments)]
fn render_block(
    distro: &Distro,
    backend: PackageBackend,
    release: &str,
    pinned_on: &str,
    base_lock: &Path,
    desktop_lock: &Path,
    set: Option<&BTreeSet<String>>,
    locked: &BTreeMap<String, Locked>,
) -> String {
    let libc = libc_of(backend);
    let k = prefix_of(libc);
    let id = distro.id();
    let file = |p: &Path| p.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let mut text = format!("// BEGIN GENERATED PINS {libc}\n");
    match set {
        None => {
            text.push_str(&format!(
                "// No {libc} build: this recipe names no seeds in SEEDS_{}. Written by\n\
                 // `cargo xtask base groups {id}`; do not edit.\n\
                 const {k}_PINNED_ON = \"\";\n\
                 const {k}_PINNED_BYTES = 0;\n\
                 const {k}_PINS = [];\n",
                libc.to_ascii_uppercase()
            ));
        }
        Some(set) => {
            let bytes: u64 = set.iter().map(|p| locked[p].bytes).sum();
            let (what, from) = match backend {
                PackageBackend::Rpm => (format!("Fedora {release} RPMs"), "lock"),
                PackageBackend::Apk => (format!("Alpine {release} APKs"), "lock"),
            };
            text.push_str(&format!(
                "// Written by `cargo xtask base groups {id}`; do not edit. This group's\n\
                 // share of the {what} in {} and {} ({from}ed on\n\
                 // {pinned_on}): {} packages, {bytes} bytes. Licences are each package's own.\n\
                 const {k}_PINNED_ON = \"{pinned_on}\";\n\
                 const {k}_PINNED_BYTES = {bytes};\n\
                 // [name, file, URL, SHA-256, bytes, licence, var/ kept as state seeds]\n\
                 const {k}_PINS = [\n",
                file(base_lock),
                file(desktop_lock),
                set.len(),
            ));
            for p in set {
                let l = &locked[p];
                text.push_str(&format!(
                    "    [\"{}\", \"{}\", \"{}\", \"{}\", {}, \"{}\", {}],\n",
                    p,
                    l.row.file_name(),
                    l.row.url,
                    l.row.sha256,
                    l.bytes,
                    l.license.replace('\\', "\\\\").replace('"', "\\\""),
                    l.seed
                ));
            }
            text.push_str("];\n");
        }
    }
    text.push_str(&format!("// END GENERATED PINS {libc}\n"));
    text
}

/// `text` with the `libc` block replaced by `block`, and the version and
/// network_max_bytes lines made to follow both blocks.
fn rewrite(text: &str, libc: &str, block: &str) -> Result<String> {
    let begin = format!("// BEGIN GENERATED PINS {libc}\n");
    let end = format!("// END GENERATED PINS {libc}\n");
    let start = text.find(&begin).with_context(|| format!("no `{}` line", begin.trim_end()))?;
    let stop = text[start..].find(&end).with_context(|| format!("no `{}` line", end.trim_end()))? + start + end.len();
    let mut out = format!("{}{block}{}", &text[..start], &text[stop..]);

    let string_const = |name: &str| -> String {
        let opener = format!("const {name} = \"");
        out.find(&opener)
            .and_then(|i| out[i + opener.len()..].split('"').next())
            .unwrap_or("")
            .to_string()
    };
    let number_const = |name: &str| -> u64 {
        let opener = format!("const {name} = ");
        out.find(&opener)
            .and_then(|i| out[i + opener.len()..].split(';').next())
            .and_then(|n| n.trim().parse().ok())
            .unwrap_or(0)
    };
    let dates = [string_const("RPM_PINNED_ON"), string_const("APK_PINNED_ON")];
    let version = dates.iter().max().cloned().unwrap_or_default().replace('-', ".");
    let bytes = number_const("RPM_PINNED_BYTES").max(number_const("APK_PINNED_BYTES"));
    // Room for the largest build, rounded up to whole MiB.
    let limit = bytes.div_ceil(1 << 20).max(1) << 20;
    if !version.is_empty() {
        out = replace_line(&out, "    version: \"", &format!("    version: \"{version}\","))?;
    }
    out = replace_line(&out, "    network_max_bytes: ", &format!("    network_max_bytes: {limit},"))?;
    Ok(out)
}

/// `text` with the first line starting with `prefix` replaced by `line`.
fn replace_line(text: &str, prefix: &str, line: &str) -> Result<String> {
    let start = if text.starts_with(prefix) {
        0
    } else {
        text.find(&format!("\n{prefix}")).with_context(|| format!("no line starting {prefix:?}"))? + 1
    };
    let stop = text[start..].find('\n').map_or(text.len(), |i| start + i);
    Ok(format!("{}{line}{}", &text[..start], &text[stop..]))
}

/// `text` with the lines between `// BEGIN <marker>` and `// END <marker>`
/// replaced by `block`'s inner lines, when the recipe has that block.
fn replace_block(text: &str, marker: &str, block: &str) -> Result<String> {
    let begin = format!("// BEGIN {marker}\n");
    let end = format!("// END {marker}\n");
    let Some(start) = text.find(&begin) else { return Ok(text.to_string()) };
    let stop = text[start..].find(&end).with_context(|| format!("no `// END {marker}` line"))? + start;
    Ok(format!("{}{begin}{block}{}", &text[..start], &text[stop..]))
}

/// The inner lines of a recipe's OS facts block for this variant: what its
/// os-release and look files say, as the builder writes them into the base.
fn render_os_facts(distro: &Distro, contract: &distro_contract::ConformanceContract) -> String {
    let libc = libc_of(contract.profile.packages);
    let k = prefix_of(libc);
    let palette = |p: &distro_contract::Palette| {
        let roles = [
            ("surface", &p.surface), ("surface_raised", &p.surface_raised), ("surface_sunken", &p.surface_sunken),
            ("text", &p.text), ("text_secondary", &p.text_secondary), ("text_muted", &p.text_muted),
            ("rule", &p.rule), ("hairline", &p.hairline), ("accent", &p.accent), ("highlight", &p.highlight),
            ("band", &p.band), ("on_band", &p.on_band), ("ok", &p.ok), ("warn", &p.warn), ("bad", &p.bad),
        ];
        let mut t = String::from("#{\n");
        for (role, hex) in roles {
            t.push_str(&format!("        {role}: \"{hex}\",\n"));
        }
        t.push_str("    }");
        t
    };
    let identity = &contract.identity;
    format!(
        "// Written by `cargo xtask base groups {id}` from distro/variants/{id}/distro.toml\n\
         // (identity, profile, look); do not edit.\n\
         const {k}_OS = #{{\n    id: \"{os_id}\",\n    name: \"{name}\",\n    home_url: \"{home}\",\n    \
         desktop: \"{desktop}\",\n    appearance: \"{appearance}\",\n    light: {light},\n    dark: {dark},\n}};\n",
        id = distro.id(),
        os_id = identity.os_id,
        name = identity.os_name,
        home = identity.home_url.as_deref().unwrap_or(""),
        desktop = contract.profile.desktop.as_str(),
        appearance = contract.profile.appearance.as_str(),
        light = palette(&contract.look.light),
        dark = palette(&contract.look.dark),
    )
}
