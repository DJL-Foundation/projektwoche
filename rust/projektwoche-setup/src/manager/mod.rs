pub mod instructions;

use crate::config;
use crate::manager::instructions::AnyInstruction;
use std::collections::HashMap;

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

type InstallationInstructions = InstructionSet<instructions::Instructions>;

type ConfigurationInstructions = InstructionSet<instructions::Instructions>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstructionMapping {
  install_instructions: InstallationInstructions,
  uninstall_instructions: InstallationInstructions,
  configuration_instructions: ConfigurationInstructions,
  deconfiguration_instructions: ConfigurationInstructions,
}

impl InstructionMapping {
  pub(crate) fn new() -> Self {
    Self {
      install_instructions: InstallationInstructions::new(),
      uninstall_instructions: InstallationInstructions::new(),
      configuration_instructions: ConfigurationInstructions::new(),
      deconfiguration_instructions: ConfigurationInstructions::new(),
    }
  }

  pub(crate) fn add_install_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.install_instructions.install.extend(instructions);
    self
  }

  pub(crate) fn add_uninstall_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.uninstall_instructions.install.extend(instructions);
    self
  }

  pub(crate) fn add_configuration_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.configuration_instructions.install.extend(instructions);
    self
  }

  pub(crate) fn add_deconfiguration_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self
      .deconfiguration_instructions
      .install
      .extend(instructions);
    self
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
  name: &'static str,
  description: &'static str,
  mapping: HashMap<config::OS, InstructionMapping>,
}

impl Package {
  pub(crate) fn new(name: &'static str, description: &'static str) -> Self {
    Self {
      name,
      description,
      mapping: HashMap::new(),
    }
  }

  pub(crate) fn add_mapping(mut self, os: config::OsMatcher, mapping: InstructionMapping) -> Self {
    for os_type in os.get_list() {
      self.mapping.insert(*os_type, mapping.clone());
    }
    self
  }
}

pub struct SoftwareBundle {
  name: &'static str,
  description: &'static str,
  programs: Vec<Package>,
}

impl SoftwareBundle {
  pub(crate) fn new(name: &'static str, description: &'static str) -> Self {
    Self {
      name,
      description,
      programs: Vec::new(),
    }
  }

  pub(crate) fn add_program(mut self, program: Package) -> Self {
    self.programs.push(program);
    self
  }

  fn installer_thread(program: &Package, os: &config::OS, dry_run: bool) {
    println!("==> Installing program: {}", program.name); // i want multiple windows in the ui but i i just use println! the multithread will just stack over each other
    let commands = program
      .mapping
      .get(os)
      .expect(&format!("No installation commands found for OS: {:?}", os));

    for instruction in &commands.install_instructions.install {
      if let Err(e) = instruction.run() {
        eprintln!("  Command failed: {}", e);
        return;
      }
      println!("  Command executed successfully.");
    }
  }

  fn installer(
    &self,
    os: &config::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let commands = program
      .mapping
      .get(os)
      .expect(&format!("No configuration commands found for OS: {:?}", os));

    for instruction in &commands.configuration_instructions.install {
      if let Err(e) = instruction.run() {
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
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = vec![];

    for program in &self.programs {
      if let Some(commands) = program.mapping.get(os) {
        if commands.configuration_instructions.install.is_empty() {
          println!("No configuration functions for program: {}", program.name);
          continue;
        }

        let os = os.clone();
        let program = program.clone();
        let handle = std::thread::spawn(move || Self::configurator_thread(&program, &os, dry_run));
        handles.push(handle);
      } else {
        println!(
          "No configuration mapping found for program: {}",
          program.name
        );
      }
    }

    for handle in handles {
      if let Err(e) = handle.join() {
        eprintln!("Thread panicked: {:?}", e);
      }
    }

    Ok(())
  }

  pub(crate) fn install(
    &self,
    os: &config::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    self.installer(os, dry_run)?;
    self.configurator(os, dry_run)?;
    Ok(())
  }
}
