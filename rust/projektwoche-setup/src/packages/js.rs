//! # JavaScript Runtime Packages
//!
//! This module defines packages for JavaScript development tools and runtimes.
//! It provides cross-platform installation instructions for popular JavaScript
//! tools used in modern web development.
//!
//! ## Available Packages
//!
//! - **Node.js**: JavaScript runtime with npm package manager via nvm
//! - **Bun**: Fast JavaScript runtime and package manager
//!
//! ## Installation Strategy
//!
//! - **Node.js**: Installed via nvm (Node Version Manager) for better version control
//! - **Bun**: Installed directly using official installation scripts
//!
//! Both packages support Windows and Linux platforms with appropriate
//! installation methods for each operating system.

use crate::config::machine::{OsCategory, OsMatcher};
use crate::manager::instructions::Instruction;
use crate::manager::{InstructionMapping, Package};

/// Creates a Node.js package with cross-platform installation instructions.
/// 
/// Node.js is installed via nvm (Node Version Manager) to allow easy version
/// switching and management. The installation includes:
/// 
/// - **nvm installation**: Downloads and installs nvm using official scripts
/// - **Node.js installation**: Installs the latest Node.js version via nvm  
/// - **PATH configuration**: Adds Node.js binaries to the system PATH
/// - **Shell integration**: Configures shell startup files for persistent access
/// 
/// # Platform Support
/// 
/// - **Windows**: Uses nvm-windows with PowerShell scripts and environment variables
/// - **Linux**: Uses standard nvm with bash configuration and shell reloading
/// 
/// # Returns
/// 
/// Returns a configured [`Package`] with platform-specific installation instructions.
pub fn nodejs() -> Package {
  Package::new("Node.js", "JavaScript runtime").add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new()
      .add_prerequisite_checks(vec![
        Instruction::new("Check if Node.js is installed").assert("node --version", "v"),
      ])
      .add_install_instructions(vec![
        Instruction::new("Install Node.js").install_application("OpenJS.NodeJS"),
      ]),
  ).add_mapping(
    OsMatcher::from_category(OsCategory::LinuxBased),
    InstructionMapping::new()
      .add_prerequisite_checks(vec![
        Instruction::new("Check if Node.js is installed").assert("node --version", "v"),
      ])
      .add_install_instructions(vec![
        Instruction::new("Install nvm").cmd("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash"),
        Instruction::new("Source nvm and install Node.js").cmd("bash -c 'source ~/.bashrc && nvm install node && nvm use node && nvm alias default node'"),
      ]),
  )
}

/// Creates a Bun package with cross-platform installation instructions.
/// 
/// Bun is a fast JavaScript runtime and package manager that serves as an
/// alternative to Node.js and npm/yarn. The installation uses official
/// installation scripts provided by the Bun team.
/// 
/// # Features
/// 
/// - **Fast runtime**: Significantly faster than Node.js for many workloads
/// - **Built-in package manager**: No need for separate npm/yarn installation
/// - **TypeScript support**: Native TypeScript execution without compilation
/// - **Bundler included**: Built-in bundling and minification capabilities
/// 
/// # Platform Support
/// 
/// - **Windows**: Uses PowerShell installation script from bun.sh
/// - **Linux**: Uses bash installation script with curl
/// 
/// # Returns
/// 
/// Returns a configured [`Package`] with platform-specific installation instructions.
pub fn bun() -> Package {
  Package::new("Bun", "JavaScript runtime and package manager")
    .add_mapping(
      OsMatcher::from_category(OsCategory::Windows),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Bun is installed").assert("bun --version", "."),
        ])
        .add_install_instructions(vec![
          Instruction::new("Install Bun").install_application("Oven-sh.Bun"),
        ]),
    )
    .add_mapping(
      OsMatcher::from_category(OsCategory::LinuxBased),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Bun is installed").assert("bun --version", "."),
        ])
        .add_install_instructions(vec![
          Instruction::new("Install Bun").cmd("curl -fsSL https://bun.sh/install | bash"),
          Instruction::new("Source Bun environment").cmd("bash -c 'source ~/.bashrc'"),
        ]),
    )
}
