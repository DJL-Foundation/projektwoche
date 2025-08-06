pub mod machine;

use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub(crate) machine: machine::Machine,
}

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

// May implement a Lockfile system in the future when needing to expand to multiple bundles
