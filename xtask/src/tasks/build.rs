//! `build`: products through distro-builder, kernels through recipe.

use anyhow::Result;
use std::path::PathBuf;

use crate::cli::{BuildCmd, Distro, KernelArgs, Product};
use install_tests::scenarios::{ScenarioId, resolve_iso_artifact_for_scenario};

pub fn run(what: BuildCmd) -> Result<()> {
    if let Some((product, args)) = what.product() {
        let distros = if args.all {
            Distro::ALL.to_vec()
        } else {
            vec![args.distro.unwrap_or(Distro::Levitate)]
        };
        return product_isos(product, &distros);
    }
    match what {
        BuildCmd::Kernel(args) => kernel(args),
        BuildCmd::RootfsSource { distro, refresh } => {
            let root = crate::util::repo::repo_root()?;
            let mut args = vec!["artifact", "preseed-rootfs-source", distro.id()];
            if refresh {
                args.push("--refresh");
            }
            crate::util::distro_builder::run(&root, &args)
        }
        BuildCmd::Erofs { product, distro } => erofs(product, distro),
        _ => unreachable!("product builds are handled above"),
    }
}

/// The scenario that boots a product's ISO.
fn scenario_for(product: Product) -> ScenarioId {
    match product {
        Product::BaseRootfs => ScenarioId::BuildPreflight,
        Product::LiveBoot => ScenarioId::LiveBoot,
        Product::LiveTools => ScenarioId::LiveTools,
    }
}

/// The ISO of the latest successful build of a product, if any.
pub fn latest_iso(distro: Distro, product: Product) -> Option<PathBuf> {
    resolve_iso_artifact_for_scenario(distro.id(), scenario_for(product))
        .ok()
        .flatten()
        .map(|iso| iso.path)
        .filter(|path| path.is_file())
}

/// The latest ISO of `product`, or a missing-prerequisite error naming the build.
pub fn require_iso(distro: Distro, product: Product) -> Result<PathBuf> {
    latest_iso(distro, product).ok_or_else(|| {
        crate::util::exit::missing(
            format!("no {} ISO has been built for {}", product.id(), distro.id()),
            format!("cargo xtask build {} {}", product.id(), distro.id()),
        )
    })
}

fn product_isos(product: Product, distros: &[Distro]) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    for &distro in distros {
        let kernel = crate::tasks::kernels::common::target_for(distro);
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
    for &distro in distros {
        crate::util::distro_builder::run(
            &root,
            &["release", "build", "iso", product.id(), distro.id()],
        )?;
        if let Some(iso) = latest_iso(distro, product) {
            println!("built: {}", iso.display());
        }
        println!(
            "next:  cargo xtask test {} {}",
            scenario_for(product).key(),
            distro.id()
        );
    }
    Ok(())
}

fn erofs(product: Product, distro: Distro) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let prepared = root
        .join(".artifacts/prepared")
        .join(distro.id())
        .join(product.id());
    if prepared.exists() {
        std::fs::remove_dir_all(&prepared)?;
    }
    std::fs::create_dir_all(&prepared)?;
    let prepared_arg = prepared.to_string_lossy();
    crate::util::distro_builder::run(
        &root,
        &["product", "prepare", product.id(), distro.id(), &prepared_arg],
    )?;
    crate::util::distro_builder::run(
        &root,
        &["transform", "build", "product-erofs", &prepared_arg],
    )?;
    println!("built: {}", prepared.display());
    Ok(())
}

fn kernel(args: KernelArgs) -> Result<()> {
    let distros = if args.all {
        Distro::ALL.to_vec()
    } else {
        vec![args.distro.expect("clap requires a distro without --all")]
    };
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
    for distro in distros {
        if args.prebuilt {
            crate::tasks::kernels::prebuilt::run(distro, args.rebuild)?;
        } else {
            crate::tasks::kernels::build::run(distro, &k)?;
        }
    }
    Ok(())
}
