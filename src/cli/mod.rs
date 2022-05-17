/*
 *
 *    Copyright 2021 Fleet Contributors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use ansi_term::Colour::{Cyan, Green, Purple, Yellow};
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Parser};
use colored::Colorize;
use std::{env, path::PathBuf, process::exit};

use crate::commands::init::enable_fleet;

use self::{
    app::App,
    help::{build_build_help_message, build_run_help_message},
};

pub mod app;
mod help;
pub mod prompt;

#[derive(Debug, Parser)]
pub enum Command {
    // Initialize a Fleet project
    Init,
    /// Run a Fleet project
    Run,
    /// Build a Fleet project
    Build,
    /// Configure a Fleet project
    Configure,
}

#[derive(Debug, Parser)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
    global_setting = AppSettings::ArgRequiredElseHelp
)]

pub struct CLI {
    #[clap(subcommand)]
    pub subcommand: Command,
}

impl CLI {
    // pub fn parse()
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

        let cargo_home_path = std::env::var("CARGO_HOME");
        let mut cargo_path = dirs::home_dir().unwrap().join(".cargo").join("bin");
        if let Ok(cargo_home) = cargo_home_path {
            cargo_path = PathBuf::from(cargo_home).join("bin");
        }

        // check if sccache is installed
        let mut sccache_path = cargo_path.join("sccache");
        if cfg!(windows) {
            sccache_path = cargo_path.join("sccache.exe");
        }

        if !sccache_path.exists() {
            println!(
                "{} You have not installed {}. Run {}.",
                Yellow.paint("=>"),
                Purple.paint("`sccache`"),
                Cyan.paint("`cargo install sccache`"),
            );
        }

        // check if lld is available (on linux) and zld on macos
        if cfg!(linux) {
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

    pub fn display_help(cmd: &str) {
        let mut help_menu = format!(
            r#"{} {}
Dimension <team@dimension.dev>
The blazing fast build tool for Rust.

{}:
    fleet <SUBCOMMAND>

{}:
    -h, --help            Print help information
    -v, --version         Print version information
    -up, --update-path    Update paths in .cargo/config.toml

{}:
    build    Build a Fleet project
    run      Run a Fleet project
    configure Configure a Fleet project"#,
            Green.paint("fleet"),
            env!("CARGO_PKG_VERSION"),
            Yellow.paint("USAGE"),
            Yellow.paint("OPTIONS"),
            Yellow.paint("SUBCOMMANDS"),
        );

        if cmd == "run" {
            help_menu = build_run_help_message()
        } else if cmd == "build" {
            help_menu = build_build_help_message()
        }
        println!("{}", help_menu)
    }

    pub fn run() {
        #[cfg(windows)]
        let _ = ansi_term::enable_ansi_support();

        let args = std::env::args().collect::<Vec<String>>();
        let app = App::new();

        if args.len() <= 1 {
            CLI::display_help("help");
        } else {
            let cmd = &args[1];

            if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
                CLI::display_help(cmd);
                std::process::exit(1)
            }
            const VERSION: &str = env!("CARGO_PKG_VERSION");

            if args.contains(&String::from("--version")) || args.contains(&String::from("-v")) {
                println!("{}", VERSION);
                std::process::exit(1)
            }

            if args.contains(&String::from("--update-path")) || args.contains(&String::from("-up"))
            {
                enable_fleet(app);
                std::process::exit(1)
            }

            match cmd.as_str() {
                "run" => {
                    enable_fleet(app);

                    // get all args after the subcommand
                    let args: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();
                    // Run the crate
                    let status = std::process::Command::new("cargo")
                        .arg("run")
                        .args(args)
                        .status()
                        .unwrap();

                    if !status.success() {
                        CLI::handle_failure();
                    }
                }
                "build" => {
                    enable_fleet(app);

                    let args: Vec<String> = args.iter().skip(2).map(|s| s.to_string()).collect();

                    let status = std::process::Command::new("cargo")
                        .arg("build")
                        .args(args)
                        .status()
                        .unwrap();

                    if !status.success() {
                        CLI::handle_failure();
                    }
                }
                "configure" => {
                    let prompt = format!("Select a {}:", "Linker".bright_cyan());

                    let linker_options = match std::env::consts::OS {
                        "windows" => {
                            vec![std::borrow::Cow::Owned(format!(
                                "🚄 lld - {} faster",
                                "4x".bright_cyan()
                            ))]
                        }
                        "macos" => {
                            vec![
                                std::borrow::Cow::Owned(format!(
                                    "🚀 zld - {} faster",
                                    "6x".bright_cyan()
                                )),
                                std::borrow::Cow::Owned(format!(
                                    "🚄 lld - {} faster",
                                    "4x".bright_cyan()
                                )),
                            ]
                        }
                        "linux" => {
                            vec![
                                std::borrow::Cow::Owned(format!(
                                    "🚀 mold - {} faster",
                                    "20x".bright_cyan()
                                )),
                                std::borrow::Cow::Owned(format!(
                                    "🚄 lld - {} faster",
                                    "5x".bright_cyan()
                                )),
                            ]
                        }
                        &_ => Vec::new(),
                    };

                    let select = prompt::prompts::Select {
                        message: std::borrow::Cow::Borrowed(prompt.as_str()),
                        paged: false,
                        selected: None,
                        items: linker_options.clone(),
                    };

                    let linker_selected = linker_options[select.run().unwrap()].to_string();

                    crate::utils::configure::install_linker(&linker_selected);
                }
                _ => {}
            }
        }
    }
}
