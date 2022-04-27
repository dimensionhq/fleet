use std::path::PathBuf;

use enable_ansi_support::enable_ansi_support;

use crate::build_warning;

/// The application's configuration & variables
/// which will be used a lot.
/// ```rs
/// let app = App::new();
/// ```
#[derive(Debug)]
pub struct App {
    pub home_dir: PathBuf,
    pub home_config_file: PathBuf,
    pub base_config_file: PathBuf,
    pub current_dir: PathBuf,

    pub args: Vec<String>,
    pub flags: Vec<String>,
    pub flags_with_a_plus: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates a new instance of the application.
    pub fn new() -> Self {
        let arguments: Vec<String> = std::env::args().collect();

        if enable_ansi_support().is_err() {
            build_warning("Failed to initialize ANSI support");
        }

        let mut flags = Vec::new();
        let mut args = Vec::new();
        let mut flag_with_a_plus = Vec::new();
        for arg in arguments {
            if arg.starts_with("--") || arg.starts_with('-') {
                flags.push(arg);
            } else if arg.starts_with('+') {
                flag_with_a_plus.push(arg);
            } else {
                args.push(arg);
            }
        }

        let home_dir = dirs::home_dir().unwrap();

        let current_dir = std::env::current_dir().unwrap();
        let config_file = current_dir.join("fleet/fleet.toml");
        let base_config_file = home_dir.join(".config/fleet/base_config.toml");

        crate::types::config::global::Config::new(base_config_file.clone());
        Self {
            home_dir,
            home_config_file: config_file,
            base_config_file,
            current_dir,
            args,
            flags,
            flags_with_a_plus: flag_with_a_plus,
        }
    }
    pub fn get_path(s: &str) -> String {
        let mut prefix = String::from("which");
        if cfg!(windows) {
            prefix = String::from("where");
        }

        let clang = std::process::Command::new(prefix)
            .arg(s)
            .output()
            .unwrap()
            .stdout;

        let clang = String::from_utf8(clang).unwrap();

        clang
    }
}
