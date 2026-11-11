//! `build`: products and kernels through distro-builder.

use anyhow::Result;
use std::path::PathBuf;

use crate::cli::{BuildCmd, Distro, ProductBuild};
use distro_builder::cli::Command;
use distro_contract::profile::Flavour;
use distro_contract::ProductId;

pub fn run(what: BuildCmd) -> Result<()> {
    match what {
        BuildCmd::Product(ProductBuild { product, distros, exact }) => {
            product_isos(product, &distros.distros()?, exact)
        }
        BuildCmd::Kernel(args) => crate::tasks::kernels::build(args),
        BuildCmd::RootfsSource { distro, desktop, refresh } => crate::util::distro_builder::run(
            &crate::util::repo::repo_root()?,
            &Command::PreseedRootfsSource {
                distro: distro.id().to_string(),
                flavour: if desktop { Flavour::Desktop } else { Flavour::Base },
                refresh,
            },
        ),
        BuildCmd::Erofs { product, distro } => erofs(product, &distro),
    }
}

/// The ISO of the latest successful build of a product, if any.
pub fn latest_iso(distro: &Distro, product: ProductId) -> Result<Option<PathBuf>> {
    let root = crate::util::repo::repo_root()?;
    Ok(distro_contract::runs::latest_release_iso(&root, distro.id(), product)?)
}

fn product_isos(product: ProductId, distros: &[Distro], exact: bool) -> Result<()> {
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
            &Command::ReleaseBuild { product, distro: distro.id().to_string(), exact },
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
    crate::util::distro_builder::run(
        &root,
        &Command::ProductPrepare { product, distro: distro.id().to_string(), output: prepared.clone() },
    )?;
    crate::util::distro_builder::run(&root, &Command::ProductErofs { prepared: prepared.clone() })?;
    println!("built: {}", prepared.display());
    Ok(())
}
