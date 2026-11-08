//! `build`: products through distro-builder, kernels through recipe.

use anyhow::Result;
use std::path::PathBuf;

use crate::cli::{BuildCmd, Distro, KernelArgs, ProductBuild};
use distro_contract::ProductId;

pub fn run(what: BuildCmd) -> Result<()> {
    match what {
        BuildCmd::Product(ProductBuild { product, distros }) => product_isos(product, &distros.distros()?),
        BuildCmd::Kernel(args) => kernel(args),
        BuildCmd::RootfsSource { distro, refresh } => {
            let root = crate::util::repo::repo_root()?;
            let mut args = vec!["artifact", "preseed-rootfs-source", distro.id()];
            if refresh {
                args.push("--refresh");
            }
            crate::util::distro_builder::run(&root, &args)
        }
        BuildCmd::Erofs { product, distro } => erofs(product, &distro),
    }
}

/// The ISO of the latest successful build of a product, if any.
pub fn latest_iso(distro: &Distro, product: ProductId) -> Result<Option<PathBuf>> {
    let root = crate::util::repo::repo_root()?;
    Ok(distro_contract::runs::latest_release_iso(&root, distro.id(), product)?)
}

/// The latest ISO of `product`, or a missing-prerequisite error naming the build.
pub fn require_iso(distro: &Distro, product: ProductId) -> Result<PathBuf> {
    latest_iso(distro, product)?.ok_or_else(|| {
        crate::util::exit::missing(
            format!("no {product} ISO has been built for {}", distro.id()),
            format!("cargo xtask build {product} {}", distro.id()),
        )
    })
}

fn product_isos(product: ProductId, distros: &[Distro]) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    for distro in distros {
        let kernel = crate::tasks::kernels::common::target_for(distro)?;
        if !crate::tasks::kernels::common::kernel_is_built(&root, &kernel) {
            return Err(crate::util::exit::missing(
                format!("the {} kernel is not built", distro.id()),
                format!(
                    "cargo xtask build kernel {0} --prebuilt (download) or cargo xtask build kernel {0}",
                    distro.id()
                ),
            ));
        }
    }
    for distro in distros {
        crate::util::distro_builder::run(
            &root,
            &["release", "build", "iso", product.name(), distro.id()],
        )?;
        if let Some(iso) = latest_iso(distro, product)? {
            println!("built: {}", iso.display());
        }
        if let Some(scenario) = product.next_scenario() {
            println!("next:  cargo xtask test {scenario} {}", distro.id());
        }
    }
    Ok(())
}

fn erofs(product: ProductId, distro: &Distro) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let prepared = distro_contract::runs::prepared_dir(&root, distro.id(), product);
    if prepared.exists() {
        std::fs::remove_dir_all(&prepared)?;
    }
    std::fs::create_dir_all(&prepared)?;
    let prepared_arg = prepared.to_string_lossy();
    crate::util::distro_builder::run(
        &root,
        &["product", "prepare", product.name(), distro.id(), &prepared_arg],
    )?;
    crate::util::distro_builder::run(
        &root,
        &["transform", "build", "product-erofs", &prepared_arg],
    )?;
    println!("built: {}", prepared.display());
    Ok(())
}

fn kernel(args: KernelArgs) -> Result<()> {
    let distros = args.distros.distros()?;
    let k = crate::tasks::kernels::common::KernelBuild {
        rebuild: args.rebuild,
        now: args.now,
        autofix: crate::tasks::kernels::common::AutoFixOptions {
            enabled: args.autofix,
            attempts: args.autofix_attempts,
            prompt_file: args.autofix_prompt_file,
            llm_profile: args.llm_profile,
        },
    };
    for distro in &distros {
        if args.prebuilt {
            crate::tasks::kernels::prebuilt::run(distro, args.rebuild)?;
        } else {
            crate::tasks::kernels::build::run(distro, &k)?;
        }
    }
    Ok(())
}
