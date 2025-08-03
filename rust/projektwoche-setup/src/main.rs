use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::thread;
use std::time::Duration;
use clap::{Parser, Subcommand};
use confy::ConfyError;

// A simple utility to get the user's home directory.
fn get_home_dir() -> PathBuf {
    dirs::home_dir().expect("Home directory not found")
}

// Structs for system and configuration management.
// `confy` crate automatically handles loading and saving this struct.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OS {
    Linux,
    Windows,
    Mac,
}

impl Default for OS {
    fn default() -> Self {
        detect_os()
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
        detect_arch()
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

// Scan the OS and architecture of the system.
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
        OS::Linux // Default fallback
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

fn check_get_create_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config: Result<Config, ConfyError> = confy::load("prowo-setup", "config")
    match config {
        Ok(config) => Ok(config),
        Err(e) => {
            eprintln!("Unbekannter Fehler beim Laden der Konfiguration: {}", e)
            exit(1)
        }
    }
}

// Program and Package Manager logic
#[derive(Debug)]
struct Program {
    name: &'static str,
    install_commands: HashMap<OS, &'static [&'static str]>,
    config_func: Option<fn(&Machine) -> Result<(), Box<dyn std::error::Error>>>,
}

impl Program {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            install_commands: HashMap::new(),
            config_func: None,
        }
    }

    fn add_install_command(mut self, os: OS, commands: &'static [&'static str]) -> Self {
        self.install_commands.insert(os, commands);
        self
    }

    fn add_config_func(
        mut self,
        func: fn(&Machine) -> Result<(), Box<dyn std::error::Error>>,
    ) -> Self {
        self.config_func = Some(func);
        self
    }
}

// Installation function for all programs
fn install_programs(
    programs: &[&Program],
    config: &Config,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for program in programs {
        println!("==> Installing program: {}", program.name);

        if let Some(commands) = program.install_commands.get(&config.machine.os) {
            for cmd_str in *commands {
                if dry_run {
                    println!("  [DRY-RUN] Would execute: {}", cmd_str);
                    continue;
                }

                println!("  Executing: {}", cmd_str);
                let parts: Vec<&str> = cmd_str.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                let program_name = parts[0];
                let args = &parts[1..];

                let mut child = Command::new(program_name)
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()?;

                let output = child.wait_with_output()?;

                if output.status.success() {
                    println!("  Command successful.");
                } else {
                    eprintln!("  Command failed with status: {:?}", output.status.code());
                    io::stderr().write_all(&output.stderr)?;
                }
            }
        } else {
            println!(
                "  No installation commands found for OS: {:?} for program {}",
                config.machine.os, program.name
            );
        }

        if let Some(config_func) = program.config_func {
            println!("  Applying configuration for {}", program.name);
            if dry_run {
                println!("  [DRY-RUN] Would apply configuration.");
            } else {
                config_func(&config.machine)?;
            }
        }
    }
    Ok(())
}

// Example configuration function for VSCode.
fn configure_vscode(_: &Machine) -> Result<(), Box<dyn std::error::Error>> {
    let settings_dir = get_home_dir().join(".config/Code/User"); // Linux path
    // You can add more logic here to check the OS and adapt the path.
    // e.g., on Windows: `%APPDATA%\Code\User` or on macOS: `~/Library/Application Support/Code/User`
    let settings_file = settings_dir.join("settings.json");

    if !settings_dir.exists() {
        fs::create_dir_all(&settings_dir)?;
    }

    let vscode_settings = r#"{
    "workbench.colorTheme": "Default Dark+",
    "editor.fontSize": 14,
    "editor.renderWhitespace": "all",
    "terminal.integrated.shell.linux": "/bin/bash"
}"#;

    fs::write(&settings_file, vscode_settings)?;
    println!("    VSCode configuration file updated at: {:?}", settings_file);
    Ok(())
}

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

    // Define the programs to be managed. This is where you add new programs.
    let programs_to_manage = [
        &Program::new("Node")
            .add_install_command(
                OS::Linux,
                &["sudo apt-get update", "sudo apt-get install -y nodejs"],
            )
            .add_install_command(OS::Mac, &["brew install node"]),
        &Program::new("VSCode")
            .add_install_command(
                OS::Linux,
                &[
                    "sudo apt-get update",
                    "sudo apt-get install -y wget apt-transport-https",
                    "wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -",
                    "sudo add-apt-repository 'deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main'",
                    "sudo apt-get update",
                    "sudo apt-get install -y code",
                ],
            )
            .add_install_command(OS::Mac, &["brew install visual-studio-code"])
            .add_config_func(configure_vscode),
        &Program::new("Bun")
            .add_install_command(OS::Linux, &["curl -fsSL https://bun.sh/install | bash"])
            .add_install_command(OS::Mac, &["curl -fsSL https://bun.sh/install | bash"]),
        &Program::new("npm")
            .add_install_command(OS::Linux, &["sudo apt-get update", "sudo apt-get install -y npm"])
            .add_install_command(OS::Mac, &["brew install npm"]),
    ];

    match check_get_create_config() {
        Ok(config) => {
            println!("Verwende Konfiguration: {:?}", config.machine);
            match &cli.command {
                Commands::Install { debug } => {
                    if *debug {
                        println!("==> INSTALLATION (DRY-RUN)");
                    } else {
                        println!("==> INSTALLATION");
                    }

                    if let Err(e) = install_programs(&programs_to_manage, &config, *debug) {
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
