//! Recipe interpretation - converts parsed S-expressions into structured data.

use std::path::PathBuf;

use crate::ast::Expr;
use crate::features::{DepSpec, Feature, FeatureSet};
use crate::version::Dependency;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecipeError {
    #[error("expected (package ...), got: {0}")]
    NotAPackage(String),
    #[error("missing package name")]
    MissingName,
    #[error("missing package version")]
    MissingVersion,
    #[error("unknown action: {0}")]
    UnknownAction(String),
    #[error("invalid action format: {0}")]
    InvalidAction(String),
    #[error("invalid dependency: {0}")]
    InvalidDependency(String),
    #[error("invalid patch: {0}")]
    InvalidPatch(String),
    #[error("invalid feature: {0}")]
    InvalidFeature(String),
    #[error("invalid subpackage: {0}")]
    InvalidSubpackage(String),
}

/// A parsed package recipe.
#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Vec<String>,
    pub homepage: Option<String>,
    pub maintainer: Option<String>,

    // Dependencies with version constraints and feature conditionals
    pub deps: Vec<DepSpec>,
    pub build_deps: Vec<DepSpec>,

    // Features/variants
    pub features: Option<FeatureSet>,

    // Provides/conflicts for virtual packages
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,

    // Patches to apply after acquire
    pub patches: Option<PatchSpec>,

    pub acquire: Option<AcquireSpec>,
    pub build: Option<BuildSpec>,
    pub install: Option<InstallSpec>,
    pub configure: Option<ConfigureSpec>,
    pub start: Option<StartSpec>,
    pub stop: Option<StopSpec>,
    pub remove: Option<RemoveSpec>,
    pub cleanup: Option<CleanupSpec>,

    // Split packages
    pub subpackages: Vec<Subpackage>,
}

/// Patch specification - patches to apply after source acquisition.
#[derive(Debug, Clone)]
pub struct PatchSpec {
    /// Patches to apply in order
    pub patches: Vec<PatchSource>,
    /// Strip level for patch command (-p N)
    pub strip: u32,
}

/// Source of a patch file.
#[derive(Debug, Clone)]
pub enum PatchSource {
    /// Local file relative to recipe
    Local(PathBuf),
    /// Remote URL with optional verification
    Remote { url: String, verify: Option<Verify> },
}

/// A split subpackage definition.
#[derive(Debug, Clone)]
pub struct Subpackage {
    pub name: String,
    pub description: Option<String>,
    pub deps: Vec<DepSpec>,
    pub install: InstallSpec,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
}

/// Shell type for completion files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

#[derive(Debug, Clone)]
pub enum AcquireSpec {
    Source { url: String, verify: Option<Verify> },
    Binary { urls: Vec<(String, String)> }, // (arch, url)
    Git { url: String, reference: Option<GitRef> },
    OsPackage { packages: Vec<(String, String)> }, // (manager, name)
}

#[derive(Debug, Clone)]
pub enum Verify {
    Sha256(String),
    Sha256Url(String),
}

#[derive(Debug, Clone)]
pub enum GitRef {
    Tag(String),
    Branch(String),
    Commit(String),
}

#[derive(Debug, Clone)]
pub enum BuildSpec {
    Skip,
    Extract(String), // format: tar-gz, tar-xz, zip
    Steps(Vec<BuildStep>),
}

#[derive(Debug, Clone)]
pub enum BuildStep {
    Configure(String),
    Compile(String),
    Test(String),
    Cargo(String),
    Meson(String),
    Ninja(String),
    Run(String),
}

#[derive(Debug, Clone)]
pub struct InstallSpec {
    pub files: Vec<InstallFile>,
}

#[derive(Debug, Clone)]
pub enum InstallFile {
    ToBin { src: String, dest: Option<String>, mode: Option<u32> },
    ToLib { src: String, dest: Option<String> },
    ToConfig { src: String, dest: String, mode: Option<u32> },
    ToMan { src: String },
    ToShare { src: String, dest: String },
    Link { src: String, dest: String },
    // New install targets
    ToInclude { src: String, dest: Option<String> },
    ToPkgconfig { src: String },
    ToCmake { src: String },
    ToSystemd { src: String },
    ToCompletions { src: String, shell: Shell },
}

#[derive(Debug, Clone)]
pub struct ConfigureSpec {
    pub steps: Vec<ConfigureStep>,
}

#[derive(Debug, Clone)]
pub enum ConfigureStep {
    CreateUser { name: String, system: bool, no_login: bool },
    CreateDir { path: String, owner: Option<String> },
    Template { path: String, vars: Vec<(String, String)> },
    Run(String),
}

#[derive(Debug, Clone)]
pub enum StartSpec {
    Exec(Vec<String>),
    Service { kind: String, name: String },
    Sandbox { config: SandboxConfig, exec: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub allow_read: Vec<String>,
    pub allow_write: Vec<String>,
    pub allow_net: bool,
}

#[derive(Debug, Clone)]
pub enum StopSpec {
    ServiceStop(String),
    Pkill(String),
    Signal { name: String, signal: String },
}

#[derive(Debug, Clone)]
pub struct RemoveSpec {
    pub stop_first: bool,
    pub steps: Vec<RemoveStep>,
}

#[derive(Debug, Clone)]
pub enum RemoveStep {
    RmPrefix,
    RmBin(String),
    RmConfig { path: String, prompt: bool },
    RmData { path: String, keep: bool },
    RmUser(String),
}

/// Cleanup specification for removing build artifacts after installation.
#[derive(Debug, Clone)]
pub struct CleanupSpec {
    /// What to clean up
    pub target: CleanupTarget,
    /// Paths to preserve (relative to build_dir)
    pub keep: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub enum CleanupTarget {
    /// Remove entire build directory (default)
    #[default]
    All,
    /// Remove only downloaded archives
    Downloads,
    /// Remove only extracted sources, keep archives for caching
    Sources,
    /// Remove build artifacts but keep sources (for debugging)
    Artifacts,
}

impl Recipe {
    /// Parse a recipe from an S-expression.
    pub fn from_expr(expr: &Expr) -> Result<Self, RecipeError> {
        let list = expr.as_list().ok_or_else(|| {
            RecipeError::NotAPackage(format!("{}", expr))
        })?;

        if list.first().and_then(|e| e.as_atom()) != Some("package") {
            return Err(RecipeError::NotAPackage(format!("{}", expr)));
        }

        let name = list.get(1)
            .and_then(|e| e.as_atom())
            .ok_or(RecipeError::MissingName)?
            .to_string();

        let version = list.get(2)
            .and_then(|e| e.as_atom())
            .ok_or(RecipeError::MissingVersion)?
            .to_string();

        let mut recipe = Recipe {
            name,
            version,
            description: None,
            license: Vec::new(),
            homepage: None,
            maintainer: None,
            deps: Vec::new(),
            build_deps: Vec::new(),
            features: None,
            provides: Vec::new(),
            conflicts: Vec::new(),
            patches: None,
            acquire: None,
            build: None,
            install: None,
            configure: None,
            start: None,
            stop: None,
            remove: None,
            cleanup: None,
            subpackages: Vec::new(),
        };

        // Parse actions (elements 3+)
        for action in list.iter().skip(3) {
            recipe.parse_action(action)?;
        }

        Ok(recipe)
    }

    fn parse_action(&mut self, expr: &Expr) -> Result<(), RecipeError> {
        let head = expr.head().ok_or_else(|| {
            RecipeError::InvalidAction(format!("{}", expr))
        })?;

        match head {
            "description" => {
                self.description = expr.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .map(|s| s.to_string());
            }
            "license" => {
                if let Some(tail) = expr.tail() {
                    self.license = tail.iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
            }
            "homepage" => {
                self.homepage = expr.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .map(|s| s.to_string());
            }
            "maintainer" => {
                self.maintainer = expr.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .map(|s| s.to_string());
            }
            "deps" => {
                self.deps = Self::parse_deps(expr)?;
            }
            "build-deps" => {
                self.build_deps = Self::parse_deps(expr)?;
            }
            "features" => {
                self.features = Self::parse_features(expr)?;
            }
            "provides" => {
                if let Some(tail) = expr.tail() {
                    self.provides = tail.iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
            }
            "conflicts" => {
                if let Some(tail) = expr.tail() {
                    self.conflicts = tail.iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
            }
            "patches" => {
                self.patches = Self::parse_patches(expr)?;
            }
            "acquire" => {
                self.acquire = Self::parse_acquire(expr)?;
            }
            "build" => {
                self.build = Self::parse_build(expr)?;
            }
            "install" => {
                self.install = Self::parse_install(expr)?;
            }
            "configure" => {
                self.configure = Self::parse_configure(expr)?;
            }
            "start" => {
                self.start = Self::parse_start(expr)?;
            }
            "stop" => {
                self.stop = Self::parse_stop(expr)?;
            }
            "remove" => {
                self.remove = Self::parse_remove(expr)?;
            }
            "cleanup" => {
                self.cleanup = Self::parse_cleanup(expr)?;
            }
            "subpackages" => {
                self.subpackages = Self::parse_subpackages(expr)?;
            }
            "update" | "hooks" => {
                // TODO: implement these
            }
            _ => {
                return Err(RecipeError::UnknownAction(head.to_string()));
            }
        }

        Ok(())
    }

    /// Parse deps with version constraints and conditionals.
    /// Formats:
    /// - "package" (any version)
    /// - "package >= 1.0" (version constraint)
    /// - (if feature "package >= 1.0") (conditional)
    fn parse_deps(expr: &Expr) -> Result<Vec<DepSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(Vec::new()),
        };

        let mut deps = Vec::new();

        for item in tail {
            // Check for conditional: (if feature "dep")
            if item.head() == Some("if") {
                if let Some(args) = item.tail() {
                    if args.len() >= 2 {
                        let feature = args[0].as_atom()
                            .ok_or_else(|| RecipeError::InvalidDependency(format!("{}", item)))?
                            .to_string();
                        let dep_str = args[1].as_atom()
                            .ok_or_else(|| RecipeError::InvalidDependency(format!("{}", item)))?;
                        let dep = Dependency::parse(dep_str)
                            .map_err(|e| RecipeError::InvalidDependency(e.to_string()))?;
                        deps.push(DepSpec::Conditional { feature, dep });
                        continue;
                    }
                }
                return Err(RecipeError::InvalidDependency(format!("{}", item)));
            }

            // Regular dependency (string)
            if let Some(dep_str) = item.as_atom() {
                let dep = Dependency::parse(dep_str)
                    .map_err(|e| RecipeError::InvalidDependency(e.to_string()))?;
                deps.push(DepSpec::Always(dep));
            }
        }

        Ok(deps)
    }

    /// Parse features section.
    /// Format:
    /// (features
    ///   (default "feat1" "feat2")
    ///   (feat1 "Description")
    ///   (feat2 "Description" (implies "other")))
    fn parse_features(expr: &Expr) -> Result<Option<FeatureSet>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut features = FeatureSet::new();
        let mut defaults = Vec::new();

        for item in tail {
            let head = match item.head() {
                Some(h) => h,
                None => continue,
            };

            if head == "default" {
                // (default "feat1" "feat2")
                if let Some(args) = item.tail() {
                    for arg in args {
                        if let Some(name) = arg.as_atom() {
                            defaults.push(name.to_string());
                        }
                    }
                }
            } else {
                // (feature-name "description" (implies "other"))
                let mut feature = Feature::new(head);
                if let Some(args) = item.tail() {
                    for arg in args {
                        if let Some(desc) = arg.as_atom() {
                            feature.description = Some(desc.to_string());
                        } else if arg.head() == Some("implies") {
                            if let Some(implied) = arg.tail() {
                                feature.implies = implied.iter()
                                    .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                                    .collect();
                            }
                        }
                    }
                }
                features.add(feature);
            }
        }

        // Set defaults after all features are added
        for name in defaults {
            features.set_default(name);
        }

        Ok(Some(features))
    }

    /// Parse patches section.
    /// Format:
    /// (patches
    ///   "local/patch.patch"
    ///   (url "https://..." (sha256 "..."))
    ///   (strip 1))
    fn parse_patches(expr: &Expr) -> Result<Option<PatchSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut patches = Vec::new();
        let mut strip = 1; // Default strip level

        for item in tail {
            // Check for strip level: (strip N)
            if item.head() == Some("strip") {
                if let Some(args) = item.tail() {
                    if let Some(n) = args.first().and_then(|e| e.as_atom()) {
                        strip = n.parse().unwrap_or(1);
                    }
                }
                continue;
            }

            // Check for remote patch: (url "..." (sha256 "..."))
            if item.head() == Some("url") {
                if let Some(args) = item.tail() {
                    let url = args.first()
                        .and_then(|e| e.as_atom())
                        .ok_or_else(|| RecipeError::InvalidPatch(format!("{}", item)))?
                        .to_string();

                    let mut verify = None;
                    if let Some(verify_expr) = args.get(1) {
                        if verify_expr.head() == Some("sha256") {
                            if let Some(hash) = verify_expr.tail()
                                .and_then(|t| t.first())
                                .and_then(|e| e.as_atom())
                            {
                                verify = Some(Verify::Sha256(hash.to_string()));
                            }
                        }
                    }

                    patches.push(PatchSource::Remote { url, verify });
                    continue;
                }
            }

            // Local patch file (string)
            if let Some(path) = item.as_atom() {
                patches.push(PatchSource::Local(PathBuf::from(path)));
            }
        }

        if patches.is_empty() {
            Ok(None)
        } else {
            Ok(Some(PatchSpec { patches, strip }))
        }
    }

    /// Parse subpackages section.
    fn parse_subpackages(expr: &Expr) -> Result<Vec<Subpackage>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(Vec::new()),
        };

        let mut subpackages = Vec::new();

        for item in tail {
            let name = item.head()
                .ok_or_else(|| RecipeError::InvalidSubpackage(format!("{}", item)))?
                .to_string();

            let mut subpkg = Subpackage {
                name,
                description: None,
                deps: Vec::new(),
                install: InstallSpec { files: Vec::new() },
                provides: Vec::new(),
                conflicts: Vec::new(),
            };

            if let Some(args) = item.tail() {
                for arg in args {
                    let head = match arg.head() {
                        Some(h) => h,
                        None => continue,
                    };

                    match head {
                        "description" => {
                            subpkg.description = arg.tail()
                                .and_then(|t| t.first())
                                .and_then(|e| e.as_atom())
                                .map(|s| s.to_string());
                        }
                        "deps" => {
                            subpkg.deps = Self::parse_deps(arg)?;
                        }
                        "install" => {
                            if let Some(install) = Self::parse_install(arg)? {
                                subpkg.install = install;
                            }
                        }
                        "provides" => {
                            if let Some(tail) = arg.tail() {
                                subpkg.provides = tail.iter()
                                    .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                                    .collect();
                            }
                        }
                        "conflicts" => {
                            if let Some(tail) = arg.tail() {
                                subpkg.conflicts = tail.iter()
                                    .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                                    .collect();
                            }
                        }
                        _ => {}
                    }
                }
            }

            subpackages.push(subpkg);
        }

        Ok(subpackages)
    }

    fn parse_acquire(expr: &Expr) -> Result<Option<AcquireSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        for item in tail {
            let head = match item.head() {
                Some(h) => h,
                None => continue,
            };

            match head {
                "source" => {
                    let args = item.tail();
                    let url = args
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();

                    // Check for verification
                    let mut verify = None;
                    if let Some(tail) = args {
                        for arg in tail.iter().skip(1) {
                            if arg.head() == Some("sha256") {
                                if let Some(hash) = arg.tail()
                                    .and_then(|t| t.first())
                                    .and_then(|e| e.as_atom())
                                {
                                    verify = Some(Verify::Sha256(hash.to_string()));
                                }
                            }
                        }
                    }

                    return Ok(Some(AcquireSpec::Source { url, verify }));
                }
                "binary" => {
                    let urls = item.tail()
                        .map(|t| {
                            t.iter()
                                .filter_map(|e| {
                                    let arch = e.head()?;
                                    let url = e.tail()?.first()?.as_atom()?;
                                    Some((arch.to_string(), url.to_string()))
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    return Ok(Some(AcquireSpec::Binary { urls }));
                }
                "git" => {
                    let url = item.tail()
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    return Ok(Some(AcquireSpec::Git { url, reference: None }));
                }
                _ => {}
            }
        }

        Ok(None)
    }

    fn parse_build(expr: &Expr) -> Result<Option<BuildSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        // Check for simple forms
        if let Some(first) = tail.first() {
            if first.is_atom("skip") {
                return Ok(Some(BuildSpec::Skip));
            }
            if first.head() == Some("extract") {
                let format = first.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .unwrap_or("tar-gz")
                    .to_string();
                return Ok(Some(BuildSpec::Extract(format)));
            }
        }

        // Parse build steps
        let mut steps = Vec::new();
        for item in tail {
            let head = match item.head() {
                Some(h) => h,
                None => continue,
            };
            let arg = item.tail()
                .and_then(|t| t.first())
                .and_then(|e| e.as_atom())
                .unwrap_or("")
                .to_string();

            let step = match head {
                "configure" => BuildStep::Configure(arg),
                "compile" => BuildStep::Compile(arg),
                "test" => BuildStep::Test(arg),
                "cargo" => BuildStep::Cargo(arg),
                "meson" => BuildStep::Meson(arg),
                "ninja" => BuildStep::Ninja(arg),
                "run" => BuildStep::Run(arg),
                _ => continue,
            };
            steps.push(step);
        }

        if steps.is_empty() {
            Ok(None)
        } else {
            Ok(Some(BuildSpec::Steps(steps)))
        }
    }

    fn parse_install(expr: &Expr) -> Result<Option<InstallSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut files = Vec::new();
        for item in tail {
            let head = match item.head() {
                Some(h) => h,
                None => continue,
            };
            let args: Vec<_> = item.tail()
                .map(|t| t.iter().filter_map(|e| e.as_atom()).collect())
                .unwrap_or_default();

            let file = match head {
                "to-bin" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).map(|s| s.to_string());
                    InstallFile::ToBin { src, dest, mode: None }
                }
                "to-lib" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).map(|s| s.to_string());
                    InstallFile::ToLib { src, dest }
                }
                "to-config" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).unwrap_or(&"").to_string();
                    InstallFile::ToConfig { src, dest, mode: None }
                }
                "to-man" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    InstallFile::ToMan { src }
                }
                "to-share" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).unwrap_or(&"").to_string();
                    InstallFile::ToShare { src, dest }
                }
                "link" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).unwrap_or(&"").to_string();
                    InstallFile::Link { src, dest }
                }
                // New install targets
                "to-include" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let dest = args.get(1).map(|s| s.to_string());
                    InstallFile::ToInclude { src, dest }
                }
                "to-pkgconfig" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    InstallFile::ToPkgconfig { src }
                }
                "to-cmake" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    InstallFile::ToCmake { src }
                }
                "to-systemd" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    InstallFile::ToSystemd { src }
                }
                "to-completions" => {
                    let src = args.first().unwrap_or(&"").to_string();
                    let shell_str = args.get(1).unwrap_or(&"bash");
                    let shell = match *shell_str {
                        "zsh" => Shell::Zsh,
                        "fish" => Shell::Fish,
                        _ => Shell::Bash,
                    };
                    InstallFile::ToCompletions { src, shell }
                }
                _ => continue,
            };
            files.push(file);
        }

        if files.is_empty() {
            Ok(None)
        } else {
            Ok(Some(InstallSpec { files }))
        }
    }

    fn parse_configure(expr: &Expr) -> Result<Option<ConfigureSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut steps = Vec::new();

        for item in tail {
            let head = match item.head() {
                Some(h) => h,
                None => continue,
            };

            match head {
                "create-user" => {
                    let args: Vec<_> = item.tail()
                        .map(|t| t.iter().collect())
                        .unwrap_or_default();
                    let name = args.first()
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    let system = args.iter().any(|e| e.is_atom("system"));
                    let no_login = args.iter().any(|e| e.is_atom("no-login"));
                    steps.push(ConfigureStep::CreateUser { name, system, no_login });
                }
                "create-dir" => {
                    let args: Vec<_> = item.tail()
                        .map(|t| t.iter().collect())
                        .unwrap_or_default();
                    let path = args.first()
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    let owner = args.get(1).and_then(|e| e.as_atom()).map(|s| s.to_string());
                    steps.push(ConfigureStep::CreateDir { path, owner });
                }
                "template" => {
                    let args: Vec<_> = item.tail()
                        .map(|t| t.iter().collect())
                        .unwrap_or_default();
                    let path = args.first()
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    let vars = Vec::new(); // TODO: parse vars
                    steps.push(ConfigureStep::Template { path, vars });
                }
                "run" => {
                    let cmd = item.tail()
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    steps.push(ConfigureStep::Run(cmd));
                }
                _ => {}
            }
        }

        if steps.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ConfigureSpec { steps }))
        }
    }

    fn parse_start(expr: &Expr) -> Result<Option<StartSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some(first) = tail.first() {
            if first.head() == Some("exec") {
                let args: Vec<_> = first.tail()
                    .map(|t| t.iter().filter_map(|e| e.as_atom().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                return Ok(Some(StartSpec::Exec(args)));
            }
            if first.head() == Some("service") {
                let args: Vec<_> = first.tail()
                    .map(|t| t.iter().filter_map(|e| e.as_atom()).collect())
                    .unwrap_or_default();
                if args.len() >= 2 {
                    return Ok(Some(StartSpec::Service {
                        kind: args[0].to_string(),
                        name: args[1].to_string(),
                    }));
                }
            }
        }

        Ok(None)
    }

    fn parse_stop(expr: &Expr) -> Result<Option<StopSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some(first) = tail.first() {
            if first.head() == Some("service-stop") {
                let name = first.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .unwrap_or("")
                    .to_string();
                return Ok(Some(StopSpec::ServiceStop(name)));
            }
            if first.head() == Some("pkill") {
                let name = first.tail()
                    .and_then(|t| t.first())
                    .and_then(|e| e.as_atom())
                    .unwrap_or("")
                    .to_string();
                return Ok(Some(StopSpec::Pkill(name)));
            }
        }

        Ok(None)
    }

    fn parse_remove(expr: &Expr) -> Result<Option<RemoveSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => return Ok(None),
        };

        let mut stop_first = false;
        let mut steps = Vec::new();

        for item in tail {
            if item.is_atom("stop-first") {
                stop_first = true;
                continue;
            }

            let head = match item.head() {
                Some(h) => h,
                None => {
                    if item.is_atom("rm-prefix") {
                        steps.push(RemoveStep::RmPrefix);
                    }
                    continue;
                }
            };

            match head {
                "rm-prefix" => steps.push(RemoveStep::RmPrefix),
                "rm-bin" => {
                    let name = item.tail()
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    steps.push(RemoveStep::RmBin(name));
                }
                "rm-user" => {
                    let name = item.tail()
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    steps.push(RemoveStep::RmUser(name));
                }
                _ => {}
            }
        }

        if steps.is_empty() && !stop_first {
            Ok(None)
        } else {
            Ok(Some(RemoveSpec { stop_first, steps }))
        }
    }

    fn parse_cleanup(expr: &Expr) -> Result<Option<CleanupSpec>, RecipeError> {
        let tail = match expr.tail() {
            Some(t) => t,
            None => {
                // (cleanup) with no args means clean all
                return Ok(Some(CleanupSpec {
                    target: CleanupTarget::All,
                    keep: Vec::new(),
                }));
            }
        };

        let mut target = CleanupTarget::All;
        let mut keep = Vec::new();

        for item in tail {
            // Check for atoms like "all", "downloads", "sources", "artifacts"
            if let Some(atom) = item.as_atom() {
                target = match atom {
                    "all" => CleanupTarget::All,
                    "downloads" => CleanupTarget::Downloads,
                    "sources" => CleanupTarget::Sources,
                    "artifacts" => CleanupTarget::Artifacts,
                    _ => continue,
                };
            }
            // Check for (keep "path1" "path2" ...) list
            if item.head() == Some("keep") {
                if let Some(paths) = item.tail() {
                    keep = paths
                        .iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
            }
        }

        Ok(Some(CleanupSpec { target, keep }))
    }

    /// Get all dependencies for the given enabled features.
    pub fn deps_for_features(&self, enabled_features: &std::collections::HashSet<String>) -> Vec<&Dependency> {
        self.deps.iter()
            .filter(|d| d.is_required(enabled_features))
            .map(|d| d.dependency())
            .collect()
    }

    /// Get all build dependencies for the given enabled features.
    pub fn build_deps_for_features(&self, enabled_features: &std::collections::HashSet<String>) -> Vec<&Dependency> {
        self.build_deps.iter()
            .filter(|d| d.is_required(enabled_features))
            .map(|d| d.dependency())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::version::VersionConstraint;

    #[test]
    fn test_parse_ripgrep() {
        let input = r#"
            (package "ripgrep" "14.1.0"
              (description "Fast grep alternative written in Rust")
              (license "MIT")
              (deps)
              (build (extract tar-gz))
              (install
                (to-bin "rg")
                (to-man "doc/rg.1"))
              (start (exec "rg" $@))
              (remove (rm-prefix)))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        assert_eq!(recipe.name, "ripgrep");
        assert_eq!(recipe.version, "14.1.0");
        assert_eq!(recipe.description, Some("Fast grep alternative written in Rust".into()));
        assert_eq!(recipe.license, vec!["MIT"]);
        assert!(recipe.deps.is_empty());
        assert!(matches!(recipe.build, Some(BuildSpec::Extract(_))));
        assert!(recipe.install.is_some());
        assert!(matches!(recipe.start, Some(StartSpec::Exec(_))));
        assert!(recipe.remove.is_some());
    }

    #[test]
    fn test_parse_deps_with_constraints() {
        let input = r#"
            (package "myapp" "1.0"
              (deps
                "openssl >= 1.1.0"
                "zlib"
                "glibc < 3.0"
                (if vulkan "vulkan-loader >= 1.3")))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        assert_eq!(recipe.deps.len(), 4);

        // Check first dep: openssl >= 1.1.0
        if let DepSpec::Always(dep) = &recipe.deps[0] {
            assert_eq!(dep.name, "openssl");
            assert!(matches!(dep.constraint, VersionConstraint::Gte(_)));
        } else {
            panic!("expected Always dep");
        }

        // Check second dep: zlib (any version)
        if let DepSpec::Always(dep) = &recipe.deps[1] {
            assert_eq!(dep.name, "zlib");
            assert!(matches!(dep.constraint, VersionConstraint::Any));
        } else {
            panic!("expected Always dep");
        }

        // Check conditional dep
        if let DepSpec::Conditional { feature, dep } = &recipe.deps[3] {
            assert_eq!(feature, "vulkan");
            assert_eq!(dep.name, "vulkan-loader");
        } else {
            panic!("expected Conditional dep");
        }
    }

    #[test]
    fn test_parse_features() {
        let input = r#"
            (package "ffmpeg" "6.1"
              (features
                (default "x264" "opus")
                (x264 "Enable H.264 support")
                (x265 "Enable HEVC support")
                (opus "Enable Opus audio")))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        let features = recipe.features.unwrap();
        assert!(features.has("x264"));
        assert!(features.has("x265"));
        assert!(features.has("opus"));
        assert!(features.default.contains("x264"));
        assert!(features.default.contains("opus"));
        assert!(!features.default.contains("x265"));
    }

    #[test]
    fn test_parse_provides_conflicts() {
        let input = r#"
            (package "neovim" "0.9.5"
              (provides "vi" "vim" "editor")
              (conflicts "vim" "vim-minimal"))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        assert_eq!(recipe.provides, vec!["vi", "vim", "editor"]);
        assert_eq!(recipe.conflicts, vec!["vim", "vim-minimal"]);
    }

    #[test]
    fn test_parse_patches() {
        let input = r#"
            (package "nginx" "1.25.0"
              (patches
                "patches/fix-ssl-crash.patch"
                (url "https://example.com/fix.patch" (sha256 "abc123"))
                (strip 1)))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        let patches = recipe.patches.unwrap();
        assert_eq!(patches.strip, 1);
        assert_eq!(patches.patches.len(), 2);

        assert!(matches!(&patches.patches[0], PatchSource::Local(p) if p.to_str() == Some("patches/fix-ssl-crash.patch")));
        assert!(matches!(&patches.patches[1], PatchSource::Remote { url, verify } if url == "https://example.com/fix.patch" && verify.is_some()));
    }

    #[test]
    fn test_parse_subpackages() {
        let input = r#"
            (package "openssl" "3.2.0"
              (install
                (to-lib "libssl.so.3")
                (to-bin "openssl"))
              (subpackages
                (openssl-dev
                  (description "OpenSSL development files")
                  (deps "openssl = 3.2.0")
                  (install
                    (to-include "include/openssl")
                    (to-pkgconfig "openssl.pc")))))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        assert_eq!(recipe.subpackages.len(), 1);
        let subpkg = &recipe.subpackages[0];
        assert_eq!(subpkg.name, "openssl-dev");
        assert_eq!(subpkg.description, Some("OpenSSL development files".to_string()));
        assert_eq!(subpkg.deps.len(), 1);
        assert_eq!(subpkg.install.files.len(), 2);
    }

    #[test]
    fn test_parse_new_install_targets() {
        let input = r#"
            (package "mylib" "1.0"
              (install
                (to-include "include/*.h" "mylib")
                (to-pkgconfig "mylib.pc")
                (to-cmake "cmake/MyLibConfig.cmake")
                (to-systemd "mylib.service")
                (to-completions "completions/mylib.bash" "bash")
                (to-completions "completions/_mylib" "zsh")))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        let install = recipe.install.unwrap();
        assert_eq!(install.files.len(), 6);

        assert!(matches!(&install.files[0], InstallFile::ToInclude { src, dest } if src == "include/*.h" && dest.as_deref() == Some("mylib")));
        assert!(matches!(&install.files[1], InstallFile::ToPkgconfig { src } if src == "mylib.pc"));
        assert!(matches!(&install.files[2], InstallFile::ToCmake { src } if src == "cmake/MyLibConfig.cmake"));
        assert!(matches!(&install.files[3], InstallFile::ToSystemd { src } if src == "mylib.service"));
        assert!(matches!(&install.files[4], InstallFile::ToCompletions { src, shell } if src == "completions/mylib.bash" && *shell == Shell::Bash));
        assert!(matches!(&install.files[5], InstallFile::ToCompletions { src, shell } if src == "completions/_mylib" && *shell == Shell::Zsh));
    }

    #[test]
    fn test_parse_cleanup() {
        // Test (cleanup) - defaults to all
        let input = r#"(package "test" "1.0" (cleanup))"#;
        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();
        assert!(recipe.cleanup.is_some());
        let cleanup = recipe.cleanup.unwrap();
        assert!(matches!(cleanup.target, CleanupTarget::All));
        assert!(cleanup.keep.is_empty());

        // Test (cleanup sources)
        let input = r#"(package "test" "1.0" (cleanup sources))"#;
        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();
        let cleanup = recipe.cleanup.unwrap();
        assert!(matches!(cleanup.target, CleanupTarget::Sources));

        // Test (cleanup all (keep "cache" "logs"))
        let input = r#"(package "test" "1.0" (cleanup all (keep "cache" "logs")))"#;
        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();
        let cleanup = recipe.cleanup.unwrap();
        assert!(matches!(cleanup.target, CleanupTarget::All));
        assert_eq!(cleanup.keep, vec!["cache", "logs"]);
    }

    #[test]
    fn test_parse_acquire_with_verification() {
        let input = r#"
            (package "nginx" "1.25.0"
              (acquire
                (source "https://nginx.org/download/nginx-1.25.0.tar.gz"
                  (sha256 "abc123def456"))))
        "#;

        let expr = parse(input).unwrap();
        let recipe = Recipe::from_expr(&expr).unwrap();

        if let Some(AcquireSpec::Source { url, verify }) = recipe.acquire {
            assert_eq!(url, "https://nginx.org/download/nginx-1.25.0.tar.gz");
            assert!(matches!(verify, Some(Verify::Sha256(h)) if h == "abc123def456"));
        } else {
            panic!("expected Source acquire");
        }
    }
}
