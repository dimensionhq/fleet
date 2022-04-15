use crate::commands::init::init;
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

use crate::cli::app::App;

pub mod app;

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
        let app = App::new();

        match cli.subcommand {
            Command::Init => init(app),
            Command::Run => {
                init(app);
                std::process::Command::new("cargo")
                    .arg("run")
                    .status()
                    .unwrap();
            }
            Command::Build => {
                init(app);
                std::process::Command::new("cargo")
                    .arg("build")
                    .status()
                    .unwrap();
            }
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
