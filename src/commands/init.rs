use crate::config::TurboConfig;
use ansi_term::Colour::Red;
use std::{
    fs::create_dir,
    path,
    process::{exit, Command},
};

pub fn init() {
    if let Err(cmd) = Command::new("cargo").arg("init").output() {
        println!("{}: Failed to run cargo init {}", Red.paint("error"), cmd);
        exit(1);
    }

    let config = TurboConfig::new();
    let config = config.run_config();

    let ramdisk_dir = path::Path::new("/dev/shm");
    let turbo_dir = ramdisk_dir.join(&config.turbo_id);
    let target_dir = std::env::current_dir().unwrap().join("target");

    if config.turbo {
        // check if target_dir is a symlink
        if !target_dir.is_symlink() && target_dir.exists() {
            std::fs::remove_dir_all(target_dir.clone()).unwrap();
        }
        if !turbo_dir.exists() {
            if let Err(err) = create_dir(turbo_dir.clone()) {
                println!("{} {}", Red.paint("Error: "), &err);
                exit(1)
            }
        }

        std::os::unix::fs::symlink(turbo_dir, target_dir).unwrap();
    }
}
