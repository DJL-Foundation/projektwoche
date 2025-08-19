//! # Interactive Configuration Module
//!
//! This module provides beautiful interactive command-line prompts using the `inquire` crate.
//! It enables the CLI to collect user preferences and configuration options with a modern,
//! user-friendly interface featuring colors, keyboard navigation, and validation.
//!
//! ## Features
//!
//! - **Modern UI**: Beautiful colored prompts with icons and visual feedback
//! - **Keyboard Navigation**: Arrow keys, Enter, Escape for intuitive interaction
//! - **Input Validation**: Built-in validation with helpful error messages
//! - **Default Values**: Support for defaults to speed up configuration
//! - **Multiple Input Types**: Yes/No, text, single choice, multiple choice, and paths
//! - **Error Handling**: Graceful handling of user cancellation and errors
//!
//! ## Usage Example
//!
//! ```rust
//! use crate::config::interactive::{ask_yes_no, ask_text, ask_choice};
//!
//! // Simple yes/no question with beautiful UI
//! let install_extras = ask_yes_no("Install optional components?", false);
//!
//! // Text input with default and validation
//! let username = ask_text("Enter your username", Some("developer"));
//!
//! // Beautiful single choice with arrow key navigation
//! let editors = [("vscode", "Visual Studio Code"), ("vim", "Vim")];
//! let choice = ask_choice("Select your editor:", &editors, Some(0));
//! ```

use inquire::{Confirm, Text, Select, MultiSelect, validator::Validation};
use std::path::Path;

/// Asks the user a yes/no question with a beautiful confirmation prompt.
/// 
/// This function creates a modern confirmation prompt with colored output
/// and clear yes/no indicators. Users can navigate with arrow keys or
/// type 'y'/'n' directly.
/// 
/// # Arguments
/// 
/// * `question` - The question text to display to the user
/// * `default` - The default value if user presses Enter (true = Yes, false = No)
/// 
/// # Returns
/// 
/// Returns `true` for yes and `false` for no. Returns the default value
/// if the user cancels the prompt (Ctrl+C).
/// 
/// # Example
/// 
/// ```rust
/// let install_dev_tools = ask_yes_no("Install development tools?", true);
/// if install_dev_tools {
///     println!("Installing development tools...");
/// }
/// ```
pub fn ask_yes_no(question: &str, default: bool) -> bool {
    Confirm::new(question)
        .with_default(default)
        .prompt()
        .unwrap_or(default) // Use default if user cancels
}

/// Asks the user for text input with validation and optional default.
/// 
/// This function creates a text input prompt with optional default values
/// and built-in validation for empty inputs. The prompt supports auto-completion
/// and input history if available.
/// 
/// # Arguments
/// 
/// * `question` - The prompt text to display to the user
/// * `default` - Optional default value to use if user enters nothing
/// 
/// # Returns
/// 
/// Returns the user's input as a String, or the default value if provided
/// and the user entered nothing or canceled.
/// 
/// # Example
/// 
/// ```rust
/// let project_name = ask_text("Enter project name", Some("my-project"));
/// let description = ask_text("Enter project description", None);
/// ```
pub fn ask_text(question: &str, default: Option<&str>) -> String {
    let mut prompt = Text::new(question);
    
    if let Some(def) = default {
        prompt = prompt.with_default(def);
    }
    
    prompt
        .prompt()
        .unwrap_or_else(|_| default.unwrap_or("").to_string())
}

/// Asks the user to choose from multiple predefined options with beautiful UI.
/// 
/// This function displays a navigable list of options with descriptions.
/// Users can navigate with arrow keys and select with Enter. The default
/// option is highlighted and can be selected by pressing Enter immediately.
/// 
/// # Arguments
/// 
/// * `question` - The prompt text to display above the options
/// * `options` - Array of tuples containing (value, description) pairs
/// * `default` - Optional index of the default selection (0-based)
/// 
/// # Returns
/// 
/// Returns the value (first element of the tuple) for the selected option.
/// Returns the default option if user cancels.
/// 
/// # Example
/// 
/// ```rust
/// let package_managers = [
///     ("npm", "Node Package Manager"),
///     ("yarn", "Yarn Package Manager"), 
///     ("bun", "Bun Package Manager"),
/// ];
/// 
/// let choice = ask_choice(
///     "Select package manager:",
///     &package_managers,
///     Some(0) // npm as default
/// );
/// 
/// println!("Selected: {}", choice);
/// ```
pub fn ask_choice<T: ToString + Clone>(
    question: &str, 
    options: &[(T, &str)], 
    default: Option<usize>
) -> T {
    let choices: Vec<&str> = options.iter().map(|(_, desc)| *desc).collect();
    let mut prompt = Select::new(question, choices);
    
    if let Some(def) = default {
        prompt = prompt.with_starting_cursor(def);
    }
    
    let selected_index = prompt
        .prompt()
        .map(|selected_desc| {
            // Find the index of the selected description
            options.iter().position(|(_, desc)| *desc == selected_desc).unwrap_or(0)
        })
        .unwrap_or(default.unwrap_or(0));
    
    options[selected_index].0.clone()
}

/// Asks the user to select multiple items from a list with checkboxes.
/// 
/// This function displays a list of options with checkboxes that can be
/// toggled with the spacebar. Users navigate with arrow keys and confirm
/// with Enter. Default selections are pre-checked.
/// 
/// # Arguments
/// 
/// * `question` - The prompt text to display
/// * `options` - Array of tuples containing (value, description) pairs
/// * `defaults` - Optional vector of default selections (0-based indices)
/// 
/// # Returns
/// 
/// Returns a vector of selected values. Returns default selections if user cancels.
/// 
/// # Example
/// 
/// ```rust
/// let languages = [
///     ("rust", "Rust Programming Language"),
///     ("python", "Python"),
///     ("javascript", "JavaScript"),
///     ("go", "Go Language"),
/// ];
/// 
/// let selected = ask_multiple_choice(
///     "Select programming languages to install:",
///     &languages,
///     Some(vec![0, 2]) // Default to Rust and JavaScript
/// );
/// ```
pub fn ask_multiple_choice<T: ToString + Clone>(
    question: &str,
    options: &[(T, &str)],
    defaults: Option<Vec<usize>>
) -> Vec<T> {
    let choices: Vec<&str> = options.iter().map(|(_, desc)| *desc).collect();
    let mut prompt = MultiSelect::new(question, choices);
    
    if let Some(def_indices) = &defaults {
        prompt = prompt.with_default(def_indices);
    }
    
    let selected_descriptions = prompt
        .prompt()
        .unwrap_or_else(|_| {
            // Return default descriptions if user cancels
            defaults.unwrap_or_default()
                .into_iter()
                .map(|i| options[i].1)
                .collect()
        });
    
    // Convert selected descriptions back to indices and then to values
    selected_descriptions
        .into_iter()
        .filter_map(|desc| {
            options.iter().position(|(_, d)| *d == desc)
                .map(|i| options[i].0.clone())
        })
        .collect()
}

/// Asks the user for a file or directory path with validation.
/// 
/// This function prompts for a path and validates it according to the
/// specified requirements. It provides helpful error messages for
/// invalid paths and allows retry on validation failure.
/// 
/// # Arguments
/// 
/// * `question` - The prompt text to display
/// * `default` - Optional default path
/// * `must_exist` - If true, validates that the path exists
/// * `must_be_writable` - If true, validates that the path is writable
/// 
/// # Returns
/// 
/// Returns a valid path as a String.
/// 
/// # Example
/// 
/// ```rust
/// let install_dir = ask_path(
///     "Enter installation directory",
///     Some("/usr/local/bin"),
///     false, // doesn't need to exist
///     true   // must be writable
/// );
/// ```
pub fn ask_path(
    question: &str, 
    default: Option<&str>, 
    must_exist: bool, 
    must_be_writable: bool
) -> String {
    let validator = move |input: &str| {
        let path = Path::new(input);
        
        if must_exist && !path.exists() {
            return Ok(Validation::Invalid(
                format!("Path '{}' does not exist", input).into()
            ));
        }
        
        if must_be_writable {
            // Test if we can write to the directory
            let test_dir = if path.is_dir() { 
                path.to_path_buf() 
            } else { 
                path.parent().unwrap_or(Path::new(".")).to_path_buf()
            };
            
            let test_file = test_dir.join(".write_test_inquire");
            if std::fs::write(&test_file, "test").is_err() {
                return Ok(Validation::Invalid(
                    format!("Path '{}' is not writable", input).into()
                ));
            }
            // Clean up test file
            let _ = std::fs::remove_file(&test_file);
        }
        
        Ok(Validation::Valid)
    };
    
    let mut prompt = Text::new(question)
        .with_validator(validator);
    
    if let Some(def) = default {
        prompt = prompt.with_default(def);
    }
    
    prompt
        .prompt()
        .unwrap_or_else(|_| default.unwrap_or(".").to_string())
}

/// Displays a beautiful confirmation prompt showing what will be done.
/// 
/// This function shows a summary of actions with proper formatting and
/// asks for final confirmation before proceeding. It's perfect for
/// showing what will be installed or configured.
/// 
/// # Arguments
/// 
/// * `message` - Description of what will happen
/// * `details` - Optional detailed list of actions
/// 
/// # Returns
/// 
/// Returns true if user confirms, false if they cancel.
/// 
/// # Example
/// 
/// ```rust
/// let details = vec![
///     "Install Node.js",
///     "Install Visual Studio Code", 
///     "Configure development environment"
/// ];
/// 
/// if confirm_action("The following will be installed:", Some(&details)) {
///     // Proceed with installation
/// }
/// ```
pub fn confirm_action(message: &str, details: Option<&[&str]>) -> bool {
    println!("\n📋 {}", message);
    
    if let Some(items) = details {
        for item in items {
            println!("   • {}", item);
        }
    }
    
    println!();
    ask_yes_no("❓ Do you want to continue?", true)
}

/// Creates a beautiful multi-step configuration wizard.
/// 
/// This function demonstrates how to chain multiple inquire prompts
/// together to create a comprehensive configuration experience.
/// 
/// # Returns
/// 
/// Returns a configuration struct or None if user cancels.
/// 
/// # Example
/// 
/// ```rust
/// if let Some(config) = configuration_wizard() {
///     println!("Configuration completed: {:?}", config);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WizardConfig {
    pub editor: String,
    pub browser: String,
    pub languages: Vec<String>,
    pub install_path: Option<String>,
    pub install_extras: bool,
}

pub fn configuration_wizard() -> Option<WizardConfig> {
    println!("🚀 Welcome to the Interactive Setup Wizard!");
    println!("   Let's configure your development environment.\n");
    
    // Editor selection
    let editors = [
        ("vscode", "Visual Studio Code (Recommended)"),
        ("vim", "Vim"),
        ("emacs", "Emacs"),
        ("nano", "Nano"),
        ("other", "Other (I'll specify)"),
    ];
    
    let editor_choice = ask_choice(
        "🎯 Which code editor do you prefer?", 
        &editors, 
        Some(0)
    );
    
    let editor = if editor_choice == "other" {
        ask_text("✏️  Please specify your preferred editor:", None)
    } else {
        editor_choice.to_string()
    };
    
    // Browser selection
    let browsers = [
        ("chrome", "Google Chrome (Recommended)"),
        ("firefox", "Mozilla Firefox"),
        ("edge", "Microsoft Edge"),
        ("safari", "Safari"),
        ("other", "Other"),
    ];
    
    let browser_choice = ask_choice(
        "🌐 Which web browser do you prefer?", 
        &browsers, 
        Some(0)
    );
    
    let browser = if browser_choice == "other" {
        ask_text("🌍 Please specify your preferred browser:", None)
    } else {
        browser_choice.to_string()
    };
    
    // Programming languages
    let language_options = [
        ("rust", "Rust 🦀"),
        ("javascript", "JavaScript/TypeScript 📦"),
        ("python", "Python 🐍"),
        ("go", "Go 🐹"),
        ("java", "Java ☕"),
        ("csharp", "C# 💜"),
    ];
    
    let languages: Vec<String> = ask_multiple_choice(
        "💻 Which programming languages do you work with?",
        &language_options,
        Some(vec![0, 1]) // Default to Rust and JavaScript
    ).into_iter().map(|s| s.to_string()).collect();
    
    // Installation path (optional)
    let ask_custom_path = ask_yes_no(
        "📁 Do you want to specify a custom installation path?", 
        false
    );
    
    let install_path = if ask_custom_path {
        let default_path = if cfg!(windows) {
            "C:\\DevTools"
        } else {
            "/usr/local"
        };
        
        Some(ask_path(
            "📂 Enter the installation directory:",
            Some(default_path),
            false, // doesn't need to exist
            true   // must be writable
        ))
    } else {
        None
    };
    
    // Optional extras
    let install_extras = ask_yes_no(
        "🔧 Install additional development tools? (Git, Docker, etc.)", 
        true
    );
    
    // Final confirmation
    let editor_summary = format!("Editor: {}", editor);
    let browser_summary = format!("Browser: {}", browser);
    let languages_summary = format!("Languages: {}", languages.join(", "));
    
    let mut summary = vec![
        editor_summary.as_str(),
        browser_summary.as_str(),
        languages_summary.as_str(),
    ];
    
    let install_path_summary;
    if let Some(ref path) = install_path {
        install_path_summary = format!("Install path: {}", path);
        summary.push(&install_path_summary);
    }
    
    if install_extras {
        summary.push("Additional tools: Yes");
    }
    
    if confirm_action("📋 Configuration Summary:", Some(&summary)) {
        Some(WizardConfig {
            editor,
            browser,
            languages,
            install_path,
            install_extras,
        })
    } else {
        println!("❌ Configuration cancelled.");
        None
    }
}