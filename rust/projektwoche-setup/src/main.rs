mod config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command, Stdio};
use clap::{Parser, Subcommand};
use confy::ConfyError;
use os_info::get;

// Program and Package Manager logic
/**
 * how id do it in ts:
 *
 * ```typescript
 * // multiples of runnerArgs because for diferent OS and Arch
 * type runnerArgs = {
 *   os: OS[],
 *   arch: Arch[],
 *   commands: {
 *     descriptor: string,
 *     command: string[],
 *   };
 *
 * interface Program {
 *   name: string;
 *   commands: {
 *     install: runnerArgs[];
 *     uninstall: runnerArgs[];
 *   };
 *  configFunc?: (machine: Machine) => Promise<void>;
 * }
 * ```
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExecutableCommand {
    descriptor: &'static str,
    command: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InstallationInstructions {
  install: Vec<ExecutableCommand>,
  uninstall: Vec<ExecutableCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OSCommandMapping {
    commands: InstallationInstructions,
    config_func: Option<fn(&config::Machine) -> Result<(), Box<dyn std::error::Error>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    name: &'static str,
    description: &'static str,
    mapping: HashMap<config::OS, OSCommandMapping>,
}

impl Program {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            mapping: HashMap::new(),
        }
    }

    fn add_os(mut self, os: config::OS_Category) -> Self {
      let os_list = config::OS_Matcher::from_category(os);
        for os_type in os_list.get_list() {
            self.mapping.insert(*os_type, OSCommandMapping {
                commands: InstallationInstructions {
                    install: Vec::new(),
                    uninstall: Vec::new(),
                },
                config_func: None,
            });
        }
        self
    }

    // fn add_install_command(mut self, os: config::OS, commands: &'static [&'static str]) -> Self {
    //     self.mapping.insert(os, OSCommandMapping {
    //         commands: InstallationInstructions {
    //             install: commands.iter().map(|&cmd| ExecutableCommand {
    //                 descriptor: cmd,
    //                 command: cmd.split_whitespace().collect(),
    //             }).collect(),
    //             uninstall: Vec::new(),
    //         },
    //         config_func: None,
    //     });
    //     self
    // }

    // fn add_config_func(
    //     mut self,
    //     func: fn(&Machine) -> Result<(), Box<dyn std::error::Error>>,
    // ) -> Self {
    //     self.config_func = Some(func);
    //     self
    // }
}

// Installation function for all programs
fn install_programs(
    programs: &[&Program],
    config: &Config,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for program in programs {
        println!("==> Installing program: {}", program.name);

        if let Some(commands) = program.install_commands.get(&config.machine.kernel) {
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

                let mut child = ExecutableCommand::new(program_name)
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
                config.machine.kernel, program.name
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
                Kernel::Linux,
                &["sudo apt-get update", "sudo apt-get install -y nodejs"],
            )
            .add_install_command(Kernel::Mac, &["brew install node"]),
        &Program::new("VSCode")
            .add_install_command(
                Kernel::Linux,
                &[
                    "sudo apt-get update",
                    "sudo apt-get install -y wget apt-transport-https",
                    "wget -q https://packages.microsoft.com/keys/microsoft.asc -O- | sudo apt-key add -",
                    "sudo add-apt-repository 'deb [arch=amd64] https://packages.microsoft.com/repos/vscode stable main'",
                    "sudo apt-get update",
                    "sudo apt-get install -y code",
                ],
            )
            .add_install_command(Kernel::Mac, &["brew install visual-studio-code"])
            .add_config_func(configure_vscode),
        &Program::new("Bun")
            .add_install_command(Kernel::Linux, &["curl -fsSL https://bun.sh/install | bash"])
            .add_install_command(Kernel::Mac, &["curl -fsSL https://bun.sh/install | bash"]),
        &Program::new("npm")
            .add_install_command(Kernel::Linux, &["sudo apt-get update", "sudo apt-get install -y npm"])
            .add_install_command(Kernel::Mac, &["brew install npm"]),
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
