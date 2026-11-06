use anyhow::{Context, Result, bail};
use distro_contract::load_variant_contract_bundle_for_distro_from;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
pub(crate) struct KernelTarget {
    pub distro_id: &'static str,
    pub kernel: &'static distro_spec::shared::KernelSource,
}

#[derive(Clone, Debug)]
pub(crate) struct AutoFixOptions {
    pub(crate) enabled: bool,
    pub(crate) attempts: u8,
    pub(crate) prompt_file: Option<PathBuf>,
    pub(crate) llm_profile: Option<String>,
}

pub(crate) fn target_for(d: crate::cli::Distro) -> KernelTarget {
    KernelTarget {
        distro_id: d.id(),
        kernel: match d {
            crate::cli::Distro::Levitate => &distro_spec::levitate::KERNEL_SOURCE,
            crate::cli::Distro::Acorn => &distro_spec::acorn::KERNEL_SOURCE,
        },
    }
}

pub(crate) fn enforce_build_hours() -> Result<()> {
    let hhmm = run_capture("date", &["+%H%M"])?;
    let hhmm = hhmm.trim();
    let now = run_capture("date", &["+%Y-%m-%d %H:%M:%S %z"])?;
    let now = now.trim();

    let hhmm_num: i32 = hhmm
        .parse()
        .with_context(|| format!("Parsing date +%H%M output: '{hhmm}'"))?;

    // Allowed: 23:00-23:59, 00:00-10:00 (inclusive)
    if hhmm_num >= 2300 || hhmm_num <= 1000 {
        return Ok(());
    }

    eprintln!(
        "[policy] Refusing to build kernels outside the allowed window.\n\
         Allowed local time window: 23:00 (11pm) through 10:00 (10am).\n\
         Current local time: {now}\n\
\n\
         If you really intend to build right now, rerun later during the window."
    );
    std::process::exit(3);
}

pub(crate) fn kernel_is_built(root: &Path, t: &KernelTarget) -> bool {
    verify_one(root, t).is_ok()
}

pub(crate) fn verify_one(root: &Path, t: &KernelTarget) -> Result<String> {
    let kernel_root = distro_builder::run_history::kernel_output_dir(root, t.distro_id);
    let rel_file = kernel_root.join("kernel-build/include/config/kernel.release");
    let vmlinuz = kernel_root.join("staging/boot/vmlinuz");

    if !rel_file.is_file() {
        bail!("Missing kernel.release: {}", rel_file.display());
    }
    if !vmlinuz.is_file() {
        bail!("Missing vmlinuz: {}", vmlinuz.display());
    }

    let rel =
        fs::read_to_string(&rel_file).with_context(|| format!("Reading {}", rel_file.display()))?;
    let rel = rel.trim_end_matches(['\n', '\r']).to_string();

    if !t.kernel.version.is_empty() && !rel.starts_with(t.kernel.version) {
        bail!(
            "{} kernel.release '{}' does not start with '{}' (expected kernel version from distro-spec)",
            t.distro_id,
            rel,
            t.kernel.version
        );
    }

    if !rel.ends_with(t.kernel.localversion) {
        bail!(
            "{} kernel.release '{}' does not end with '{}' (wrong kernel localversion; expected a distro-specific kernel build)",
            t.distro_id,
            rel,
            t.kernel.localversion
        );
    }

    let m1 = kernel_root.join(format!("staging/lib/modules/{rel}"));
    let m2 = kernel_root.join(format!("staging/usr/lib/modules/{rel}"));
    if !m1.is_dir() && !m2.is_dir() {
        bail!(
            "Missing modules dir for {} ({}) under staging/{{lib,usr/lib}}/modules/",
            t.distro_id,
            rel
        );
    }

    Ok(rel)
}

pub(crate) fn build_recipe_bin(root: &Path) -> Result<PathBuf> {
    run_cmd(
        Command::new("cargo")
            .current_dir(root)
            .args(["build", "-p", "levitate-recipe"]),
    )?;

    let recipe_bin = root.join("target/debug/recipe");
    if !recipe_bin.is_file() {
        bail!("Expected recipe binary at {}", recipe_bin.display());
    }
    Ok(recipe_bin)
}

pub(crate) fn build_kernel_via_recipe(
    recipe_bin: &Path,
    root: &Path,
    distro_id: &str,
    force_rebuild: bool,
    autofix: &AutoFixOptions,
) -> Result<()> {
    let bundle = load_variant_contract_bundle_for_distro_from(root, distro_id)
        .with_context(|| format!("loading canonical variant contract for '{}'", distro_id))?;
    let recipe_rhai = root.join("distro-builder/recipes/linux.rhai");
    let build_dir = distro_builder::recipe::recipe_work_dir(root, distro_id)?;
    let recipes_path = root.join("distro-builder/recipes");
    let kconfig_path = bundle
        .paths
        .build_host_declared_path(&bundle.contract.build.kernel.kconfig_path);
    let kernel_artifact_root = distro_builder::run_history::kernel_output_dir(root, distro_id);

    let mut cmd = Command::new(recipe_bin);
    cmd.current_dir(root);

    if let Some(p) = autofix.llm_profile.as_deref() {
        cmd.args(["--llm-profile", p]);
    }

    cmd.arg("install")
        .arg(recipe_rhai)
        .args(["--build-dir", build_dir.to_string_lossy().as_ref()])
        .args(["--recipes-path", recipes_path.to_string_lossy().as_ref()])
        .args(["--define", &format!("KERNEL_TARGET={distro_id}")])
        .args(["--input", &format!("kconfig={}", kconfig_path.display())]);
    let fragments = kconfig_path
        .parent()
        .context("kconfig has no parent")?
        .join("kconfig.d");
    if fragments.is_dir() {
        cmd.args([
            "--input",
            &format!("kconfig_fragments={}", fragments.display()),
        ]);
    }
    if kernel_artifact_root.is_dir() {
        cmd.args([
            "--input",
            &format!("target={}", kernel_artifact_root.display()),
        ]);
    }
    if force_rebuild {
        cmd.args(["--force-build", "--force-install"]);
    }

    if autofix.enabled {
        cmd.args(["--autofix"]);
        cmd.args(["--autofix-attempts", &autofix.attempts.max(1).to_string()]);
        cmd.args(["--autofix-cwd", root.to_string_lossy().as_ref()]);
        cmd.args(["--autofix-allow-path", "distro-builder/recipes"]);
        if let Some(p) = autofix.prompt_file.as_deref() {
            cmd.args(["--autofix-prompt-file", p.to_string_lossy().as_ref()]);
        }
    }

    run_kernel_and_select(&mut cmd, &build_dir, &kernel_artifact_root)
}

pub(crate) fn install_prebuilt_kernel_via_recipe(
    recipe_bin: &Path,
    root: &Path,
    distro_id: &str,
    force_redownload: bool,
) -> Result<()> {
    let recipe_rhai = root.join("distro-builder/recipes/linux-prebuilt.rhai");
    let build_dir = distro_builder::recipe::recipe_work_dir(root, distro_id)?;
    let recipes_path = root.join("distro-builder/recipes");
    let kernel_artifact_root = distro_builder::run_history::kernel_output_dir(root, distro_id);

    let mut cmd = Command::new(recipe_bin);
    cmd.current_dir(root)
        .arg("install")
        .arg(recipe_rhai)
        .args(["--build-dir", build_dir.to_string_lossy().as_ref()])
        .args(["--recipes-path", recipes_path.to_string_lossy().as_ref()])
        .args(["--define", &format!("KERNEL_TARGET={distro_id}")]);
    if kernel_artifact_root.is_dir() {
        cmd.args([
            "--input",
            &format!("target={}", kernel_artifact_root.display()),
        ]);
    }
    if force_redownload {
        cmd.args(["--force-acquire", "--force-install"]);
    }
    run_kernel_and_select(&mut cmd, &build_dir, &kernel_artifact_root)
}

fn run_kernel_and_select(command: &mut Command, work: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(work)?;
    let result = tempfile::Builder::new()
        .prefix(".kernel-result-")
        .suffix(".json")
        .tempfile_in(work)?;
    command.arg("--json-output").arg(result.path());
    run_cmd(command)?;
    let ctx: serde_json::Value = serde_json::from_slice(&fs::read(result.path())?)?;
    distro_builder::recipe::linux::select_kernel_bundle(&ctx, destination)
}

fn run_capture(prog: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(prog)
        .args(args)
        .output()
        .with_context(|| format!("Running {prog}"))?;
    if !out.status.success() {
        bail!("{prog} failed with status {}", out.status);
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn run_cmd(cmd: &mut Command) -> Result<()> {
    let status = cmd.status().with_context(|| "Spawning command")?;
    if !status.success() {
        bail!("Command failed with status {status}");
    }
    Ok(())
}
