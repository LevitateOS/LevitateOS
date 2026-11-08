//! `build`: products and kernels through distro-builder.

use anyhow::Result;
use std::path::PathBuf;

use crate::cli::{BuildCmd, Distro, ProductBuild};
use distro_contract::ProductId;

pub fn run(what: BuildCmd) -> Result<()> {
    match what {
        BuildCmd::Product(ProductBuild { product, distros }) => product_isos(product, &distros.distros()?),
        BuildCmd::Kernel(args) => crate::tasks::kernels::build(args),
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
        if let Err(reason) = crate::tasks::kernels::status(distro)? {
            return Err(crate::util::exit::missing(
                format!("the {} kernel is {reason}", distro.id()),
                crate::tasks::kernels::fix(distro),
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
