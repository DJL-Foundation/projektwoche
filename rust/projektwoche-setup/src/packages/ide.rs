//! # IDE and Code Editor Packages
//!
//! This module provides packages for integrated development environments
//! and code editors commonly used in software development workflows.
//!
//! ## Available Packages
//!
//! - **Visual Studio Code**: Microsoft's popular code editor with extensive extension ecosystem
//!
//! ## Installation Strategy
//!
//! Different installation methods are used based on the operating system:
//! - **Windows**: Direct download and execution of installer packages
//! - **Linux (RHEL-based)**: Package manager installation using system repositories
//!
//! Additional platform support can be added by extending the OS mappings.

use crate::config::machine::{OsCategory, OsMatcher};
use crate::manager::instructions::Instruction;
use crate::manager::{InstructionMapping, Package};

/// Creates a Visual Studio Code package with cross-platform installation instructions.
/// 
/// Visual Studio Code is Microsoft's free, open-source code editor that has become
/// the most popular development environment. It features:
/// 
/// - **Rich extension ecosystem**: Thousands of extensions for different languages and tools
/// - **Integrated terminal**: Built-in terminal for running commands
/// - **Git integration**: Native Git support with visual diff tools
/// - **IntelliSense**: Advanced code completion and error detection
/// - **Debugging support**: Built-in debugger for multiple languages
/// 
/// # Platform Support
/// 
/// - **Windows**: Downloads and executes the official Windows installer
/// - **RHEL-based Linux**: Installs via system package manager (yum/dnf)
/// 
/// # Installation Methods
/// 
/// - **Windows**: Uses [`download_and_exec`] to download and run the official installer
/// - **Linux**: Uses [`install_package`] to install via the system package manager
/// 
/// # Returns
/// 
/// Returns a configured [`Package`] with platform-specific installation instructions.
/// 
/// # Note
/// 
/// Additional Linux distributions can be supported by adding more OS mappings
/// with appropriate installation instructions for their package managers.
pub fn vscode() -> Package {
  Package::new("Visual Studio Code", "Code editor")
  .add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new()
      .add_prerequisite_checks(vec![
        Instruction::new("Check if VSCode is installed").assert("code --version", "."),
      ])
       .add_install_instructions(vec![
         Instruction::new("Install VSCode").install_application("Microsoft.VisualStudioCode"),
       ]),
  )
  .add_mapping(
    OsMatcher::from_category(OsCategory::RHELBased),
    InstructionMapping::new()
      .add_prerequisite_checks(vec![
        Instruction::new("Check if VSCode is installed").assert("code --version", "."),
      ])
       .add_install_instructions(vec![
         Instruction::new("Install VSCode").install_application("code"),
       ]),
  )
  .add_mapping(
    OsMatcher::from_category(OsCategory::DebianBased),
    InstructionMapping::new()
      .add_prerequisite_checks(vec![
        Instruction::new("Check if VSCode is installed").assert("code --version", "."),
      ])
      .add_install_instructions(vec![
        Instruction::new("Install VSCode").install_application("code"),
      ]),
  )
}
