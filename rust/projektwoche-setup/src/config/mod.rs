//! # Configuration System
//!
//! This module handles system configuration management and machine detection.
//! It provides automatic detection of the operating system and architecture,
//! storing this information persistently for use during package installation.
//!
//! ## Features
//!
//! - **Automatic OS Detection**: Identifies the operating system and architecture
//! - **Persistent Configuration**: Stores configuration using the `confy` crate
//! - **Cross-Platform Support**: Works on Windows, macOS, and Linux distributions
//! - **Flexible OS Matching**: Supports matching by specific OS or OS categories
//!
//! ## Configuration Storage
//!
//! Configuration is stored in platform-specific locations:
//! - **Windows**: `%APPDATA%\prowo-setup\config.toml`
//! - **macOS**: `~/Library/Application Support/prowo-setup/config.toml`
//! - **Linux**: `~/.config/prowo-setup/config.toml`

pub mod interactive;
pub mod machine;

use crate::logger::LogLevel;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::process::exit;

/// Main configuration structure containing machine information.
///
/// This struct is automatically populated with detected system information
/// and persisted to disk for future use.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
  /// Machine-specific information (OS, architecture)
  #[serde(default)]
  pub(crate) machine: machine::Machine,
  /// Log level configuration
  #[serde(default = "default_log_level")]
  pub log_level: LogLevel,
}

fn default_log_level() -> LogLevel {
  LogLevel::Info
}

/// Loads or creates the application configuration.
///
/// This function attempts to load existing configuration from disk,
/// or creates a new configuration with detected system information.
///
/// # Returns
///
/// Returns the loaded/created configuration on success.
///
/// # Errors
///
/// If configuration loading fails, the function prints an error message
/// and exits the program with code 1.
pub fn use_config() -> Result<Config, Box<dyn std::error::Error>> {
  let config: Result<Config, ConfyError> = confy::load("prowo-setup", "config");
  match config {
    Ok(config) => Ok(config),
    Err(e) => {
      eprintln!("Unbekannter Fehler beim Laden der Konfiguration: {}", e);
      exit(1)
    }
  }
}

/// Saves the configuration to disk.
///
/// # Arguments
///
/// * `config` - The configuration to save
///
/// # Returns
///
/// Returns Ok(()) on success, or an error if the save operation fails.
pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  confy::store("prowo-setup", "config", config)?;
  Ok(())
}

// May implement a Lockfile system in the future when needing to expand to multiple bundles
