//! # Projektwoche Setup Library
//!
//! A comprehensive package management system for setting up development environments
//! using cross-platform software bundles. This library provides the core functionality
//! behind the `projektwoche-setup` CLI tool.
//!
//! ## Overview
//!
//! The library is built around a flexible, extensible architecture that allows for:
//! - **Cross-platform package management**: Automatic OS detection and appropriate installation methods
//! - **Bundle-based installation**: Group related packages for convenient installation
//! - **Parallel processing**: Concurrent package installation for improved performance
//! - **Dry-run capabilities**: Preview operations without making system changes
//! - **Flexible instruction system**: Extensible commands for various installation scenarios
//!
//! ## Architecture
//!
//! The system is organized into several key modules:
//!
//! ### Core Components
//! - [`manager`]: Package and bundle management with threading support
//! - [`config`]: System detection and configuration persistence
//!
//! ### Content Modules  
//! - [`packages`]: Individual software package definitions
//! - [`bundles`]: Pre-configured collections of related packages
//!
//! ## Key Concepts
//!
//! ### Packages
//! Individual software programs with OS-specific installation instructions.
//! Each package can support multiple operating systems with different installation methods.
//!
//! ### Bundles
//! Collections of related packages that are installed together. Bundles provide
//! a convenient way to set up complete development environments.
//!
//! ### Instructions
//! Atomic operations that packages use to install, configure, and manage software.
//! Instructions are cross-platform and support dry-run mode.
//!
//! ### OS Matching
//! Flexible system for targeting specific operating systems or categories of systems
//! (e.g., all Linux distributions, RHEL-based systems, etc.).
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use projektwoche_setup::bundles::projektwoche;
//! use projektwoche_setup::config;
//!
//! // Load system configuration
//! let config = config::use_config()?;
//!
//! // Get the Projektwoche bundle
//! let bundle = projektwoche::bundle();
//!
//! // Install with dry-run to preview
//! bundle.install(&config.machine.os, true)?;
//!
//! // Actually install
//! bundle.install(&config.machine.os, false)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Threading Model
//!
//! The system uses multi-threading extensively:
//! - **Bundle level**: Multiple packages within a bundle install concurrently
//! - **Phase separation**: Installation and configuration happen in separate phases
//! - **Error isolation**: Thread failures don't affect other packages
//!
//! ## Platform Support
//!
//! Currently supported platforms:
//! - **Windows**: Full support with PowerShell-based installation
//! - **Linux**: Support for major distributions (Debian, RHEL, Arch families)
//! - **macOS**: Partial support (can be extended)
//!
//! ## Extensibility
//!
//! The system is designed to be easily extensible:
//! - Add new packages by implementing the [`packages`] pattern
//! - Create new bundles by combining existing packages
//! - Extend OS support by adding new instruction mappings
//! - Add new instruction types by implementing [`manager::instructions::AnyInstruction`]

pub mod bundles;
pub mod config;
pub mod manager;
pub mod packages;

// Re-export commonly used types for convenience
pub use manager::{Package, SoftwareBundle};
pub use config::{Config, use_config};