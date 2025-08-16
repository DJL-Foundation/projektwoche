//! # Software Bundles
//!
//! This module contains predefined software bundles that group related packages
//! together for convenient installation. Each bundle represents a complete
//! development environment or workflow setup.
//!
//! ## Available Bundles
//!
//! - **Projektwoche**: Complete web development environment for the Athenaeum Stade Projektwoche
//!
//! ## Bundle Philosophy
//!
//! Bundles are designed to provide:
//! - **Cohesive environments**: All tools needed for a specific development workflow
//! - **Tested combinations**: Packages that work well together
//! - **Quick setup**: One-command installation of complete development stacks
//! - **Educational focus**: Bundles tailored for learning and teaching scenarios
//!
//! ## Adding New Bundles
//!
//! To add a new bundle:
//! 1. Create a new module file (e.g., `web_dev.rs`)
//! 2. Add it to this module with `pub mod web_dev;`
//! 3. Implement a bundle function that returns a [`SoftwareBundle`]
//! 4. Add the bundle to the main CLI enum in `main.rs`

pub mod projektwoche;
