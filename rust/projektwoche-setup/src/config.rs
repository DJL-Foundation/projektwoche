use confy::ConfyError;
use os_info::get;
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Architectures {
  X86_64,
  AArch64,
}
impl Default for Architectures {
  fn default() -> Self {
    match get().architecture() {
      Some("x86_64") => Architectures::X86_64,
      Some("aarch64") => Architectures::AArch64,
      _ => {
        eprintln!("Unsupported architecture detected. Defaulting to X86_64.");
        Architectures::X86_64
      }
    }
  }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OS(os_info::Type);
impl Default for OS {
  fn default() -> Self {
    Self(get().os_type())
  }
}

#[derive(Copy, Clone)]
pub enum OsCategory {
  Windows,
  LinuxBased,
  MacOS,
  ArchBased,
  RHELBased,
  DebianBased,
  GentooBased,
  AndroidBased,
}

pub enum OsSelector {
  OS(OS),
  OsCategory(OsCategory),
}

pub struct OsMatcher {
  os_list: Vec<OS>,
}
impl OsMatcher {
  pub fn new(os_list: &[OS]) -> Self {
    Self {
      os_list: os_list.to_vec(),
    }
  }

  pub fn matches(&self, os: &OS) -> bool {
    self.os_list.iter().any(|o| o.0 == os.0)
  }

  pub fn from_selector(selector: OsSelector) -> Self {
    match selector {
      OsSelector::OS(os) => OsMatcher::new(&[os]),
      OsSelector::OsCategory(category) => OsMatcher::from_category(category),
    }
  }

  pub fn from_category(category: OsCategory) -> Self {
    match category {
      OsCategory::Windows => OsMatcher::new(&WINDOWS_BASED_OS),
      OsCategory::LinuxBased => OsMatcher::new(&LINUX_BASED_OS),
      OsCategory::MacOS => OsMatcher::new(&MAC_BASED_OS),
      OsCategory::ArchBased => OsMatcher::new(&ARCH_BASED_OS),
      OsCategory::RHELBased => OsMatcher::new(&RHEL_BASED_OS),
      OsCategory::DebianBased => OsMatcher::new(&DEBIAN_BASED_OS),
      OsCategory::GentooBased => OsMatcher::new(&GENTOO_BASED_OS),
      OsCategory::AndroidBased => OsMatcher::new(&ANDROID_BASED_OS),
    }
  }

  pub fn from_categorys(categories: &[OsCategory]) -> Self {
    let mut os_list = Vec::new();
    for category in categories {
      os_list.extend(OsMatcher::from_category(*category).os_list);
    }
    OsMatcher::new(&os_list)
  }

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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
  #[serde(default)]
  pub(crate) os: OS,
  #[serde(default)]
  pub(crate) arch: Architectures,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub(crate) machine: Machine,
}

pub fn use_config() -> Result<Config, Box<dyn std::error::Error>> {
  let config: Result<Config, ConfyError> = confy::load("prowo-setup", "config");
  match config {
    Ok(config) => Ok(config),
    Err(e) => {
      eprintln!("Unbekannter Fehler beim Laden der Konfiguration: {}", e);
      exit(1)
    }
  }
}
