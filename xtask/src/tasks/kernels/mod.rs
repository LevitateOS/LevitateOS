//! `build kernel`: the CLI arm and the kernel build window. distro-builder's
//! kernel module owns the kernel (its pin, the Recipe run, the bundle check).

use anyhow::{Context, Result, bail};
use distro_builder::kernel::{self, Autofix, KernelStatus, Source};

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
        Source::Compile {
            autofix: args.autofix.then(|| Autofix {
                attempts: args.autofix_attempts,
                prompt_file: args.autofix_prompt_file.clone(),
                llm_profile: args.llm_profile.clone(),
            }),
        }
    };
    for distro in args.distros.distros()? {
        let variant = distro_contract::load(&root, distro.id())?;
        eprintln!("[info] {} kernel: {}", distro.id(), kernel::pin(&variant)?.release);
        if !args.rebuild
            && let KernelStatus::Ready(release) = kernel::status(&variant)?
        {
            eprintln!("[skip] {} kernel {release} is installed (--rebuild to install it again)", distro.id());
            continue;
        }
        if !args.prebuilt {
            enforce_build_hours(args.now)?;
        }
        let release = kernel::install(&variant, &source, args.rebuild)
            .with_context(|| format!("installing the {} kernel", distro.id()))?;
        eprintln!("[ok] {}: {release}", distro.id());
    }
    Ok(())
}

/// Kernel compiles run 10:00-23:00 so the build's fan noise stays out of the
/// night; `--now` overrides.
pub(crate) fn enforce_build_hours(now: bool) -> Result<()> {
    if now {
        return Ok(());
    }
    let (hour, minute) = local_time()?;
    if (10..23).contains(&hour) {
        return Ok(());
    }
    Err(crate::util::exit::missing(
        format!("kernel compiles run between 10:00 and 23:00 (it is {hour:02}:{minute:02})"),
        "add --now to compile anyway, or use --prebuilt to download it",
    ))
}

/// The local hour and minute.
fn local_time() -> Result<(i32, i32)> {
    // SAFETY: `time(NULL)` reads the clock; `localtime_r` fills the `tm` it is
    // given and touches nothing else.
    let tm = unsafe {
        let now = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&now, &mut tm).is_null() {
            bail!("reading the local time");
        }
        tm
    };
    Ok((tm.tm_hour, tm.tm_min))
}
