use crate::config::machine::{OsCategory, OsMatcher};
use crate::manager::instructions::Instruction;
use crate::manager::{InstructionMapping, Package};

pub fn vscode() -> Package {
  Package::new("Visual Studio Code", "Code editor").add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new().add_install_instructions(vec![
      Instruction::new("Download VSCode").download_and_exec(
        "https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-user",
      ),
    ]),
  )
}
