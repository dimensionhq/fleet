use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Debug, Serialize, Deserialize)]
pub struct Global {
    pub clang: PathBuf,
    pub linker: PathBuf,

    pub cranelift: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub global: Global,
}

impl Default for Config {
    fn default() -> Self {
        let clang = App::get_path("clang");
        let clang_trimmed = clang.trim_end();
        let mut linker = String::new();

        if cfg!(windows) {
            linker.push_str("rust-lld.exe");
        } else {
            linker.push_str(&App::get_path("lld").trim_end());
        }

        Self {
            global: Global {
                clang: PathBuf::from(clang_trimmed),
                linker: PathBuf::from(linker),

                cranelift: PathBuf::from("cranelift"),
            },
        }
    }
}
impl Config {
    pub fn new(config_path: PathBuf) -> Self {
        if !config_path.exists() {
            let mut base_config_file = config_path.clone();
            base_config_file.pop();

            std::fs::create_dir_all(base_config_file).unwrap();

            std::fs::File::create(config_path.clone()).unwrap();
            std::fs::write(config_path, toml::to_string(&Config::default()).unwrap()).unwrap();
        }
        Self {
            global: Global {
                clang: PathBuf::from("clang"),
                linker: PathBuf::from("ld"),

                cranelift: PathBuf::from("cranelift"),
            },
        }
    }
}
