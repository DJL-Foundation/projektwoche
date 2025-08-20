//! # Projektwoche Setup CLI
//!
//! A fast and efficient command-line tool for setting up development environments
//! using customizable software bundles. This tool provides a simple interface for
//! installing, configuring, and managing development tools across different platforms.
//!
//! ## Architecture
//!
//! The CLI is structured around the following modules:
//! - [`bundles`] - Defines software bundles (collections of related packages)
//! - [`config`] - Handles system configuration and OS detection
//! - [`manager`] - Core package management and installation logic
//! - [`packages`] - Individual software package definitions

mod bundles;
mod config;
mod manager;
mod packages;

mod logger;

use clap::{Parser, Subcommand};
use logger::{LogLevel, LoggerSystem, ConsoleOutput, LevelFilter};

/// Main CLI application structure that defines the command-line interface
/// using the `clap` derive macros for automatic argument parsing.
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

/// Available CLI commands that users can execute.
///
/// Each command supports different operations on software bundles,
/// with optional dry-run functionality for safe testing.
#[derive(Subcommand, Debug)]
enum Commands {
  /// Install a Software Bundle
  ///
  /// Downloads and installs all packages contained within the specified bundle.
  /// This includes both the software installation and any necessary configuration.
  #[clap(
    visible_alias = "i",
    long_about = "Install a Software Bundle containing various packages for a specific use case. \nIf you expect to use a bundle but dont find it here, please run `projektwoche-setup self-update` to update the CLI tool itself."
  )]
  Install {
    /// Which Bundle to install
    package: Bundles,

    /// Dry run: show what would be installed without doing it
    ///
    /// When enabled, this will display all installation steps that would
    /// be executed without actually making any changes to the system.
    #[clap(short, long)]
    debug: bool,
  },

  /// Uninstall a Software Bundle
  ///
  /// Removes all packages contained within the specified bundle and
  /// reverts any configuration changes that were made during installation.
  #[clap(
    visible_alias = "u",
    long_about = "Uninstall a Software Bundle that was previously installed. \nIf you expect to uninstall a bundle but dont find it here, please run `projektwoche-setup self-update` to update the CLI tool itself."
  )]
  Uninstall {
    /// Which Bundle to uninstall
    package: Bundles,

    /// Dry run: show what would be uninstalled without doing it
    ///
    /// When enabled, this will display all uninstallation steps that would
    /// be executed without actually making any changes to the system.
    #[clap(short, long)]
    debug: bool,
  },

  /// Update the CLI tool itself
  ///
  /// Downloads and installs the latest version of the projektwoche-setup tool.
  /// This ensures you have access to the latest bundles and features.
  ///
  /// **Note:** This feature is not yet implemented.
  SelfUpdate,

  /// Configure the CLI tool interactively
  ///
  /// Opens an interactive configuration wizard that allows you to customize
  /// the CLI tool's behavior, set preferences, and configure installation options.
  // #[clap(
  //   long_about = "Interactive configuration wizard for customizing CLI behavior, setting user preferences, and configuring installation options."
  // )]
  // Configure,

  /// Manage configuration settings
  ///
  /// Allows you to view and modify various configuration settings such as log levels.
  #[clap(visible_alias = "cfg")]
  Config {
    #[clap(subcommand)]
    action: ConfigAction,
  },
}

/// Configuration management commands.
#[derive(Subcommand, Debug)]
enum ConfigAction {
  /// Manage log level settings
  Loglevel {
    #[clap(subcommand)]
    action: LogLevelAction,
  },
}

/// Log level management commands.
#[derive(Subcommand, Debug)]
enum LogLevelAction {
  /// Show current log level
  Default,
  /// Set log level
  Set {
    /// Log level to set
    level: LogLevel,
  },
}

/// Available software bundles that can be installed or uninstalled.
///
/// Each bundle represents a collection of related software packages
/// designed for specific development scenarios or workflows.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
enum Bundles {
  /// Complete development environment for the Projektwoche project
  ///
  /// This bundle includes:
  /// - Node.js (JavaScript runtime via nvm)
  /// - Bun (Fast JavaScript runtime and package manager)  
  /// - Visual Studio Code (Modern code editor)
  ///
  /// Designed specifically for web development workflows used in
  /// the Athenaeum Stade Projektwoche.
  #[default]
  Projektwoche,
}

/// Application entry point that orchestrates the CLI workflow.
///
/// This function:
/// 1. Parses command-line arguments using clap
/// 2. Loads or creates system configuration
/// 3. Executes the requested command (install/uninstall/self-update)
/// 4. Handles errors and provides user feedback
///
/// # Error Handling
///
/// Configuration errors are printed to stderr and cause the program to exit.
/// Installation/uninstallation errors are caught and displayed with context.
fn main() {
  let cli = Cli::parse();

  // Initialize logger system for configuration errors
  let (logger_system, mut collector) = LoggerSystem::new();
  collector.add_output(Box::new(ConsoleOutput::new(true)));
  collector.add_filter(Box::new(LevelFilter::new(LogLevel::Info)));
  
  let (logger_system, collector_handle) = logger_system.start_collector(collector);
  let main_logger = logger_system.create_logger("main", "main".to_string());

  match config::use_config() {
    Ok(config) => {
      main_logger.debug(format!("Verwende Konfiguration: {:?}", config.machine));
      match &cli.command {
        Commands::Install { debug, package } => {
          // Map the selected bundle enum to its implementation
          let mut bundle = match *package {
            Bundles::Projektwoche => bundles::projektwoche::bundle(),
          };

          // Display installation mode to user
          if *debug {
            main_logger.info("==> INSTALLATION (DRY-RUN)");
          } else {
            main_logger.info("==> INSTALLATION");
          }

          // Execute bundle installation with error handling
          if let Err(e) = bundle.install(&config.machine.os, *debug, &logger_system) {
            main_logger.error(format!("Fehler bei der Installation: {}", e));
          }
          main_logger.info("==> Installation abgeschlossen.");
        }
        Commands::Uninstall { debug, package } => {
          // Map the selected bundle enum to its implementation
          let mut bundle = match *package {
            Bundles::Projektwoche => bundles::projektwoche::bundle(),
          };

          // Display uninstallation mode to user
          if *debug {
            main_logger.info("==> DEINSTALLATION (DRY-RUN)");
          } else {
            main_logger.info("==> DEINSTALLATION");
          }

          // Execute bundle uninstallation with error handling
          if let Err(e) = bundle.uninstall(&config.machine.os, *debug, &logger_system) {
            main_logger.error(format!("Fehler bei der Deinstallation: {}", e));
          }
          main_logger.info("==> Deinstallation abgeschlossen.");
        }
        Commands::SelfUpdate => {
          main_logger.info("==> SELF-UPDATE (noch nicht implementiert)");
          // TODO: Implement self-update functionality
          // This should download and install the latest version of the CLI tool
        }
        // Commands::Configure => {
        //   main_logger.info("==> CONFIGURATION WIZARD");
        //   if let Some(config) = config::interactive::configuration_wizard() {
        //     main_logger.info(format!("Configuration saved: {:?}", config));
        //     main_logger.info("Configuration saved successfully.");
        //   } else {
        //     main_logger.info("Configuration cancelled by user.");
        //   }
        //   main_logger.info("==> Konfiguration abgeschlossen.");
        // }
        Commands::Config { action } => {
          match action {
            ConfigAction::Loglevel { action } => {
              match action {
                LogLevelAction::Default => {
                  main_logger.info(format!("Current log level: {:?}", config.log_level));
                }
                LogLevelAction::Set { level } => {
                  let mut new_config = config.clone();
                  new_config.log_level = level.clone();
                  
                  match config::save_config(&new_config) {
                    Ok(()) => {
                      main_logger.info(format!("Log level set to: {:?}", level));
                    }
                    Err(e) => {
                      main_logger.error(format!("Failed to save configuration: {}", e));
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(e) => {
      main_logger.critical(format!("Fehler beim Laden/Erstellen der Konfiguration: {}", e));
    }
  }
  
  // Properly shutdown the logger system
  logger_system.shutdown();
  let _ = collector_handle.join();
}
