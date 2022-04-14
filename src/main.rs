use cli::cli::CLI;

pub mod cli;
pub mod commands;
pub mod config;

fn main() {
    let cli = CLI::run();
}
