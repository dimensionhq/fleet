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

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UdepsAnalysis {
    pub success: bool,
    #[serde(rename = "unused_deps")]
    pub unused_deps: HashMap<String, UnusedDep>,
    pub note: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnusedDep {
    #[serde(rename = "manifest_path")]
    pub manifest_path: String,
    pub normal: Option<Vec<String>>,
    pub development: Option<Vec<Value>>,
    pub build: Option<Vec<String>>,
}

/// Panics:
pub fn run(_app: App, _args: Option<Values>) -> Result<()> {
    // Run cargo bloat
    let mut command = ProcessBuilder::new("cargo");
    let spinner = ProgressBar::new_spinner();

    command.arg("udeps").arg("--output=json");

    spinner.set_style(ProgressStyle::default_spinner().template("{spinner} {msg}"));

    spinner.set_message("Initializing".bright_green().to_string());

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

    spinner.finish();

    

    Ok(())
}
