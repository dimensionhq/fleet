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

pub mod cargo;
pub mod global;

use global::FleetGlobalConfig;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::exit};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: Option<PathBuf>,
    pub lld: Option<PathBuf>,
    pub clang: Option<PathBuf>,
}

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
    #[must_use]
    pub fn new() -> Self {
        Self {
            rd_enabled: false,
            fleet_id: String::from(""),
            build: Build {
                sccache: None,
                lld: None,
                clang: None,
            },
        }
    }

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

                // todo: use global variable to update path of sccache, lld and clang  everytime
                // config.update_sccache(&sccache_path);

                config
            } else {
                println!("Invalid fleet configuration");
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
                },
            };
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();
            config
        }
    }

    #[allow(unused)]
    fn update_sccache(&mut self, sccache_path: &PathBuf) -> Self {
        let home_dir = dirs::home_dir().unwrap();

        if &(self.build.sccache.clone().unwrap()) == sccache_path {
            let sccache_path = std::path::Path::new(&home_dir)
                .join(".cargo")
                .join("bin")
                .join("sccache");

            self.build.sccache = Some(sccache_path);
            let config_path = std::env::current_dir().unwrap().join("fleet.toml");
            let config_file = toml::to_string(&self).unwrap();
            std::fs::write(config_path, config_file).unwrap();
        }
        self.clone()
    }
}
