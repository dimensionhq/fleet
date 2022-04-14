use std::{env::current_dir, path::PathBuf};

use crate::config::TurboConfig;

pub struct App {
    pub config: TurboConfig,
    pub current_dir: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let current_dir = current_dir().unwrap();
        let config = TurboConfig::new();
        let config = config.run_config();

        Self {
            config,
            current_dir,
        }
    }
}
