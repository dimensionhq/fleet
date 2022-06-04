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
use cargo_util::ProcessBuilder;
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Parser};
use colored::Colorize;
use comfy_table::ContentArrangement;
use comfy_table::Width::Percentage;
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, ColumnConstraint,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::{env, path::PathBuf, process::exit};

use crate::{
    commands::init::enable_fleet,
    utils::bloat::{BloatCrateAnalysis, BloatFunctionAnalysis},
};

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
    /// # Panics
    /// Can panic if unable to access rust version meta
    pub fn handle_failure() {
        // check if it's a configuration issue
        match rustc_version::version_meta()
            .expect("Unable to see rust version meta")
            .channel
        {
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
            help_menu = build_run_help_message();
        } else if cmd == "build" {
            help_menu = build_build_help_message();
        }
        println!("{}", help_menu);
    }

    /// # Panics
    /// Can panic if unable to cargo run
    pub fn run() {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

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
                    let args: Vec<String> = args
                        .iter()
                        .skip(2)
                        .map(std::string::ToString::to_string)
                        .collect();
                    // Run the crate
                    let status = std::process::Command::new("cargo")
                        .arg("run")
                        .args(args)
                        .status()
                        .expect("Unable to cargo run");

                    if !status.success() {
                        CLI::handle_failure();
                    }
                }
                "build" => {
                    enable_fleet(app);

                    let args: Vec<String> = args
                        .iter()
                        .skip(2)
                        .map(std::string::ToString::to_string)
                        .collect();

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
                "bloat" => {
                    let spinner = ProgressBar::new_spinner();

                    spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}"));

                    spinner.set_message("Initializing".bright_green().to_string());

                    spinner.enable_steady_tick(10);

                    // Run cargo bloat
                    let mut command = ProcessBuilder::new("cargo");

                    command
                        .arg("bloat")
                        .arg("--crates")
                        .arg("--message-format=json");

                    // TODO: run both analysis at the same time

                    let mut warning_count = 0;
                    let mut error_count = 0;

                    let output = command.exec_with_streaming(
                        &mut |_| Ok(()),
                        &mut |on_stderr| {
                            let contents = on_stderr.trim().to_string();

                            if contents != "" {
                                let chunks: Vec<&str> = contents.split(" ").collect();

                                if contents.starts_with("Compiling") {
                                    let name = chunks[1].to_string();

                                    let mut version = chunks[2].to_string();

                                    if version.starts_with("v") {
                                        version.remove(0);
                                    }

                                    spinner.set_message(format!(
                                        "{} ({}{}{})",
                                        "Compile".bright_cyan(),
                                        name.bright_yellow(),
                                        "@".bright_magenta(),
                                        version.bright_black(),
                                    ));
                                }

                                if contents.starts_with("warning:") {
                                    warning_count += 1;
                                    spinner.set_message(format!(
                                        "{} ({} {}, {} {})",
                                        "Check".bright_cyan(),
                                        warning_count.to_string().bright_magenta(),
                                        "warnings".bright_yellow(),
                                        error_count.to_string().bright_red(),
                                        "errors".bright_yellow(),
                                    ));
                                }

                                if contents.starts_with("error") {
                                    error_count += 1;
                                }
                            }

                            Ok(())
                        },
                        true,
                    );

                    if output.is_err() {
                        spinner.finish();

                        std::process::Command::new("cargo")
                            .arg("check")
                            .status()
                            .unwrap();

                        std::process::exit(1);
                    }

                    spinner.set_message("Analysing".bright_cyan().to_string());

                    let stdout = String::from_utf8(output.unwrap().stdout).unwrap();

                    let data = serde_json::from_str::<BloatCrateAnalysis>(&stdout).unwrap();

                    let total_size = byte_unit::Byte::from_bytes(data.file_size as u128);
                    let adjusted_size = total_size.get_appropriate_unit(true);

                    println!("Total Size: {}", adjusted_size.to_string().bright_yellow());

                    let mut crates_table = comfy_table::Table::new();

                    crates_table
                        .load_preset(UTF8_FULL)
                        .apply_modifier(UTF8_ROUND_CORNERS)
                        .set_content_arrangement(ContentArrangement::DynamicFullWidth);

                    crates_table.set_header(vec!["Name", "Size"]);

                    for _crate in data.crates.iter() {
                        let size = byte_unit::Byte::from_bytes(_crate.size as u128);
                        let adjusted_size = size.get_appropriate_unit(true);

                        crates_table.add_row(vec![
                            Cell::new(_crate.name.to_string()).fg(Color::Blue),
                            Cell::new(adjusted_size.to_string()).fg(Color::Cyan),
                        ]);
                    }

                    let output = std::process::Command::new("cargo")
                        .arg("bloat")
                        .arg("--message-format=json")
                        .output()
                        .unwrap();

                    let stdout = String::from_utf8(output.stdout).unwrap();

                    let data = serde_json::from_str::<BloatFunctionAnalysis>(&stdout).unwrap();

                    let mut function_table = comfy_table::Table::new();

                    function_table
                        .set_content_arrangement(ContentArrangement::Dynamic)
                        .load_preset(UTF8_FULL)
                        .apply_modifier(UTF8_ROUND_CORNERS);

                    function_table.set_header(vec!["Crate", "Function", "Size"]);

                    for function in data.functions.iter() {
                        let size = byte_unit::Byte::from_bytes(function.size as u128);
                        let adjusted_size = size.get_appropriate_unit(true);

                        function_table.add_row(vec![
                            Cell::new(function.crate_field.to_string()).fg(Color::Blue),
                            Cell::new(function.name.to_string()),
                            Cell::new(adjusted_size.to_string()).fg(Color::Cyan),
                        ]);
                    }

                    spinner.finish_and_clear();

                    println!("{crates_table}");
                    println!("{function_table}");

                    println!(
                        "{}: All sizes shown are estimates and will not be 100% accurate.",
                        "Note".bright_yellow()
                    );
                }
                "udeps" => {
                    println!("cargo udeps");
                }
                _ => {}
            }
        }
    }
}
