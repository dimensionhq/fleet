use std::process::exit;

use colored::Colorize;

pub mod app;
pub mod types;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn build_warning(err: &str) {
    println!(
        "{}: {}
    ",
        "warning".yellow().bold(),
        err,
    );
}

pub fn build_error(err: &str) {
    println!(
        "{}: {}

{}: Consider opening an issue here https://github.com/dimensionhq/fleet/issues
    ",
        "error".red().bold(),
        err,
        "Help".green().bold()
    );
    exit(1)
}
