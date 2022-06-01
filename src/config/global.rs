use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Build {
    pub sccache: PathBuf,
    pub lld: PathBuf,
    pub clang: PathBuf,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct FleetGlobalConfig {
    pub build: Build,
}

impl FleetGlobalConfig {

    /// # Panics
    /// can panic if home dir not found
    #[must_use]
    pub fn run_config() -> Self {
        let config_dir = dirs::home_dir().unwrap().join(".config").join("fleet");

        if !config_dir.join(".config").join("fleet").exists() {
            fs::create_dir_all(&config_dir).unwrap();
        }

        let config_path = config_dir.join("config.toml");

        // todo: use where/which to autofind binaries
        let config = FleetGlobalConfig {
            build: Build {
                sccache: PathBuf::from("~/.cargo/bin/sccache"),
                lld: PathBuf::from("rust-lld.exe"),
                clang: PathBuf::from("user/bin/clang"),
            },
        };

        fs::write(config_path, toml::to_string(&config).unwrap())
            .expect("Failed to generate Fleet Global Config");

        println!("📝 Generated Fleet Global Config");

        config
    }
}
