use aether::setup;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aether-setup", version, about = "Install and manage Aether")]
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
    /// Downloads the latest release and updates Aether
    Update,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install => {
            if let Err(e) = setup::install() {
                eprintln!("\nInstallation failed:\n{e}");
                std::process::exit(1);
            }
        }
        Commands::Uninstall => {}
        Commands::Doctor => {
            if let Err(e) = setup::doctor() {
                eprintln!("\nDoctor failed:\n{e}");
                std::process::exit(1);
            }
        }
        Commands::Update => {
            if let Err(e) = setup::update() {
                eprintln!("\nUpdate failed:\n{e}");
                std::process::exit(1);
            }
        }
    }
}
