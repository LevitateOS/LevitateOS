//! Levitate - Package manager for LevitateOS
//!
//! Installs packages from S-expression recipes.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use levitate_recipe::{parse, Context as ExecContext, Executor, Recipe};
use std::fs;
use std::path::PathBuf;

/// Default recipe directory
const RECIPE_DIR: &str = "/usr/share/levitate/recipes";

#[derive(Parser)]
#[command(name = "levitate")]
#[command(about = "LevitateOS package manager")]
#[command(version)]
struct Cli {
    /// Recipe directory (default: /usr/share/levitate/recipes)
    #[arg(long, env = "LEVITATE_RECIPE_DIR")]
    recipe_dir: Option<PathBuf>,

    /// Installation prefix (default: /usr/local)
    #[arg(long, default_value = "/usr/local")]
    prefix: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Dry run - show commands without executing
    #[arg(short = 'n', long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        /// Package name or path to .recipe file
        package: String,
    },

    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },

    /// List available packages
    List,

    /// Show package info
    Info {
        /// Package name
        package: String,
    },

    /// Install the complete Sway desktop environment
    Desktop,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let recipe_dir = cli
        .recipe_dir
        .unwrap_or_else(|| PathBuf::from(RECIPE_DIR));

    match cli.command {
        Commands::Install { package } => {
            install_package(&package, &recipe_dir, &cli.prefix, cli.verbose, cli.dry_run)
        }
        Commands::Remove { package } => {
            remove_package(&package, &recipe_dir, &cli.prefix, cli.verbose, cli.dry_run)
        }
        Commands::List => list_packages(&recipe_dir),
        Commands::Info { package } => show_info(&package, &recipe_dir),
        Commands::Desktop => install_desktop(&recipe_dir, &cli.prefix, cli.verbose, cli.dry_run),
    }
}

/// Find a recipe by name or path
fn find_recipe(package: &str, recipe_dir: &PathBuf) -> Result<(PathBuf, String)> {
    // If it's a path to a .recipe file, use it directly
    if package.ends_with(".recipe") {
        let path = PathBuf::from(package);
        if path.exists() {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read recipe: {}", path.display()))?;
            return Ok((path, content));
        }
    }

    // Look in recipe directory
    let recipe_path = recipe_dir.join(format!("{}.recipe", package));
    if recipe_path.exists() {
        let content = fs::read_to_string(&recipe_path)
            .with_context(|| format!("Failed to read recipe: {}", recipe_path.display()))?;
        return Ok((recipe_path, content));
    }

    bail!(
        "Recipe not found: {}\nLooked in: {}",
        package,
        recipe_dir.display()
    )
}

/// Parse a recipe from content
fn parse_recipe(content: &str, path: &PathBuf) -> Result<Recipe> {
    let expr = parse(content)
        .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", path.display(), e))?;

    Recipe::from_expr(&expr)
        .map_err(|e| anyhow::anyhow!("Recipe error in {}: {}", path.display(), e))
}

/// Install a single package
fn install_package(
    package: &str,
    recipe_dir: &PathBuf,
    prefix: &PathBuf,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let (path, content) = find_recipe(package, recipe_dir)?;
    let recipe = parse_recipe(&content, &path)?;

    println!("Installing {} {}...", recipe.name, recipe.version);

    let ctx = ExecContext::with_prefix(prefix)
        .verbose(verbose)
        .dry_run(dry_run);

    let executor = Executor::new(ctx);

    executor
        .execute(&recipe)
        .map_err(|e| anyhow::anyhow!("Execution failed: {}", e))?;

    if !dry_run {
        println!("Installed {} {}", recipe.name, recipe.version);
    } else {
        println!("[dry-run] Would install {} {}", recipe.name, recipe.version);
    }

    Ok(())
}

/// Remove a package
fn remove_package(
    package: &str,
    recipe_dir: &PathBuf,
    prefix: &PathBuf,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let (path, content) = find_recipe(package, recipe_dir)?;
    let recipe = parse_recipe(&content, &path)?;

    println!("Removing {} {}...", recipe.name, recipe.version);

    let ctx = ExecContext::with_prefix(prefix)
        .verbose(verbose)
        .dry_run(dry_run);

    let executor = Executor::new(ctx);

    if let Some(ref remove_spec) = recipe.remove {
        executor
            .remove(remove_spec, &recipe)
            .map_err(|e| anyhow::anyhow!("Remove failed: {}", e))?;
    } else {
        bail!("Recipe {} does not have a remove section", recipe.name);
    }

    if !dry_run {
        println!("Removed {} {}", recipe.name, recipe.version);
    }

    Ok(())
}

/// List available packages
fn list_packages(recipe_dir: &PathBuf) -> Result<()> {
    if !recipe_dir.exists() {
        bail!("Recipe directory not found: {}", recipe_dir.display());
    }

    let mut packages = Vec::new();

    for entry in fs::read_dir(recipe_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map(|e| e == "recipe").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(expr) = parse(&content) {
                    if let Ok(recipe) = Recipe::from_expr(&expr) {
                        packages.push((recipe.name, recipe.version, recipe.description));
                    }
                }
            }
        }
    }

    packages.sort_by(|a, b| a.0.cmp(&b.0));

    if packages.is_empty() {
        println!("No recipes found in {}", recipe_dir.display());
    } else {
        println!("Available packages:\n");
        for (name, version, desc) in packages {
            let desc = desc.as_deref().unwrap_or("");
            println!("  {:<20} {:<10} {}", name, version, desc);
        }
    }

    Ok(())
}

/// Show package info
fn show_info(package: &str, recipe_dir: &PathBuf) -> Result<()> {
    let (path, content) = find_recipe(package, recipe_dir)?;
    let recipe = parse_recipe(&content, &path)?;

    println!("Package: {}", recipe.name);
    println!("Version: {}", recipe.version);

    if let Some(desc) = &recipe.description {
        println!("Description: {}", desc);
    }
    if !recipe.license.is_empty() {
        println!("License: {}", recipe.license.join(", "));
    }
    if let Some(homepage) = &recipe.homepage {
        println!("Homepage: {}", homepage);
    }

    println!("Recipe: {}", path.display());

    Ok(())
}

/// Install the complete Sway desktop
fn install_desktop(
    recipe_dir: &PathBuf,
    prefix: &PathBuf,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    // Desktop packages in dependency order
    let packages = [
        // Wayland core
        "wayland",
        "wayland-protocols",
        "libxkbcommon",
        "libinput",
        // Session
        "seatd",
        // Compositor
        "wlroots",
        // Sway
        "sway",
        "swaybg",
        "swaylock",
        "swayidle",
        // Desktop apps
        "gtk-layer-shell",
        "foot",
        "waybar",
        "wofi",
        "mako",
        // Utilities
        "grim",
        "slurp",
        "wl-clipboard",
    ];

    println!("Installing Sway desktop environment...\n");
    println!("Packages to install:");
    for pkg in &packages {
        println!("  - {}", pkg);
    }
    println!();

    let mut installed = 0;
    let mut skipped = 0;
    let mut failed = Vec::new();

    for package in &packages {
        match install_package(package, recipe_dir, prefix, verbose, dry_run) {
            Ok(_) => {
                installed += 1;
            }
            Err(e) => {
                eprintln!("Warning: Failed to install {}: {}", package, e);
                // Check if it's just missing recipe (might be a system dep)
                if e.to_string().contains("Recipe not found") {
                    eprintln!("  (might be a system dependency - skipping)");
                    skipped += 1;
                } else {
                    failed.push(package.to_string());
                }
            }
        }
    }

    println!();
    println!("Desktop installation complete:");
    println!("  Installed: {}", installed);
    println!("  Skipped (system deps): {}", skipped);
    println!("  Failed: {}", failed.len());

    if !failed.is_empty() {
        println!("\nFailed packages:");
        for pkg in &failed {
            println!("  - {}", pkg);
        }
    }

    if !dry_run {
        println!("\nTo start Sway, run: sway");
    }

    Ok(())
}
