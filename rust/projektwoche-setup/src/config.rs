use std::process::exit;
use confy::ConfyError;
use os_info::get;
use serde::{Deserialize, Serialize};

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

pub enum OS_Category {
  Windows,
  LinuxBased,
  MacOS,
  ArchBased,
  RHELBased,
  DebianBased,
  PacmanBased,
  GentooBased,
  AndroidBased,
}

pub struct OS_Matcher {
    os_list: &'static [OS],
}
impl OS_Matcher {
    pub fn new(os_list: &'static [OS]) -> Self {
        Self { os_list }
    }

    pub fn matches(&self, os: &OS) -> bool {
        self.os_list.iter().any(|o| o.0 == os.0)
    }

    pub fn from_category(category: OS_Category) -> Self {
        match category {
            OS_Category::Windows => OS_Matcher::new(&WINDOWS_BASED_OS),
            OS_Category::LinuxBased => OS_Matcher::new(&LINUX_BASED_OS),
            OS_Category::MacOS => OS_Matcher::new(&MAC_BASED_OS),
            OS_Category::ArchBased => OS_Matcher::new(&ARCH_BASED_OS),
            OS_Category::RHELBased => OS_Matcher::new(&RHEL_BASED_OS),
            OS_Category::DebianBased => OS_Matcher::new(&DEBIAN_BASED_OS),
            OS_Category::PacmanBased => OS_Matcher::new(&PACMAN_BASED_OS),
            OS_Category::GentooBased => OS_Matcher::new(&GENTOO_BASED_OS),
            OS_Category::AndroidBased => OS_Matcher::new(&ANDROID_BASED_OS),
        }
    }
    pub fn get_list(&self) -> &'static [OS] {
        self.os_list
    }
}

pub const WINDOWS_BASED_OS: &[OS] = &[
  OS(os_info::Type::Windows),
];

pub const MAC_BASED_OS: &[OS] = &[
  OS(os_info::Type::Macos),
];

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

pub const PACMAN_BASED_OS: &[OS] = &[
  OS(os_info::Type::Arch),
  OS(os_info::Type::Artix),
  OS(os_info::Type::EndeavourOS),
  OS(os_info::Type::Garuda),
  OS(os_info::Type::Manjaro),
  OS(os_info::Type::CachyOS),
];

pub const GENTOO_BASED_OS: &[OS] = &[
  OS(os_info::Type::Gentoo),
];

pub const ANDROID_BASED_OS: &[OS] = &[
  OS(os_info::Type::Android),
];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    #[serde(default)]
    os: OS,
    #[serde(default)]
    arch: Architectures,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    machine: Machine,
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