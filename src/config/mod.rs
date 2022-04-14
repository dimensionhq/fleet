pub mod cargo;
use std::{path::PathBuf, process::exit};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: PathBuf,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TurboConfig {
    pub turbo: bool,
    pub turbo_id: String,
    pub build: Build,
}

impl TurboConfig {
    pub fn new() -> Self {
        Self {
            turbo: false,
            turbo_id: "".to_string(),
            build: Build {
                sccache: PathBuf::from("~/.cargo/bin/sccache"),
            },
        }
    }

    pub fn run_config(&self) -> Self {
        let home_dir = dirs::home_dir().unwrap();

        let sccache_path = std::path::Path::new(&home_dir)
            .join(".cargo")
            .join("bin")
            .join("sccache");

        if !sccache_path.exists() {
            println!("sccache not found at {:?}", sccache_path);
        }

        let config_path = std::env::current_dir().unwrap().join(".turbo.toml");

        if config_path.exists() {
            let config_file = std::fs::read_to_string(config_path).unwrap();
            let config = toml::from_str::<Self>(&config_file);

            if let Ok(mut config) = config {
                self.update_sccache(&mut config, sccache_path);
                return config;
            } else {
                println!("Invalid turbo config");
                exit(1)
            }
        } else {
            let config = TurboConfig {
                turbo: true,
                turbo_id: uuid::Uuid::new_v4().to_string(),
                build: Build {
                    sccache: sccache_path,
                },
            };
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();
            return config;
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
            let config_path = std::env::current_dir().unwrap().join(".turbo.toml");
            let config_file = toml::to_string(&config).unwrap();
            std::fs::write(config_path, config_file).unwrap();

            return config.clone();
        } else {
            return config.clone();
        }
    }
}
