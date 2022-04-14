use std::process::exit;

use crate::commands::init::init;
use ansi_term::Color::Red;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, AppSettings, Parser, Subcommand,
};

#[derive(Debug, Parser)]
pub enum Command {
    /// Initialize a new turbo project
    Init,
    /// Run a turbo project
    Run,
    /// Build a turbo project
    Build,
    /// Configure the dependencies for a turbo project
    Configure,
}

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
)]
pub struct CLI {
    #[clap(subcommand)]
    pub subcommand: Command,
}
impl CLI {
    pub fn run() {
        let cli = CLI::parse();
        match cli.subcommand {
            Command::Init => init(),
            Command::Run => {
                init();
                std::process::Command::new("cargo")
                    .arg("run")
                    .status()
                    .unwrap();
            }
            Command::Build => todo!(),
            Command::Configure => {
                std::process::Command::new("cargo")
                    .arg("install")
                    .arg("sccache")
                    .status()
                    .unwrap();
            }
        }
    }
}
