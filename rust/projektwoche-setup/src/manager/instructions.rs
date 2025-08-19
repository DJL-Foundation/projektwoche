//! # Instruction System
//!
//! This module provides a flexible instruction system for executing various operations
//! across different platforms. Instructions are the atomic operations that packages
//! use to install, configure, and manage software.
//!
//! ## Architecture
//!
//! The instruction system is built around:
//! - **Trait-based design**: All instructions implement [`AnyInstruction`]
//! - **Cross-platform support**: Instructions handle platform differences automatically
//! - **Dry-run capability**: All instructions support preview mode without making changes
//! - **Builder pattern**: Instructions are created using a fluent builder API
//!
//! ## Available Instruction Types
//!
//! ### File Operations
//! - [`DownloadTo`]: Download files to specific locations
//! - [`DownloadAndExec`]: Download and execute installers
//! - [`ExtractArchive`]: Extract various archive formats
//! - [`BackupFile`]: Create timestamped backups of files
//! - [`EditFile`]: Perform find-and-replace operations in files
//!
//! ### System Operations  
//! - [`Run`]: Execute shell commands
//! - [`InstallPackage`]: Install packages using system package managers
//! - [`RestartService`]: Restart system services
//! - [`RequestSudo`]: Request administrator privileges
//!
//! ### Environment Setup
//! - [`AddEnvVar`]: Set environment variables persistently
//! - [`CreateShortcut`]: Create desktop shortcuts
//! - [`CloneRepository`]: Clone Git repositories
//!
//! ### Validation and Control Flow
//! - [`Assert`]: Validate command output
//! - [`WaitForCondition`]: Wait for commands to succeed with timeout
//!
//! ## Usage Example
//!
//! ```rust
//! use crate::manager::instructions::Instruction;
//!
//! // Create a command to download and install Node.js
//! let install_node = Instruction::new("Install Node.js")
//!   .download_and_exec_silent("https://nodejs.org/dist/latest/node-x64.msi");
//!
//! // Execute with dry-run to preview
//! install_node.run(true)?; // Prints what would happen
//! install_node.run(false)?; // Actually executes
//! ```

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

/// Core trait that all instruction types must implement.
///
/// This trait provides a uniform interface for executing different types
/// of operations, with built-in support for dry-run mode.
pub trait AnyInstruction {
  /// Execute the instruction.
  ///
  /// # Arguments
  ///
  /// * `dry_run` - If true, print what would be done without executing
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` on success, or an error describing what went wrong.
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Downloads and executes installers with cross-platform support.
///
/// This instruction handles downloading executable files and running them
/// with appropriate platform-specific installation flags. It supports:
///
/// - **Windows**: .exe and .msi files with silent installation flags
/// - **Linux/macOS**: Executable files without extensions
/// - **Archives**: .zip files (use [`ExtractArchive`] instead)
///
/// # Silent Installation
///
/// When `silent` is enabled, the instruction will attempt various common
/// silent installation flags if the custom arguments fail.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DownloadAndExec {
  /// URL to download the installer from
  url: &'static str,
  /// Whether to attempt silent installation
  silent: bool,
  /// Custom arguments to pass to the installer
  custom_args: Option<&'static [&'static str]>,
}

impl DownloadAndExec {
  fn new(url: &'static str, silent: bool, custom_args: Option<&'static [&'static str]>) -> Self {
    Self {
      url,
      silent,
      custom_args,
    }
  }
}

impl AnyInstruction for DownloadAndExec {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = std::env::temp_dir();
    let filename = self.url.split('/').last().unwrap_or("download");
    let file_path = temp_dir.join(filename);

    if dry_run {
      println!(
        "Dry run: would download {} to {}",
        self.url,
        file_path.display()
      );
      return Ok(());
    }
    // Download the file
    let response = std::process::Command::new("curl")
      .arg("-L")
      .arg("-o")
      .arg(&file_path)
      .arg(self.url)
      .output()?;

    if !response.status.success() {
      return Err("Download failed".into());
    }

    let file_extension = file_path
      .extension()
      .and_then(|ext| ext.to_str())
      .unwrap_or("")
      .to_lowercase();

    match file_extension.as_str() {
      "exe" => {
        #[cfg(windows)]
        {
          let mut cmd = Command::new(&file_path);

          // Add custom arguments if provided
          if let Some(args) = self.custom_args {
            cmd.args(args);
          } else if self.silent {
            // Try common silent installation flags for EXE files
            cmd.args(&["/S"]); // NSIS installers
          }

          let status = cmd.status()?;
          if !status.success() {
            // If /S failed and we're in silent mode, try other common flags
            if self.silent && self.custom_args.is_none() {
              let silent_flags = [
                &["/SILENT"][..],
                &["/VERYSILENT"][..],
                &["/quiet"][..],
                &["/Q"][..],
                &["/s"][..],
                &["--silent"][..],
                &["-s"][..],
              ];

              for flags in &silent_flags {
                let mut retry_cmd = Command::new(&file_path);
                retry_cmd.args(*flags);
                if let Ok(status) = retry_cmd.status() {
                  if status.success() {
                    break;
                  }
                }
              }
            }
          }
        }
        #[cfg(not(windows))]
        {
          return Err("EXE files can only be executed on Windows".into());
        }
      }
      "msi" => {
        #[cfg(windows)]
        {
          let mut cmd = Command::new("msiexec");
          cmd.arg("/i").arg(&file_path);

          if let Some(args) = self.custom_args {
            cmd.args(args);
          } else if self.silent {
            // MSI silent installation flags
            cmd.args(&["/quiet", "/qn", "/norestart"]);
          }

          cmd.status()?;
        }
        #[cfg(not(windows))]
        {
          return Err("MSI files can only be executed on Windows".into());
        }
      }
      // TODO: ADD DEFAULT PM EXEC LIKE .deb .rpm ...
      "" => {
        // Handle Linux and macOS executables (no file extension)
        #[cfg(any(unix, target_os = "macos"))]
        {
          let mut cmd = Command::new(&file_path);

          // Add custom arguments if provided
          if let Some(args) = self.custom_args {
            cmd.args(args);
          }

          let status = cmd.status()?;
          if !status.success() {
            return Err(format!("Execution failed with exit code: {:?}", status.code()).into());
          }
        }
        #[cfg(not(any(unix, target_os = "macos")))]
        {
          return Err("Linux/macOS executables can only be executed on Unix-like systems".into());
        }
      }
      "zip" => {
        // ZIP files should be extracted, not executed
        return Err("ZIP files should be extracted using extract_archive, not executed".into());
      }
      _ => return Err("Unsupported filetype for execution".into()),
    }

    // Clean up downloaded file
    if file_path.exists() {
      std::fs::remove_file(&file_path)?;
    }

    Ok(())
  }
}

/// Executes shell commands with cross-platform compatibility.
///
/// This instruction runs arbitrary shell commands, automatically handling
/// argument parsing and execution. Commands are split on whitespace.
///
/// # Example
///
/// ```rust
/// Run::new("npm install -g yarn")
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Run {
  /// Command split into program and arguments
  command: Vec<String>,
}

impl Run {
  fn new(command: &str) -> Self {
    Self {
      command: command.split_whitespace().map(|s| s.to_string()).collect(),
    }
  }
}

impl AnyInstruction for Run {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if self.command.is_empty() {
      return Err("Empty command".into());
    }

    if dry_run {
      println!("Dry run: would execute command: {}", self.command.join(" "));
      return Ok(());
    }

    let mut cmd = Command::new(&self.command[0]);
    cmd.args(&self.command[1..]);

    let status = cmd.status()?;

    if !status.success() {
      return Err(format!("Command failed with exit code: {:?}", status.code()).into());
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DownloadTo {
  url: &'static str,
  path: &'static str,
}

impl DownloadTo {
  fn new(url: &'static str, path: &'static str) -> Self {
    Self { url, path }
  }
}

impl AnyInstruction for DownloadTo {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!("Dry run: would download {} to {}", self.url, self.path);
      return Ok(());
    }
    let response = std::process::Command::new("curl")
      .arg("-L")
      .arg("-o")
      .arg(self.path)
      .arg(self.url)
      .output()?;

    if !response.status.success() {
      return Err("Download failed".into());
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assert {
  command: Vec<String>,
  expect: &'static str,
}

impl Assert {
  fn new(command: &str, expect: &'static str) -> Self {
    Self {
      command: command.split_whitespace().map(|s| s.to_string()).collect(),
      expect,
    }
  }
}

impl AnyInstruction for Assert {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if self.command.is_empty() {
      return Err("Empty command".into());
    }

    if dry_run {
      println!(
        "Dry run: expect the result of: {} to be {}",
        self.command.join(" "),
        self.expect
      );
      return Ok(());
    }

    let mut cmd = Command::new(&self.command[0]);
    cmd.args(&self.command[1..]);

    let output = cmd.output()?;

    if !output.status.success() {
      return Err(format!("Command failed with exit code: {:?}", output.status.code()).into());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    if !output_str.contains(self.expect) {
      return Err(format!("Expected '{}' but got '{}'", self.expect, output_str).into());
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtractArchive {
  archive_path: &'static str,
  destination: &'static str,
}

impl ExtractArchive {
  fn new(archive_path: &'static str, destination: &'static str) -> Self {
    Self {
      archive_path,
      destination,
    }
  }
}

impl AnyInstruction for ExtractArchive {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(self.archive_path);
    let extension = path
      .extension()
      .and_then(|s| s.to_str())
      .ok_or("No file extension")?;

    if dry_run {
      println!(
        "Dry run: would extract {} to {}",
        self.archive_path, self.destination
      );
      return Ok(());
    }

    fs::create_dir_all(self.destination)?;

    match extension.to_lowercase().as_str() {
      "zip" => {
        Command::new("unzip")
          .arg("-o")
          .arg(self.archive_path)
          .arg("-d")
          .arg(self.destination)
          .status()?;
      }
      "gz" | "tgz" => {
        Command::new("tar")
          .arg("-xzf")
          .arg(self.archive_path)
          .arg("-C")
          .arg(self.destination)
          .status()?;
      }
      "bz2" | "tbz2" => {
        Command::new("tar")
          .arg("-xjf")
          .arg(self.archive_path)
          .arg("-C")
          .arg(self.destination)
          .status()?;
      }
      "xz" | "txz" => {
        Command::new("tar")
          .arg("-xJf")
          .arg(self.archive_path)
          .arg("-C")
          .arg(self.destination)
          .status()?;
      }
      _ => return Err(format!("Unsupported archive format: {}", extension).into()),
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddEnvVar {
  name: &'static str,
  value: &'static str,
}

impl AddEnvVar {
  fn new(name: &'static str, value: &'static str) -> Self {
    Self { name, value }
  }
}

impl AnyInstruction for AddEnvVar {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would set environment variable {}={}",
        self.name, self.value
      );
      return Ok(());
    }
    {
      Command::new("setx")
        .arg(self.name)
        .arg(self.value)
        .status()?;
    }

    {
      let home = std::env::var("HOME")?;
      let bashrc_path = format!("{}/.bashrc", home);
      let env_line = format!("export {}=\"{}\"\n", self.name, self.value);

      let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&bashrc_path)?;
      file.write_all(env_line.as_bytes())?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateShortcut {
  name: &'static str,
  target: &'static str,
  icon: Option<&'static str>,
}

impl CreateShortcut {
  fn new(name: &'static str, target: &'static str, icon: Option<&'static str>) -> Self {
    Self { name, target, icon }
  }
}

impl AnyInstruction for CreateShortcut {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would create shortcut '{}' pointing to '{}'{}",
        self.name,
        self.target,
        if let Some(icon) = self.icon {
          format!(" with icon '{}'", icon)
        } else {
          String::new()
        }
      );
      return Ok(());
    }
    {
      let desktop = std::env::var("USERPROFILE")? + "\\Desktop";
      let shortcut_path = format!("{}\\{}.lnk", desktop, self.name);

      let ps_script = format!(
        r#"$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut("{}"); $Shortcut.TargetPath = "{}"; $Shortcut.Save()"#,
        shortcut_path, self.target
      );

      Command::new("powershell")
        .arg("-Command")
        .arg(&ps_script)
        .status()?;
    }

    {
      let home = std::env::var("HOME")?;
      let desktop_path = format!("{}/Desktop/{}.desktop", home, self.name);

      let desktop_entry = format!(
        "[Desktop Entry]\nVersion=1.0\nType=Application\nName={}\nExec={}\n{}Terminal=false\n",
        self.name,
        self.target,
        if let Some(icon) = self.icon {
          format!("Icon={}\n", icon)
        } else {
          String::new()
        }
      );

      fs::write(&desktop_path, desktop_entry)?;
      Command::new("chmod")
        .arg("+x")
        .arg(&desktop_path)
        .status()?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WaitForCondition {
  check_command: Vec<String>,
  timeout_secs: u64,
}

impl WaitForCondition {
  fn new(check_command: &str, timeout_secs: u64) -> Self {
    Self {
      check_command: check_command
        .split_whitespace()
        .map(|s| s.to_string())
        .collect(),
      timeout_secs,
    }
  }
}

impl AnyInstruction for WaitForCondition {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would wait up to {} seconds for command '{}' to succeed",
        self.timeout_secs,
        self.check_command.join(" ")
      );
      return Ok(());
    }
    let start = Instant::now();
    let timeout = Duration::from_secs(self.timeout_secs);

    while start.elapsed() < timeout {
      if self.check_command.is_empty() {
        return Err("Empty check command".into());
      }

      let mut cmd = Command::new(&self.check_command[0]);
      cmd.args(&self.check_command[1..]);

      if let Ok(status) = cmd.status() {
        if status.success() {
          return Ok(());
        }
      }

      std::thread::sleep(Duration::from_secs(1));
    }

    Err("Timeout waiting for condition".into())
  }
}

/// Automatically installs packages using the system's package manager.
///
/// This instruction detects the available package manager on the system
/// and uses it to install the specified package. Supported managers:
///
/// **Linux**: apt, yum, dnf, pacman, zypper
/// **macOS**: brew  
/// **Windows**: choco, winget
///
/// The instruction tries managers in order until one succeeds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstallApplication {
  /// Name of the package to install
  package_name: &'static str,
}

impl InstallApplication {
  fn new(package_name: &'static str) -> Self {
    Self { package_name }
  }
}

impl AnyInstruction for InstallApplication {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!("Dry run: would install package '{}'", self.package_name);
      return Ok(());
    }

    #[cfg(not(windows))]
    {
      let package_managers = [
        ("apt", vec!["apt", "install", "-y", self.package_name]),
        ("yum", vec!["yum", "install", "-y", self.package_name]),
        ("dnf", vec!["dnf", "install", "-y", self.package_name]),
        (
          "pacman",
          vec!["pacman", "-S", "--noconfirm", self.package_name],
        ),
        ("zypper", vec!["zypper", "install", "-y", self.package_name]),
        ("brew", vec!["brew", "install", self.package_name]),
      ];

      for (pm, args) in &package_managers {
        if Command::new("which")
          .arg(pm)
          .output()
          .map(|o| o.status.success())
          .unwrap_or(false)
        {
          let status = Command::new(args[0]).args(&args[1..]).status()?;

          if status.success() {
            return Ok(());
          }
        }
      }
    }

    #[cfg(windows)]
    {
      if Command::new("choco")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        let status = Command::new("choco")
          .args(&["install", self.package_name, "-y"])
          .status()?;
        if status.success() {
          return Ok(());
        }
      }

      if Command::new("winget")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        let status = Command::new("winget")
          .args(&["install", "--id", self.package_name, "-e"])
          .status()?;
        if status.success() {
          return Ok(());
        }
      }
    }

    Err("No suitable package manager found".into())
  }
}

/// Installs packages using programming language package managers.
///
/// This instruction detects available language package managers and uses them
/// to install packages globally. Supported managers:
///
/// **JavaScript/TypeScript**: npm, yarn, bun, pnpm
/// **Rust**: cargo
/// **Python**: pip, pipx
/// **Ruby**: gem
/// **Go**: go install
///
/// The instruction tries managers in order until one succeeds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstallPackage {
  /// Name of the package to install
  package_name: &'static str,
}

impl InstallPackage {
  fn new(package_name: &'static str) -> Self {
    Self { package_name }
  }
}

impl AnyInstruction for InstallPackage {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would install package '{}' using language package manager",
        self.package_name
      );
      return Ok(());
    }

    let package_managers = [
      // JavaScript/TypeScript package managers
      ("npm", vec!["npm", "install", "-g", self.package_name]),
      ("yarn", vec!["yarn", "global", "add", self.package_name]),
      ("bun", vec!["bun", "add", "-g", self.package_name]),
      ("pnpm", vec!["pnpm", "add", "-g", self.package_name]),
      // Rust package manager
      ("cargo", vec!["cargo", "install", self.package_name]),
      // Python package managers
      ("pipx", vec!["pipx", "install", self.package_name]),
      ("pip", vec!["pip", "install", "--user", self.package_name]),
      // Ruby package manager
      ("gem", vec!["gem", "install", self.package_name]),
    ];

    for (pm, args) in &package_managers {
      // Check if package manager is available
      let check_cmd = if *pm == "go" {
        Command::new("go").arg("version").output()
      } else {
        Command::new(pm).arg("--version").output()
      };

      if check_cmd.map(|o| o.status.success()).unwrap_or(false) {
        let status = Command::new(args[0]).args(&args[1..]).status()?;

        if status.success() {
          return Ok(());
        }
      }
    }

    // Special case for Go (different command structure)
    if Command::new("go")
      .arg("version")
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
    {
      let status = Command::new("go")
        .args(&["install", &format!("{}@latest", self.package_name)])
        .status()?;
      if status.success() {
        return Ok(());
      }
    }

    Err("No suitable language package manager found".into())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloneRepository {
  url: &'static str,
  path: Option<&'static str>,
}

impl CloneRepository {
  fn new(url: &'static str, path: Option<&'static str>) -> Self {
    Self { url, path }
  }
}

impl AnyInstruction for CloneRepository {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would clone repository '{}' {}",
        self.url,
        if let Some(path) = self.path {
          format!("to '{}'", path)
        } else {
          "to current directory".to_string()
        }
      );
      return Ok(());
    }
    let mut cmd = Command::new("git");
    cmd.arg("clone").arg(self.url);

    if let Some(path) = self.path {
      cmd.arg(path);
    }

    let status = cmd.status()?;

    if !status.success() {
      return Err("Git clone failed".into());
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestSudo {
  reason: &'static str,
}

impl RequestSudo {
  fn new(reason: &'static str) -> Self {
    Self { reason }
  }
}

impl AnyInstruction for RequestSudo {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would request administrator privileges: {}",
        self.reason
      );
      return Ok(());
    }
    println!("Administrator privileges required: {}", self.reason);

    {
      Command::new("sudo").arg("-v").status()?;
    }

    {
      // On Windows, this would typically be handled by UAC prompts in individual commands
      println!("Please ensure you are running as Administrator or have UAC enabled");
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RestartService {
  service_name: &'static str,
}

impl RestartService {
  fn new(service_name: &'static str) -> Self {
    Self { service_name }
  }
}

impl AnyInstruction for RestartService {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!("Dry run: would restart service '{}'", self.service_name);
      return Ok(());
    }
    {
      Command::new("sc")
        .args(&["stop", self.service_name])
        .status()?;

      std::thread::sleep(Duration::from_secs(2));

      Command::new("sc")
        .args(&["start", self.service_name])
        .status()?;
    }

    {
      if Command::new("systemctl")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        Command::new("systemctl")
          .args(&["restart", self.service_name])
          .status()?;
      } else if Command::new("service")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        Command::new("service")
          .args(&[self.service_name, "restart"])
          .status()?;
      } else {
        return Err("No service manager found".into());
      }
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackupFile {
  path: &'static str,
}

impl BackupFile {
  fn new(path: &'static str) -> Self {
    Self { path }
  }
}

impl AnyInstruction for BackupFile {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!("Dry run: would backup file '{}'", self.path);
      return Ok(());
    }
    if !Path::new(self.path).exists() {
      return Ok(()); // Nothing to backup
    }

    let timestamp = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)?
      .as_secs();

    let backup_path = format!("{}.backup.{}", self.path, timestamp);
    fs::copy(self.path, &backup_path)?;

    println!("Backed up {} to {}", self.path, backup_path);
    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EditFile {
  path: &'static str,
  find: &'static str,
  replace: &'static str,
}

impl EditFile {
  fn new(path: &'static str, find: &'static str, replace: &'static str) -> Self {
    Self {
      path,
      find,
      replace,
    }
  }
}

impl AnyInstruction for EditFile {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if dry_run {
      println!(
        "Dry run: would edit file '{}' replacing '{}' with '{}'",
        self.path, self.find, self.replace
      );
      return Ok(());
    }
    let content = fs::read_to_string(self.path)?;
    let new_content = content.replace(self.find, self.replace);
    fs::write(self.path, new_content)?;

    Ok(())
  }
}

/// Unified instruction enum that contains all available instruction types.
///
/// This enum serves as a type-safe container for all instruction variants,
/// allowing them to be stored in collections and executed polymorphically.
///
/// Each variant corresponds to a specific instruction type and provides
/// the same functionality through the [`AnyInstruction`] trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instructions {
  /// Download and execute an installer
  DownloadAndExec(DownloadAndExec),
  /// Run a shell command
  Run(Run),
  /// Download a file to a specific location
  DownloadTo(DownloadTo),
  /// Assert that a command produces expected output
  Assert(Assert),
  /// Extract an archive file
  ExtractArchive(ExtractArchive),
  /// Add an environment variable
  AddEnvVar(AddEnvVar),
  /// Create a desktop shortcut
  CreateShortcut(CreateShortcut),
  /// Wait for a condition to become true
  WaitForCondition(WaitForCondition),
  /// Install an application using system package manager
  InstallApplication(InstallApplication),
  /// Install a package using language package manager
  InstallPackage(InstallPackage),
  /// Clone a Git repository
  CloneRepository(CloneRepository),
  /// Request administrator privileges
  RequestSudo(RequestSudo),
  /// Restart a system service
  RestartService(RestartService),
  /// Create a backup of a file
  BackupFile(BackupFile),
  /// Edit a file using find and replace
  EditFile(EditFile),
}

impl Instructions {
  fn from_instruction(instruction: Instruction) -> Self {
    match instruction.instruction {
      Some(inst) => inst,
      None => panic!("Instruction must have an associated instruction"),
    }
  }
}

impl AnyInstruction for Instructions {
  fn run(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match self {
      Instructions::DownloadAndExec(inst) => inst.run(dry_run),
      Instructions::Run(inst) => inst.run(dry_run),
      Instructions::DownloadTo(inst) => inst.run(dry_run),
      Instructions::Assert(inst) => inst.run(dry_run),
      Instructions::ExtractArchive(inst) => inst.run(dry_run),
      Instructions::AddEnvVar(inst) => inst.run(dry_run),
      Instructions::CreateShortcut(inst) => inst.run(dry_run),
      Instructions::WaitForCondition(inst) => inst.run(dry_run),
      Instructions::InstallApplication(inst) => inst.run(dry_run),
      Instructions::InstallPackage(inst) => inst.run(dry_run),
      Instructions::CloneRepository(inst) => inst.run(dry_run),
      Instructions::RequestSudo(inst) => inst.run(dry_run),
      Instructions::RestartService(inst) => inst.run(dry_run),
      Instructions::BackupFile(inst) => inst.run(dry_run),
      Instructions::EditFile(inst) => inst.run(dry_run),
    }
  }
}

/// Builder for creating and configuring instructions.
///
/// This struct provides a fluent interface for creating instructions with
/// a human-readable description. The builder pattern allows for clean,
/// expressive instruction creation.
///
/// # Example
///
/// ```rust
/// let instruction = Instruction::new("Install Node.js")
///   .download_and_exec_silent("https://nodejs.org/dist/latest/node-x64.msi");
///
/// instruction.run(false)?; // Execute the instruction
/// ```
///
/// # Available Methods
///
/// - **File Operations**: `download_and_exec`, `download_to`, `extract_archive`
/// - **Commands**: `cmd`, `install_package`, `clone_repository`  
/// - **System**: `add_env_var`, `create_shortcut`, `restart_service`
/// - **Validation**: `check`, `wait_for_condition`
/// - **Utilities**: `backup_file`, `edit_file`, `request_sudo`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction {
  /// Human-readable description of what this instruction does
  descriptor: &'static str,
  /// The actual instruction implementation (set by builder methods)
  instruction: Option<Instructions>,
}

impl Instruction {
  /// Creates a new instruction builder with a description.
  ///
  /// # Arguments
  ///
  /// * `descriptor` - Human-readable description of what this instruction does
  pub fn new(descriptor: &'static str) -> Self {
    Self {
      descriptor,
      instruction: None,
    }
  }

  /// Download and execute an installer normally.
  ///
  /// # Arguments
  ///
  /// * `url` - URL to download the installer from
  pub fn download_and_exec(mut self, url: &'static str) -> Instructions {
    self.instruction = Some(Instructions::DownloadAndExec(DownloadAndExec::new(
      url, false, None,
    )));
    Instructions::from_instruction(self)
  }

  /// Download and execute an installer with silent/quiet flags.
  ///
  /// This method automatically tries common silent installation flags
  /// if no custom arguments are provided.
  ///
  /// # Arguments
  ///
  /// * `url` - URL to download the installer from
  pub fn download_and_exec_silent(mut self, url: &'static str) -> Instructions {
    self.instruction = Some(Instructions::DownloadAndExec(DownloadAndExec::new(
      url, true, None,
    )));
    Instructions::from_instruction(self)
  }

  /// Download and execute an installer with custom arguments.
  ///
  /// # Arguments
  ///
  /// * `url` - URL to download the installer from  
  /// * `args` - Custom arguments to pass to the installer
  pub fn download_and_exec_with_args(
    mut self,
    url: &'static str,
    args: &'static [&'static str],
  ) -> Instructions {
    self.instruction = Some(Instructions::DownloadAndExec(DownloadAndExec::new(
      url,
      false,
      Some(args),
    )));
    Instructions::from_instruction(self)
  }

  /// Execute a shell command.
  ///
  /// # Arguments
  ///
  /// * `command` - Shell command to execute (will be split on whitespace)
  pub fn cmd(mut self, command: &str) -> Instructions {
    self.instruction = Some(Instructions::Run(Run::new(command)));
    Instructions::from_instruction(self)
  }

  /// Install an application using the system package manager.
  ///
  /// Automatically detects and uses the appropriate package manager
  /// for the current operating system.
  ///
  /// # Arguments
  ///
  /// * `package_name` - Name of the application to install
  pub fn install_application(mut self, package_name: &'static str) -> Instructions {
    self.instruction = Some(Instructions::InstallApplication(InstallApplication::new(
      package_name,
    )));
    Instructions::from_instruction(self)
  }

  /// Install a package using language package managers.
  ///
  /// Automatically detects and uses available language package managers
  /// like npm, cargo, pip, etc.
  ///
  /// # Arguments
  ///
  /// * `package_name` - Name of the package to install
  pub fn install_package(mut self, package_name: &'static str) -> Instructions {
    self.instruction = Some(Instructions::InstallPackage(InstallPackage::new(
      package_name,
    )));
    Instructions::from_instruction(self)
  }

  /// Create an assertion that checks if a command produces expected output.
  ///
  /// This is commonly used for prerequisite checks to verify if software
  /// is already installed.
  ///
  /// # Arguments
  ///
  /// * `command` - Command to execute (will be split on whitespace)
  /// * `expect` - String that should be present in the command output
  ///
  /// # Example
  ///
  /// ```rust
  /// // Check if Node.js is installed
  /// let check = Instruction::new("Check Node.js")
  ///   .assert("node --version", "v");
  /// ```
  pub fn assert(mut self, command: &str, expect: &'static str) -> Instructions {
    self.instruction = Some(Instructions::Assert(Assert::new(command, expect)));
    Instructions::from_instruction(self)
  }

  /// Execute the instruction immediately.
  ///
  /// This is a convenience method for running an instruction without
  /// going through the Instructions enum.
  ///
  /// # Arguments
  ///
  /// * `dry_run` - If true, only print what would be done
  pub fn execute(&self, dry_run: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(ref instruction) = self.instruction {
      instruction.run(dry_run)?;
    }

    Ok(())
  }
}
