//! # Package Management System
//!
//! This module provides the core functionality for managing software packages and bundles.
//! It defines the architecture for installing, configuring, and uninstalling software
//! across different operating systems using a flexible instruction-based system.
//!
//! ## Architecture Overview
//!
//! The system is built around several key concepts:
//! - **Instructions**: Individual commands or operations (defined in [`instructions`])
//! - **Packages**: Individual software programs with OS-specific installation instructions
//! - **Bundles**: Collections of related packages that are installed together
//! - **Instruction Mappings**: OS-specific sets of instructions for different operations
//!
//! ## Threading Model
//!
//! The system uses multi-threading to install multiple packages concurrently within a bundle,
//! significantly reducing overall installation time. Each package is processed in its own thread.

pub mod instructions;

use crate::config;
use crate::manager::instructions::AnyInstruction;
use std::collections::HashMap;

/// A set of instructions for a specific operation (install/uninstall/configure).
/// 
/// This generic structure allows the same pattern to be used for different
/// types of operations while maintaining type safety.
/// 
/// # Type Parameters
/// 
/// * `T` - The type of instruction stored in this set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InstructionSet<T> {
  /// Instructions to execute for the primary operation
  install: Vec<T>,
  /// Instructions to execute for the reverse operation (currently unused)
  uninstall: Vec<T>,
}

impl<T> InstructionSet<T> {
  /// Creates a new empty instruction set.
  fn new() -> Self {
    Self {
      install: Vec::new(),
      uninstall: Vec::new(),
    }
  }
}

/// Type alias for instruction sets used during software installation.
type InstallationInstructions = InstructionSet<instructions::Instructions>;

/// Type alias for instruction sets used during software configuration.
type ConfigurationInstructions = InstructionSet<instructions::Instructions>;

/// Maps different types of operations to their corresponding instruction sets.
/// 
/// This structure organizes all the different operations that can be performed
/// on a package, allowing for fine-grained control over the installation process.
/// 
/// # Operations Supported
/// 
/// - **Prerequisites**: Check if software is already installed before proceeding
/// - **Installation**: Download and install the software
/// - **Uninstallation**: Remove the software from the system  
/// - **Configuration**: Apply settings and configurations after installation
/// - **Deconfiguration**: Revert configurations during uninstallation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstructionMapping {
  /// Instructions for checking if the software is already installed
  prerequisite_checks: Vec<instructions::Instructions>,
  /// Instructions for installing the software
  install_instructions: InstallationInstructions,
  /// Instructions for uninstalling the software
  uninstall_instructions: InstallationInstructions,
  /// Instructions for configuring the software after installation
  configuration_instructions: ConfigurationInstructions,
  /// Instructions for reverting configuration during uninstallation
  deconfiguration_instructions: ConfigurationInstructions,
}

impl InstructionMapping {
  /// Creates a new empty instruction mapping.
  /// 
  /// All instruction sets are initialized as empty and can be populated
  /// using the builder methods.
  pub(crate) fn new() -> Self {
    Self {
      prerequisite_checks: Vec::new(),
      install_instructions: InstallationInstructions::new(),
      uninstall_instructions: InstallationInstructions::new(),
      configuration_instructions: ConfigurationInstructions::new(),
      deconfiguration_instructions: ConfigurationInstructions::new(),
    }
  }

  /// Adds prerequisite check instructions to this mapping.
  /// 
  /// These instructions will be executed before installation to check if
  /// the software is already installed or if prerequisites are met.
  /// Only Assert instructions are accepted for prerequisite checks.
  /// 
  /// # Arguments
  /// 
  /// * `checks` - Vector of Assert instructions to check prerequisites
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_prerequisite_checks(
    mut self,
    checks: Vec<instructions::Instructions>,
  ) -> Self {
    // Validate that all instructions are Assert variants
    for check in &checks {
      match check {
        instructions::Instructions::Assert(_) => {
          // Valid - this is an Assert instruction
        }
        _ => {
          panic!("Only Assert instructions are allowed for prerequisite checks");
        }
      }
    }
    self.prerequisite_checks.extend(checks);
    self
  }

  /// Adds installation instructions to this mapping.
  /// 
  /// These instructions will be executed when the package is being installed.
  /// 
  /// # Arguments
  /// 
  /// * `instructions` - Vector of instructions to add to the installation process
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_install_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.install_instructions.install.extend(instructions);
    self
  }

  /// Adds uninstallation instructions to this mapping.
  /// 
  /// These instructions will be executed when the package is being removed.
  /// 
  /// # Arguments
  /// 
  /// * `instructions` - Vector of instructions to add to the uninstallation process
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_uninstall_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.uninstall_instructions.install.extend(instructions);
    self
  }

  /// Adds configuration instructions to this mapping.
  /// 
  /// These instructions will be executed after the package is installed
  /// to apply necessary configurations.
  /// 
  /// # Arguments
  /// 
  /// * `instructions` - Vector of instructions to add to the configuration process
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_configuration_instructions(
    mut self,
    instructions: Vec<instructions::Instructions>,
  ) -> Self {
    self.configuration_instructions.install.extend(instructions);
    self
  }

  /// Adds deconfiguration instructions to this mapping.
  /// 
  /// These instructions will be executed before the package is uninstalled
  /// to revert any configurations that were applied.
  /// 
  /// # Arguments
  /// 
  /// * `instructions` - Vector of instructions to add to the deconfiguration process
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
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

/// Represents a single software package with OS-specific installation instructions.
/// 
/// A package encapsulates all the information needed to install, configure, and
/// uninstall a specific piece of software across different operating systems.
/// 
/// # Example
/// 
/// ```rust
/// let node_package = Package::new("Node.js", "JavaScript runtime")
///   .add_mapping(
///     OsMatcher::from_category(OsCategory::Windows),
///     InstructionMapping::new()
///       .add_install_instructions(vec![
///         Instruction::new("Download Node.js").cmd("curl -o node.msi https://nodejs.org/dist/latest/node-x64.msi"),
///         Instruction::new("Install Node.js").cmd("msiexec /i node.msi /quiet"),
///       ])
///   );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
  /// Human-readable name of the package
  name: &'static str,
  /// Brief description of what the package provides
  description: &'static str,
  /// OS-specific instruction mappings for this package
  mapping: HashMap<config::machine::OS, InstructionMapping>,
}

impl Package {
  /// Creates a new package with the given name and description.
  /// 
  /// The package starts with no instruction mappings and must have
  /// mappings added using [`add_mapping`](Self::add_mapping).
  /// 
  /// # Arguments
  /// 
  /// * `name` - Display name for the package
  /// * `description` - Brief description of the package's purpose
  pub(crate) fn new(name: &'static str, description: &'static str) -> Self {
    Self {
      name,
      description,
      mapping: HashMap::new(),
    }
  }

  /// Adds an instruction mapping for specific operating systems.
  /// 
  /// This method associates a set of installation/configuration instructions
  /// with one or more operating systems using an OS matcher.
  /// 
  /// # Arguments
  /// 
  /// * `os` - An OS matcher that specifies which operating systems this mapping applies to
  /// * `mapping` - The instruction mapping containing install/uninstall/config instructions
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_mapping(
    mut self,
    os: config::machine::OsMatcher,
    mapping: InstructionMapping,
  ) -> Self {
    for os_type in os.get_list() {
      self.mapping.insert(*os_type, mapping.clone());
    }
    self
  }
}

/// A collection of related software packages that are installed together.
/// 
/// Software bundles provide a convenient way to install multiple related tools
/// as a single unit. For example, a "web development" bundle might include
/// Node.js, a package manager, and a code editor.
/// 
/// # Threading and Performance
/// 
/// Bundles use multi-threading to install packages concurrently, which significantly
/// reduces installation time compared to sequential installation. Each package
/// within a bundle is processed in its own thread.
/// 
/// # Installation Process
/// 
/// 1. **Installation Phase**: All packages are installed concurrently
/// 2. **Configuration Phase**: Packages are configured after installation
/// 
/// # Uninstallation Process
/// 
/// 1. **Deconfiguration Phase**: Package configurations are reverted
/// 2. **Uninstallation Phase**: Packages are removed from the system
pub struct SoftwareBundle {
  /// Human-readable name of the bundle
  name: &'static str,
  /// Description of what this bundle provides
  description: &'static str,
  /// List of packages included in this bundle
  programs: Vec<Package>,
}

impl SoftwareBundle {
  /// Creates a new empty software bundle.
  /// 
  /// # Arguments
  /// 
  /// * `name` - Display name for the bundle
  /// * `description` - Description of the bundle's purpose and contents
  pub(crate) fn new(name: &'static str, description: &'static str) -> Self {
    Self {
      name,
      description,
      programs: Vec::new(),
    }
  }

  /// Adds a package to this bundle.
  /// 
  /// Packages are installed in the order they are added, but within
  /// the same phase (installation/configuration) they run concurrently.
  /// 
  /// # Arguments
  /// 
  /// * `program` - The package to add to this bundle
  /// 
  /// # Returns
  /// 
  /// Returns `self` for method chaining.
  pub(crate) fn add_program(mut self, program: Package) -> Self {
    self.programs.push(program);
    self
  }

  fn installer_thread(program: &Package, os: &config::machine::OS, dry_run: bool) {
    println!("==> Installing program: {}", program.name); // i want multiple windows in the ui but i i just use println! the multithread will just stack over each other
    let commands = program
      .mapping
      .get(os)
      .expect(&format!("No installation commands found for OS: {:?}", os));

    // Check prerequisites first
    if !commands.prerequisite_checks.is_empty() {
      println!("  Checking prerequisites...");
      for check in &commands.prerequisite_checks {
        match check.run(dry_run) {
          Ok(_) => {
            println!("  Program already installed, skipping installation.");
            return;
          }
          Err(_) => {
            // Prerequisites not met, continue with installation
            println!("  Prerequisites not met, proceeding with installation.");
          }
        }
      }
    }

    for instruction in &commands.install_instructions.install {
      if let Err(e) = instruction.run(dry_run) {
        eprintln!("  Command failed: {}", e);
        return;
      }
      println!("  Command executed successfully.");
    }
  }

  fn installer(
    &self,
    os: &config::machine::OS,
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
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("==> Configuring program: {}", program.name);
    let commands = program
      .mapping
      .get(os)
      .expect(&format!("No configuration commands found for OS: {:?}", os));

    for instruction in &commands.configuration_instructions.install {
      if let Err(e) = instruction.run(dry_run) {
        eprintln!("  Configuration failed: {}", e);
        return Err(e);
      }
      println!("  Configuration applied successfully.");
    }
    Ok(())
  }

  fn configurator(
    &self,
    os: &config::machine::OS,
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
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("==> Installing bundle: {}", self.name);
    println!("Description: {}", self.description);
        self.installer(os, dry_run)?;
    self.configurator(os, dry_run)?;
    Ok(())
  }

  fn uninstaller_thread(
    program: &Package,
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("==> Uninstalling program: {}", program.name);
    let commands = program.mapping.get(os).expect(&format!(
      "No uninstallation commands found for OS: {:?}",
      os
    ));

    for instruction in &commands.uninstall_instructions.install {
      if let Err(e) = instruction.run(dry_run) {
        eprintln!("  Uninstallation failed: {}", e);
        return Err(e);
      }
      println!("  Uninstallation executed successfully.");
    }
    Ok(())
  }

  fn uninstaller(
    &self,
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = vec![];

    for program in &self.programs {
      if let Some(commands) = program.mapping.get(os) {
        if commands.uninstall_instructions.install.is_empty() {
          println!("No uninstallation functions for program: {}", program.name);
          continue;
        }

        let os = os.clone();
        let program = program.clone();
        let handle = std::thread::spawn(move || Self::uninstaller_thread(&program, &os, dry_run));
        handles.push(handle);
      } else {
        println!(
          "No uninstallation mapping found for program: {}",
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

  fn deconfigurator_thread(
    program: &Package,
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("==> Deconfiguring program: {}", program.name);
    let commands = program.mapping.get(os).expect(&format!(
      "No deconfiguration commands found for OS: {:?}",
      os
    ));

    for instruction in &commands.deconfiguration_instructions.install {
      if let Err(e) = instruction.run(dry_run) {
        eprintln!("  Deconfiguration failed: {}", e);
        return Err(e);
      }
      println!("  Deconfiguration applied successfully.");
    }
    Ok(())
  }

  fn deconfigurator(
    &self,
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = vec![];

    for program in &self.programs {
      if let Some(commands) = program.mapping.get(os) {
        if commands.deconfiguration_instructions.install.is_empty() {
          println!("No deconfiguration functions for program: {}", program.name);
          continue;
        }

        let os = os.clone();
        let program = program.clone();
        let handle =
          std::thread::spawn(move || Self::deconfigurator_thread(&program, &os, dry_run));
        handles.push(handle);
      } else {
        println!(
          "No deconfiguration mapping found for program: {}",
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

  pub(crate) fn uninstall(
    &self,
    os: &config::machine::OS,
    dry_run: bool,
  ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("==> Uninstalling bundle: {}", self.name);
    println!("Description: {}", self.description);
    self.uninstaller(os, dry_run)?;
    self.deconfigurator(os, dry_run)?;
    Ok(())
  }
}
