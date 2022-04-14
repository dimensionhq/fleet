use crate::config::{cargo::add_rustc_wrapper, TurboConfig};
use ansi_term::Colour::Red;
use std::{
    fs::create_dir,
    path,
    process::{exit, Command},
};

pub fn init(app: crate::cli::app::App) {
    if let Err(cmd) = Command::new("cargo").arg("init").status() {
        println!("{}: Failed to run cargo init {}", Red.paint("error"), cmd);
        exit(1);
    }

    let mut config = app.config;
    let os = std::env::consts::OS;

    if os == "windows" {
    } else {
        let ramdisk_dir = path::Path::new("/dev/shm");
        let turbo_dir = ramdisk_dir.join(&config.turbo_id);
        let target_dir = std::env::current_dir().unwrap().join("target");

        let sccache_path = std::path::Path::new(&dirs::home_dir().unwrap())
            .join(".cargo")
            .join("bin")
            .join("sccache");

        config.build.sccache = sccache_path;

        let config_path = std::env::current_dir().unwrap().join(".turbo.toml");
        let config_file = toml::to_string(&config).unwrap();

        std::fs::write(config_path, config_file).unwrap();

        if config.turbo {
            // check if target_dir is not a symlink, if yes delete it
            if !target_dir.is_symlink() && target_dir.exists() {
                std::fs::remove_dir_all(target_dir.clone()).unwrap();
            }
            if !turbo_dir.exists() {
                if let Err(err) = create_dir(turbo_dir.clone()) {
                    println!("{} {}", Red.paint("Error: "), &err);
                    exit(1)
                }
            }
            if !cfg!(windows) {
                std::os::unix::fs::symlink(turbo_dir, target_dir).unwrap();
            }
        }
    }

    let cargo_toml = std::env::current_dir().unwrap().join("Cargo.toml");

    add_rustc_wrapper(
        cargo_toml.to_str().unwrap(),
        &config.build.sccache.to_str().unwrap(),
    );

    println!("{}", "Successfully initialized turbo project");
}
