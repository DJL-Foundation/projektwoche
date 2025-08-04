mod config;

use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExecutableCommand {
    descriptor: &'static str,
    command: &'static [&'static str],
}

impl ExecutableCommand {
    fn new(descriptor: &'static str, command: &'static [&'static str]) -> Self {
        Self { descriptor, command }
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        for cmd in self.command {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
          continue;
            }
            let program_name = parts[0];
            let args = &parts[1..];

            let child = Command::new(program_name)
          .args(args)
          .stdout(Stdio::piped())
          .stderr(Stdio::piped())
          .spawn()?;

            let output = child.wait_with_output()?;

            if output.status.success() {
          println!("Command '{}' executed successfully.", cmd);
            } else {
          eprintln!(
              "Command '{}' failed with status: {:?}",
              cmd,
              output.status.code()
          );
          io::stderr().write_all(&output.stderr)?;
          return Err(format!("Command '{}' failed.", cmd).into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InstructionSet<T> {
    install: Vec<T>,
    uninstall: Vec<T>,
}

impl<T> InstructionSet<T> {
    fn new() -> Self {
        Self {
            install: Vec::new(),
            uninstall: Vec::new(),
        }
    }
}

type InstallationInstructions = InstructionSet<ExecutableCommand>;

type ConfigurationFunction = fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

type ConfigurationInstructions = InstructionSet<ConfigurationFunction>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OSCommandMapping {
    program: &'static str, // For "Installing Node.js..."
    commands: InstallationInstructions,
    config_func: ConfigurationInstructions,
}

impl OSCommandMapping {
   fn new(program: &Package) -> Self {
        Self {
            program: program.name,
            commands: InstallationInstructions::new(),
            config_func: ConfigurationInstructions::new(),
        }
    }

    fn add_install_commands(&mut self, commands: Vec<ExecutableCommand>) {
        self.commands.install.extend(commands);
    }

    fn add_uninstall_commands(&mut self, commands: Vec<ExecutableCommand>) {
        self.commands.uninstall.extend(commands);
    }

    fn add_config_creator(
        &mut self,
        config_func: ConfigurationFunction,
    ) {
        self.config_func.install.push(config_func);
    }

    fn add_config_remover(
        &mut self,
        config_func: ConfigurationFunction,
    ) {
        self.config_func.uninstall.push(config_func);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Package {
    name: &'static str,
    description: &'static str,
    mapping: HashMap<config::OS, OSCommandMapping>,
}

impl Package {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            mapping: HashMap::new(),
        }
    }

    fn add_mapping(
        mut self,
        os: config::OsMatcher,
        mapping: OSCommandMapping,
    ) -> Self {
      for os_type in os.get_list() {
          self.mapping.insert(*os_type, mapping.clone());
      }
      self
    }
}

struct SoftwareBundle {
    name: &'static str,
    description: &'static str,
    programs: Vec<Package>,
}

impl SoftwareBundle {
    fn new(name: &'static str, description: &'static str) -> Self {
        Self {
            name,
            description,
            programs: Vec::new(),
        }
    }

    fn add_program(mut self, program: Package) -> Self {
        self.programs.push(program);
        self
    }

    fn installer_thread(program: &Package, os: &config::OS, dry_run: bool) {
      println!("==> Installing program: {}", program.name); // i want multiple windows in the ui but i i just use println! the multithread will just stack over each other
      let commands = program.mapping.get(os).expect(&format!("No installation commands found for OS: {:?}", os));

      for instruction in &commands.commands.install {
        println!("  Executing: {}", instruction.command.join(" "));
        if dry_run {
            println!("  [DRY-RUN] Would execute: {}", instruction.command.join(" "));
            continue;
        }

        if let Err(e) = instruction.run() {
            eprintln!("  Command failed: {}", e);
            return;
        }
        println!("  Command executed successfully.");
      }
    }

    fn installer(&self, os: &config::OS, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
      let mut handles = vec![];

      for program in &self.programs {
          let os = os.clone();
          let program = program.clone();
          let handle = std::thread::spawn(move || {
        Self::installer_thread(&program, &os, dry_run);
          });
          handles.push(handle);
      }

      for handle in handles {
          if let Err(e) = handle.join() {
        eprintln!("Thread panicked: {:?}", e);
          }
      }

      Ok(())
    }

    fn configurator_thread(
        program: &Package,
        os: &config::OS,
        dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("==> Configuring program: {}", program.name);
        let commands = program.mapping.get(os).expect(&format!("No configuration commands found for OS: {:?}", os));

        for instruction in &commands.config_func.install {
            if dry_run {
                println!("  [DRY-RUN] Would apply configuration for {}", program.name);
                continue;
            }
            if let Err(e) = instruction() {
                eprintln!("  Configuration failed: {}", e);
                return Err(e);
            }
            println!("  Configuration applied successfully.");
        }
        Ok(())
    }

    fn configurator(
      &self,
      os: &config::OS,
      dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
      let mut handles = vec![];

      for program in &self.programs {
        if let Some(commands) = program.mapping.get(os) {
          if commands.config_func.install.is_empty() {
            println!("No configuration functions for program: {}", program.name);
            continue;
          }

          let os = os.clone();
          let program = program.clone();
          let handle = std::thread::spawn(move || {
            Self::configurator_thread(&program, &os, dry_run)
          });
          handles.push(handle);
        } else {
          println!("No configuration mapping found for program: {}", program.name);
        }
      }

      for handle in handles {
        if let Err(e) = handle.join() {
          eprintln!("Thread panicked: {:?}", e);
        }
      }

      Ok(())
    }

    fn install(
        &self,
        os: &config::OS,
        dry_run: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.installer(os, dry_run)?;
        self.configurator(os, dry_run)?;
        Ok(())
    }
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

    let projektwochen_bundle = SoftwareBundle::new("Projektwochen", "A bundle for the Projektwochen");

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

                    if let Err(e) = projektwochen_bundle.install(&config.machine.os, *debug) {
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