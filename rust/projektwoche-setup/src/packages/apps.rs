//! # Application Packages
//!
//! This module defines packages for common applications and tools used in
//! development and general computing workflows.
//!
//! ## Available Packages
//!
//! - **Google Chrome**: Popular web browser with developer tools
//! - **Git**: Distributed version control system
//!
//! ## Installation Strategy
//!
//! Different installation methods are used based on the operating system:
//! - **Windows**: Package manager installation using winget/chocolatey
//! - **Linux**: System package manager installation (apt/yum/dnf/etc.)
//!
//! Additional platform support can be added by extending the OS mappings.

use crate::config::machine::{OsCategory, OsMatcher};
use crate::manager::instructions::Instruction;
use crate::manager::{InstructionMapping, Package};

/// Creates a Google Chrome package with cross-platform installation instructions.
/// 
/// Google Chrome is a popular web browser developed by Google that provides:
/// 
/// - **Developer tools**: Built-in debugging and profiling tools for web development
/// - **Extensions ecosystem**: Large marketplace of browser extensions
/// - **Sync capabilities**: Cross-device synchronization of bookmarks and settings
/// - **Performance**: Fast JavaScript engine and rendering performance
/// - **Security**: Regular security updates and sandboxing features
/// 
/// # Platform Support
/// 
/// - **Windows**: Uses package managers (winget/chocolatey) for installation
/// - **Debian-based Linux**: Downloads and installs .deb package directly
/// - **RHEL-based Linux**: Downloads and installs .rpm package directly
/// 
/// # Installation Methods
/// 
/// - **Windows**: Uses [`install_application`] with Google.Chrome package ID
/// - **Debian**: Uses [`download_and_exec`] for .deb package installation
/// - **RHEL**: Uses [`download_and_exec`] for .rpm package installation
/// 
/// # Returns
/// 
/// Returns a configured [`Package`] with platform-specific installation instructions.
/// 
/// # Note
/// 
/// Additional Linux distributions can be supported by adding more OS mappings
/// with appropriate installation instructions for their package managers.
pub fn chrome() -> Package {
  Package::new("Google Chrome", "Web browser")
    .add_mapping(
      OsMatcher::from_category(OsCategory::Windows),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Chrome is installed").assert("chrome --version", "Google Chrome"),
        ])
        .add_install_instructions(vec![
          Instruction::new("Install Chrome").install_application("Google.Chrome"),
        ]),
    )
    .add_mapping(
      OsMatcher::from_category(OsCategory::DebianBased),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Chrome is installed").assert("google-chrome --version", "Google Chrome"),
        ])
        .add_install_instructions(vec![
          Instruction::new("Download Chrome").download_and_exec("https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb"),
        ]),
    )
    .add_mapping(
      OsMatcher::from_category(OsCategory::RHELBased),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Chrome is installed").assert("google-chrome --version", "Google Chrome"),
        ])
        .add_install_instructions(vec![
          Instruction::new("Download Chrome").download_and_exec("https://dl.google.com/linux/direct/google-chrome-stable_current_x86_64.rpm"),
        ]),
    )
}

/// Creates a Git package with cross-platform installation instructions.
/// 
/// Git is the most popular distributed version control system used for:
/// 
/// - **Source code management**: Track changes and collaborate on code
/// - **Branching and merging**: Advanced workflow management
/// - **Remote repositories**: Integration with GitHub, GitLab, and other services
/// - **Command line tools**: Full-featured CLI for all operations
/// - **GUI integration**: Works with various graphical Git clients
/// 
/// # Platform Support
/// 
/// - **Windows**: Uses package managers (winget/chocolatey) for installation
/// - **Linux**: Uses system package managers (apt, yum, dnf, etc.)
/// 
/// # Installation Methods
/// 
/// - **Windows**: Uses [`install_application`] with Microsoft.Git package ID
/// - **Linux**: Uses [`install_application`] with git package name
/// 
/// # Returns
/// 
/// Returns a configured [`Package`] with platform-specific installation instructions.
/// 
/// # Note
/// 
/// Git is essential for most development workflows and is often a prerequisite
/// for other development tools and package managers.
pub fn git() -> Package {
  Package::new("Git", "Version control system")
    .add_mapping(
      OsMatcher::from_category(OsCategory::Windows),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Git is installed").assert("git --version", "git version"),
        ])
        .add_install_instructions(vec![
          Instruction::new("Install Git").install_application("Microsoft.Git"),
        ]),
    )
    .add_mapping(
      OsMatcher::from_category(OsCategory::LinuxBased),
      InstructionMapping::new()
        .add_prerequisite_checks(vec![
          Instruction::new("Check if Git is installed").assert("git --version", "git version"),
        ])
        .add_install_instructions(vec![
          Instruction::new("Install Git").install_application("git"),
        ]),
    )
}