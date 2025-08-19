//! # Machine Detection and OS Matching
//!
//! This module provides comprehensive system detection and operating system
//! categorization functionality. It allows packages to specify installation
//! instructions for specific operating systems or categories of similar systems.
//!
//! ## Architecture Support
//!
//! Currently supports:
//! - **x86_64**: Standard 64-bit Intel/AMD processors
//! - **AArch64**: ARM 64-bit processors (Apple Silicon, ARM servers)
//!
//! ## Operating System Categories
//!
//! The system organizes operating systems into logical categories:
//! - **Windows**: All Windows versions
//! - **macOS**: Apple macOS/OS X
//! - **Linux-based**: All Linux distributions
//! - **Arch-based**: Arch Linux and derivatives (Manjaro, EndeavourOS, etc.)
//! - **RHEL-based**: Red Hat family (Fedora, CentOS, Rocky Linux, etc.)
//! - **Debian-based**: Debian family (Ubuntu, Mint, Pop!_OS, etc.)
//! - **Gentoo-based**: Gentoo and derivatives
//! - **Android-based**: Android systems

use os_info::get;
use serde::{Deserialize, Serialize};

/// Supported CPU architectures.
///
/// The enum automatically detects the current system architecture
/// and falls back to x86_64 for unsupported architectures.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Architectures {
  /// Standard 64-bit Intel/AMD processors
  X86_64,
  /// ARM 64-bit processors (Apple Silicon, ARM servers)
  AArch64,
}
impl Default for Architectures {
  fn default() -> Self {
    match get().architecture() {
      Some("x86_64") => Architectures::X86_64,
      Some("aarch64") => Architectures::AArch64,
      _ => {
        eprintln!("Unsupported architecture detected, defaulting to x86_64.");
        Architectures::X86_64
      }
    }
  }
}

/// Operating system wrapper with serialization support.
///
/// This struct wraps the `os_info::Type` to provide serialization
/// capabilities while maintaining compatibility with the os_info crate.
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OS(os_info::Type);

impl Default for OS {
  /// Automatically detects the current operating system.
  fn default() -> Self {
    Self(get().os_type())
  }
}

/// Broad categories of operating systems for easier targeting.
///
/// These categories allow packages to specify instructions for
/// groups of similar operating systems rather than individual OSes.
#[derive(Copy, Clone)]
pub enum OsCategory {
  /// Microsoft Windows (all versions)
  Windows,
  /// Apple macOS/OS X
  MacOS,
  /// All Linux-based operating systems
  LinuxBased,
  /// Arch Linux and derivatives (Manjaro, EndeavourOS, etc.)
  ArchBased,
  /// Red Hat Enterprise Linux family (Fedora, CentOS, etc.)
  RHELBased,
  /// Debian family (Ubuntu, Mint, Pop!_OS, etc.)
  DebianBased,
  /// Gentoo and derivatives
  GentooBased,
  /// Android-based systems
  AndroidBased,
}

/// Selector for choosing specific OS or OS categories.
///
/// This enum allows flexible specification of which operating
/// systems an instruction mapping should apply to.
pub enum OsSelector {
  /// Target a specific operating system
  OS(OS),
  /// Target a category of operating systems
  OsCategory(OsCategory),
}

/// Matches operating systems against a list of supported systems.
///
/// This struct provides flexible OS matching capabilities, allowing
/// packages to specify which operating systems they support through
/// either specific OS types or broad categories.
///
/// # Example
///
/// ```rust
/// // Create a matcher for all Linux distributions
/// let linux_matcher = OsMatcher::from_category(OsCategory::LinuxBased);
///
/// // Create a matcher for specific OS types
/// let specific_matcher = OsMatcher::new(&[OS::Windows, OS::MacOS]);
///
/// // Check if current OS is supported
/// if linux_matcher.matches(&current_os) {
///     // Install using Linux-specific instructions
/// }
/// ```
pub struct OsMatcher {
  /// List of supported operating systems
  os_list: Vec<OS>,
}
impl OsMatcher {
  /// Creates a new OS matcher with a specific list of supported systems.
  ///
  /// # Arguments
  ///
  /// * `os_list` - Array of operating systems this matcher should support
  pub fn new(os_list: &[OS]) -> Self {
    Self {
      os_list: os_list.to_vec(),
    }
  }

  /// Checks if the given OS is supported by this matcher.
  ///
  /// # Arguments
  ///
  /// * `os` - Operating system to check
  ///
  /// # Returns
  ///
  /// Returns `true` if the OS is in the supported list.
  pub fn matches(&self, os: &OS) -> bool {
    self.os_list.iter().any(|o| o.0 == os.0)
  }

  /// Creates a matcher from an OS selector.
  ///
  /// # Arguments
  ///
  /// * `selector` - Either a specific OS or an OS category
  pub fn from_selector(selector: OsSelector) -> Self {
    match selector {
      OsSelector::OS(os) => OsMatcher::new(&[os]),
      OsSelector::OsCategory(category) => OsMatcher::from_category(category),
    }
  }

  /// Creates a matcher for an entire OS category.
  ///
  /// This is the most common way to create matchers, as it allows
  /// targeting broad groups of similar operating systems.
  ///
  /// # Arguments
  ///
  /// * `category` - The OS category to match against
  pub fn from_category(category: OsCategory) -> Self {
    match category {
      OsCategory::Windows => OsMatcher::new(&WINDOWS_BASED_OS),
      OsCategory::MacOS => OsMatcher::new(&MAC_BASED_OS),
      OsCategory::LinuxBased => OsMatcher::new(&LINUX_BASED_OS),
      OsCategory::ArchBased => OsMatcher::new(&ARCH_BASED_OS),
      OsCategory::RHELBased => OsMatcher::new(&RHEL_BASED_OS),
      OsCategory::DebianBased => OsMatcher::new(&DEBIAN_BASED_OS),
      OsCategory::GentooBased => OsMatcher::new(&GENTOO_BASED_OS),
      OsCategory::AndroidBased => OsMatcher::new(&ANDROID_BASED_OS),
    }
  }

  /// Creates a matcher for multiple OS categories.
  ///
  /// # Arguments
  ///
  /// * `categories` - Array of OS categories to combine
  pub fn from_categories(categories: &[OsCategory]) -> Self {
    let mut os_list = Vec::new();
    for category in categories {
      os_list.extend(OsMatcher::from_category(*category).os_list);
    }
    OsMatcher::new(&os_list)
  }

  /// Returns the list of supported operating systems.
  pub fn get_list(&self) -> &[OS] {
    &self.os_list
  }
}

pub const WINDOWS_BASED_OS: &[OS] = &[OS(os_info::Type::Windows)];

pub const MAC_BASED_OS: &[OS] = &[OS(os_info::Type::Macos)];

pub const LINUX_BASED_OS: &[OS] = &[
  OS(os_info::Type::Linux),
  OS(os_info::Type::AlmaLinux),
  OS(os_info::Type::Alpaquita),
  OS(os_info::Type::Alpine),
  OS(os_info::Type::Amazon),
  OS(os_info::Type::AOSC),
  OS(os_info::Type::Arch),
  OS(os_info::Type::Artix),
  OS(os_info::Type::Bluefin),
  OS(os_info::Type::CachyOS),
  OS(os_info::Type::CentOS),
  OS(os_info::Type::Debian),
  OS(os_info::Type::EndeavourOS),
  OS(os_info::Type::Fedora),
  OS(os_info::Type::Garuda),
  OS(os_info::Type::Gentoo),
  OS(os_info::Type::Kali),
  OS(os_info::Type::Mabox),
  OS(os_info::Type::Manjaro),
  OS(os_info::Type::Mint),
  OS(os_info::Type::NixOS),
  OS(os_info::Type::Nobara),
  OS(os_info::Type::openEuler),
  OS(os_info::Type::openSUSE),
  OS(os_info::Type::OracleLinux),
  OS(os_info::Type::Pop),
  OS(os_info::Type::Raspbian),
  OS(os_info::Type::Redhat),
  OS(os_info::Type::RedHatEnterprise),
  OS(os_info::Type::RockyLinux),
  OS(os_info::Type::SUSE),
  OS(os_info::Type::Ubuntu),
  OS(os_info::Type::Ultramarine),
  OS(os_info::Type::Void),
];

pub const ARCH_BASED_OS: &[OS] = &[
  OS(os_info::Type::Arch),
  OS(os_info::Type::Artix),
  OS(os_info::Type::EndeavourOS),
  OS(os_info::Type::Garuda),
  OS(os_info::Type::Manjaro),
  OS(os_info::Type::CachyOS),
];

pub const RHEL_BASED_OS: &[OS] = &[
  OS(os_info::Type::AlmaLinux),
  OS(os_info::Type::CentOS),
  OS(os_info::Type::Fedora),
  OS(os_info::Type::Nobara),
  OS(os_info::Type::OracleLinux),
  OS(os_info::Type::Redhat),
  OS(os_info::Type::RedHatEnterprise),
  OS(os_info::Type::RockyLinux),
];

pub const DEBIAN_BASED_OS: &[OS] = &[
  OS(os_info::Type::Debian),
  OS(os_info::Type::Ubuntu),
  OS(os_info::Type::Mint),
  OS(os_info::Type::Pop),
  OS(os_info::Type::Raspbian),
];

pub const GENTOO_BASED_OS: &[OS] = &[OS(os_info::Type::Gentoo)];

pub const ANDROID_BASED_OS: &[OS] = &[OS(os_info::Type::Android)];

/// Complete machine information including OS and architecture.
///
/// This struct represents all the detected information about the current
/// machine that packages need to make installation decisions.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
  /// Detected operating system
  #[serde(default)]
  pub(crate) os: OS,
  /// Detected CPU architecture
  #[serde(default)]
  pub(crate) arch: Architectures,
}
