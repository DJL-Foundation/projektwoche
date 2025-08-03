use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OS {
    Linux,
    Windows,
    Mac,
}

impl Default for OS {
    fn default() -> Self {
        OS::Linux
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Arch {
    X86,
    X86_64,
    Arm64,
}

impl Default for Arch {
    fn default() -> Self {
        Arch::X86_64
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Machine {
    #[serde(default)]
    os: OS,
    #[serde(default)]
    arch: Arch,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    machine: Machine,
}

// Scan which OS and arch
fn detect_system() -> Machine {
    let os = detect_os();
    let arch = detect_arch();
    
    Machine { os, arch }
}

fn detect_os() -> OS {
    if cfg!(target_os = "linux") {
        OS::Linux
    } else if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "macos") {
        OS::Mac
    } else {
        // Default fallback
        OS::Linux
    }
}

fn detect_arch() -> Arch {
    match env::consts::ARCH {
        "x86" => Arch::X86,
        "x86_64" => Arch::X86_64,
        "aarch64" => Arch::Arm64,
        _ => Arch::X86_64, // Default fallback
    }
}

fn create_config_with_detected_system() -> Config {
    let detected_machine = detect_system();
    Config {
        machine: detected_machine,
    }
}

fn check_get_create_config() -> Result<Config, Box<dyn std::error::Error>> {
    match confy::load("prowo-setup", "config") {
        Ok(config) => {
            println!("Config loaded successfully");
            Ok(config)
        }
        Err(e) => {
            println!("Config not found or invalid ({}), creating default config with detected system", e);
            let default_config = create_config_with_detected_system();
            confy::store("prowo-setup", "config", &default_config)?;
            println!("Default config created and saved with detected system: {:?}", default_config.machine);
            Ok(default_config)
        }
    }
}

fn main() {
    // Demonstrate system detection
    let detected = detect_system();
    println!("Detected system: OS = {:?}, Arch = {:?}", detected.os, detected.arch);
    
    // Load or create config
    match check_get_create_config() {
        Ok(config) => {
            println!("Using config: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error with config: {}", e);
        }
    }
}