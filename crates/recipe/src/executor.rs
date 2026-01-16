//! Recipe executor - runs parsed recipes to acquire, build, and install packages.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use thiserror::Error;

use crate::{
    AcquireSpec, BuildSpec, BuildStep, CleanupSpec, CleanupTarget, ConfigureSpec, ConfigureStep,
    GitRef, InstallFile, InstallSpec, Recipe, RemoveSpec, RemoveStep, StartSpec, StopSpec, Verify,
};

/// Errors that can occur during recipe execution.
#[derive(Error, Debug)]
pub enum ExecuteError {
    #[error("command failed: {cmd}\nstderr: {stderr}")]
    CommandFailed { cmd: String, stderr: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("no binary URL for architecture: {0}")]
    NoUrlForArch(String),

    #[error("sha256 verification failed: expected {expected}, got {actual}")]
    Sha256Mismatch { expected: String, actual: String },

    #[error("unsupported archive format: {0}")]
    UnsupportedFormat(String),

    #[error("missing required field: {0}")]
    MissingField(String),
}

/// Execution context providing configuration for recipe execution.
#[derive(Debug, Clone)]
pub struct Context {
    /// Installation prefix (default: /usr/local)
    pub prefix: PathBuf,
    /// Temporary build directory
    pub build_dir: PathBuf,
    /// Target architecture (e.g., "x86_64", "aarch64")
    pub arch: String,
    /// Number of parallel jobs for builds
    pub nproc: usize,
    /// If true, log commands without executing them
    pub dry_run: bool,
    /// If true, print commands as they execute
    pub verbose: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            prefix: PathBuf::from("/usr/local"),
            build_dir: std::env::temp_dir().join("levitate-build"),
            arch: std::env::consts::ARCH.to_string(),
            nproc: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            dry_run: false,
            verbose: false,
        }
    }
}

impl Context {
    /// Create a new context with the given prefix.
    pub fn with_prefix(prefix: impl Into<PathBuf>) -> Self {
        Self {
            prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Set the build directory.
    pub fn build_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.build_dir = dir.into();
        self
    }

    /// Set the target architecture.
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = arch.into();
        self
    }

    /// Set dry run mode.
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Set verbose mode.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

/// Recipe executor that runs acquire, build, install, and other actions.
pub struct Executor {
    ctx: Context,
}

impl Executor {
    /// Create a new executor with the given context.
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    /// Execute a complete recipe.
    pub fn execute(&self, recipe: &Recipe) -> Result<(), ExecuteError> {
        // Ensure build directory exists
        if !self.ctx.dry_run {
            std::fs::create_dir_all(&self.ctx.build_dir)?;
        }

        // Execute each phase in order
        if let Some(ref acquire) = recipe.acquire {
            self.acquire(acquire)?;
        }

        if let Some(ref build) = recipe.build {
            self.build(build)?;
        }

        if let Some(ref install) = recipe.install {
            self.install(install)?;
        }

        if let Some(ref configure) = recipe.configure {
            self.configure(configure)?;
        }

        // Cleanup build artifacts if specified
        if let Some(ref cleanup) = recipe.cleanup {
            self.cleanup(cleanup)?;
        }

        Ok(())
    }

    /// Execute the acquire phase.
    pub fn acquire(&self, spec: &AcquireSpec) -> Result<(), ExecuteError> {
        match spec {
            AcquireSpec::Source { url, verify } => {
                self.acquire_source(url, verify.as_ref())?;
            }
            AcquireSpec::Binary { urls } => {
                self.acquire_binary(urls)?;
            }
            AcquireSpec::Git { url, reference } => {
                self.acquire_git(url, reference.as_ref())?;
            }
            AcquireSpec::OsPackage { packages: _ } => {
                // Skip OS package installation - out of scope
            }
        }
        Ok(())
    }

    fn acquire_source(&self, url: &str, verify: Option<&Verify>) -> Result<(), ExecuteError> {
        let filename = url_filename(url);
        let dest = self.ctx.build_dir.join(&filename);

        // Download the file
        let cmd = format!(
            "curl -fsSL -o {} {}",
            shell_quote(dest.display()),
            shell_quote(url)
        );
        self.run_cmd(&cmd)?;

        // Verify checksum if provided
        if let Some(verify) = verify {
            self.verify_checksum(&dest, verify)?;
        }

        Ok(())
    }

    fn acquire_binary(&self, urls: &[(String, String)]) -> Result<(), ExecuteError> {
        // Find URL for current architecture
        let url = urls
            .iter()
            .find(|(arch, _)| arch == &self.ctx.arch)
            .map(|(_, url)| url)
            .ok_or_else(|| ExecuteError::NoUrlForArch(self.ctx.arch.clone()))?;

        let filename = url_filename(url);
        let dest = self.ctx.build_dir.join(&filename);

        let cmd = format!(
            "curl -fsSL -o {} {}",
            shell_quote(dest.display()),
            shell_quote(url)
        );
        self.run_cmd(&cmd)?;

        Ok(())
    }

    fn acquire_git(&self, url: &str, reference: Option<&GitRef>) -> Result<(), ExecuteError> {
        let repo_name = url
            .rsplit('/')
            .next()
            .unwrap_or("repo")
            .trim_end_matches(".git");
        let dest = self.ctx.build_dir.join(repo_name);

        // Clone with depth=1 for faster checkout
        let mut cmd = format!(
            "git clone --depth 1 {} {}",
            shell_quote(url),
            shell_quote(dest.display())
        );

        // Add branch/tag if specified
        if let Some(git_ref) = reference {
            let ref_arg = match git_ref {
                GitRef::Tag(t) | GitRef::Branch(t) => format!(" --branch {}", shell_quote(t)),
                GitRef::Commit(_) => String::new(), // Will checkout after clone
            };
            cmd = format!(
                "git clone --depth 1{} {} {}",
                ref_arg,
                shell_quote(url),
                shell_quote(dest.display())
            );
        }

        self.run_cmd(&cmd)?;

        // Checkout specific commit if needed
        if let Some(GitRef::Commit(commit)) = reference {
            let checkout_cmd = format!(
                "cd {} && git fetch --depth 1 origin {} && git checkout {}",
                shell_quote(dest.display()),
                shell_quote(commit),
                shell_quote(commit)
            );
            self.run_cmd(&checkout_cmd)?;
        }

        Ok(())
    }

    fn verify_checksum(&self, path: &Path, verify: &Verify) -> Result<(), ExecuteError> {
        match verify {
            Verify::Sha256(expected) => {
                let cmd = format!("sha256sum {}", shell_quote(path.display()));
                let output = self.run_cmd(&cmd)?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                let actual = stdout.split_whitespace().next().unwrap_or("");

                if actual != expected {
                    return Err(ExecuteError::Sha256Mismatch {
                        expected: expected.clone(),
                        actual: actual.to_string(),
                    });
                }
            }
            Verify::Sha256Url(checksum_url) => {
                // Download checksum file and verify
                let checksum_file = self.ctx.build_dir.join("checksum.sha256");
                let cmd = format!(
                    "curl -fsSL -o {} {}",
                    shell_quote(checksum_file.display()),
                    shell_quote(checksum_url)
                );
                self.run_cmd(&cmd)?;

                // Verify using the checksum file
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                let cmd = format!(
                    "cd {} && grep {} checksum.sha256 | sha256sum -c -",
                    shell_quote(self.ctx.build_dir.display()),
                    shell_quote(&filename)
                );
                self.run_cmd(&cmd)?;
            }
        }
        Ok(())
    }

    /// Execute the build phase.
    pub fn build(&self, spec: &BuildSpec) -> Result<(), ExecuteError> {
        match spec {
            BuildSpec::Skip => {}
            BuildSpec::Extract(format) => {
                self.build_extract(format)?;
            }
            BuildSpec::Steps(steps) => {
                for step in steps {
                    self.build_step(step)?;
                }
            }
        }
        Ok(())
    }

    fn build_extract(&self, format: &str) -> Result<(), ExecuteError> {
        // Find the archive in build_dir
        let archive = self.find_archive()?;

        let cmd = match format {
            "tar-gz" | "tar.gz" | "tgz" => {
                format!(
                    "tar xzf {} -C {}",
                    shell_quote(archive.display()),
                    shell_quote(self.ctx.build_dir.display())
                )
            }
            "tar-xz" | "tar.xz" | "txz" => {
                format!(
                    "tar xJf {} -C {}",
                    shell_quote(archive.display()),
                    shell_quote(self.ctx.build_dir.display())
                )
            }
            "tar-bz2" | "tar.bz2" | "tbz2" => {
                format!(
                    "tar xjf {} -C {}",
                    shell_quote(archive.display()),
                    shell_quote(self.ctx.build_dir.display())
                )
            }
            "tar" => {
                format!(
                    "tar xf {} -C {}",
                    shell_quote(archive.display()),
                    shell_quote(self.ctx.build_dir.display())
                )
            }
            "zip" => {
                format!(
                    "unzip -o {} -d {}",
                    shell_quote(archive.display()),
                    shell_quote(self.ctx.build_dir.display())
                )
            }
            _ => return Err(ExecuteError::UnsupportedFormat(format.to_string())),
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    fn find_archive(&self) -> Result<PathBuf, ExecuteError> {
        // Look for common archive extensions in build_dir
        let extensions = [
            ".tar.gz",
            ".tgz",
            ".tar.xz",
            ".txz",
            ".tar.bz2",
            ".tbz2",
            ".tar",
            ".zip",
        ];

        if let Ok(entries) = std::fs::read_dir(&self.ctx.build_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    for ext in &extensions {
                        if name.ends_with(ext) {
                            return Ok(path);
                        }
                    }
                }
            }
        }

        Err(ExecuteError::MissingField("archive file".to_string()))
    }

    fn build_step(&self, step: &BuildStep) -> Result<(), ExecuteError> {
        let cmd = match step {
            BuildStep::Configure(args) => {
                if args.is_empty() {
                    format!("./configure --prefix={}", shell_quote(self.ctx.prefix.display()))
                } else {
                    self.expand_vars(args)
                }
            }
            BuildStep::Compile(args) => {
                if args.is_empty() {
                    format!("make -j{}", self.ctx.nproc)
                } else {
                    self.expand_vars(args)
                }
            }
            BuildStep::Test(args) => {
                if args.is_empty() {
                    "make test".to_string()
                } else {
                    self.expand_vars(args)
                }
            }
            BuildStep::Cargo(args) => {
                if args.is_empty() {
                    "cargo build --release".to_string()
                } else {
                    format!("cargo {}", self.expand_vars(args))
                }
            }
            BuildStep::Meson(args) => {
                if args.is_empty() {
                    format!(
                        "meson setup build --prefix={}",
                        shell_quote(self.ctx.prefix.display())
                    )
                } else {
                    format!("meson {}", self.expand_vars(args))
                }
            }
            BuildStep::Ninja(args) => {
                if args.is_empty() {
                    format!("ninja -C build -j{}", self.ctx.nproc)
                } else {
                    format!("ninja {}", self.expand_vars(args))
                }
            }
            BuildStep::Run(cmd) => self.expand_vars(cmd),
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the install phase.
    pub fn install(&self, spec: &InstallSpec) -> Result<(), ExecuteError> {
        for file in &spec.files {
            self.install_file(file)?;
        }
        Ok(())
    }

    fn install_file(&self, file: &InstallFile) -> Result<(), ExecuteError> {
        let cmd = match file {
            InstallFile::ToBin { src, dest, mode } => {
                let src_path = self.ctx.build_dir.join(src);
                let dest_name = dest.as_ref().map(|d| d.as_str()).unwrap_or_else(|| {
                    src.rsplit('/').next().unwrap_or(src)
                });
                let dest_path = self.ctx.prefix.join("bin").join(dest_name);
                let mode = mode.unwrap_or(0o755);

                format!(
                    "install -Dm{:o} {} {}",
                    mode,
                    shell_quote(src_path.display()),
                    shell_quote(dest_path.display())
                )
            }
            InstallFile::ToLib { src, dest } => {
                let src_path = self.ctx.build_dir.join(src);
                let dest_name = dest.as_ref().map(|d| d.as_str()).unwrap_or_else(|| {
                    src.rsplit('/').next().unwrap_or(src)
                });
                let dest_path = self.ctx.prefix.join("lib").join(dest_name);

                format!(
                    "install -Dm644 {} {}",
                    shell_quote(src_path.display()),
                    shell_quote(dest_path.display())
                )
            }
            InstallFile::ToConfig { src, dest, mode } => {
                let src_path = self.ctx.build_dir.join(src);
                let dest_path = PathBuf::from(dest);
                let mode = mode.unwrap_or(0o644);

                format!(
                    "install -Dm{:o} {} {}",
                    mode,
                    shell_quote(src_path.display()),
                    shell_quote(dest_path.display())
                )
            }
            InstallFile::ToMan { src } => {
                let src_path = self.ctx.build_dir.join(src);
                // Detect man section from filename (e.g., "rg.1" -> "man1")
                let section = src
                    .rsplit('.')
                    .next()
                    .and_then(|s| s.parse::<u8>().ok())
                    .unwrap_or(1);
                let dest_dir = self
                    .ctx
                    .prefix
                    .join("share")
                    .join("man")
                    .join(format!("man{}", section));

                format!(
                    "install -Dm644 {} {}/",
                    shell_quote(src_path.display()),
                    shell_quote(dest_dir.display())
                )
            }
            InstallFile::ToShare { src, dest } => {
                let src_path = self.ctx.build_dir.join(src);
                let dest_path = self.ctx.prefix.join("share").join(dest);

                format!(
                    "install -Dm644 {} {}",
                    shell_quote(src_path.display()),
                    shell_quote(dest_path.display())
                )
            }
            InstallFile::Link { src, dest } => {
                let src_path = self.expand_vars(src);
                let dest_path = self.expand_vars(dest);

                format!(
                    "ln -sf {} {}",
                    shell_quote(&src_path),
                    shell_quote(&dest_path)
                )
            }
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the configure phase.
    pub fn configure(&self, spec: &ConfigureSpec) -> Result<(), ExecuteError> {
        for step in &spec.steps {
            self.configure_step(step)?;
        }
        Ok(())
    }

    fn configure_step(&self, step: &ConfigureStep) -> Result<(), ExecuteError> {
        let cmd = match step {
            ConfigureStep::CreateUser {
                name,
                system,
                no_login,
            } => {
                let mut args = vec!["useradd"];
                if *system {
                    args.push("-r");
                }
                if *no_login {
                    args.push("-s");
                    args.push("/sbin/nologin");
                }
                args.push(name);
                args.join(" ")
            }
            ConfigureStep::CreateDir { path, owner } => {
                let expanded = self.expand_vars(path);
                let mut cmd = format!("mkdir -p {}", shell_quote(&expanded));
                if let Some(owner) = owner {
                    cmd.push_str(&format!(" && chown {} {}", shell_quote(owner), shell_quote(&expanded)));
                }
                cmd
            }
            ConfigureStep::Template { path, vars } => {
                // Simple template substitution using sed
                let expanded_path = self.expand_vars(path);
                let mut cmd = format!("cp {} {}.bak", shell_quote(&expanded_path), shell_quote(&expanded_path));
                for (key, value) in vars {
                    // Template uses {{KEY}} syntax, we need to escape braces for format!
                    cmd.push_str(&format!(
                        " && sed -i 's/{{{{{}}}}}/{}/g' {}",
                        key,
                        value.replace('/', "\\/"),
                        shell_quote(&expanded_path)
                    ));
                }
                cmd
            }
            ConfigureStep::Run(cmd) => self.expand_vars(cmd),
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the start action.
    pub fn start(&self, spec: &StartSpec) -> Result<(), ExecuteError> {
        let cmd = match spec {
            StartSpec::Exec(args) => args.join(" "),
            StartSpec::Service { kind, name } => match kind.as_str() {
                "systemd" => format!("systemctl start {}", shell_quote(name)),
                "openrc" => format!("rc-service {} start", shell_quote(name)),
                _ => format!("systemctl start {}", shell_quote(name)),
            },
            StartSpec::Sandbox { config: _, exec } => {
                // TODO: implement sandboxing with landlock/seccomp
                exec.join(" ")
            }
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the stop action.
    pub fn stop(&self, spec: &StopSpec) -> Result<(), ExecuteError> {
        let cmd = match spec {
            StopSpec::ServiceStop(name) => format!("systemctl stop {}", shell_quote(name)),
            StopSpec::Pkill(name) => format!("pkill {}", shell_quote(name)),
            StopSpec::Signal { name, signal } => {
                format!("pkill -{} {}", shell_quote(signal), shell_quote(name))
            }
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the remove action.
    pub fn remove(&self, spec: &RemoveSpec, recipe: &Recipe) -> Result<(), ExecuteError> {
        // Stop first if requested
        if spec.stop_first {
            if let Some(ref stop) = recipe.stop {
                let _ = self.stop(stop); // Ignore errors, process might not be running
            }
        }

        for step in &spec.steps {
            self.remove_step(step)?;
        }

        Ok(())
    }

    fn remove_step(&self, step: &RemoveStep) -> Result<(), ExecuteError> {
        let cmd = match step {
            RemoveStep::RmPrefix => {
                format!("rm -rf {}", shell_quote(self.ctx.prefix.display()))
            }
            RemoveStep::RmBin(name) => {
                let path = self.ctx.prefix.join("bin").join(name);
                format!("rm -f {}", shell_quote(path.display()))
            }
            RemoveStep::RmConfig { path, prompt: _ } => {
                // TODO: implement prompting
                format!("rm -f {}", shell_quote(&self.expand_vars(path)))
            }
            RemoveStep::RmData { path, keep } => {
                if *keep {
                    return Ok(()); // Keep data, don't remove
                }
                format!("rm -rf {}", shell_quote(&self.expand_vars(path)))
            }
            RemoveStep::RmUser(name) => {
                format!("userdel {}", shell_quote(name))
            }
        };

        self.run_cmd(&cmd)?;
        Ok(())
    }

    /// Execute the cleanup phase - remove build artifacts to save space.
    pub fn cleanup(&self, spec: &CleanupSpec) -> Result<(), ExecuteError> {
        if self.ctx.verbose || self.ctx.dry_run {
            eprintln!(
                "[{}] cleanup: {:?} (keep: {:?})",
                if self.ctx.dry_run { "dry-run" } else { "exec" },
                spec.target,
                spec.keep
            );
        }

        if self.ctx.dry_run {
            return Ok(());
        }

        match spec.target {
            CleanupTarget::All => {
                // Remove entire build directory, preserving 'keep' paths
                if spec.keep.is_empty() {
                    std::fs::remove_dir_all(&self.ctx.build_dir)?;
                } else {
                    self.cleanup_with_keep(&spec.keep)?;
                }
            }
            CleanupTarget::Downloads => {
                // Remove only archive files
                self.cleanup_by_extension(&[
                    ".tar.gz", ".tgz", ".tar.xz", ".txz", ".tar.bz2", ".tbz2", ".tar", ".zip",
                ])?;
            }
            CleanupTarget::Sources => {
                // Remove extracted directories but keep archives
                self.cleanup_directories()?;
            }
            CleanupTarget::Artifacts => {
                // Remove build artifacts (target/, build/, *.o, etc.) but keep sources
                self.cleanup_build_artifacts()?;
            }
        }

        Ok(())
    }

    /// Remove all files except those in the keep list.
    fn cleanup_with_keep(&self, keep: &[String]) -> Result<(), ExecuteError> {
        let entries = std::fs::read_dir(&self.ctx.build_dir)?;

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !keep.iter().any(|k| name == *k || name.starts_with(k)) {
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
            }
        }

        Ok(())
    }

    /// Remove files matching certain extensions.
    fn cleanup_by_extension(&self, extensions: &[&str]) -> Result<(), ExecuteError> {
        let entries = std::fs::read_dir(&self.ctx.build_dir)?;

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if extensions.iter().any(|ext| name.ends_with(ext)) {
                std::fs::remove_file(entry.path())?;
            }
        }

        Ok(())
    }

    /// Remove directories only (keep archive files).
    fn cleanup_directories(&self) -> Result<(), ExecuteError> {
        let entries = std::fs::read_dir(&self.ctx.build_dir)?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            }
        }

        Ok(())
    }

    /// Remove common build artifact directories.
    fn cleanup_build_artifacts(&self) -> Result<(), ExecuteError> {
        let artifact_dirs = ["target", "build", "_build", "out", "dist", ".cache"];
        let artifact_exts = [".o", ".a", ".so", ".dylib", ".rlib", ".rmeta"];

        let entries = std::fs::read_dir(&self.ctx.build_dir)?;

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() && artifact_dirs.contains(&name.as_str()) {
                std::fs::remove_dir_all(&path)?;
            } else if path.is_file() && artifact_exts.iter().any(|ext| name.ends_with(ext)) {
                std::fs::remove_file(&path)?;
            }
        }

        Ok(())
    }

    /// Run a shell command with variable expansion.
    fn run_cmd(&self, cmd: &str) -> Result<Output, ExecuteError> {
        let expanded = self.expand_vars(cmd);

        if self.ctx.verbose || self.ctx.dry_run {
            eprintln!("[{}] {}", if self.ctx.dry_run { "dry-run" } else { "exec" }, expanded);
        }

        if self.ctx.dry_run {
            return Ok(Output {
                status: std::process::ExitStatus::default(),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(&expanded)
            .current_dir(&self.ctx.build_dir)
            .output()?;

        if !output.status.success() {
            return Err(ExecuteError::CommandFailed {
                cmd: expanded,
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(output)
    }

    /// Expand variables in a string.
    fn expand_vars(&self, s: &str) -> String {
        s.replace("$PREFIX", &self.ctx.prefix.display().to_string())
            .replace("$NPROC", &self.ctx.nproc.to_string())
            .replace("$ARCH", &self.ctx.arch)
            .replace("$BUILD_DIR", &self.ctx.build_dir.display().to_string())
    }
}

/// Extract filename from a URL.
fn url_filename(url: &str) -> String {
    url.rsplit('/')
        .next()
        .unwrap_or("download")
        .split('?')
        .next()
        .unwrap_or("download")
        .to_string()
}

/// Shell-quote a value for safe interpolation.
fn shell_quote(s: impl std::fmt::Display) -> String {
    let s = s.to_string();
    if s.chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '/')
    {
        s
    } else {
        format!("'{}'", s.replace('\'', "'\"'\"'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_vars() {
        let ctx = Context {
            prefix: PathBuf::from("/opt/myapp"),
            build_dir: PathBuf::from("/tmp/build"),
            arch: "x86_64".to_string(),
            nproc: 4,
            dry_run: false,
            verbose: false,
        };
        let executor = Executor::new(ctx);

        assert_eq!(
            executor.expand_vars("--prefix=$PREFIX"),
            "--prefix=/opt/myapp"
        );
        assert_eq!(executor.expand_vars("make -j$NPROC"), "make -j4");
        assert_eq!(
            executor.expand_vars("arch is $ARCH"),
            "arch is x86_64"
        );
    }

    #[test]
    fn test_url_filename() {
        assert_eq!(
            url_filename("https://example.com/ripgrep-14.1.0.tar.gz"),
            "ripgrep-14.1.0.tar.gz"
        );
        assert_eq!(
            url_filename("https://example.com/file.zip?token=abc"),
            "file.zip"
        );
    }

    #[test]
    fn test_shell_quote() {
        assert_eq!(shell_quote("simple"), "simple");
        assert_eq!(shell_quote("/path/to/file"), "/path/to/file");
        assert_eq!(shell_quote("has space"), "'has space'");
        assert_eq!(shell_quote("has'quote"), "'has'\"'\"'quote'");
    }

    #[test]
    fn test_context_default() {
        let ctx = Context::default();
        assert_eq!(ctx.prefix, PathBuf::from("/usr/local"));
        assert!(!ctx.dry_run);
        assert!(!ctx.verbose);
    }

    #[test]
    fn test_context_builder() {
        let ctx = Context::with_prefix("/opt/app")
            .arch("aarch64")
            .dry_run(true)
            .verbose(true);

        assert_eq!(ctx.prefix, PathBuf::from("/opt/app"));
        assert_eq!(ctx.arch, "aarch64");
        assert!(ctx.dry_run);
        assert!(ctx.verbose);
    }
}
