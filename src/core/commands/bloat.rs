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

use crate::cli::app::App;
use anyhow::Result;
use cargo_util::ProcessBuilder;
use clap::Values;
use colored::Colorize;
use comfy_table::ContentArrangement;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatCrateAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub crates: Vec<Crate>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
    pub name: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BloatFunctionAnalysis {
    #[serde(rename = "file-size")]
    pub file_size: i64,
    #[serde(rename = "text-section-size")]
    pub text_section_size: i64,
    pub functions: Vec<Function>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "crate")]
    pub crate_field: String,
    pub name: String,
    pub size: i64,
}

/// Panics:
pub fn run(_app: App, _args: Option<Values>) -> Result<()> {
    let mut handles: Vec<JoinHandle<comfy_table::Table>> = vec![];
    let spinner = ProgressBar::new_spinner();

    handles.push(std::thread::spawn({
        let spinner = spinner;
        move || -> comfy_table::Table {
            spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}"));

            spinner.set_message("Initializing".bright_green().to_string());

            spinner.enable_steady_tick(10);

            let mut warning_count: u64 = 0;
            let mut error_count: u64 = 0;

            // Run cargo bloat
            let mut command = ProcessBuilder::new("cargo");

            command
                .arg("bloat")
                .arg("--crates")
                .arg("--message-format=json");

            let output = command.exec_with_streaming(
                &mut |_| Ok(()),
                &mut |on_stderr| {
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

                // Show all errors in the codebase:
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

            for crate_ in &data.crates {
                let size = byte_unit::Byte::from_bytes(crate_.size as u128);
                let adjusted_size = size.get_appropriate_unit(true);

                crates_table.add_row(vec![
                    Cell::new(crate_.name.to_string()).fg(Color::Blue),
                    Cell::new(adjusted_size.to_string()).fg(Color::Cyan),
                ]);
            }

            crates_table
        }
    }));

    handles.push(std::thread::spawn(move || -> comfy_table::Table {
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

        for function in &data.functions {
            let size = byte_unit::Byte::from_bytes(function.size as u128);
            let adjusted_size = size.get_appropriate_unit(true);

            function_table.add_row(vec![
                Cell::new(function.crate_field.to_string()).fg(Color::Blue),
                Cell::new(function.name.to_string()),
                Cell::new(adjusted_size.to_string()).fg(Color::Cyan),
            ]);
        }

        function_table
    }));

    let mut tables: Vec<comfy_table::Table> = vec![];

    for handle in handles {
        tables.push(handle.join().unwrap());
    }

    for table in tables {
        println!("{table}");
    }

    println!(
        "\n{}: All sizes shown are estimates and will not be 100% accurate.",
        "Note".bright_yellow()
    );

    Ok(())
}
