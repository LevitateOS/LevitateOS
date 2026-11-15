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
//!
//! A glibc group may also name 32-bit seeds (`const SEEDS_I686`): no lock
//! holds an i686 build (the locks are x86_64/noarch), so those come from
//! Fedora's release and updates repositories; see [`add_i686`]. Only then does
//! this read the network. The i686 packages are pinned with the group's share
//! but stay out of the comparison with the locks, which still covers exactly
//! the locked packages; they are listed on their own, and `--check` then also
//! follows the repositories (a newer i686 build there makes the pins stale).

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
    /// `const SEEDS_I686`, as `NAME.i686`: 32-bit seeds, from Fedora's
    /// repositories (see [`add_i686`]). Empty when generating for musl.
    added: Vec<String>,
    /// The `SEEDS_I686` list whatever is being generated, for the libc check.
    added_glibc: Vec<String>,
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
    let (universe, meta, locked_rpms) = match backend {
        PackageBackend::Rpm => {
            let (u, m, r) = rpm_universe(&rows, &files, &base_contract.provided_elsewhere)?;
            (u, m, r)
        }
        PackageBackend::Apk => {
            let (u, m) = apk_universe(&rows, &files)?;
            (u, m, BTreeMap::new())
        }
    };
    let mut locked: BTreeMap<String, Locked> = rows
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
        .filter(|g| g.name == SHARED || !g.seeds.is_empty() || !g.added.is_empty())
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
            .filter(|(l, s)| !s.is_empty() || g.name == SHARED || (**l == "glibc" && !g.added_glibc.is_empty()))
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
    listed.extend(base::group_seeds(&root, backend)?);
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

    // The i686 packages (SEEDS_I686): resolved after the locked share-out is
    // final, so they never move a locked package, and kept out of the
    // comparison with the locks above.
    let added: BTreeMap<String, BTreeSet<String>> = if present.values().any(|g| !g.added.is_empty()) {
        let found = add_i686(
            &release_of(&base_contract.release)?,
            &present,
            &order,
            &base_set,
            &assigned,
            &locked_rpms,
            &base_contract.provided_elsewhere,
            &mut problems,
        )?;
        for (name, extra) in &found.locked {
            locked.insert(name.clone(), extra.clone());
        }
        found.sets
    } else {
        BTreeMap::new()
    };
    let added_count: usize = added.values().map(BTreeSet::len).sum();
    for (g, set) in &added {
        for p in set {
            let l = locked[p].license.to_ascii_lowercase();
            if l.contains("proprietary") || l.contains("nonfree") || l.contains("non-free") {
                problems.push(format!("{g}: {p} is licensed {:?}", locked[p].license));
            }
        }
        assigned.get_mut(g.as_str()).unwrap().extend(set.iter().cloned());
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
        let extra = added.get(&g.name);
        let block =
            render_block(distro, backend, release, &pinned_on, &base_lock, &desktop_lock, set, extra, &locked);
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
    total -= added_count;
    if added_count > 0 {
        println!(
            "  i686, from Fedora {}'s release and updates repositories (in the counts above, \
             not in the comparison with the locks):",
            base_contract.release
        );
        for (g, set) in &added {
            println!(
                "    {g}: {} package(s), {:.1} MiB: {}",
                set.len(),
                mib(set),
                set.iter().map(String::as_str).collect::<Vec<_>>().join(" ")
            );
        }
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
fn rpm_universe(
    rows: &[Row],
    files: &[PathBuf],
    assumed: &[String],
) -> Result<(Universe, Meta, BTreeMap<String, base::Rpm>)> {
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
    let rpms = pkgs.clone();
    Ok((Universe::Rpm { index: base::Index::new(pkgs), assumed: assumed.to_vec() }, meta, rpms))
}

/// What [`add_i686`] found: each group's i686 packages, and what the pin
/// blocks need to know about each.
struct Added {
    sets: BTreeMap<String, BTreeSet<String>>,
    locked: BTreeMap<String, Locked>,
}

/// The Fedora release number of the variant's base.
fn release_of(release: &str) -> Result<String> {
    release.parse::<u32>().with_context(|| format!("base.release {release:?} is not a Fedora release number"))?;
    Ok(release.to_string())
}

/// A group's 32-bit packages (`const SEEDS_I686`), from Fedora's release and
/// updates repositories, since no lock holds an i686 build. They are resolved
/// over the base, the group's own locked share, and the locked shares and
/// i686 packages of every group it declares, from a universe of exactly
/// today's locked builds plus the repositories' i686 builds, so a locked
/// package always satisfies what it can. What else they need must already be
/// beneath the group: a locked package another group owns is named with that
/// owner to declare; an x86_64/noarch package no lock holds is named to be
/// seeded in SEEDS_GLIBC, so `base lock` locks it (and checks its signature)
/// like every other 64-bit package. An i686 build whose x86_64 twin is locked
/// is taken at the twin's very version (the two share their non-library
/// files, which must be identical); when the repositories no longer have that
/// version, the locks are named as behind them. Bytes and licence come from
/// the repositories' metadata (checked against repomd.xml; the RPMs' own
/// signatures are not checked here, the SHA-256 pins are what Recipe checks);
/// an i686 package's var/ is never kept as a state seed (the 32-bit libraries
/// have none of their own).
#[allow(clippy::too_many_arguments)]
fn add_i686(
    release: &str,
    present: &BTreeMap<&str, &Group>,
    order: &[&str],
    base_set: &BTreeSet<String>,
    assigned: &BTreeMap<&str, BTreeSet<String>>,
    locked_rpms: &BTreeMap<String, base::Rpm>,
    assumed: &[String],
    problems: &mut Vec<String>,
) -> Result<Added> {
    let mirror = "https://dl.fedoraproject.org/pub/fedora/linux";
    let mut repo = RepoBuilds::default();
    for url in [
        format!("{mirror}/releases/{release}/Everything/x86_64/os/"),
        format!("{mirror}/updates/{release}/Everything/x86_64/"),
    ] {
        eprintln!("[step] Read {url} (i686 packages)");
        read_repo(&url, &mut repo)?;
    }

    // The universe: today's locked builds, the repositories' x86_64/noarch
    // builds of every other name (only so that needing one can be named), and
    // one i686 build per name.
    let mut pkgs: BTreeMap<String, base::Rpm> = locked_rpms.clone();
    let mut info: BTreeMap<String, (u64, String)> = BTreeMap::new();
    for (name, (rpm, size, license)) in &repo.native {
        if !pkgs.contains_key(name) {
            pkgs.insert(name.clone(), rpm.clone());
            info.insert(name.clone(), (*size, license.clone()));
        }
    }
    let mut twin_problems: BTreeMap<String, String> = BTreeMap::new();
    for (name, builds) in &repo.i686 {
        let vr = |r: &base::Rpm| format!("{}:{}-{}", r.epoch, r.version, r.release);
        // Its twin: the locked x86_64 build, else the repositories' own.
        let twin_locked = locked_rpms.contains_key(name);
        let twin = locked_rpms.get(name).or_else(|| repo.native.get(name).map(|(r, _, _)| r));
        let pick = match twin.filter(|t| t.arch != "noarch") {
            Some(t) => match builds.iter().find(|(b, _, _)| vr(b) == vr(t)) {
                Some(b) => b,
                None => {
                    twin_problems.insert(
                        format!("{name}.i686"),
                        format!(
                            "{name}.i686: {} {name} at {}, Fedora's repositories have the i686 build only at {}; \
                             the two must be the same version: run cargo xtask base lock levitate, then this again",
                            if twin_locked { "today's lock has" } else { "the repositories have" },
                            vr(t),
                            builds.iter().map(|(b, _, _)| vr(b)).collect::<Vec<_>>().join(", ")
                        ),
                    );
                    builds.last().unwrap()
                }
            },
            None => builds.last().unwrap(),
        };
        let (rpm, size, license) = pick;
        let key = format!("{name}.i686");
        let mut rpm = rpm.clone();
        rpm.name = key.clone();
        info.insert(key.clone(), (*size, license.clone()));
        pkgs.insert(key, rpm);
    }
    let index = base::Index::new(pkgs.clone());
    let assumed: HashSet<&str> = assumed.iter().map(String::as_str).collect();

    let owner_of = |p: &str| -> String {
        assigned.iter().find(|(_, s)| s.contains(p)).map(|(g, _)| g.to_string()).unwrap_or_else(|| "nobody".into())
    };
    let mut sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for name in order {
        let g = present[name];
        if g.added.is_empty() {
            continue;
        }
        let mut bad = false;
        for s in &g.added {
            if !pkgs.contains_key(s) {
                problems.push(format!(
                    "{name}: SEEDS_I686 names {}, which has no i686 build in Fedora {release}'s release and updates repositories",
                    s.trim_end_matches(".i686")
                ));
                bad = true;
            }
        }
        if bad {
            continue;
        }
        let declared = transitive_deps(present, name);
        let mut under = base_set.clone();
        under.extend(assigned[name].iter().cloned());
        for d in &declared {
            under.extend(assigned.get(d).into_iter().flatten().cloned());
            under.extend(sets.get(*d).into_iter().flatten().cloned());
        }
        let closure = index.resolve(&g.added, under.clone(), &assumed)?;
        let mut mine = BTreeSet::new();
        for p in closure.difference(&under) {
            if locked_rpms.contains_key(p) {
                problems.push(format!(
                    "{name}: its i686 packages need {p}, a locked package of {}, without declaring it in deps",
                    owner_of(p)
                ));
            } else if !p.ends_with(".i686") {
                problems.push(format!(
                    "{name}: its i686 packages need {p}, which no lock holds; name it in this group's SEEDS_GLIBC \
                     and run cargo xtask base lock levitate, then this again"
                ));
            } else if let Some((other, _)) = sets.iter().find(|(_, s)| s.contains(p)) {
                problems.push(format!(
                    "{p} is needed by the i686 packages of both {other} and {name}; one of them declares the other"
                ));
            } else {
                if let Some(why) = twin_problems.get(p) {
                    problems.push(why.clone());
                }
                mine.insert(p.clone());
            }
        }
        sets.insert(name.to_string(), mine);
    }

    let mut locked = BTreeMap::new();
    for p in sets.values().flatten() {
        let rpm = &pkgs[p];
        let (bytes, license) = info[p].clone();
        locked.insert(
            p.clone(),
            Locked {
                row: Row {
                    name: p.clone(),
                    nv: format!("{}-{}-{}.{}", p.trim_end_matches(".i686"), rpm.version, rpm.release, rpm.arch),
                    url: rpm.url.clone(),
                    sha256: rpm.sha256.clone(),
                },
                bytes,
                license,
                seed: false,
            },
        );
    }
    Ok(Added { sets, locked })
}

/// One build from a repository's primary metadata: the build, its package
/// size and its licence.
type RepoBuild = (base::Rpm, u64, String);

/// What [`read_repo`] keeps.
#[derive(Default)]
struct RepoBuilds {
    /// x86_64/noarch: the newest build of each name (the updates repository,
    /// read after the release one, replaces the release build).
    native: BTreeMap<String, RepoBuild>,
    /// i686: every build of each name, release builds first.
    i686: BTreeMap<String, Vec<RepoBuild>>,
}

/// Read a repository's primary metadata (checked against repomd.xml) into
/// `out`. `base lock` reads the same metadata for x86_64/noarch only and
/// without sizes or licences, which is why this reads it again.
fn read_repo(repo: &str, out: &mut RepoBuilds) -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let repomd_path = tmp.path().join("repomd.xml");
    curl(&format!("{repo}repodata/repomd.xml"), &repomd_path)?;
    let repomd = fs::read_to_string(&repomd_path)?;
    let section = repomd
        .split("<data ")
        .find(|s| s.starts_with("type=\"primary\""))
        .context("repomd.xml lists no primary index")?;
    let after = |text: &str, open: &str, close: char| -> Option<String> {
        let start = text.find(open)? + open.len();
        Some(text[start..].split(close).next()?.to_string())
    };
    let href = after(section, "<location href=\"", '"').context("primary index has no location")?;
    let sha = after(section, "<checksum type=\"sha256\">", '<').context("primary index has no sha256 checksum")?;
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
        .stdout(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("running {decompress}"))?;
    parse_repo(std::io::BufReader::new(child.stdout.take().unwrap()), repo, out)?;
    if !child.wait()?.success() {
        bail!("decompressing {href} failed");
    }
    Ok(())
}

fn parse_repo(reader: impl std::io::BufRead, repo: &str, out: &mut RepoBuilds) -> Result<()> {
    use quick_xml::events::Event;

    let unescape = |s: &str| s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&");
    let mut xml = quick_xml::Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut current: Option<RepoBuild> = None;
    let mut field: Option<&'static str> = None;
    let mut deps = "";
    loop {
        match xml.read_event_into(&mut buf)? {
            Event::Start(e) | Event::Empty(e) => {
                let attr = |key: &[u8]| {
                    e.attributes()
                        .flatten()
                        .find(|a| a.key.as_ref() == key)
                        .map(|a| unescape(&String::from_utf8_lossy(&a.value)))
                };
                match e.name().as_ref() {
                    b"package" => {
                        current = Some((
                            base::Rpm {
                                name: String::new(),
                                epoch: "0".into(),
                                version: String::new(),
                                release: String::new(),
                                arch: String::new(),
                                url: String::new(),
                                sha256: String::new(),
                                provides: Vec::new(),
                                requires: Vec::new(),
                            },
                            0,
                            String::new(),
                        ))
                    }
                    b"rpm:provides" => deps = "provides",
                    b"rpm:requires" => deps = "requires",
                    b"rpm:recommends" | b"rpm:suggests" | b"rpm:supplements" | b"rpm:enhances" | b"rpm:conflicts"
                    | b"rpm:obsoletes" => deps = "",
                    b"rpm:entry" => {
                        if let (Some((rpm, _, _)), Some(n)) = (current.as_mut(), attr(b"name")) {
                            match deps {
                                "provides" => rpm.provides.push(n),
                                "requires" if !n.starts_with("rpmlib(") => rpm.requires.push(n),
                                _ => {}
                            }
                        }
                    }
                    b"file" => field = Some("file"),
                    b"name" => field = Some("name"),
                    b"arch" => field = Some("arch"),
                    b"rpm:license" => field = Some("license"),
                    b"checksum" => field = (attr(b"type").as_deref() == Some("sha256")).then_some("sha256"),
                    b"version" => {
                        if let Some((rpm, _, _)) = current.as_mut() {
                            rpm.epoch = attr(b"epoch").unwrap_or_else(|| "0".into());
                            rpm.version = attr(b"ver").unwrap_or_default();
                            rpm.release = attr(b"rel").unwrap_or_default();
                        }
                    }
                    b"size" => {
                        if let Some((_, size, _)) = current.as_mut() {
                            *size = attr(b"package").and_then(|s| s.parse().ok()).unwrap_or(0);
                        }
                    }
                    b"location" => {
                        if let (Some((rpm, _, _)), Some(href)) = (current.as_mut(), attr(b"href")) {
                            rpm.url = format!("{repo}{href}");
                        }
                    }
                    _ => {}
                }
            }
            Event::Text(t) => {
                if let (Some(f), Some((rpm, _, license))) = (field, current.as_mut()) {
                    let text = unescape(&String::from_utf8_lossy(&t));
                    match f {
                        "name" => rpm.name = text,
                        "arch" => rpm.arch = text,
                        "sha256" => rpm.sha256 = text,
                        "file" => rpm.provides.push(text),
                        "license" => *license = text,
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
                    && let Some(build) = current.take()
                    && !build.0.sha256.is_empty()
                {
                    match build.0.arch.as_str() {
                        "x86_64" | "noarch" => {
                            out.native.insert(build.0.name.clone(), build);
                        }
                        "i686" => out.i686.entry(build.0.name.clone()).or_default().push(build),
                        _ => {}
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    Ok(())
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
        // Optional: only a group that takes packages beyond the locks has it.
        let added_glibc: Vec<String> = literal_list(&text, "const SEEDS_I686 = [")
            .unwrap_or_default()
            .into_iter()
            .map(|s| format!("{s}.i686"))
            .collect();
        let seeds = if libc == "glibc" { glibc.clone() } else { musl.clone() };
        let added = if libc == "glibc" { added_glibc.clone() } else { Vec::new() };
        groups.push(Group {
            name,
            path,
            text,
            seeds,
            seeds_by_libc: BTreeMap::from([("glibc", glibc), ("musl", musl)]),
            deps,
            about_libc,
            added,
            added_glibc,
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
    added: Option<&BTreeSet<String>>,
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
                 const {k}_PINNED_BYTES = {bytes};\n",
                file(base_lock),
                file(desktop_lock),
                set.len(),
            ));
            if let Some(added) = added.filter(|a| !a.is_empty()) {
                let i686 = added.iter().filter(|p| p.ends_with(".i686")).count();
                text.push_str(&format!(
                    "// {i686} of them are i686 builds (SEEDS_I686), which no lock holds: those Fedora\n\
                     // {release}'s release and updates repositories held when this was written, each\n\
                     // at the version of its locked x86_64 twin.\n"
                ));
            }
            text.push_str("// [name, file, URL, SHA-256, bytes, licence, var/ kept as state seeds]\n");
            text.push_str(&format!("const {k}_PINS = [\n"));
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
    // A recipe that also downloads something besides its packages (steam:
    // Valve's launcher) says how much in OTHER_DOWNLOAD_BYTES.
    let bytes = number_const("RPM_PINNED_BYTES").max(number_const("APK_PINNED_BYTES"))
        + number_const("OTHER_DOWNLOAD_BYTES");
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
