use regex::Regex;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub trait AnyInstruction {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>; // Todo: implement dry_run
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DownloadAndExec {
  url: &'static str,
  silent: bool,
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = std::env::temp_dir();
    let filename = self.url.split('/').last().unwrap_or("download");
    let file_path = temp_dir.join(filename);

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

    // match self.filetype {
    //   Filetype::EXE => {
    //     #[cfg(windows)]
    //     {
    //       let mut cmd = Command::new(&file_path);

    //       // Add custom arguments if provided
    //       if let Some(args) = self.custom_args {
    //         cmd.args(args);
    //       } else if self.silent {
    //         // Try common silent installation flags for EXE files
    //         // Different installers use different flags, so we try the most common ones
    //         cmd.args(&["/S"]); // NSIS installers
    //         // Some installers might not recognize /S, so we could try multiple approaches
    //         // but for safety, we'll start with the most common one
    //       }

    //       let status = cmd.status()?;
    //       if !status.success() {
    //         // If /S failed and we're in silent mode, try other common flags
    //         if self.silent && self.custom_args.is_none() {
    //           let silent_flags = [
    //             &["/SILENT"][..],
    //             &["/VERYSILENT"][..],
    //             &["/quiet"][..],
    //             &["/Q"][..],
    //             &["/s"][..],
    //             &["--silent"][..],
    //             &["-s"][..],
    //           ];

    //           for flags in &silent_flags {
    //             let mut retry_cmd = Command::new(&file_path);
    //             retry_cmd.args(*flags);
    //             if let Ok(status) = retry_cmd.status() {
    //               if status.success() {
    //                 break;
    //               }
    //             }
    //           }
    //         }
    //       }
    //     }
    //     #[cfg(not(windows))]
    //     {
    //       return Err("EXE files can only be executed on Windows".into());
    //     }
    //   }
    //   Filetype::MSI => {
    //     #[cfg(windows)]
    //     {
    //       let mut cmd = Command::new("msiexec");
    //       cmd.arg("/i").arg(&file_path);

    //       if let Some(args) = self.custom_args {
    //         cmd.args(args);
    //       } else if self.silent {
    //         // MSI silent installation flags
    //         cmd.args(&["/quiet", "/qn", "/norestart"]);
    //       }

    //       cmd.status()?;
    //     }
    //     #[cfg(not(windows))]
    //     {
    //       return Err("MSI files can only be executed on Windows".into());
    //     }
    //   }
    //   Filetype::ZIP => {
    //     // ZIP files should be extracted, not executed
    //     return Err("ZIP files should be extracted using extract_archive, not executed".into());
    //   }
    //   _ => return Err("Unsupported filetype for execution".into()),
    // }

    // Clean up downloaded file
    if file_path.exists() {
      std::fs::remove_file(&file_path)?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Run {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if self.command.is_empty() {
      return Err("Empty command".into());
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
struct DownloadTo {
  url: &'static str,
  path: &'static str,
}

impl DownloadTo {
  fn new(url: &'static str, path: &'static str) -> Self {
    Self { url, path }
  }
}

impl AnyInstruction for DownloadTo {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
struct Assert {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if self.command.is_empty() {
      return Err("Empty command".into());
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
struct ExtractArchive {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(self.archive_path);
    let extension = path
      .extension()
      .and_then(|s| s.to_str())
      .ok_or("No file extension")?;

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
struct AddEnvVar {
  name: &'static str,
  value: &'static str,
}

impl AddEnvVar {
  fn new(name: &'static str, value: &'static str) -> Self {
    Self { name, value }
  }
}

impl AnyInstruction for AddEnvVar {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(windows)]
    {
      Command::new("setx")
        .arg(self.name)
        .arg(self.value)
        .status()?;
    }

    #[cfg(unix)]
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
struct CreateShortcut {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(windows)]
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

    #[cfg(unix)]
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
struct WaitForCondition {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InstallPackage {
  package_name: &'static str,
}

impl InstallPackage {
  fn new(package_name: &'static str) -> Self {
    Self { package_name }
  }
}

impl AnyInstruction for InstallPackage {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    #[cfg(windows)]
    {
      if Command::new("choco")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        Command::new("choco")
          .args(&["install", self.package_name, "-y"])
          .status()?;
        return Ok(());
      }

      if Command::new("winget")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
      {
        Command::new("winget")
          .args(&["install", self.package_name])
          .status()?;
        return Ok(());
      }
    }

    Err("No suitable package manager found".into())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CloneRepository {
  url: &'static str,
  path: Option<&'static str>,
}

impl CloneRepository {
  fn new(url: &'static str, path: Option<&'static str>) -> Self {
    Self { url, path }
  }
}

impl AnyInstruction for CloneRepository {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
struct RequestSudo {
  reason: &'static str,
}

impl RequestSudo {
  fn new(reason: &'static str) -> Self {
    Self { reason }
  }
}

impl AnyInstruction for RequestSudo {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Administrator privileges required: {}", self.reason);

    #[cfg(unix)]
    {
      Command::new("sudo").arg("-v").status()?;
    }

    #[cfg(windows)]
    {
      // On Windows, this would typically be handled by UAC prompts in individual commands
      println!("Please ensure you are running as Administrator or have UAC enabled");
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RestartService {
  service_name: &'static str,
}

impl RestartService {
  fn new(service_name: &'static str) -> Self {
    Self { service_name }
  }
}

impl AnyInstruction for RestartService {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(windows)]
    {
      Command::new("sc")
        .args(&["stop", self.service_name])
        .status()?;

      std::thread::sleep(Duration::from_secs(2));

      Command::new("sc")
        .args(&["start", self.service_name])
        .status()?;
    }

    #[cfg(unix)]
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
struct BackupFile {
  path: &'static str,
}

impl BackupFile {
  fn new(path: &'static str) -> Self {
    Self { path }
  }
}

impl AnyInstruction for BackupFile {
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
struct EditFile {
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content = fs::read_to_string(self.path)?;
    let new_content = content.replace(self.find, self.replace);
    fs::write(self.path, new_content)?;

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instructions {
  DownloadAndExec(DownloadAndExec),
  Run(Run),
  DownloadTo(DownloadTo),
  Assert(Assert),
  ExtractArchive(ExtractArchive),
  AddEnvVar(AddEnvVar),
  CreateShortcut(CreateShortcut),
  WaitForCondition(WaitForCondition),
  InstallPackage(InstallPackage),
  CloneRepository(CloneRepository),
  RequestSudo(RequestSudo),
  RestartService(RestartService),
  BackupFile(BackupFile),
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
  fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match self {
      Instructions::DownloadAndExec(inst) => inst.run(),
      Instructions::Run(inst) => inst.run(),
      Instructions::DownloadTo(inst) => inst.run(),
      Instructions::Assert(inst) => inst.run(),
      Instructions::ExtractArchive(inst) => inst.run(),
      Instructions::AddEnvVar(inst) => inst.run(),
      Instructions::CreateShortcut(inst) => inst.run(),
      Instructions::WaitForCondition(inst) => inst.run(),
      Instructions::InstallPackage(inst) => inst.run(),
      Instructions::CloneRepository(inst) => inst.run(),
      Instructions::RequestSudo(inst) => inst.run(),
      Instructions::RestartService(inst) => inst.run(),
      Instructions::BackupFile(inst) => inst.run(),
      Instructions::EditFile(inst) => inst.run(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction {
  descriptor: &'static str,
  instruction: Option<Instructions>,
  path_to_add: Option<&'static str>,
}

impl Instruction {
  pub fn new(descriptor: &'static str) -> Self {
    Self {
      descriptor,
      instruction: None,
      path_to_add: None,
    }
  }

  pub fn download_and_exec(mut self, url: &'static str) -> Instructions {
    self.instruction = Some(Instructions::DownloadAndExec(DownloadAndExec::new(
      url, false, None,
    )));
    Instructions::from_instruction(self)
  }

  pub fn download_and_exec_silent(mut self, url: &'static str) -> Instructions {
    self.instruction = Some(Instructions::DownloadAndExec(DownloadAndExec::new(
      url, true, None,
    )));
    Instructions::from_instruction(self)
  }

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

  pub fn cmd(mut self, command: &str) -> Instructions {
    self.instruction = Some(Instructions::Run(Run::new(command)));
    Instructions::from_instruction(self)
  }

  pub fn download_to(mut self, url: &'static str, path: &'static str) -> Instructions {
    self.instruction = Some(Instructions::DownloadTo(DownloadTo::new(url, path)));
    Instructions::from_instruction(self)
  }

  pub fn check(mut self, command: &str, expect: &'static str) -> Instructions {
    self.instruction = Some(Instructions::Assert(Assert::new(command, expect)));
    Instructions::from_instruction(self)
  }

  pub fn extract_archive(
    mut self,
    archive_path: &'static str,
    destination: &'static str,
  ) -> Instructions {
    self.instruction = Some(Instructions::ExtractArchive(ExtractArchive::new(
      archive_path,
      destination,
    )));
    Instructions::from_instruction(self)
  }

  pub fn add_env_var(mut self, name: &'static str, value: &'static str) -> Instructions {
    self.instruction = Some(Instructions::AddEnvVar(AddEnvVar::new(name, value)));
    Instructions::from_instruction(self)
  }

  pub fn create_shortcut(
    mut self,
    name: &'static str,
    target: &'static str,
    icon: Option<&'static str>,
  ) -> Instructions {
    self.instruction = Some(Instructions::CreateShortcut(CreateShortcut::new(
      name, target, icon,
    )));
    Instructions::from_instruction(self)
  }

  pub fn wait_for_condition(mut self, check_command: &str, timeout_secs: u64) -> Instructions {
    self.instruction = Some(Instructions::WaitForCondition(WaitForCondition::new(
      check_command,
      timeout_secs,
    )));
    Instructions::from_instruction(self)
  }

  pub fn install_package(mut self, package_name: &'static str) -> Instructions {
    self.instruction = Some(Instructions::InstallPackage(InstallPackage::new(
      package_name,
    )));
    Instructions::from_instruction(self)
  }

  pub fn clone_repository(mut self, url: &'static str, path: Option<&'static str>) -> Instructions {
    self.instruction = Some(Instructions::CloneRepository(CloneRepository::new(
      url, path,
    )));
    Instructions::from_instruction(self)
  }

  pub fn request_sudo(mut self, reason: &'static str) -> Instructions {
    self.instruction = Some(Instructions::RequestSudo(RequestSudo::new(reason)));
    Instructions::from_instruction(self)
  }

  pub fn restart_service(mut self, service_name: &'static str) -> Instructions {
    self.instruction = Some(Instructions::RestartService(RestartService::new(
      service_name,
    )));
    Instructions::from_instruction(self)
  }

  pub fn backup_file(mut self, path: &'static str) -> Instructions {
    self.instruction = Some(Instructions::BackupFile(BackupFile::new(path)));
    Instructions::from_instruction(self)
  }

  pub fn edit_file(
    mut self,
    path: &'static str,
    find: &'static str,
    replace: &'static str,
  ) -> Instructions {
    self.instruction = Some(Instructions::EditFile(EditFile::new(path, find, replace)));
    Instructions::from_instruction(self)
  }

  pub fn add_to_path(mut self, path: &'static str) -> Instructions {
    self.path_to_add = Some(path);
    Instructions::from_instruction(self)
  }

  pub fn execute(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(ref instruction) = self.instruction {
      instruction.run()?;
    }

    if let Some(path) = self.path_to_add {
      #[cfg(windows)]
      {
        std::process::Command::new("setx")
          .args(&["PATH", &format!("%PATH%;{}", path)])
          .status()?;
      }
      #[cfg(unix)]
      {
        println!("Add '{}' to your PATH", path);
      }
    }

    Ok(())
  }
}
