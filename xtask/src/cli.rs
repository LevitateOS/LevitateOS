//! The command line: clap types over the variants (discovered under
//! `distro/variants`) and the registry of products and scenarios
//! (`distro_contract::{ProductId, ScenarioId}`); nothing here lists either.

use clap::builder::{PossibleValue, PossibleValuesParser, TypedValueParser};
use clap::error::ErrorKind;
use clap::{Arg, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand};
use distro_contract::{ProductId, ScenarioId};
use std::ffi::OsStr;
use std::path::PathBuf;

/// A variant: a directory under `distro/variants` with a `distro.toml`,
/// loaded (and so validated) when the argument is parsed.
#[derive(Clone, Debug)]
pub struct Distro {
    id: String,
    os_name: String,
}

impl Distro {
    pub fn load(id: &str) -> anyhow::Result<Self> {
        let root = crate::util::repo::repo_root()?;
        let variant = distro_contract::load(&root, id)?;
        Ok(Self {
            id: variant.id,
            os_name: variant.contract.identity.os_name,
        })
    }

    /// Every variant.
    pub fn all() -> anyhow::Result<Vec<Self>> {
        distro_ids().iter().map(|id| Self::load(id)).collect()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// The variant's display name (its contract's `identity.os_name`).
    pub fn name(&self) -> &str {
        &self.os_name
    }
}

/// The variant ids of this checkout (none outside one).
pub fn distro_ids() -> Vec<String> {
    crate::util::repo::repo_root()
        .map(|root| distro_contract::distro_ids(&root))
        .unwrap_or_default()
}

/// A name that lives as long as the process: clap's possible values need one.
fn leak(name: String) -> &'static str {
    Box::leak(name.into_boxed_str())
}

/// Parses a variant id into its loaded `Distro`; `--help` lists the ids.
#[derive(Clone)]
struct DistroParser;

impl TypedValueParser for DistroParser {
    type Value = Distro;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Distro, clap::Error> {
        let id = PossibleValuesParser::new(self.possible_values().into_iter().flatten())
            .parse_ref(cmd, arg, value)?;
        Distro::load(&id)
            .map_err(|err| clap::Error::raw(ErrorKind::ValueValidation, format!("{err:#}\n")))
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            distro_ids()
                .into_iter()
                .map(|id| PossibleValue::new(leak(id))),
        ))
    }
}

/// One of a fixed set of named values, each with its help line.
#[derive(Clone)]
struct Choices<T: Clone + Send + Sync + 'static>(fn() -> Vec<(&'static str, &'static str, T)>);

impl<T: Clone + Send + Sync + 'static> TypedValueParser for Choices<T> {
    type Value = T;

    fn parse_ref(&self, cmd: &Command, arg: Option<&Arg>, value: &OsStr) -> Result<T, clap::Error> {
        let name = PossibleValuesParser::new(self.possible_values().into_iter().flatten())
            .parse_ref(cmd, arg, value)?;
        (self.0)()
            .into_iter()
            .find(|(choice, _, _)| *choice == name)
            .map(|(_, _, value)| value)
            .ok_or_else(|| clap::Error::new(ErrorKind::InvalidValue).with_cmd(cmd))
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            (self.0)()
                .into_iter()
                .map(|(name, help, _)| PossibleValue::new(name).help(help)),
        ))
    }
}

/// What `cargo xtask test` runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestSelection {
    /// Every scenario `ScenarioId::in_test_all` takes, in order; each runs
    /// against the artifacts that exist.
    All,
    One(ScenarioId),
}

fn test_selections() -> Vec<(&'static str, &'static str, TestSelection)> {
    let left_out: Vec<&str> = ScenarioId::ALL
        .into_iter()
        .filter(|scenario| !scenario.in_test_all())
        .map(ScenarioId::key)
        .collect();
    let all = format!(
        "every scenario below but {}, in order, each on its own",
        left_out.join(" and ")
    );
    std::iter::once(("all", &*all.leak(), TestSelection::All))
        .chain(ScenarioId::ALL.into_iter().map(|scenario| {
            (
                scenario.key(),
                scenario.summary(),
                TestSelection::One(scenario),
            )
        }))
        .collect()
}

fn runnable_scenarios() -> Vec<(&'static str, &'static str, ScenarioId)> {
    ScenarioId::ALL
        .into_iter()
        .filter(|scenario| scenario.is_runnable())
        .map(|scenario| (scenario.key(), scenario.summary(), scenario))
        .collect()
}

fn release_products() -> Vec<(&'static str, &'static str, ProductId)> {
    ProductId::releases()
        .map(|product| (product.name(), product.title(), product))
        .collect()
}

fn after_help() -> String {
    format!(
        "Typical loop, for a distro (<distro>: {}):
  cargo xtask setup                       once: QEMU tools, firmware, SSH key
  cargo xtask build kernel <distro> --prebuilt
  cargo xtask build live-tools <distro>
  cargo xtask test all <distro>
  cargo xtask status <distro>

Exit codes: 0 ok, 1 failed, 2 bad usage, 3 missing prerequisite (the message names the command that provides it).
Nothing waits for input unless it needs a terminal; without one it stops and names the command that works without it.",
        distro_ids().join(", ")
    )
}

/// The workspace front door for the LevitateOS family's variants: set up,
/// build, boot, test and check.
#[derive(Parser)]
#[command(name = "xtask", bin_name = "cargo xtask", after_help = after_help())]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Publish existing signed x86_64 releases, refresh metadata or verify delivery (no OS builds).
    #[command(disable_help_flag = true)]
    Release {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Create the repo SSH key guests trust, then run doctor.
    Setup,

    /// Check tools, firmware, kernels and ISOs; print the command that fixes each gap.
    Doctor {
        /// Print the checks as JSON.
        #[arg(long)]
        json: bool,
        /// Only print the host packages that provide every host tool, for a dnf or apt host.
        #[arg(long, value_name = "dnf|apt")]
        packages: Option<String>,
    },

    /// Build a product ISO, a kernel, the rootfs source, or a product's EROFS images.
    Build {
        #[command(subcommand)]
        what: BuildCmd,
    },

    /// Kernel maintenance: publish a new product kernel.
    Kernel {
        #[command(subcommand)]
        what: KernelCmd,
    },

    /// Boot a scenario for interactive use (needs a terminal unless --no-shell).
    Run(RunArgs),

    /// The packages a variant takes from its distribution base.
    Base {
        #[command(subcommand)]
        what: BaseCmd,
    },

    /// Run automated scenarios; results are cached per input (--force reruns).
    Test(TestArgs),

    /// Show which scenarios pass, fail, are stale or have not run.
    Status {
        #[arg(value_parser = DistroParser)]
        distro: Distro,
        /// Print the statuses as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Remove a distro's scenario runs (their results and disks).
    Reset {
        #[arg(value_parser = DistroParser)]
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

    /// Run a terminal app from tui/ in this terminal, or list them.
    ///
    /// `cargo xtask tui <APP> [ARGS]...` runs the app in this terminal (it needs one). First
    /// it installs the root JavaScript workspace and builds the Rust tools the app's manifest
    /// lists, which go first on PATH: these are the real tools, so disk-plan's apply really
    /// partitions and recab-tui's actions really change boot entries. ARGS go to the app;
    /// `cargo xtask tui <APP> --help` prints its own flags and keys.
    ///
    /// To try an app's cases by hand without touching anything real, add `--stand-ins`: the
    /// app runs with its stand-in tools (its manifest's `inspect.path`), as headless runs do.
    /// `--story NAME` (or its number in `cargo xtask tui list <APP>`; implies `--stand-ins`)
    /// starts it with that story's arguments and environment, and you drive it. An app that
    /// is told which distro it runs on (as the install screen tells the hub and the docs)
    /// needs `--distro ID` or a story. These go before the app's own arguments; after a
    /// `--`, everything goes to the app.
    ///
    /// `cargo xtask tui list` lists the apps with what each is for, where it lives, the tools
    /// it runs, its pages and its stories; `cargo xtask tui list <APP>` lists that app's
    /// stories in detail: what each shows, how it starts, its keys and what they must show.
    ///
    /// Without a terminal, or without the real tools: `cargo xtask inspect <APP>` renders a
    /// screen headlessly with stand-in tools (`--story NAME` opens a story, `--input` types
    /// keys), and `cargo xtask check tui` runs every TUI check.
    ///
    /// Examples:
    ///   cargo xtask tui list
    ///   cargo xtask tui list <APP>
    ///   cargo xtask tui <APP> --help
    ///   cargo xtask tui <APP> --story 1
    ///   cargo xtask tui <APP> --distro <ID> --stand-ins
    #[command(
        verbatim_doc_comment,
        arg_required_else_help = true,
        disable_help_subcommand = true,
        override_usage = "cargo xtask tui <APP> [--stand-ins | --story NAME] [--distro ID] [ARGS]...\n       cargo xtask tui list [APP]"
    )]
    Tui {
        #[command(subcommand)]
        cmd: TuiCmd,
    },

    /// Render a terminal app headlessly (optionally typing keys) and write its screen.
    Inspect(InspectArgs),
}

#[derive(Subcommand)]
pub enum TuiCmd {
    /// List the apps under tui/apps, or one app's stories in detail.
    List {
        /// An app: list its stories (what each shows, how it starts, its steps).
        app: Option<String>,
    },
    /// Any other name runs that app, with the rest of the line as its arguments.
    #[command(external_subcommand)]
    Run(Vec<String>),
}

#[derive(Subcommand)]
pub enum BaseCmd {
    /// Resolve the newest signed build of each base package (and, for Alpine, its dependencies) and write the lock.
    Lock {
        #[arg(value_parser = DistroParser)]
        distro: Distro,
        /// Only report packages with newer builds (exit 1 if any).
        #[arg(long)]
        check: bool,
    },
}

#[derive(Subcommand)]
pub enum BuildCmd {
    /// A release product's ISO (one subcommand per release product).
    #[command(flatten)]
    Product(ProductBuild),
    /// The kernel: compiled from source, or downloaded with --prebuilt.
    Kernel(KernelArgs),
    /// Download (or refresh) the rootfs source that product builds start from.
    #[command(name = "rootfs-source")]
    RootfsSource {
        #[arg(value_parser = DistroParser)]
        distro: Distro,
        /// The desktop lock's packages instead, which live-desktop builds
        /// the desktop system from (beside the base lock's).
        #[arg(long)]
        desktop: bool,
        /// Download it again even if it is present.
        #[arg(long)]
        refresh: bool,
    },
    /// Prepare a product and build its rootfs and overlay EROFS images (no ISO).
    Erofs {
        #[arg(value_parser = Choices(release_products))]
        product: ProductId,
        #[arg(value_parser = DistroParser)]
        distro: Distro,
    },
}

/// `cargo xtask build <product> <distro>`: a subcommand per release product.
pub struct ProductBuild {
    pub product: ProductId,
    pub distros: DistroSelection,
}

/// "The live-tools ISO (input of the live-tools, install and ab-update scenarios)".
fn product_about(product: ProductId) -> String {
    let scenarios: Vec<&str> = product.scenarios().map(ScenarioId::key).collect();
    let list = match scenarios.split_last() {
        Some((last, [])) => format!("the {last} scenario"),
        Some((last, rest)) => format!("the {} and {last} scenarios", rest.join(", ")),
        None => return format!("The {product} ISO"),
    };
    format!("The {product} ISO (input of {list})")
}

impl FromArgMatches for ProductBuild {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        let (name, sub) = matches
            .subcommand()
            .ok_or_else(|| clap::Error::new(ErrorKind::MissingSubcommand))?;
        let product = ProductId::from_name(name)
            .filter(|product| product.is_release())
            .ok_or_else(|| clap::Error::new(ErrorKind::InvalidSubcommand))?;
        Ok(Self {
            product,
            distros: DistroSelection::from_arg_matches(sub)?,
        })
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl Subcommand for ProductBuild {
    fn augment_subcommands(cmd: Command) -> Command {
        ProductId::releases().fold(cmd, |cmd, product| {
            cmd.subcommand(
                DistroSelection::augment_args(Command::new(product.name()))
                    .about(product_about(product))
                    .long_about(None),
            )
        })
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        Self::augment_subcommands(cmd)
    }

    fn has_subcommand(name: &str) -> bool {
        ProductId::from_name(name).is_some_and(ProductId::is_release)
    }
}

/// A distro, or every distro with `--all`.
#[derive(Args)]
pub struct DistroSelection {
    #[arg(value_parser = DistroParser, required_unless_present = "all")]
    pub distro: Option<Distro>,
    /// Every distro.
    #[arg(long, conflicts_with = "distro")]
    pub all: bool,
}

impl DistroSelection {
    pub fn distros(&self) -> anyhow::Result<Vec<Distro>> {
        match (&self.distro, self.all) {
            (_, true) => Distro::all(),
            (Some(distro), false) => Ok(vec![distro.clone()]),
            (None, false) => Err(crate::util::exit::usage(
                "name a distro, or --all for every distro",
            )),
        }
    }
}

#[derive(Subcommand)]
pub enum KernelCmd {
    /// Fetch, configure, build, package and boot-test the pinned kernel; with --upload, publish it.
    ///
    /// Steps: fetch, config, build, package, boot, upload. It runs in the foreground, logs each
    /// step under .artifacts/kernel/<distro>/<release>/, prints a heartbeat every minute during
    /// the build, kills a build whose log stalls for 10 minutes (120 minute hard limit), and ends
    /// with a `[fail] <step>: <reason> (log: <path>)` line and a nonzero exit on failure. A rerun
    /// skips the steps that finished (state.json); --from redoes a step and the ones after it.
    /// Without --upload it stops after the boot test and prints what upload would do.
    Publish(PublishArgs),
}

#[derive(Args, Clone)]
pub struct PublishArgs {
    #[arg(value_parser = DistroParser)]
    pub distro: Distro,
    /// Also upload the assets to the GitHub release and repoint linux-prebuilt.rhai's digest.
    #[arg(long)]
    pub upload: bool,
    /// Redo this step and every step after it.
    #[arg(long, value_name = "STEP", value_parser = ["fetch", "config", "build", "package", "boot", "upload"])]
    pub from: Option<String>,
    /// Compile even outside the 10:00-23:00 kernel build window.
    #[arg(long)]
    pub now: bool,
    /// make -j (default: half the CPUs).
    #[arg(long, value_name = "N", value_parser = clap::value_parser!(usize))]
    pub jobs: Option<usize>,
    /// A static busybox for the boot test (default: the one Recipe's busybox-static installs).
    #[arg(long, value_name = "PATH")]
    pub busybox: Option<PathBuf>,
}

#[derive(Args)]
pub struct KernelArgs {
    #[command(flatten)]
    pub distros: DistroSelection,
    /// Download the distro's prebuilt kernel instead of compiling.
    #[arg(long, conflicts_with = "autofix")]
    pub prebuilt: bool,
    /// Build (or download) again even if verified artifacts are present.
    #[arg(long)]
    pub rebuild: bool,
    /// Compile even outside the 10:00-23:00 kernel build window.
    #[arg(long, conflicts_with = "prebuilt")]
    pub now: bool,
    /// On failure, let recipe ask the configured LLM for a patch and retry.
    #[arg(long)]
    pub autofix: bool,
    /// Autofix attempts per failing build.
    #[arg(long, default_value_t = 2, requires = "autofix", value_parser = clap::value_parser!(u8).range(1..))]
    pub autofix_attempts: u8,
    /// Extra instructions appended to the autofix prompt.
    #[arg(long, value_name = "PATH", requires = "autofix")]
    pub autofix_prompt_file: Option<PathBuf>,
    /// recipe LLM profile (from XDG `recipe/llm.toml`).
    #[arg(long, requires = "autofix")]
    pub llm_profile: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    #[command(flatten)]
    pub artifacts: ScenarioArtifacts,
    /// A scenario whose input it boots: its ISO, or the latest install's disk.
    #[arg(value_parser = Choices(runnable_scenarios))]
    pub scenario: ScenarioId,
    #[arg(value_parser = DistroParser)]
    pub distro: Distro,
    /// Wait for SSH on a forwarded port and open a shell over it (live scenarios).
    #[arg(long)]
    pub ssh: bool,
    /// Boot, check that it came up (and that SSH logs in, with --ssh), then stop. Needs no terminal.
    #[arg(long)]
    pub no_shell: bool,
    #[command(flatten)]
    pub boot: BootArgs,
    /// Host port forwarded to the guest's SSH (with --ssh). Default: a free one.
    #[arg(long, requires = "ssh")]
    pub ssh_port: Option<u16>,
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
    /// Private key to log in with (its .pub is injected into live boots). Default:
    /// the repo key in .artifacts/ssh, created on first use.
    #[arg(long, value_name = "PATH")]
    pub ssh_key: Option<PathBuf>,
}

#[derive(Args)]
pub struct ScenarioArtifacts {
    /// Select the installed system profile and its existing installer artifacts.
    #[arg(long, default_value = "base", value_parser = str::parse::<distro_contract::profile::Flavour>)]
    pub flavour: distro_contract::profile::Flavour,
    /// Exact existing successful release run directory (no build or newest-product substitution).
    #[arg(long, value_name = "DIR")]
    pub release_run: Option<PathBuf>,
    /// Exact existing successful install run directory, booted through a disposable overlay.
    #[arg(long, value_name = "DIR")]
    pub install_run: Option<PathBuf>,
}

#[derive(Args)]
pub struct TestArgs {
    #[command(flatten)]
    pub artifacts: ScenarioArtifacts,
    /// Public candidate metadata descriptor; no local server, signing key or duplicate payload media.
    #[arg(long, value_name = "JSON", conflicts_with = "update_fixture")]
    pub public_update: Option<PathBuf>,
    /// Existing RECAB_P13 ISO with signed feed, public source policy and native origin helper.
    /// Install uses its policy before sealing; ab-update uses it as its newer update input.
    #[arg(long, value_name = "ISO")]
    pub update_fixture: Option<PathBuf>,
    /// A scenario, or all of them.
    #[arg(value_parser = Choices(test_selections))]
    pub scenario: TestSelection,
    #[arg(value_parser = DistroParser)]
    pub distro: Distro,
    /// Rerun even if the cached result for the same inputs passed.
    #[arg(long)]
    pub force: bool,
    /// Print a JSON summary of the results on stdout (progress goes to stderr).
    #[arg(long)]
    pub json: bool,
    /// The layout install installs, and the install run the installed-system
    /// scenarios boot (ab-update needs ab).
    #[arg(long, value_enum, default_value_t = recpart::InstallMode::Ab)]
    pub mode: recpart::InstallMode,
    #[command(flatten)]
    pub boot: BootArgs,
}

#[derive(Subcommand)]
pub enum CheckCmd {
    /// The terminal apps: typecheck, lint, format, render (also at the install screen's
    /// docs-pane size), snapshots and each app's stories.
    Tui {
        /// Rewrite snapshots from what the apps render now.
        #[arg(long)]
        bless: bool,
    },
}

/// A roomy terminal, as a developer runs the apps (columns, rows): `inspect`'s default
/// and the size `check tui` renders at, so a failing check reproduces without flags.
pub const INSPECT_SIZE: (u16, u16) = (120, 36);
/// How long a screen must stay quiet to count as drawn (ms), and how long to wait
/// for that (s).
pub const INSPECT_WAIT: (u64, u64) = (500, 30);

#[derive(Args)]
pub struct InspectArgs {
    /// App name (its package.json `bin`).
    pub app: String,
    /// Open one of the app's stories (its manifest's `inspect.stories`), by name or number:
    /// its arguments, environment and keys come first.
    #[arg(long, value_name = "NAME|NUMBER")]
    pub story: Option<String>,
    /// The distro the app is told it runs on, for an app that takes one (without a story).
    #[arg(long, value_name = "ID", value_parser = DistroParser)]
    pub distro: Option<Distro>,
    /// Keys to type once it has drawn, one at a time (printf escapes: '\n', '\e[B', 'jj').
    #[arg(long, value_name = "KEYS")]
    pub input: Option<String>,
    /// Render every page the app lists (its manifest's `inspect.pages`).
    #[arg(long)]
    pub all_pages: bool,
    /// Terminal columns.
    #[arg(long, default_value_t = INSPECT_SIZE.0, value_parser = clap::value_parser!(u16).range(1..))]
    pub columns: u16,
    /// Terminal rows.
    #[arg(long, default_value_t = INSPECT_SIZE.1, value_parser = clap::value_parser!(u16).range(1..))]
    pub rows: u16,
    /// Milliseconds without output that count as "drawn".
    #[arg(long, default_value_t = INSPECT_WAIT.0, value_parser = clap::value_parser!(u64).range(1..))]
    pub settle_ms: u64,
    /// Give up if the screen never settles within this many seconds.
    #[arg(long, default_value_t = INSPECT_WAIT.1, value_parser = clap::value_parser!(u64).range(1..))]
    pub timeout_seconds: u64,
    /// Base directory (default `.artifacts/tui-inspect`); a new run folder is created
    /// under it, named by the app and the time.
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
