use crate::manager::SoftwareBundle;
use crate::packages::{ide, js};

pub fn bundle() -> SoftwareBundle {
  SoftwareBundle::new("Projektwoche", "A Bundle containing Packages to set up a development environment for the Projektwoche of the Athenaeum Stade")
      .add_program(js::nodejs())
      .add_program(js::bun())
      .add_program(ide::vscode())
}
