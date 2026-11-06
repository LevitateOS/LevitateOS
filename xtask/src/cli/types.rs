use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// A LevitateOS variant (`distro/variants/<id>`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Distro {
    #[value(name = "levitate")]
    Levitate,
    #[value(name = "acorn")]
    Acorn,
}

impl Distro {
    pub const ALL: [Distro; 2] = [Distro::Levitate, Distro::Acorn];

    pub fn id(self) -> &'static str {
        match self {
            Self::Levitate => "levitate",
            Self::Acorn => "acorn",
        }
    }

    /// The variant's display name (its contract's `identity.os_name`).
    pub fn pretty_name(self) -> String {
        crate::util::repo::repo_root()
            .ok()
            .and_then(|root| distro_contract::load_variant_contract_for_distro_from(&root, self.id()).ok())
            .map(|contract| contract.identity.os_name)
            .unwrap_or_else(|| self.id().to_string())
    }
}

const AFTER_HELP: &str = "\
Typical loop:
  cargo xtask setup                       once: QEMU tools, firmware, SSH key
  cargo xtask build kernel levitate --prebuilt
  cargo xtask build live-tools levitate
  cargo xtask test all levitate
  cargo xtask status levitate

Exit codes: 0 ok, 1 failed, 2 bad usage, 3 missing prerequisite (the message names the command that provides it).
Nothing waits for input unless it needs a terminal; without one it stops and names the command that works without it.";

/// The LevitateOS workspace front door: set up, build, boot, test and check.
#[derive(Parser)]
#[command(name = "xtask", bin_name = "cargo xtask", after_help = AFTER_HELP)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Install the QEMU/OVMF tools into .artifacts/tools, create the repo SSH key, then run doctor.
    Setup,

    /// Check tools, firmware, kernels and ISOs; print the command that fixes each gap.
    Doctor {
        /// Print the checks as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Print shell exports for the repo's QEMU tools: eval "$(cargo xtask env)".
    Env,

    /// Build a product ISO, a kernel, the rootfs source, or a product's EROFS images.
    Build {
        #[command(subcommand)]
        what: BuildCmd,
    },

    /// Boot a scenario for interactive use (needs a terminal unless --no-shell).
    Run(RunArgs),

    /// The packages a variant takes from its Fedora base.
    Base {
        #[command(subcommand)]
        what: BaseCmd,
    },

    /// Run automated scenarios; results are cached per input (--force reruns).
    Test(TestArgs),

    /// Show which scenarios pass, fail, are stale or have not run.
    Status {
        #[arg(value_enum, default_value_t = Distro::Levitate)]
        distro: Distro,
        /// Print the statuses as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Forget saved scenario results for a distro.
    Reset {
        #[arg(value_enum, default_value_t = Distro::Levitate)]
        distro: Distro,
    },

    /// Regenerate files derived from the Rust types and CLIs (schemas, TS declarations, tool help).
    Gen {
        /// Only report files that are out of date (exit 1 if any).
        #[arg(long)]
        check: bool,
    },

    /// Quick checks before committing: cargo check, generated files current, TypeScript typecheck.
    Check {
        #[command(subcommand)]
        what: Option<CheckCmd>,
    },

    /// Launch a terminal app from tui/ in this terminal.
    Tui {
        /// App name: the `bin` name in its package.json (e.g. install-docs, disk-plan, recab-tui).
        app: String,
        /// Arguments passed to the app.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Render a terminal app headlessly (optionally typing keys) and write its screen.
    Inspect(InspectArgs),
}

#[derive(Subcommand)]
pub enum BaseCmd {
    /// Resolve the newest signed build of each base package (release + updates) and write the lock.
    Lock {
        #[arg(value_enum, default_value_t = Distro::Levitate)]
        distro: Distro,
        /// Only report packages with newer builds (exit 1 if any).
        #[arg(long)]
        check: bool,
    },
}

/// A product ISO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Product {
    #[value(name = "base-rootfs")]
    BaseRootfs,
    #[value(name = "live-boot")]
    LiveBoot,
    #[value(name = "live-tools")]
    LiveTools,
}

impl Product {
    pub fn id(self) -> &'static str {
        match self {
            Self::BaseRootfs => "base-rootfs",
            Self::LiveBoot => "live-boot",
            Self::LiveTools => "live-tools",
        }
    }
}

#[derive(Subcommand)]
pub enum BuildCmd {
    /// The base-rootfs ISO.
    #[command(name = "base-rootfs")]
    BaseRootfs(ProductArgs),
    /// The live-boot ISO.
    #[command(name = "live-boot")]
    LiveBoot(ProductArgs),
    /// The live-tools ISO (the one the install scenarios use).
    #[command(name = "live-tools")]
    LiveTools(ProductArgs),
    /// The kernel: compiled from source, or downloaded with --prebuilt.
    Kernel(KernelArgs),
    /// Download (or refresh) the rootfs source that product builds start from.
    #[command(name = "rootfs-source")]
    RootfsSource {
        #[arg(value_enum, default_value_t = Distro::Levitate)]
        distro: Distro,
        /// Download it again even if it is present.
        #[arg(long)]
        refresh: bool,
    },
    /// Prepare a product and build its rootfs and overlay EROFS images (no ISO).
    Erofs {
        #[arg(value_enum)]
        product: Product,
        #[arg(value_enum, default_value_t = Distro::Levitate)]
        distro: Distro,
    },
}

impl BuildCmd {
    pub fn product(&self) -> Option<(Product, &ProductArgs)> {
        match self {
            Self::BaseRootfs(args) => Some((Product::BaseRootfs, args)),
            Self::LiveBoot(args) => Some((Product::LiveBoot, args)),
            Self::LiveTools(args) => Some((Product::LiveTools, args)),
            _ => None,
        }
    }
}

#[derive(Args)]
pub struct ProductArgs {
    /// Defaults to levitate.
    #[arg(value_enum)]
    pub distro: Option<Distro>,
    /// Build it for every distro.
    #[arg(long, conflicts_with = "distro")]
    pub all: bool,
}

#[derive(Args)]
pub struct KernelArgs {
    #[arg(value_enum, required_unless_present = "all")]
    pub distro: Option<Distro>,
    /// Every distro.
    #[arg(long, conflicts_with = "distro")]
    pub all: bool,
    /// Download the distro's prebuilt kernel instead of compiling.
    #[arg(long, conflicts_with = "autofix")]
    pub prebuilt: bool,
    /// Build (or download) again even if verified artifacts are present.
    #[arg(long)]
    pub rebuild: bool,
    /// Compile even outside the 23:00-10:00 kernel build window.
    #[arg(long)]
    pub now: bool,
    /// On failure, let recipe ask the configured LLM for a patch and retry.
    #[arg(long)]
    pub autofix: bool,
    /// Autofix attempts per failing build.
    #[arg(long, default_value_t = 2, requires = "autofix")]
    pub autofix_attempts: u8,
    /// Extra instructions appended to the autofix prompt.
    #[arg(long, value_name = "PATH", requires = "autofix")]
    pub autofix_prompt_file: Option<PathBuf>,
    /// recipe LLM profile (from XDG `recipe/llm.toml`).
    #[arg(long, requires = "autofix")]
    pub llm_profile: Option<String>,
}

/// A scenario that boots for interactive use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum RunScenario {
    #[value(name = "live-boot")]
    LiveBoot,
    #[value(name = "live-tools")]
    LiveTools,
    /// The disk the latest install scenario produced.
    #[value(name = "installed-boot")]
    InstalledBoot,
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(value_enum)]
    pub scenario: RunScenario,
    #[arg(value_enum, default_value_t = Distro::Levitate)]
    pub distro: Distro,
    /// Wait for SSH on a forwarded port and open a shell over it (live scenarios).
    #[arg(long)]
    pub ssh: bool,
    /// Boot, check that it came up (and that SSH logs in, with --ssh), then stop. Needs no terminal.
    #[arg(long)]
    pub no_shell: bool,
    #[command(flatten)]
    pub boot: BootArgs,
    /// Host port forwarded to the guest's SSH (with --ssh).
    #[arg(long, default_value_t = 2222)]
    pub ssh_port: u16,
    /// Seconds to wait for SSH (with --ssh).
    #[arg(long, default_value_t = 90)]
    pub ssh_timeout: u64,
}

/// Boot injection and the SSH key the guest trusts.
#[derive(Args)]
pub struct BootArgs {
    /// Extra boot injection, KEY=VALUE[,KEY=VALUE...].
    #[arg(long, value_name = "KEY=VALUE[,...]")]
    pub inject: Option<String>,
    /// Extra boot injection from a KEY=VALUE file.
    #[arg(long, value_name = "PATH")]
    pub inject_file: Option<PathBuf>,
    /// Private key to log in with (its .pub is injected). Default: the repo key in
    /// .artifacts/ssh, created on first use.
    #[arg(long, value_name = "PATH")]
    pub ssh_key: Option<PathBuf>,
}

/// An automated scenario, or all of the ones that boot (in run order).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum TestScenario {
    /// live-boot, live-tools, install, installed-boot, automated-login, runtime.
    All,
    #[value(name = "build-preflight")]
    BuildPreflight,
    #[value(name = "live-boot")]
    LiveBoot,
    #[value(name = "live-tools")]
    LiveTools,
    Install,
    #[value(name = "installed-boot")]
    InstalledBoot,
    #[value(name = "automated-login")]
    AutomatedLogin,
    Runtime,
    /// On a copy of the install disk: stage the release into slot B, then trial
    /// boot, fallback, commit and rollback (A/B installs only).
    #[value(name = "ab-update")]
    AbUpdate,
}

#[derive(Args)]
pub struct TestArgs {
    #[arg(value_enum)]
    pub scenario: TestScenario,
    #[arg(value_enum, default_value_t = Distro::Levitate)]
    pub distro: Distro,
    /// Rerun even if the cached result for the same inputs passed.
    #[arg(long)]
    pub force: bool,
    /// Print a JSON summary of the results at the end.
    #[arg(long)]
    pub json: bool,
    #[command(flatten)]
    pub boot: BootArgs,
}

#[derive(Subcommand)]
pub enum CheckCmd {
    /// The terminal apps: typecheck, lint, format, render (also at the install screen's
    /// docs-pane size), snapshots and each app's key walk.
    Tui {
        /// Rewrite snapshots from what the apps render now.
        #[arg(long)]
        bless: bool,
    },
}

#[derive(Args)]
pub struct InspectArgs {
    /// App name (its package.json `bin`).
    pub app: String,
    /// Keys to type once it has drawn, one at a time (printf escapes: '\n', '\e[B', 'jj').
    #[arg(long, value_name = "KEYS")]
    pub input: Option<String>,
    /// Render every page the app lists (its manifest's `inspect.pages`).
    #[arg(long)]
    pub all_pages: bool,
    /// Terminal columns.
    #[arg(long, default_value_t = crate::tasks::tui::inspect::DEFAULT_SIZE.columns)]
    pub columns: u16,
    /// Terminal rows.
    #[arg(long, default_value_t = crate::tasks::tui::inspect::DEFAULT_SIZE.rows)]
    pub rows: u16,
    /// Milliseconds without output that count as "drawn".
    #[arg(long, default_value_t = crate::tasks::tui::inspect::DEFAULT_WAIT.settle.as_millis() as u64)]
    pub settle_ms: u64,
    /// Give up if the screen never settles within this many seconds.
    #[arg(long, default_value_t = crate::tasks::tui::inspect::DEFAULT_WAIT.timeout.as_secs())]
    pub timeout_seconds: u64,
    /// Base directory; a timestamped run folder is created under it.
    #[arg(long, value_name = "PATH")]
    pub out_dir: Option<PathBuf>,
    /// Also print the plain screen(s).
    #[arg(long)]
    pub stdout: bool,
    /// Also write ANSI screens (`.ansi`).
    #[arg(long)]
    pub ansi: bool,
    /// Keep raw terminal transcripts.
    #[arg(long)]
    pub keep_transcript: bool,
    /// Arguments passed to the app.
    #[arg(last = true)]
    pub args: Vec<String>,
}
