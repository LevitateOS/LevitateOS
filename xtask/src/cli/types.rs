use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Distro {
    #[value(name = "leviso")]
    Leviso,
    #[value(name = "acorn")]
    AcornOS,
    #[value(name = "iuppiter")]
    IuppiterOS,
    #[value(name = "ralph")]
    RalphOS,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BootDistro {
    #[value(name = "levitate")]
    Levitate,

    #[value(name = "acorn")]
    Acorn,

    #[value(name = "iuppiter")]
    Iuppiter,

    #[value(name = "ralph")]
    Ralph,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum HarnessDistro {
    #[value(name = "levitate")]
    Levitate,

    #[value(name = "acorn")]
    Acorn,

    #[value(name = "iuppiter")]
    Iuppiter,

    #[value(name = "ralph")]
    Ralph,
}

impl HarnessDistro {
    pub fn id(self) -> &'static str {
        match self {
            Self::Levitate => "levitate",
            Self::Acorn => "acorn",
            Self::Iuppiter => "iuppiter",
            Self::Ralph => "ralph",
        }
    }
}

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "LevitateOS repo developer tasks (scaffolding; complements justfile)")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Print the environment exports that the justfile sets for QEMU/tooling.
    ///
    /// Usage:
    ///   eval "$(cargo xtask env bash)"
    Env {
        #[arg(value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },

    /// Check that the local toolchain/tools match what the justfile expects.
    Doctor,

    /// Kernel-related tasks.
    Kernels {
        #[command(subcommand)]
        cmd: KernelsCmd,
    },

    /// Install/remove shared git hooks (pre-commit) across the workspace + Rust submodules.
    Hooks {
        #[command(subcommand)]
        cmd: HooksCmd,
    },

    /// Install-test scenario runner.
    #[command(name = "scenarios")]
    Scenarios {
        #[command(subcommand)]
        cmd: ScenariosCmd,
    },

    /// Repository policy checks.
    Policy {
        #[command(subcommand)]
        cmd: PolicyCmd,
    },

    /// Docs tooling tasks.
    Docs {
        #[command(subcommand)]
        cmd: DocsCmd,
    },

    /// Generic TUI tooling tasks.
    Tui {
        #[command(subcommand)]
        cmd: TuiCmd,
    },
}

#[derive(Subcommand)]
pub enum KernelsCmd {
    /// Build the kernel for one distro (policy window enforced).
    #[command(name = "build")]
    Build {
        #[arg(value_enum)]
        distro: Distro,

        #[arg(
            long = "rebuild",
            help = "Force the selected distro to rebuild+reinstall its kernel even if artifacts are already present. Does not bypass the 23:00-10:00 build-hours policy."
        )]
        rebuild: bool,

        #[arg(
            long = "autofix",
            help = "On kernel build failure, rerun via `recipe install --autofix ...` so Recipe can ask the configured LLM provider to propose a patch and retry."
        )]
        autofix: bool,

        #[arg(
            long = "autofix-attempts",
            default_value_t = 2,
            help = "Maximum number of auto-fix attempts per failing kernel build."
        )]
        autofix_attempts: u8,

        #[arg(
            long = "autofix-prompt-file",
            help = "Optional extra instructions appended to the built-in Codex autofix prompt."
        )]
        autofix_prompt_file: Option<PathBuf>,

        #[arg(
            long = "llm-profile",
            help = "Pass through to `recipe --llm-profile <name>` (selects a profile from XDG `recipe/llm.toml`)."
        )]
        llm_profile: Option<String>,
    },

    /// Build kernels for all distros (policy window enforced).
    #[command(name = "build-all")]
    BuildAll {
        #[arg(
            long = "rebuild",
            help = "Force every distro to rebuild+reinstall its kernel even if artifacts are already present. Does not bypass the 23:00-10:00 build-hours policy."
        )]
        rebuild: bool,

        #[arg(
            long = "autofix",
            help = "On kernel build failure, rerun via `recipe install --autofix ...` so Recipe can ask the configured LLM provider to propose a patch and retry."
        )]
        autofix: bool,

        #[arg(
            long = "autofix-attempts",
            default_value_t = 2,
            help = "Maximum number of auto-fix attempts per failing kernel build."
        )]
        autofix_attempts: u8,

        #[arg(
            long = "autofix-prompt-file",
            help = "Optional extra instructions appended to the built-in Codex autofix prompt."
        )]
        autofix_prompt_file: Option<PathBuf>,

        #[arg(
            long = "llm-profile",
            help = "Pass through to `recipe --llm-profile <name>` (selects a profile from XDG `recipe/llm.toml`)."
        )]
        llm_profile: Option<String>,
    },

    /// Install prebuilt kernel artifacts for one distro (no source compile).
    #[command(name = "prebuilt")]
    Prebuilt {
        #[arg(value_enum)]
        distro: Distro,

        #[arg(
            long = "refresh",
            help = "Force re-download and reinstall of prebuilt artifacts even when local kernel artifacts already verify."
        )]
        refresh: bool,
    },

    /// Verify built kernel artifacts for one distro (or all distros if omitted).
    Check {
        #[arg(value_enum)]
        distro: Option<Distro>,
    },
}

#[derive(Subcommand)]
pub enum HooksCmd {
    /// Install the shared pre-commit hook into the workspace + Rust submodules.
    Install,

    /// Remove the shared pre-commit hook from the workspace + Rust submodules.
    Remove,
}

#[derive(Subcommand)]
pub enum ScenariosCmd {
    /// Boot into an interactive scenario.
    ///
    /// Interactive targets: `live-boot`, `live-tools`, `installed-boot`
    Boot {
        target: String,
        #[arg(value_enum, default_value_t = BootDistro::Levitate)]
        distro: BootDistro,
        #[arg(long, value_name = "KEY=VALUE[,KEY=VALUE...]")]
        inject: Option<String>,
        #[arg(long, value_name = "PATH")]
        inject_file: Option<PathBuf>,
        /// Boot the scenario and wait for SSH readiness on the host forwarded port.
        #[arg(long)]
        ssh: bool,
        /// SSH host-forward port when `--ssh` is enabled.
        #[arg(long, default_value_t = 2222)]
        ssh_port: u16,
        /// Timeout in seconds to wait for SSH readiness and probe when `--ssh` is enabled.
        #[arg(long, default_value_t = 90)]
        ssh_timeout: u64,
        /// Connect and verify SSH only, without opening an interactive shell.
        #[arg(long)]
        no_shell: bool,
        /// Open a graphical window via local VNC viewer instead of serial-only headless mode.
        #[arg(long)]
        window: bool,
        /// SSH private key used for interactive or probe login when `--ssh` is enabled.
        #[arg(long, value_name = "PATH")]
        ssh_private_key: Option<PathBuf>,
    },

    /// Run one automated scenario.
    Test {
        target: String,
        #[arg(value_enum, default_value_t = HarnessDistro::Levitate)]
        distro: HarnessDistro,
        #[arg(long, value_name = "KEY=VALUE[,KEY=VALUE...]")]
        inject: Option<String>,
        #[arg(long, value_name = "PATH")]
        inject_file: Option<PathBuf>,
        /// Re-run the requested scenario even if it is already cached as passed.
        #[arg(long)]
        force: bool,
    },

    /// Run all automated scenarios up to the given scenario.
    TestUpTo {
        target: String,
        #[arg(value_enum, default_value_t = HarnessDistro::Levitate)]
        distro: HarnessDistro,
        #[arg(long, value_name = "KEY=VALUE[,KEY=VALUE...]")]
        inject: Option<String>,
        #[arg(long, value_name = "PATH")]
        inject_file: Option<PathBuf>,
    },

    /// Show scenario test status.
    Status {
        #[arg(value_enum, default_value_t = HarnessDistro::Levitate)]
        distro: HarnessDistro,
    },

    /// Reset cached scenario state for a distro.
    Reset {
        #[arg(value_enum, default_value_t = HarnessDistro::Levitate)]
        distro: HarnessDistro,
    },
}

#[derive(Subcommand)]
pub enum PolicyCmd {
    /// Fail if forbidden legacy bindings appear in code/config for stage wiring.
    #[command(name = "audit-legacy-bindings")]
    AuditLegacyBindings,
}

#[derive(Subcommand)]
pub enum DocsCmd {
    /// Render install-docs TUI pages and write plain-text terminal snapshots.
    Inspect {
        /// Explicit slug(s) to inspect. If omitted, xtask inspects every slug from docs/content nav.
        #[arg(long = "slug", value_name = "SLUG")]
        slug: Vec<String>,

        /// Terminal columns used for rendering.
        #[arg(long, default_value_t = 140)]
        columns: u16,

        /// Terminal rows used for rendering.
        #[arg(long, default_value_t = 40)]
        rows: u16,

        /// Seconds to keep each page running before snapshot capture.
        #[arg(long, default_value_t = 2)]
        seconds: u64,

        /// Optional output base directory. xtask always creates a timestamped run folder under this path.
        #[arg(long, value_name = "PATH")]
        out_dir: Option<PathBuf>,

        /// Also print the plain snapshot(s) to stdout.
        #[arg(long)]
        stdout: bool,

        /// Also write ANSI-formatted snapshots (`.ansi`) for color/style review.
        #[arg(long)]
        ansi: bool,

        /// Keep raw `script` transcripts next to plain outputs for debugging.
        #[arg(long)]
        keep_transcript: bool,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum TuiInspectApp {
    #[value(name = "s02-install-docs")]
    S02InstallDocs,
    #[value(name = "s03-disk-plan")]
    S03DiskPlan,
}

#[derive(Subcommand)]
pub enum TuiCmd {
    /// Render a TUI app and write terminal snapshots.
    Inspect {
        /// Preset app target. If omitted, provide both --cwd and --command.
        #[arg(long, value_enum)]
        app: Option<TuiInspectApp>,

        /// Working directory for custom target mode.
        #[arg(long, value_name = "PATH")]
        cwd: Option<PathBuf>,

        /// Launch command for custom target mode.
        #[arg(long, value_name = "CMD")]
        command: Option<String>,

        /// Optional printable key sequence sent to stdin shortly after launch.
        /// Supports printf %b escapes, for example: 'n', '\\n', 'jj'.
        #[arg(long, value_name = "KEYS")]
        input: Option<String>,

        /// Delay before sending --input.
        #[arg(long, default_value_t = 1)]
        input_delay_seconds: u64,

        /// Terminal columns used for rendering.
        #[arg(long, default_value_t = 140)]
        columns: u16,

        /// Terminal rows used for rendering.
        #[arg(long, default_value_t = 40)]
        rows: u16,

        /// Seconds to keep the app running before snapshot capture.
        #[arg(long, default_value_t = 2)]
        seconds: u64,

        /// Optional output base directory. xtask always creates a timestamped run folder under this path.
        #[arg(long, value_name = "PATH")]
        out_dir: Option<PathBuf>,

        /// Also print the plain snapshot to stdout.
        #[arg(long)]
        stdout: bool,

        /// Also write ANSI-formatted snapshot (`.ansi`) for color/style review.
        #[arg(long)]
        ansi: bool,

        /// Keep raw `script` transcript next to plain output for debugging.
        #[arg(long)]
        keep_transcript: bool,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Shell {
    Bash,
    Sh,
}
