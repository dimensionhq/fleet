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
use crate::cli::prompt;
use anyhow::Result;
use clap::Values;
use colored::Colorize;

///
/// # Panics
///
/// can panic is fails to run
pub fn run(_app: App, _args: Option<Values>) -> Result<()> {
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
                std::borrow::Cow::Owned(format!("🚀 zld - {} faster", "6x".bright_cyan())),
                std::borrow::Cow::Owned(format!("🚄 lld - {} faster", "4x".bright_cyan())),
            ]
        }
        "linux" => {
            vec![
                std::borrow::Cow::Owned(format!("🚀 mold - {} faster", "20x".bright_cyan())),
                std::borrow::Cow::Owned(format!("🚄 lld - {} faster", "5x".bright_cyan())),
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

    Ok(())
}
