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

use std::collections::HashMap;

use crate::cli::app::App;
use anyhow::Result;
use cargo_util::ProcessBuilder;
use clap::Values;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use ptree::print_tree_with;
use ptree::Color;
use ptree::PrintConfig;
use ptree::Style;
use ptree::TreeBuilder;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UdepsAnalysis {
    pub success: bool,
    #[serde(rename = "unused_deps")]
    pub unused_deps: Option<HashMap<String, UnusedDep>>,
    pub note: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnusedDep {
    #[serde(rename = "manifest_path")]
    pub manifest_path: String,
    pub normal: Option<Vec<String>>,
    pub development: Option<Vec<String>>,
    pub build: Option<Vec<String>>,
}

pub fn pretty_print_notes(_args: Option<Values>) {
    let false_positive_note = format!(
        r#"
{}: There might be false positives.
      For example, `{}` cannot detect crates only used in doc-tests.
      To ignore dependencies, write `{}` in {}."#,
        "Note".bright_blue(),
        "fleet udeps".bright_cyan(),
        "package.metadata.cargo-udeps.ignore".bright_green(),
        "Cargo.toml".bright_yellow(),
    );

    println!("{false_positive_note}");

    let all_targets_note = format!(
        r#"
{}: These dependencies might be used by other targets.
      To find dependencies that are not used by any target, enable `{}`."#,
        "Note".bright_blue(),
        "--all-targets".bright_cyan(),
    );

    println!("{all_targets_note}");
}

pub fn pretty_print_udeps_analysis(analysis: UdepsAnalysis) {
    if let Some(unused_deps) = analysis.unused_deps {
        for (crate_name, dependencies) in unused_deps.iter() {
            let split = crate_name.split(' ').collect::<Vec<&str>>();

            let name = split[0].trim();
            let version = split[1].trim();

            let mut unused_dependencies_found = false;

            if let Some(normal) = &dependencies.normal {
                if !normal.is_empty() {
                    unused_dependencies_found = true;
                }
            }

            if let Some(development) = &dependencies.development {
                if !development.is_empty() {
                    unused_dependencies_found = true;
                }
            }

            if let Some(build) = &dependencies.build {
                if !build.is_empty() {
                    unused_dependencies_found = true;
                }
            }

            if unused_dependencies_found {
                let mut tree = TreeBuilder::new(format!(
                    "unused deps for {}{}{}",
                    name.bright_yellow(),
                    "@".bright_magenta(),
                    version.bright_black()
                ));

                if let Some(normal) = &dependencies.normal {
                    if !normal.is_empty() {
                        tree.begin_child("dependencies".bright_green().to_string());
                        for unused_dependency in normal {
                            tree.add_empty_child(unused_dependency.to_string());
                        }
                        tree.end_child();
                    }
                }

                if let Some(development) = &dependencies.development {
                    if !development.is_empty() {
                        tree.begin_child("dev-dependencies".bright_blue().to_string());
                        for unused_dependency in development {
                            tree.add_empty_child(unused_dependency.to_string());
                        }
                        tree.end_child();
                    }
                }

                if let Some(build) = &dependencies.build {
                    if !build.is_empty() {
                        tree.begin_child("build-dependencies".bright_cyan().to_string());
                        for unused_dependency in build {
                            tree.add_empty_child(unused_dependency.to_string());
                        }
                        tree.end_child();
                    }
                }

                let mut print_config = PrintConfig::default();
                let mut style = Style::default();
                style.foreground = Some(Color::RGB(128, 128, 128));

                print_config.branch = style;

                print_tree_with(&tree.build(), &print_config).unwrap();
            } else {
                continue;
            }
        }

        pretty_print_notes(None);
    }
}

/// Panics:
pub fn run(_app: App, _args: Option<Values>) -> Result<()> {
    // Run cargo bloat
    let mut command = ProcessBuilder::new("cargo");
    let spinner = ProgressBar::new_spinner();

    command.arg("udeps").arg("--output=json");

    spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}"));

    spinner.set_message("Analysing".bright_green().to_string());

    spinner.enable_steady_tick(10);

    let mut warning_count: u64 = 0;
    let mut error_count: u64 = 0;

    let mut stdout_contents: String = String::new();

    let output = command.exec_with_streaming(
        &mut |on_stdout| {
            // spinner.println(format!("Stdout: {}", on_stdout));
            stdout_contents.push_str(&on_stdout);
            Ok(())
        },
        &mut |on_stderr| {
            // spinner.println(format!("Stderr: {}", on_stderr));

            let contents = on_stderr.trim().to_string();

            if !contents.is_empty() {
                let chunks: Vec<&str> = contents.split(' ').collect();

                if contents.starts_with("Compiling") {
                    let name = chunks[1].to_string();

                    let mut version = chunks[2].to_string();

                    if version.starts_with('v') {
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

        if output
            .as_ref()
            .err()
            .unwrap()
            .to_string()
            .contains("exit code: 101")
        {
            // TODO: recommend to install cargo-udeps here.
            std::process::exit(1);
        }

        if error_count >= 1 {
            // Show all errors in the codebase:
            std::process::Command::new("cargo")
                .arg("check")
                .status()
                .unwrap();

            std::process::exit(1);
        }
    }

    spinner.set_message("Analysing".bright_cyan().to_string());

    let data = serde_json::from_str::<UdepsAnalysis>(&stdout_contents.trim()).unwrap();

    spinner.finish_and_clear();

    pretty_print_udeps_analysis(data);

    Ok(())
}
