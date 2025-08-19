//! # Projektwoche Bundle
//!
//! This module defines the Projektwoche bundle, a complete web development
//! environment specifically designed for the Athenaeum Stade Projektwoche
//! educational program.
//!
//! ## Bundle Contents
//!
//! The Projektwoche bundle includes the essential tools for modern web development:
//!
//! - **Node.js**: JavaScript runtime environment via nvm for version management
//! - **Bun**: Fast JavaScript runtime and package manager for improved performance
//! - **Visual Studio Code**: Feature-rich code editor with extensive extension support
//!
//! ## Educational Context
//!
//! This bundle is specifically tailored for:
//! - **Student environments**: Easy setup for educational computer labs
//! - **Web development courses**: Complete toolchain for JavaScript/TypeScript development
//! - **Project-based learning**: All tools needed for modern web applications
//! - **Cross-platform compatibility**: Works on both Windows and Linux lab machines
//!
//! ## Installation Strategy
//!
//! The bundle uses parallel installation to minimize setup time:
//! 1. All packages are installed concurrently using multi-threading
//! 2. Configuration is applied after installation completes
//! 3. Each tool is configured for optimal educational use

use crate::manager::SoftwareBundle;
use crate::packages::{apps, ide, js};

/// Creates the Projektwoche software bundle.
///
/// This function assembles a complete web development environment suitable
/// for educational use in the Athenaeum Stade Projektwoche program.
///
/// # Bundle Philosophy
///
/// The bundle is designed around modern JavaScript/TypeScript development:
/// - **Node.js** provides the runtime foundation
/// - **Bun** offers faster package management and execution
/// - **VS Code** gives students a professional development environment
///
/// # Performance Characteristics
///
/// - **Parallel installation**: All packages install concurrently
/// - **Cross-platform**: Automatically adapts to Windows/Linux environments
/// - **Educational optimized**: Focuses on tools that enhance learning
///
/// # Returns
///
/// Returns a fully configured [`SoftwareBundle`] ready for installation.
///
/// # Example Usage
///
/// ```rust
/// let bundle = projektwoche::bundle();
/// bundle.install(&os, false)?; // Install on detected OS
/// ```
pub fn bundle() -> SoftwareBundle {
  SoftwareBundle::new("Projektwoche", "A Bundle containing Packages to set up a development environment for the Projektwoche of the Athenaeum Stade")
      .add_program(apps::git())
      .add_program(js::nodejs())
      .add_program(js::bun())
      .add_program(ide::vscode())
      .add_program(apps::chrome())
}
