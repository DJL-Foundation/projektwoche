# Projektwoche Setup

A fast and efficient CLI tool for setting up development environments using customizable software bundles.

## Overview

`projektwoche-setup` is a cross-platform package manager designed to quickly install and configure complete development environments. It uses a bundle-based approach where each bundle contains multiple related packages that are installed and configured together.

## Features

- **Bundle-based installation**: Install related packages together as cohesive bundles
- **Cross-platform support**: Works on Windows and Linux-based systems
- **Parallel installation**: Packages within bundles are installed concurrently for speed
- **Dry-run mode**: Preview what will be installed without making changes
- **Simple CLI interface**: Easy-to-use commands with helpful aliases

## Installation

### Recommended: Quick Install Script

The easiest way to install `projektwoche-setup`:

```bash
curl -fsSL https://prowo.hackclub-stade.de/setup/installer.sh | bash
```

### Download Pre-built Binary

Download the latest executable from the [releases page](https://github.com/DJL-Foundation/projektwoche/releases) and run it directly.

### Alternative Methods

#### From crates.io

```bash
cargo install projektwoche-setup
```

#### From source

```bash
git clone https://github.com/DJL-Foundation/projektwoche
cd projektwoche/rust/projektwoche-setup
cargo install --path .
```

## Usage

### Install a Bundle

```bash
# Install the Projektwoche bundle (default)
projektwoche-setup install projektwoche

# Or use the short alias
projektwoche-setup i projektwoche

# Preview installation without making changes
projektwoche-setup install --debug projektwoche
```

### Uninstall a Bundle

```bash
# Uninstall the Projektwoche bundle
projektwoche-setup uninstall projektwoche

# Or use the short alias
projektwoche-setup u projektwoche

# Preview uninstallation without making changes
projektwoche-setup uninstall --debug projektwoche
```

### Update the CLI Tool

```bash
# Update the CLI tool itself (coming soon)
projektwoche-setup self-update
```

## Available Bundles

### Projektwoche

A complete development environment for the Projektwoche project including:

- **Node.js**: JavaScript runtime via nvm
- **Bun**: Fast JavaScript runtime and package manager
- **Visual Studio Code**: Modern code editor

## Platform Support

- **Windows**: Full support with PowerShell-based installation
- **Linux**: Support for RHEL-based distributions and other Linux variants
- **macOS**: Coming soon

## Configuration

The tool automatically detects your operating system and uses appropriate installation methods. Configuration is stored locally and managed automatically.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests to the [main repository](https://github.com/DJL-Foundation/projektwoche).

## License

Licensed under the Apache License 2.0. See LICENSE file for details.

## Links

- [Homepage](https://prowo.hackclub-stade.de/setup)
- [Repository](https://github.com/DJL-Foundation/projektwoche/tree/main/rust/projektwoche-setup)
- [Issues](https://github.com/DJL-Foundation/projektwoche/issues)
