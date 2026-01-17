//! Development tasks for LevitateOS.
//!
//! Usage: cargo xtask <command>
//!
//! Note: Recipe VM testing has moved to the recipe submodule.
//! Use: cd recipe && cargo xtask vm <command>

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Development tasks for LevitateOS")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the LevitateOS initramfs
    Build {
        /// Output directory
        #[arg(short, long, default_value = "target/initramfs")]
        output: String,
    },

    /// Show help for recipe VM testing (moved to recipe submodule)
    RecipeVm,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { output } => {
            println!("Building initramfs to {}...", output);
            println!("TODO: Implement initramfs build");
            Ok(())
        }
        Commands::RecipeVm => {
            println!("Recipe VM testing has moved to the recipe submodule.\n");
            println!("Usage:");
            println!("  cd recipe");
            println!("  cargo xtask vm setup    # Download Arch cloud image");
            println!("  cargo xtask vm prepare  # Build recipe binary");
            println!("  cargo xtask vm start    # Start VM");
            println!("  cargo xtask vm copy     # Copy recipe to VM");
            println!("  cargo xtask vm ssh      # SSH into VM");
            println!("  cargo xtask vm stop     # Stop VM");
            Ok(())
        }
    }
}
