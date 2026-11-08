//! `cargo xtask gen`: each tool exports its own contract (its `contracts`
//! feature: the JSON Schemas of what it prints and one TypeScript module of
//! types, protocol constants and stream readers). This writes every exported
//! contract, the module into each TUI app whose manifest lists one of the
//! contract's binaries (`src/generated/<module>.ts`), and captures each
//! tool's `--help` for the docs site. Each variant's facts (identity and
//! distro profile) go into docs-content's `src/distros.ts`, for the docs and
//! the TUIs.

use anyhow::{Context, Result, bail};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tool_contract::Contract;

/// The tools' contract exports, one per tool.
const CONTRACTS: &[fn() -> Result<Contract>] =
    &[recpart::contract::export, recab::contract::export, install_tools::contract::export];

/// Where generated text goes: written when it changed, or (with `check`)
/// only compared.
pub struct Out {
    check: bool,
    stale: Vec<PathBuf>,
    written: usize,
}

impl Out {
    fn emit(&mut self, path: &Path, text: &str) -> Result<()> {
        let current = match fs::read_to_string(path) {
            Ok(current) => Some(current),
            Err(err) if err.kind() == ErrorKind::NotFound => None,
            Err(err) => return Err(err).with_context(|| format!("reading {}", path.display())),
        };
        if current.as_deref() == Some(text) {
            return Ok(());
        }
        if self.check {
            self.stale.push(path.to_path_buf());
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, text).with_context(|| format!("writing {}", path.display()))?;
        println!("wrote {}", path.display());
        self.written += 1;
        Ok(())
    }
}

pub fn run(check: bool) -> Result<()> {
    let mut out = Out { check, stale: Vec::new(), written: 0 };
    generate(&mut out)?;
    if !out.stale.is_empty() {
        for path in &out.stale {
            eprintln!("out of date: {}", path.display());
        }
        bail!("{} generated file(s) are out of date; run cargo xtask gen", out.stale.len());
    }
    if out.written == 0 {
        println!("generated files are current");
    }
    Ok(())
}

fn generate(out: &mut Out) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let apps = distro_builder::tui_apps::Workspace::load(&root)?.apps;
    for export in CONTRACTS {
        let contract = export()?;
        for (path, json) in &contract.schemas {
            out.emit(path, json)?;
        }
        for app in &apps {
            if app.tools.iter().any(|tool| contract.binaries.contains(&tool.as_str())) {
                let path = app.dir.join(format!("src/generated/{}.ts", contract.module));
                out.emit(&path, &format_ts(&root, &path, &contract.typescript)?)?;
            }
        }
    }

    write_tool_help(out, &root)?;
    write_distros(out, &root)?;
    Ok(())
}

/// One record per variant, as `distro_contract::load` reads it: identity, the names
/// derived from it, and the profile, plus the profile vocabulary and the os-release
/// field each fact is written to on the image.
fn write_distros(out: &mut Out, root: &Path) -> Result<()> {
    use distro_contract::profile::{self, Desktop, InitSystem, KernelLine, Libc, PackageBackend, PrivilegeTool};
    use serde_json::json;

    let mut distros = Vec::new();
    for id in distro_contract::distro_ids(root) {
        let variant = distro_contract::load(root, &id)?;
        let c = &variant.contract;
        let kernel = distro_builder::kernel::pin(&variant)?;
        let privilege = c.profile.privilege;
        distros.push(json!({
            "id": variant.id,
            "osName": c.identity.os_name,
            "osId": c.identity.os_id,
            "osVersion": c.identity.os_version,
            "isoLabel": c.identity.iso_label,
            "defaultHostname": c.identity.default_hostname,
            "loginPrompt": c.identity.login_prompt,
            "isoFile": c.artifacts.release_iso,
            "desktop": {
                "isoFile": c.artifacts.iso(distro_contract::ProductId::LiveDesktop),
                "displayManager": c.desktop.display_manager,
                "liveUser": distro_contract::install_ux::LIVE_DESKTOP_USER,
                "livePassword": distro_contract::install_ux::LIVE_DESKTOP_PASSWORD,
                "installScreen": format!("/{}", distro_contract::install_ux::LAUNCHER),
            },
            "installedInitramfs": c.artifacts.initramfs_installed,
            "installShell": distro_contract::install_ux::SHELL,
            "kernel": { "version": kernel.version, "localversion": kernel.localversion },
            "profile": c.profile,
            "privilegeTool": {
                "configFile": privilege.config_file(),
                "adminRuleFile": format!("/{}", privilege.admin_rule_file()),
                "adminRule": privilege.admin_rule(),
            },
        }));
    }
    let values = json!({
        "packages": PackageBackend::ALL,
        "init": InitSystem::ALL,
        "libc": Libc::ALL,
        "privilege": PrivilegeTool::ALL,
        "desktop": Desktop::ALL,
        "kernel_line": KernelLine::ALL,
    });
    let fields = json!({
        "packages": profile::REC_PACKAGES,
        "init": profile::REC_INIT,
        "libc": profile::REC_LIBC,
        "privilege": profile::REC_PRIVILEGE,
        "desktop": profile::REC_DESKTOP,
        "kernel_line": profile::REC_KERNEL_LINE,
    });
    let text = format!(
        "// Generated by `cargo xtask gen` from distro/variants/*/distro.toml and distro-profile. Do not edit.\n\n\
         /** Every variant, by id: its identity, the names derived from it, and its distro profile. */\n\
         export const DISTROS = {} as const;\n\n\
         export type Distro = (typeof DISTROS)[number];\n\
         export type DistroId = Distro[\"id\"];\n\n\
         /** The values each profile fact can take. */\n\
         export const PROFILE_VALUES = {} as const;\n\n\
         /** The /etc/os-release field each fact is written to on the image. */\n\
         export const OS_RELEASE_FIELDS = {} as const;\n\n\
         /** The group whose members become root through the privilege tool. */\n\
         export const ADMIN_GROUP = {} as const;\n\n\
         export type Profile = {{ -readonly [K in keyof typeof PROFILE_VALUES]: (typeof PROFILE_VALUES)[K][number] }};\n\n\
         {}",
        serde_json::to_string_pretty(&distros)?,
        serde_json::to_string_pretty(&values)?,
        serde_json::to_string_pretty(&fields)?,
        serde_json::to_string(profile::ADMIN_GROUP)?,
        DISTROS_HELPERS,
    );
    let path = root.join("docs/content/src/distros.ts");
    out.emit(&path, &format_ts(root, &path, &text)?)
}

/// Reads the profile back from an os-release, the way `Profile::from_os_release` does.
const DISTROS_HELPERS: &str = r#"
/** The value of `field` in the text of an /etc/os-release (quotes removed; the last setting wins). */
export function osReleaseValue(osRelease: string, field: string): string | null {
	let found: string | null = null;
	for (const line of osRelease.split("\n")) {
		const trimmed = line.trim();
		if (!trimmed.startsWith(`${field}=`)) continue;
		const value = trimmed.slice(field.length + 1).trim().replace(/^["']|["']$/g, "").trim();
		if (value) found = value;
	}
	return found;
}

/** One profile fact from an os-release; null when it is missing or not a known value. */
export function osReleaseFact<K extends keyof Profile>(osRelease: string, fact: K): Profile[K] | null {
	const value = osReleaseValue(osRelease, OS_RELEASE_FIELDS[fact]);
	const known = PROFILE_VALUES[fact] as readonly string[];
	return value !== null && known.includes(value) ? (value as Profile[K]) : null;
}

/** The variant whose os-release ID is `osId`, if there is one. */
export function distroByOsId(osId: string): Distro | undefined {
	return DISTROS.find((distro) => distro.osId === osId);
}
"#;

/// The packages whose CLIs the docs site shows (`--help` of every binary each
/// builds, as `cargo metadata` lists them).
const DOCUMENTED_PACKAGES: &[&str] = &["recab", "recinit", "reciso", "recpart", "recstrap"];

/// Each documented package's binaries, from `cargo metadata`.
fn documented_binaries(root: &Path) -> Result<Vec<String>> {
    let output = std::process::Command::new("cargo")
        .current_dir(root)
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .context("running cargo metadata")?;
    if !output.status.success() {
        bail!("cargo metadata failed: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let mut binaries = Vec::new();
    for package in DOCUMENTED_PACKAGES {
        let found = metadata["packages"]
            .as_array()
            .context("cargo metadata has no packages")?
            .iter()
            .find(|p| p["name"] == *package)
            .with_context(|| format!("cargo metadata lists no package {package}"))?;
        for target in found["targets"].as_array().into_iter().flatten() {
            if target["kind"].as_array().is_some_and(|kinds| kinds.iter().any(|k| k == "bin")) {
                binaries.push(target["name"].as_str().context("a target without a name")?.to_string());
            }
        }
    }
    Ok(binaries)
}

fn write_tool_help(out: &mut Out, root: &Path) -> Result<()> {
    let mut build = std::process::Command::new("cargo");
    build.current_dir(root).args(["build", "-q"]);
    for package in DOCUMENTED_PACKAGES {
        build.args(["-p", package]);
    }
    if !build.status()?.success() {
        anyhow::bail!("building the documented tools failed");
    }
    let mut text = String::from(
        "// Generated by `cargo xtask gen` from each tool's `--help`. Do not edit.\n\n\
         export const TOOL_HELP = {\n",
    );
    // `cargo build` without `--release` writes the dev profile's binaries.
    let bin_dir = crate::util::repo::cargo_target_dir(root).join("debug");
    for binary in documented_binaries(root)? {
        let output = std::process::Command::new(bin_dir.join(&binary))
            .arg("--help")
            .output()
            .with_context(|| format!("running {binary} --help"))?;
        if !output.status.success() {
            anyhow::bail!("{binary} --help failed");
        }
        let help = String::from_utf8(output.stdout)?;
        text.push_str(&format!("\t{binary}: {},\n", serde_json::to_string(help.trim_end())?));
    }
    text.push_str("} as const\n");
    let path = root.join("docs/content/src/tool-help.ts");
    out.emit(&path, &format_ts(root, &path, &text)?)
}

fn format_ts(root: &Path, path: &Path, text: &str) -> Result<String> {
    use std::io::Write;
    // The workspace's formatter; gen installs nothing (`cargo xtask check`
    // installs the JS workspace before it runs `gen --check`).
    let oxfmt = root.join("node_modules/.bin/oxfmt");
    if !oxfmt.exists() {
        bail!("{} is missing: run `bun install` at the repo root", oxfmt.display());
    }
    let mut child = std::process::Command::new(oxfmt)
        .current_dir(root)
        .arg(format!("--stdin-filepath={}", path.display()))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .context("running oxfmt")?;
    child.stdin.take().context("oxfmt stdin")?.write_all(text.as_bytes())?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        bail!("oxfmt failed on {}", path.display());
    }
    Ok(String::from_utf8(output.stdout)?)
}
