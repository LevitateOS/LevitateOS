//! `setup`, `doctor` and `env`: getting a checkout ready to build and boot.

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use crate::cli::{Distro, Product};

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
    let qemu = match &repo_tools {
        Ok(Some(tools)) if tools.join("usr/bin/qemu-system-x86_64").is_file() => {
            Some(tools.join("usr/bin/qemu-system-x86_64"))
        }
        _ => which::which("qemu-system-x86_64").ok(),
    };
    checks.push(Check::new(
        "qemu-system-x86_64",
        true,
        qemu.map(|p| p.display().to_string())
            .ok_or_else(|| ("not in the repo tools or on PATH".into(), "cargo xtask setup (or install the host's qemu-system-x86)".into())),
    ));
    checks.push(Check::new(
        "OVMF firmware",
        true,
        match crate::util::repo::ovmf_path(root) {
            Ok(p) if p.is_file() => Ok(p.display().to_string()),
            Ok(p) => Err((format!("missing {}", p.display()), "cargo xtask setup".into())),
            Err(e) => Err(gap(&e)),
        },
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

    for distro in Distro::ALL {
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
        for product in [Product::LiveBoot, Product::LiveTools] {
            checks.push(Check::new(
                format!("{} {} ISO", distro.id(), product.id()),
                false,
                crate::tasks::build::latest_iso(distro, product)
                    .map(|p| p.display().to_string())
                    .ok_or_else(|| {
                        (
                            "not built".into(),
                            format!("cargo xtask build {} {}", product.id(), distro.id()),
                        )
                    }),
            ));
        }
    }
    checks
}
