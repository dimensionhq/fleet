use std::{env::current_dir, path::PathBuf};

use crate::config::FleetConfig;

pub struct App {
    pub config: FleetConfig,
    pub current_dir: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let current_dir = current_dir().unwrap();
        let config = FleetConfig::new();
        let config = config.run_config();

        Self {
            config,
            current_dir,
        }
    }
}
