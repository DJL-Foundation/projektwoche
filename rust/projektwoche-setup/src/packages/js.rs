use crate::config::machine::{OsCategory, OsMatcher};
use crate::manager::instructions::Instruction;
use crate::manager::{InstructionMapping, Package};

pub fn nodejs() -> Package {
  Package::new("Node.js", "JavaScript runtime").add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new().add_install_instructions(vec![
      Instruction::new("Install nvm").cmd("wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash"),      Instruction::new("Install Node.js").cmd("nvm install latest"),
      Instruction::new("Set Node.js version").cmd("nvm use latest"),
      Instruction::new("Add Node.js to PATH").cmd("setx PATH \"%PATH%;%NVM_HOME%\\nodejs\\node_modules\\.bin\""),
    ]),
  ).add_mapping(
    OsMatcher::from_category(OsCategory::LinuxBased),
    InstructionMapping::new().add_install_instructions(vec![
      Instruction::new("Install nvm").cmd("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash"),
      Instruction::new("Install Node.js").cmd("nvm install latest"),
      Instruction::new("Set Node.js version").cmd("nvm use latest"),
      Instruction::new("Add Node.js to PATH").cmd("echo 'export PATH=\"$PATH:$HOME/.nvm/versions/node/$(nvm version)/bin\"' >> ~/.bashrc"),
      Instruction::new("Reload shell").cmd("source ~/.bashrc"),
    ]),
  )
}

pub fn bun() -> Package {
  Package::new("Bun", "JavaScript runtime and package manager")
    .add_mapping(
      OsMatcher::from_category(OsCategory::Windows),
      InstructionMapping::new().add_install_instructions(vec![
        Instruction::new("Install Bun").cmd("powershell -c \"irm bun.sh/install.ps1 | iex\""),
      ]),
    )
    .add_mapping(
      OsMatcher::from_category(OsCategory::LinuxBased),
      InstructionMapping::new().add_install_instructions(vec![
        Instruction::new("Install Bun").cmd("curl -fsSL https://bun.sh/install | bash"),
      ]),
    )
}
