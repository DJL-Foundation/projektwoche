use crate::config::{OsCategory, OsMatcher};

use crate::manager::{InstructionMapping, Package, SoftwareBundle};

use crate::manager::instructions::{Filetype, Instruction};

fn nodejs() -> Package {
  Package::new("Node.js", "JavaScript runtime").add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new().add_install_instructions(vec![
      Instruction::new("Install windows_nvm").download_and_exec(
        Filetype::EXE,
        "https://github.com/coreybutler/nvm-windows/releases/download/1.2.2/nvm-setup.exe",
      ),
      Instruction::new("Install Node.js").cmd("nvm install latest"),
      Instruction::new("Set Node.js version").cmd("nvm use latest"),
      Instruction::new("Add Node.js to PATH").add_to_path("%NVM_HOME%\\nodejs\\node_modules\\.bin"),
    ]),
    // config func to add to windows path export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")" \n [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm
  )
}

fn bun() -> Package {
  Package::new("Bun", "JavaScript runtime and package manager")
    .add_mapping(
      OsMatcher::from_category(OsCategory::Windows),
      InstructionMapping::new().add_install_instructions(vec![
        Instruction::new("Install Bun").cmd("powershell -c \"irm bun.sh/install.ps1 | iex\""),
      ]),
    )
    .add_mapping(
      OsMatcher::from_categorys(&[OsCategory::LinuxBased, OsCategory::MacOS]),
      InstructionMapping::new().add_install_instructions(vec![
        Instruction::new("Install Bun").cmd("curl -fsSL https://bun.sh/install | bash"),
      ]),
    )
}

pub fn bundle() -> SoftwareBundle {
  SoftwareBundle::new("Projektwoche", "A Bundle containing Packages to set up a development environment for the Projektwoche of the Athenaeum Stade")
      .add_program(nodejs())
      .add_program(bun())
}
