use anyhow::{Context, Result};

pub fn run(distro: &crate::cli::Distro, k: &super::common::KernelBuild) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let t = super::common::target_for(distro)?;
    eprintln!(
        "[info] Kernel: {} ({}{})",
        &t.distro_id, t.version, t.localversion
    );

    if !k.rebuild && super::common::kernel_is_built(&root, &t) {
        eprintln!("[skip] {} kernel already built+verified (--rebuild to build again)", t.distro_id);
        return Ok(());
    }
    super::common::enforce_build_hours(k.now)?;

    let recipe_bin = super::common::build_recipe_bin(&root).context("Building recipe binary")?;
    eprintln!("[step] Build kernel: {}", t.distro_id);
    super::common::build_kernel_via_recipe(&recipe_bin, &root, &t.distro_id, k.rebuild, &k.autofix)
        .with_context(|| format!("Kernel build failed for {}", t.distro_id))?;
    let rel = super::common::verify_one(&root, &t).with_context(|| {
        format!(
            "Kernel build finished for {} but artifacts failed verification",
            t.distro_id
        )
    })?;
    eprintln!("[ok] {}: {}", &t.distro_id, rel);
    Ok(())
}
