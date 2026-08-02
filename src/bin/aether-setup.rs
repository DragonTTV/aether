use aether::installer;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "aether-setup",
    version,
    about = "Install and manage Aether"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Aether
    Install,

    /// Uninstall Aether
    Uninstall,

    /// Check installation health
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install => {
            if let Err(e) = installer::install() {
                eprintln!("\nInstallation failed:\n{e}");
                std::process::exit(1);
            }
        }
        Commands::Uninstall => {}
        Commands::Doctor => {}
    }
}