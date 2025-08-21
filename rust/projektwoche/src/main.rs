mod logger;

use clap::{Parser, Subcommand};
use logger::LogLevel;

#[derive(Parser, Debug)]
#[clap(
  author = "JackatDJL",
  version,
  about = "The Projektwoche CLI",
  long_about = "A command-line interface to sync a basic html project to our hosting solution cross repository"
)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Setup your environment
  Setup,
  /// Configuration management
  Config {
    #[clap(subcommand)]
    action: ConfigAction,
  },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
  /// Set log level
  LogLevel {
    #[clap(subcommand)]
    action: LogLevelAction,
  }
}

#[derive(Subcommand, Debug)]
enum LogLevelAction {
  /// Set to default
  Default,
  /// Show current value
  Show,
  /// Set to a specific value
  Set { value: LogLevel },
}

fn main() {
  println!("Hello, world!");
}
