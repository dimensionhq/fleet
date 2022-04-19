use std::process::exit;

use crate::cli::app::App;
use crate::commands::init::init;
use ansi_term::Colour::{Cyan, Purple, Yellow};
use clap::{crate_authors, crate_description, crate_name, crate_version, Parser};

pub mod app;

#[derive(Debug, Parser)]
pub enum Command {
    /// Run a Fleet project
    Run,
    /// Build a Fleet project
    Build,
}

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
)]
// ignore clap parsing errors
pub struct CLI {
    #[clap(subcommand)]
    pub subcommand: Command,
}

impl CLI {
    pub fn handle_failure() {
        // check if it's a configuration issue
        match rustc_version::version_meta().unwrap().channel {
            rustc_version::Channel::Nightly => {
                // no issues here
            }
            _ => {
                println!(
                    "{} You are not using a {} compiler. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`nightly`"),
                    Cyan.paint("`rustup default nightly`"),
                );
            }
        }

        // check if sccache is installed
        let sccache_path = std::path::Path::new(&dirs::home_dir().unwrap())
            .join(".cargo")
            .join("bin")
            .join("sccache");

        if !sccache_path.exists() {
            println!(
                "{} You have not installed {}. Run {}.",
                Yellow.paint("=>"),
                Purple.paint("`sccache`"),
                Cyan.paint("`cargo install sccache`"),
            );
        }

        // check if lld is available (on linux) and zld on macos
        if cfg!(unix) {
            let lld_path = std::path::Path::new("/usr/bin/lld");

            if !lld_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`lld`"),
                    Cyan.paint("`sudo apt install lld`"),
                );
            }

            // check if clang is available
            let clang_path = std::path::Path::new("/usr/bin/clang");

            if !clang_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`clang`"),
                    Cyan.paint("`sudo apt install clang`"),
                );
            }
        } else if cfg!(macos) {
            let zld_path = std::path::Path::new("/usr/bin/zld");

            if !zld_path.exists() {
                println!(
                    "{} You have not installed {}. Run {}.",
                    Yellow.paint("=>"),
                    Purple.paint("`zld`"),
                    Cyan.paint("`brew install zld`"),
                );
            }
        }

        exit(1);
    }

    pub fn run() {
        let cli = CLI::parse();
        let app = App::new();

        match cli.subcommand {
            Command::Run => {
                init(app);

                // Run the crate
                let status = std::process::Command::new("cargo")
                    .arg("run")
                    .status()
                    .unwrap();

                if !status.success() {
                    CLI::handle_failure();
                }
            }
            Command::Build => {
                init(app);

                let status = std::process::Command::new("cargo")
                    .arg("build")
                    .status()
                    .unwrap();

                if !status.success() {
                    CLI::handle_failure();
                }
            }
        }
    }
}
