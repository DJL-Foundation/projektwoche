use crate::config::{OsCategory, OsMatcher};

use crate::manager::{InstructionMapping, Package, SoftwareBundle};

fn nodejs() -> Package {
  Package::new("Node.js", "JavaScript runtime").add_mapping(
    OsMatcher::from_category(OsCategory::Windows),
    InstructionMapping::new().add_install_instructions(vec![
      ExecutableCommand::new(
        "Install nvm",
        &["wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash"],
      ),
      ExecutableCommand::new("Install Node.js", &["nvm install"]),
    ]),
    // config func to add to windows path export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")" \n [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm
  )
}

pub fn bundle() -> SoftwareBundle {
  SoftwareBundle::new("Projektwoche", "A Bundle containing Packages to set up a development environment for the Projektwoche of the Athenaeum Stade")
      .add_program(nodejs())
}
