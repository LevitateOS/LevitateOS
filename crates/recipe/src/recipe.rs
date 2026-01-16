//! Recipe interpretation - converts parsed S-expressions into structured data.

use crate::ast::Expr;
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
    pub deps: Vec<String>,
    pub build_deps: Vec<String>,
    pub acquire: Option<AcquireSpec>,
    pub build: Option<BuildSpec>,
    pub install: Option<InstallSpec>,
    pub configure: Option<ConfigureSpec>,
    pub start: Option<StartSpec>,
    pub stop: Option<StopSpec>,
    pub remove: Option<RemoveSpec>,
    pub cleanup: Option<CleanupSpec>,
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
            acquire: None,
            build: None,
            install: None,
            configure: None,
            start: None,
            stop: None,
            remove: None,
            cleanup: None,
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
                if let Some(tail) = expr.tail() {
                    self.deps = tail.iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
            }
            "build-deps" => {
                if let Some(tail) = expr.tail() {
                    self.build_deps = tail.iter()
                        .filter_map(|e| e.as_atom().map(|s| s.to_string()))
                        .collect();
                }
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
            "update" | "hooks" => {
                // TODO: implement these
            }
            _ => {
                return Err(RecipeError::UnknownAction(head.to_string()));
            }
        }

        Ok(())
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
                    let url = item.tail()
                        .and_then(|t| t.first())
                        .and_then(|e| e.as_atom())
                        .unwrap_or("")
                        .to_string();
                    return Ok(Some(AcquireSpec::Source { url, verify: None }));
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

    fn parse_configure(_expr: &Expr) -> Result<Option<ConfigureSpec>, RecipeError> {
        // TODO: implement
        Ok(None)
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

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
}
