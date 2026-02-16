use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
pub(crate) struct KernelTarget {
    pub distro_dir: &'static str,
    pub kernel: &'static distro_spec::shared::KernelSource,
    pub module_install_path: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct AutoFixOptions {
    pub(crate) enabled: bool,
    pub(crate) attempts: u8,
    pub(crate) prompt_file: Option<PathBuf>,
    pub(crate) llm_profile: Option<String>,
}

pub(crate) fn target_for(d: crate::cli::Distro) -> KernelTarget {
    match d {
        crate::cli::Distro::Leviso => KernelTarget {
            distro_dir: "leviso",
            kernel: &distro_spec::levitate::KERNEL_SOURCE,
            module_install_path: distro_spec::levitate::MODULE_INSTALL_PATH,
        },
        crate::cli::Distro::AcornOS => KernelTarget {
            distro_dir: "AcornOS",
            kernel: &distro_spec::acorn::KERNEL_SOURCE,
            module_install_path: distro_spec::acorn::MODULE_INSTALL_PATH,
        },
        crate::cli::Distro::IuppiterOS => KernelTarget {
            distro_dir: "IuppiterOS",
            kernel: &distro_spec::iuppiter::KERNEL_SOURCE,
            module_install_path: distro_spec::iuppiter::MODULE_INSTALL_PATH,
        },
        crate::cli::Distro::RalphOS => KernelTarget {
            distro_dir: "RalphOS",
            kernel: &distro_spec::ralph::KERNEL_SOURCE,
            module_install_path: distro_spec::ralph::MODULE_INSTALL_PATH,
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
    let out_dir = root.join(".artifacts/out").join(t.distro_dir);
    let rel_file = out_dir.join("kernel-build/include/config/kernel.release");
    let vmlinuz = out_dir.join("staging/boot/vmlinuz");

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
            t.distro_dir,
            rel,
            t.kernel.version
        );
    }

    if !rel.ends_with(t.kernel.localversion) {
        bail!(
            "{} kernel.release '{}' does not end with '{}' (wrong kernel localversion; expected a distro-specific kernel build)",
            t.distro_dir,
            rel,
            t.kernel.localversion
        );
    }

    let m1 = out_dir.join(format!("staging/lib/modules/{rel}"));
    let m2 = out_dir.join(format!("staging/usr/lib/modules/{rel}"));
    if !m1.is_dir() && !m2.is_dir() {
        bail!(
            "Missing modules dir for {} ({}) under staging/{{lib,usr/lib}}/modules/",
            t.distro_dir,
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
    distro_dir: &str,
    force_rebuild: bool,
    kernel: &distro_spec::shared::KernelSource,
    module_install_path: &str,
    autofix: &AutoFixOptions,
) -> Result<()> {
    let recipe_rhai = root.join("distro-builder/recipes/linux.rhai");
    let build_dir = root.join(distro_dir).join("downloads");
    let recipes_path = root.join("distro-builder/recipes");

    let mut cmd = Command::new(recipe_bin);
    cmd.current_dir(root);

    if let Some(p) = autofix.llm_profile.as_deref() {
        cmd.args(["--llm-profile", p]);
    }

    cmd.arg("install")
        .arg(recipe_rhai)
        .args(["--build-dir", build_dir.to_string_lossy().as_ref()])
        .args(["--recipes-path", recipes_path.to_string_lossy().as_ref()])
        .args(["--define", &format!("KERNEL_VERSION={}", kernel.version)])
        .args(["--define", &format!("KERNEL_SHA256={}", kernel.sha256)])
        .args([
            "--define",
            &format!("KERNEL_LOCALVERSION={}", kernel.localversion),
        ])
        .args([
            "--define",
            &format!("KERNEL_FORCE_REBUILD={}", if force_rebuild { 1 } else { 0 }),
        ])
        .args([
            "--define",
            &format!("MODULE_INSTALL_PATH={module_install_path}"),
        ]);

    if autofix.enabled {
        cmd.args(["--autofix"]);
        cmd.args(["--autofix-attempts", &autofix.attempts.max(1).to_string()]);
        cmd.args(["--autofix-cwd", root.to_string_lossy().as_ref()]);
        cmd.args(["--autofix-allow-path", "distro-builder/recipes"]);
        if let Some(p) = autofix.prompt_file.as_deref() {
            cmd.args(["--autofix-prompt-file", p.to_string_lossy().as_ref()]);
        }
    }

    run_cmd(&mut cmd)
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
