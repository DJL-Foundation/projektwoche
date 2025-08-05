mod bundles;
mod config;
mod manager;

use clap::{Parser, Subcommand};

// Define the CLI application and its subcommands using `clap`.
#[derive(Parser, Debug)]
#[clap(author, version, about = "A CLI for setting up developer environment", long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Install all specified programs
  Install {
    /// Dry run: show what would be installed without doing it
    #[clap(short, long)]
    debug: bool,
  },
  /// Uninstall programs (Not yet implemented)
  Uninstall,
  /// Update the CLI tool itself (Not yet implemented)
  SelfUpdate,
}

fn main() {
  let cli = Cli::parse();

  match config::use_config() {
    Ok(config) => {
      println!("Verwende Konfiguration: {:?}", config.machine);
      match &cli.command {
        Commands::Install { debug } => {
          if *debug {
            println!("==> INSTALLATION (DRY-RUN)");
          } else {
            println!("==> INSTALLATION");
          }

          if let Err(e) = bundles::projektwoche::bundle().install(&config.machine.os, *debug) {
            eprintln!("Fehler bei der Installation: {}", e);
          }
          println!("==> Installation abgeschlossen.");
        }
        Commands::Uninstall => {
          println!("==> UNINSTALL (noch nicht implementiert)");
        }
        Commands::SelfUpdate => {
          println!("==> SELF-UPDATE (noch nicht implementiert)");
        }
      }
    }
    Err(e) => {
      eprintln!("Fehler beim Laden/Erstellen der Konfiguration: {}", e);
    }
  }
}
