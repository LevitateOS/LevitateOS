//! `setup` and `doctor`: getting a checkout ready to build and boot. VMs run
//! under the host's libvirt, so its QEMU, OVMF and qemu-img are the VM tools.

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use crate::cli::Distro;
use distro_builder::process::{HostFamily, HostNeed, HOST_TOOLS};
use distro_contract::ProductId;

pub fn setup() -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    eprintln!("[step] Repo SSH key");
    install_tests::scenarios::ensure_login_key(&root)?;
    doctor(false)
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

/// `doctor --packages <dnf|apt>`: the host packages that provide every host tool, on one
/// line (CI installs them).
pub fn packages(family: &str) -> Result<()> {
    let family = HostFamily::from_name(family).ok_or_else(|| {
        crate::util::exit::usage(format!("unknown host package manager '{family}'; expected dnf or apt"))
    })?;
    println!("{}", distro_builder::process::host_packages(family).join(" "));
    Ok(())
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
    if !missing.is_empty() {
        let mut fixes: Vec<String> = missing.iter().filter_map(|c| c.fix.clone()).collect();
        fixes.dedup();
        return Err(crate::util::exit::missing(
            format!(
                "{} required check(s) failed: {}",
                missing.len(),
                missing.iter().map(|c| c.check.as_str()).collect::<Vec<_>>().join(", ")
            ),
            fixes.join("; "),
        ));
    }
    Ok(())
}

fn checks(root: &Path) -> Vec<Check> {
    let mut checks = Vec::new();

    // Scenario VMs are libvirt domains on the user session (Cockpit shows them).
    let uri = recqemu::libvirt::uri();
    checks.push(Check::new(
        format!("libvirt connection {uri}"),
        true,
        recqemu::libvirt::virsh(&uri, &["version"])
            .map(|out| out.lines().find(|l| l.contains("hypervisor")).unwrap_or("connected").trim().to_string())
            .map_err(|e| (format!("{e:#}"), "install libvirt-daemon-kvm (the session daemon starts on demand)".into())),
    ));
    checks.push(Check::new(
        "OVMF firmware",
        true,
        recqemu::find_ovmf()
            .map(|p| p.display().to_string())
            .map_err(|e| (format!("{e:#}"), "install edk2-ovmf".into())),
    ));
    let key = distro_contract::runs::ssh_key(root);
    checks.push(Check::new(
        "repo SSH key",
        true,
        if key.is_file() {
            Ok(key.display().to_string())
        } else {
            Err((format!("missing {}", key.display()), "cargo xtask setup".into()))
        },
    ));
    // The host tools: the one list the build checks and CI installs from.
    let family = HostFamily::detect();
    for tool in HOST_TOOLS {
        // Only `cargo xtask base lock` needs a base-lock-only tool.
        let required = tool.needs != [HostNeed::BaseLock];
        checks.push(Check::new(
            tool.program,
            required,
            tool.status()
                .map(|p| p.display().to_string())
                .map_err(|why| (why, tool.fix(family))),
        ));
    }

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
        let kernel = crate::tasks::kernels::status(distro).unwrap_or_else(|err| Err(format!("{err:#}")));
        checks.push(Check::new(
            format!("{} kernel", distro.id()),
            false,
            kernel.map_err(|reason| (reason, crate::tasks::kernels::fix(distro))),
        ));
        for product in ProductId::releases() {
            checks.push(Check::new(
                format!("{} {product} ISO", distro.id()),
                false,
                match crate::tasks::build::latest_iso(distro, product) {
                    Ok(Some(iso)) => Ok(iso.display().to_string()),
                    Ok(None) => Err(("not built".into(), format!("cargo xtask build {product} {}", distro.id()))),
                    Err(err) => Err((format!("{err:#}"), format!("cargo xtask build {product} {}", distro.id()))),
                },
            ));
        }
    }
    checks
}

/// How current a variant's base lock is, for either package backend.
fn base_lock_check(root: &Path, distro: &Distro) -> Option<Check> {
    let contract = distro_contract::load(root, distro.id()).ok()?.contract;
    let base = &contract.sources.rootfs_source.base;
    let (what, asked) = match contract.profile.packages {
        distro_contract::profile::PackageBackend::Rpm => ("Fedora", base.release.clone()),
        distro_contract::profile::PackageBackend::Apk => ("Alpine", base.release.clone()),
    };
    let fix = format!("cargo xtask base lock {}", distro.id());
    let path = crate::tasks::base::lock_path(root, distro, contract.profile.packages);
    let name = format!("{} {what} base lock", distro.id());
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Some(Check::new(name, true, Err(("missing".into(), fix))));
    };
    let field = |f: &str| {
        text.lines()
            .find_map(|l| l.trim().strip_prefix(&format!("fn {f}() {{ "))?.strip_suffix(" }"))
            .map(|v| v.trim_matches('"').to_string())
    };
    let release = field("release").unwrap_or_default();
    if release != asked {
        return Some(Check::new(
            name,
            true,
            Err((format!("locks {what} {release}, the contract asks for {asked}"), fix)),
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
            Some(days) if days <= 30 => Ok(format!("{what} {release}, locked {locked_on}")),
            Some(days) => Err((format!("{what} {release}, locked {days} days ago; updates are likely waiting"), fix)),
            None => Err((format!("cannot read the lock date in {}", path.display()), fix)),
        },
    ))
}
