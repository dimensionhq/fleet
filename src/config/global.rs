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

use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::exit};
use which::which;

#[must_use]
pub fn find(bin: &str) -> Option<PathBuf> {
    if let Ok(path) = which(bin) {
        Some(path)
    } else {
        None
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: Option<PathBuf>,
    pub lld: Option<PathBuf>,
    pub clang: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FleetGlobalConfig {
    pub build: Build,
}

impl FleetGlobalConfig {
    /// # Panics
    /// can panic if home dir not found
    pub fn run_config() -> Self {
        let config_dir = dirs::home_dir().unwrap().join(".config").join("fleet");

        if !config_dir.join(".config").join("fleet").exists() {
            fs::create_dir_all(&config_dir).unwrap();
        }

        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            let config_file = fs::read_to_string(&config_path).unwrap();
            if let Ok(config) = toml::from_str::<Self>(&config_file) {
                return config;
            }else {
                println!("Invalid fleet global configuration");
                exit(1)
            }
        }

        let config = FleetGlobalConfig {
            build: Build {
                sccache: find("sccache"),
                lld: find("rust-lld.exe"),
                clang: find("clang"),
            },
        };

        fs::write(config_path, toml::to_string(&config).unwrap())
            .expect("Failed to generate Fleet Global Config");

        println!("📝 Generated Fleet Global Config");

        config
    }
}
