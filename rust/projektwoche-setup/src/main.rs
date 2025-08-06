mod bundles;
mod config;
mod manager;
mod packages;

use clap::{Parser, Subcommand};

// Define the CLI application and its subcommands using `clap`.
#[derive(Parser, Debug)]
#[clap(
  author = "JackatDJL",
  version,
  about = "A CLI for setting up an environment fast",
  long_about = "This CLI tool is a custom package (bundle) manager used to set up development environments fast. \nIt allows users to install, uninstall, and update software packages easily."
)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Install a Bundle
  #[clap(
    visible_alias = "i",
    long_about = "Install a Software Bundle containing various packages for a specific use case. \nIf you expect to use a bundle but dont find it here, please run `projektwoche-setup self-update` to update the CLI tool itself."
  )]
  Install {
    /// Which Bundle to install
    package: Bundles,

    /// Dry run: show what would be installed without doing it
    #[clap(short, long)]
    debug: bool,
  },
  /// Uninstall a Bundle
  #[clap(
    visible_alias = "u",
    long_about = "Uninstall a Software Bundle that was previously installed. \nIf you expect to uninstall a bundle but dont find it here, please run `projektwoche-setup self-update` to update the CLI tool itself."
  )]
  Uninstall {
    /// Which Bundle to uninstall
    package: Bundles,

    /// Dry run: show what would be uninstalled without doing it
    #[clap(short, long)]
    debug: bool,
  },
  /// Update the CLI tool itself (Not yet implemented)
  SelfUpdate,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
enum Bundles {
  /// Projektwoche Bundle including Node, Bun, VSCode...
  #[default]
  Projektwoche,
}

fn main() {
  let cli = Cli::parse();

  match config::use_config() {
    Ok(config) => {
      println!("Verwende Konfiguration: {:?}", config.machine);
      match &cli.command {
        Commands::Install { debug, package } => {
          let bundle = match *package {
            Bundles::Projektwoche => bundles::projektwoche::bundle(),
          };
          if *debug {
            println!("==> INSTALLATION (DRY-RUN)");
          } else {
            println!("==> INSTALLATION");
          }

          if let Err(e) = bundle.install(&config.machine.os, *debug) {
            eprintln!("Fehler bei der Installation: {}", e);
          }
          println!("==> Installation abgeschlossen.");
        }
        Commands::Uninstall { debug, package } => {
          let bundle = match *package {
            Bundles::Projektwoche => bundles::projektwoche::bundle(),
          };
          if *debug {
            println!("==> DEINSTALLATION (DRY-RUN)");
          } else {
            println!("==> DEINSTALLATION");
          }

          if let Err(e) = bundle.uninstall(&config.machine.os, *debug) {
            eprintln!("Fehler bei der Deinstallation: {}", e);
          }
          println!("==> Deinstallation abgeschlossen.");
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
