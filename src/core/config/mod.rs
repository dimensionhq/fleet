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

/// Handles configuration of the fleet setup and execution
pub mod cargo;
pub mod enable;
pub mod global;

use global::FleetGlobalConfig;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::exit};
use which::which;

/// Finds the path of a binary
///
/// Finds the path of a binary and returns the path if it exists
#[must_use]
pub fn find(bin: &str) -> Option<PathBuf> {
    if let Ok(path) = which(bin) {
        Some(path)
    } else {
        None
    }
}

/// Represents the build table of the `fleet.toml` file
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: Option<PathBuf>,
    pub lld: Option<PathBuf>,
    pub clang: Option<PathBuf>,
    pub zld: Option<PathBuf>,
}

/// Represents the `fleet.toml` file
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FleetConfig {
    pub rd_enabled: bool,
    pub fleet_id: String,
    pub build: Build,
}

impl Default for FleetConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetConfig {
    /// Initialize an empty `FleetConfig` instance with empty data
    #[must_use]
    pub fn new() -> Self {
        Self {
            rd_enabled: false,
            fleet_id: String::from(""),
            build: Build {
                sccache: None,
                lld: None,
                clang: None,
                zld: None,
            },
        }
    }

    /// Creates and read the `fleet.toml` file
    ///
    ///
    /// If the fleet.toml does not exist, it is created with the basic settings and the basic config is returned.
    ///
    /// If it does exist, the `fleet.toml` file is read and the data is parsed into `FleetConfig` and returned.
    ///
    /// When a particular field of the `build` table is empty, it is substituted with the value from the global fleet config.
    ///
    /// T
    /// # Panics
    /// Can panic if cannot find home directory
    #[must_use]
    pub fn run_config() -> Self {
        let global_config = FleetGlobalConfig::run_config();

        let config_path = std::env::current_dir()
            .expect("cannot find current directory")
            .join("fleet.toml");

        if config_path.exists() {
            let config_file = std::fs::read_to_string(config_path).unwrap();
            let config = toml::from_str::<Self>(&config_file);

            if let Ok(mut config) = config {
                if config.build.sccache.is_none() {
                    config.build.sccache = global_config.build.sccache;
                }

                if config.build.lld.is_none() {
                    config.build.lld = global_config.build.lld;
                }

                if config.build.clang.is_none() {
                    config.build.clang = global_config.build.clang;
                }

                if config.build.zld.is_none() {
                    config.build.zld = global_config.build.zld;
                }

                config
            } else {
                eprintln!("Invalid fleet configuration");
                exit(1)
            }
        } else {
            let config = FleetConfig {
                rd_enabled: true,
                fleet_id: uuid::Uuid::new_v4().to_string(),
                build: Build {
                    sccache: None,
                    lld: None,
                    clang: None,
                    zld: None,
                },
            };
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();
            config
        }
    }
}
