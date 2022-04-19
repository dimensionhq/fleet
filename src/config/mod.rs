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
        let home_dir = dirs::home_dir().unwrap();

        let sccache_path = std::path::Path::new(&home_dir)
            .join(".cargo")
            .join("bin")
            .join("sccache");

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
