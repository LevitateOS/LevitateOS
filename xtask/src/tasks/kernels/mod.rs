//! `build kernel`: the CLI arm. distro-builder's
//! kernel module owns the kernel (its pin, the Recipe run, the bundle check).

use anyhow::{Context, Result};
use distro_builder::kernel::{self, KernelStatus, Source};

mod publish;
pub use publish::publish;

use crate::cli::{Distro, KernelArgs};

/// The command that installs `distro`'s kernel.
pub fn fix(distro: &Distro) -> String {
    format!(
        "cargo xtask build kernel {0} --prebuilt (download) or cargo xtask build kernel {0}",
        distro.id()
    )
}

/// `distro`'s installed kernel release, or why it is not usable
/// ("not built: ..." or "mismatch: ...").
pub fn status(distro: &Distro) -> Result<Result<String, String>> {
    let root = crate::util::repo::repo_root()?;
    let variant = distro_contract::load(&root, distro.id())?;
    Ok(match kernel::status(&variant)? {
        KernelStatus::Ready(release) => Ok(release),
        KernelStatus::Missing(reason) => Err(format!("not built: {reason}")),
        KernelStatus::Mismatch(reason) => Err(format!("mismatch: {reason}")),
    })
}

pub fn build(args: KernelArgs) -> Result<()> {
    let root = crate::util::repo::repo_root()?;
    let source = if args.prebuilt {
        Source::Prebuilt
    } else {
        Source::Compile
    };
    for distro in args.distros.distros()? {
        let variant = distro_contract::load(&root, distro.id())?;
        eprintln!(
            "[info] {} kernel: {}",
            distro.id(),
            kernel::pin(&variant)?.release
        );
        if !args.rebuild
            && let KernelStatus::Ready(release) = kernel::status(&variant)?
        {
            eprintln!(
                "[skip] {} kernel {release} is installed (--rebuild to install it again)",
                distro.id()
            );
            continue;
        }
        let release = kernel::install(&variant, &source, args.rebuild)
            .with_context(|| format!("installing the {} kernel", distro.id()))?;
        eprintln!("[ok] {}: {release}", distro.id());
    }
    Ok(())
}
