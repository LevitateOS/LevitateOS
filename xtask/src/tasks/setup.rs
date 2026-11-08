//! `setup`, `doctor` and `env`: getting a checkout ready to build and boot.

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use crate::cli::Distro;
use distro_contract::ProductId;

pub fn setup() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    eprintln!("[step] Install the QEMU/OVMF tools into .artifacts/tools");
    crate::util::distro_builder::run_bare(&root, &["artifact", "prepare-qemu-tools"])?;
    eprintln!("[step] Repo SSH key");
    crate::util::ssh::ensure_repo_key(&root)?;
    doctor(false)
}

pub fn env() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    print!("{}", crate::util::tools_env::shell_exports(&root)?);
    Ok(())
}

#[derive(Serialize)]
struct Check {
    check: String,
    ok: bool,
    /// Without it, booting or testing cannot work at all.
    required: bool,
    detail: String,
    /// The command that fixes it, when it is not ok.
    #[serde(skip_serializing_if = "Option::is_none")]
    fix: Option<String>,
}

impl Check {
    fn new(check: impl Into<String>, required: bool, result: Result<String, (String, String)>) -> Self {
        let check = check.into();
        match result {
            Ok(detail) => Self { check, ok: true, required, detail, fix: None },
            Err((detail, fix)) => Self { check, ok: false, required, detail, fix: Some(fix) },
        }
    }
}

pub fn doctor(json: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let checks = checks(&root);

    if json {
        println!("{}", serde_json::to_string_pretty(&checks)?);
    } else {
        for c in &checks {
            let mark = match (c.ok, c.required) {
                (true, _) => "[ok]  ",
                (false, true) => "[FAIL]",
                (false, false) => "[--]  ",
            };
            eprintln!("{mark} {:<26} {}", c.check, c.detail);
            if let Some(fix) = &c.fix {
                eprintln!("       fix: {fix}");
            }
        }
    }

    let missing: Vec<&Check> = checks.iter().filter(|c| c.required && !c.ok).collect();
    if let Some(first) = missing.first() {
        return Err(crate::util::exit::missing(
            format!("{} required check(s) failed", missing.len()),
            first.fix.clone().unwrap_or_default(),
        ));
    }
    Ok(())
}

/// A failed prerequisite as (detail, fix).
fn gap(error: &anyhow::Error) -> (String, String) {
    match error.downcast_ref::<crate::util::exit::MissingPrereq>() {
        Some(missing) => (missing.what.clone(), missing.fix.clone()),
        None => (format!("{error:#}"), "cargo xtask setup".into()),
    }
}

fn checks(root: &Path) -> Vec<Check> {
    let mut checks = Vec::new();

    let repo_tools = crate::util::repo::repo_tools(root);
    checks.push(Check::new(
        "repo QEMU tools",
        false,
        match &repo_tools {
            Ok(Some(path)) => Ok(path.display().to_string()),
            Ok(None) => Err(("not installed; the host's QEMU is used".into(), "cargo xtask setup".into())),
            Err(e) => Err(gap(e)),
        },
    ));
    // Scenario VMs are libvirt domains on the user session (Cockpit shows them).
    let uri = recqemu::libvirt::uri();
    checks.push(Check::new(
        "libvirt (virsh)",
        true,
        which::which("virsh")
            .map(|p| p.display().to_string())
            .map_err(|_| ("not on PATH; scenario VMs run under libvirt".into(), "install libvirt-client and libvirt-daemon-kvm".into())),
    ));
    checks.push(Check::new(
        format!("libvirt connection {uri}"),
        true,
        recqemu::libvirt::virsh(&uri, &["version"])
            .map(|out| out.lines().find(|l| l.contains("hypervisor")).unwrap_or("connected").trim().to_string())
            .map_err(|e| (format!("{e:#}"), "install libvirt-daemon-kvm (the session daemon starts on demand)".into())),
    ));
    checks.push(Check::new(
        "passt",
        true,
        which::which("passt")
            .map(|p| p.display().to_string())
            .map_err(|_| ("not on PATH (user networking and the SSH forward of scenario VMs)".into(), "install passt".into())),
    ));
    checks.push(Check::new(
        "qemu-img",
        true,
        which::which("qemu-img")
            .map(|p| p.display().to_string())
            .map_err(|_| ("not on PATH (scenario disk images)".into(), "cargo xtask setup (or install qemu-img)".into())),
    ));
    checks.push(Check::new(
        "OVMF firmware",
        true,
        recqemu::find_ovmf()
            .map(|p| p.display().to_string())
            .map_err(|e| (format!("{e:#}"), "install edk2-ovmf".into())),
    ));
    let key = crate::util::ssh::repo_key_path(root);
    checks.push(Check::new(
        "repo SSH key",
        true,
        if key.is_file() {
            Ok(key.display().to_string())
        } else {
            Err((format!("missing {}", key.display()), "cargo xtask setup".into()))
        },
    ));
    for (tool, required, why) in [
        ("ssh", true, "logs in to scenario guests"),
        ("ssh-keygen", true, "creates the repo SSH key"),
        ("bun", false, "TypeScript apps, docs and `cargo xtask check`"),
    ] {
        checks.push(Check::new(
            tool,
            required,
            which::which(tool)
                .map(|p| p.display().to_string())
                .map_err(|_| (format!("not on PATH ({why})"), format!("install {tool}"))),
        ));
    }
    checks.push(Check::new(
        "ISO tool",
        true,
        ["xorriso", "genisoimage", "mkisofs"]
            .into_iter()
            .find_map(|t| which::which(t).ok())
            .map(|p| p.display().to_string())
            .ok_or_else(|| ("none of xorriso/genisoimage/mkisofs (boot injection media)".into(), "install xorriso".into())),
    ));

    for id in crate::cli::distro_ids() {
        let distro = match Distro::load(&id) {
            Ok(distro) => distro,
            Err(err) => {
                checks.push(Check::new(
                    format!("{id} variant"),
                    true,
                    Err((format!("{err:#}"), format!("fix distro/variants/{id}/distro.toml"))),
                ));
                continue;
            }
        };
        let distro = &distro;
        if let Some(check) = base_lock_check(root, distro) {
            checks.push(check);
        }
        let target = crate::tasks::kernels::common::target_for(distro);
        checks.push(Check::new(
            format!("{} kernel", distro.id()),
            false,
            target.and_then(|t| crate::tasks::kernels::common::verify_one(root, &t)).map_err(|_| {
                (
                    "not built".into(),
                    format!(
                        "cargo xtask build kernel {0} --prebuilt (download) or cargo xtask build kernel {0}",
                        distro.id()
                    ),
                )
            }),
        ));
        for product in ProductId::releases() {
            checks.push(Check::new(
                format!("{} {product} ISO", distro.id()),
                false,
                crate::tasks::build::latest_iso(distro, product)
                    .map(|p| p.display().to_string())
                    .ok_or_else(|| {
                        (
                            "not built".into(),
                            format!("cargo xtask build {product} {}", distro.id()),
                        )
                    }),
            ));
        }
    }
    checks
}

/// How current a variant's Fedora base lock is (none for a non-RPM base).
fn base_lock_check(root: &Path, distro: &Distro) -> Option<Check> {
    let contract = distro_contract::load(root, distro.id()).ok()?.contract;
    let base = contract.sources.rootfs_source.rpm_base?;
    let fix = format!("cargo xtask base lock {}", distro.id());
    let path = crate::tasks::base::lock_path(root, distro);
    let name = format!("{} Fedora base lock", distro.id());
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Some(Check::new(name, true, Err(("missing".into(), fix))));
    };
    let field = |f: &str| {
        text.lines()
            .find_map(|l| l.trim().strip_prefix(&format!("fn {f}() {{ "))?.strip_suffix(" }"))
            .map(|v| v.trim_matches('"').to_string())
    };
    let release = field("release").unwrap_or_default();
    if release != base.fedora_release.to_string() {
        return Some(Check::new(
            name,
            true,
            Err((format!("locks Fedora {release}, the contract asks for {}", base.fedora_release), fix)),
        ));
    }
    let locked_on = field("locked_on").unwrap_or_default();
    let age_days = std::process::Command::new("date")
        .args(["-d", &locked_on, "+%s"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok()?.trim().parse::<i64>().ok())
        .map(|then| (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(then) - then) / 86_400);
    Some(Check::new(
        name,
        false,
        match age_days {
            Some(days) if days <= 30 => Ok(format!("Fedora {release}, locked {locked_on}")),
            Some(days) => Err((format!("Fedora {release}, locked {days} days ago; updates are likely waiting"), fix)),
            None => Err((format!("cannot read the lock date in {}", path.display()), fix)),
        },
    ))
}
