use crate::config::cargo::add_rustc_wrapper_and_target_configs;
use ansi_term::Colour::Red;
use std::{
    fs::create_dir,
    path,
    process::{exit, Command},
};
use sysinfo::{DiskExt, DiskType, System, SystemExt};

pub fn init(app: crate::cli::app::App) {
    let system = System::new_all();

    let cargo_toml = path::Path::new("./Cargo.toml");
    if !cargo_toml.exists() {
        if let Err(cmd) = Command::new("cargo").arg("init").status() {
            println!("{}: Failed to run cargo init {}", Red.paint("error"), cmd);
            exit(1);
        }
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

        let hdd = system.disks().get(0).unwrap();

        // ramdisk improvements are only found if the disk is a HDD and the program is using WSL
        if config.turbo && hdd.type_() == DiskType::HDD && wsl::is_wsl() {
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

            if !cfg!(windows) && !target_dir.exists() {
                std::os::unix::fs::symlink(turbo_dir, target_dir).unwrap();
            }
        }
    }

    // https://doc.rust-lang.org/cargo/reference/config.html
    let cargo_manifest_dir = std::env::current_dir().unwrap().join(".cargo");

    std::fs::create_dir_all(cargo_manifest_dir.clone()).unwrap();

    let cargo_toml = cargo_manifest_dir.join("config.toml");
    if !cargo_toml.exists() {
        std::fs::File::create(cargo_toml.clone());
    }

    add_rustc_wrapper_and_target_configs(
        cargo_toml.to_str().unwrap(),
        config.build.sccache.to_str().unwrap(),
    );

    println!("Successfully initialized turbo project");
}
