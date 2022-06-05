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

use crate::core::config::FleetConfig;
use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, Command as CliCommand, Values,
};
use colored::Colorize;
use std::{env::current_dir, path::PathBuf};

use crate::cli::help;
use crate::core::commands::{bloat, build, configure, init, run};
use anyhow::Result;
use std::process::{self, exit};

pub enum Command {
    Init(Option<Values<'static>>),
    Build(Option<Values<'static>>),
    Run(Option<Values<'static>>),
    Bloat(Option<Values<'static>>),
    Configure(Option<Values<'static>>),
}

pub struct App {
    pub config: FleetConfig,
    pub current_dir: PathBuf,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    ///Creates a new app
    ///
    /// # Panics
    /// Can panic if current directory is not found (eg. doesn't exist or invalid perms)
    #[must_use]
    pub fn new() -> Self {
        let current_dir = current_dir().expect("Unable to find current directory for app!");
        // let config = FleetConfig::new();
        // check
        // let config = config.run_config();

        //wasn't using the self argument anyway, so unsure of use
        Self {
            config: FleetConfig::run_config(),
            current_dir,
        }
    }

    fn build<'a>(&self) -> CliCommand<'a> {
        CliCommand::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
            .arg(arg!(-c --command <CMD>).required(false).default_missing_value(""))
            .subcommand(
                CliCommand::new("init")
                    .about("Initialize a fleet project")
                    .arg(arg!([EXTRA]).multiple_values(true)),
            )
            .subcommand(
                CliCommand::new("run")
                    .about("Runs the fleet project")
                    .arg(arg!([EXTRA]).multiple_values(true)),
            )
            .subcommand(
                CliCommand::new("build")
                    .about("Builds a fleet project")
                    .arg(arg!([EXTRA]).multiple_values(true)),
            )
            .subcommand(CliCommand::new("configure").about("Configure a fleet project"))
            .subcommand(CliCommand::new("bloat").about("?"))
    }

    fn get_command(&self) -> Command {
        let mut options = self.build();

        // There should be a better way to implement this
        let matches = Box::leak(options.clone().get_matches().into());

        match matches.value_of("command") {
            Some(cmd) => {
                match cmd {
                    "run" => {
                        println!("{}", help::run_help())
                    }
                    "build" => {
                        println!("{}", help::build_help())
                    }
                    _ => options.print_help().unwrap(),
                }

                process::exit(1)
            }
            None => {}
        }

        match matches.subcommand() {
            Some(("init", _sub)) => Command::Init(None),
            Some(("build", sub)) => Command::Build(sub.values_of("EXTRA")),
            Some(("run", sub)) => Command::Run(sub.values_of("EXTRA")),
            Some(("bloat", _sub)) => Command::Bloat(None),
            Some(("configure", _sub)) => Command::Configure(None),
            _ => {
                options.print_help().unwrap_or_else(|_| {
                    eprintln!("{}", "Failed to display help.".red(),);
                    exit(1);
                });
                exit(0)
            }
        }
    }

    pub fn run(self) -> Result<()> {
        let command = self.get_command();

        match command {
            Command::Init(args) => init::run(self, args),
            Command::Build(args) => build::run(self, args),
            Command::Run(args) => run::run(self, args),
            Command::Bloat(args) => bloat::run(self, args),
            Command::Configure(args) => configure::run(self, args),
        }
    }
}
