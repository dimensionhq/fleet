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
use ansi_term::Colour::{Cyan, Yellow};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::exit};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: PathBuf,
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
    pub fn new() -> Self {
        Self {
            rd_enabled: false,
            fleet_id: String::from(""),
            build: Build {
                sccache: PathBuf::from("~/.cargo/bin/sccache"),
            },
        }
    }

    pub fn run_config(&self) -> Self {
        let cargo_home_path = std::env::var("CARGO_HOME");
        let mut cargo_path = dirs::home_dir().unwrap().join(".cargo").join("bin");
        if let Ok(cargo_home) = cargo_home_path {
            cargo_path = PathBuf::from(cargo_home).join("bin");
        }

        let sccache_path = cargo_path.join("sccache");

        if !sccache_path.exists() {
            println!(
                "`{}` {} at {:?}, run {}",
                Cyan.paint("sccache"),
                Yellow.paint("not found"),
                sccache_path,
                Cyan.paint("`cargo install sccache`")
            );
        }

        let config_path = std::env::current_dir().unwrap().join("fleet.toml");

        if config_path.exists() {
            let config_file = std::fs::read_to_string(config_path).unwrap();
            let config = toml::from_str::<Self>(&config_file);

            if let Ok(mut config) = config {
                self.update_sccache(&mut config, sccache_path);
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
                    sccache: sccache_path,
                },
            };
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();
            config
        }
    }

    fn update_sccache(&self, config: &mut Self, sccache_path: PathBuf) -> Self {
        let home_dir = dirs::home_dir().unwrap();

        if config.build.sccache != sccache_path {
            let sccache_path = std::path::Path::new(&home_dir)
                .join(".cargo")
                .join("bin")
                .join("sccache");

            config.build.sccache = sccache_path;
            let config_path = std::env::current_dir().unwrap().join("fleet.toml");
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();

            config.clone()
        } else {
            config.clone()
        }
    }
}
