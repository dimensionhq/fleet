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
use crate::core::config::enable::enable_fleet;
use anyhow::Result;
use clap::Values;

pub fn run(app: App, args: Option<Values>) -> Result<()> {
    enable_fleet(app);

    let args = args.unwrap_or_default();
    std::process::Command::new("cargo")
        .arg("run")
        .args(args)
        .status()?;

    Ok(())
}
