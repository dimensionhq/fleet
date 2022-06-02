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

use crate::{cli::app::App, commands::init::enable_fleet, config::{global::FleetGlobalConfig, FleetConfig}};
use std::fs;

pub fn run_doctor(option: &str, app: App) {
    match option {
        "Fix dependencies" => {
            enable_fleet(app);

            let config_file = dirs::home_dir()
                .unwrap()
                .join(".config")
                .join("fleet")
                .join("config.toml");

            if config_file.exists() {
                fs::remove_file(config_file).expect("Failed to delete file");
            }
            FleetGlobalConfig::run_config();
        },
        "Regenerate local fleet setup" => {
            fs::remove_file("fleet.toml").expect("Failed to delete file");
            fs::remove_dir_all("./.cargo").expect("Failed to delete ./.cargo dir");

            FleetConfig::run_config();
        }

        _ => {}
    }
}
