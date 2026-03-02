use anyhow::{Context, Result};

pub fn run(distro: crate::cli::Distro, refresh: bool) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let t = super::common::target_for(distro);

    eprintln!("[info] Repo: {}", root.display());
    eprintln!(
        "[info] Target: {} ({}{})",
        t.distro_id, t.kernel.version, t.kernel.localversion
    );

    if !refresh && super::common::kernel_is_built(&root, &t) {
        eprintln!("[skip] {} kernel already present+verified", t.distro_id);
        return Ok(());
    }

    let recipe_bin = super::common::build_recipe_bin(&root).context("Building recipe binary")?;

    eprintln!("[step] Install prebuilt kernel: {}", t.distro_id);
    super::common::install_prebuilt_kernel_via_recipe(
        &recipe_bin,
        &root,
        t.distro_id,
        refresh,
        t.kernel,
        t.module_install_path,
    )
    .with_context(|| format!("Prebuilt kernel install failed for {}", t.distro_id))?;

    let rel = super::common::verify_one(&root, &t).with_context(|| {
        format!(
            "Prebuilt kernel install finished for {} but artifacts failed verification",
            t.distro_id
        )
    })?;
    eprintln!("[ok] {}: {}", t.distro_id, rel);
    Ok(())
}
