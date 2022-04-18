use crate::config::cargo::add_rustc_wrapper_and_target_configs;
use ansi_term::Colour::{Green, Red};
use std::{
    fs::create_dir,
    path,
    process::{exit, Command},
};
use sysinfo::{DiskExt, DiskType, RefreshKind, System, SystemExt};

pub fn init(app: crate::cli::app::App) {
    let refresh_kind = RefreshKind::new();
    let disks = refresh_kind.with_disks_list();

    let system = System::new_with_specifics(disks);

    let cargo_toml = path::Path::new("./Cargo.toml");

    if !cargo_toml.exists() {
        if let Err(cmd) = Command::new("cargo").arg("init").status() {
            println!("{}: failed to run cargo init: {}", Red.paint("error"), cmd);
            exit(1);
        }
    }

    let mut config = app.config;
    let os = std::env::consts::OS;

    if os == "windows" {
        panic!("Turbo doesn't support windows yet.");
    } else {
        let ramdisk_dir = path::Path::new("/dev/shm");
        let turbo_dir = ramdisk_dir.join(&config.turbo_id);
        let target_dir = std::env::current_dir().unwrap().join("target");

        let sccache_path = std::path::Path::new(&dirs::home_dir().unwrap())
            .join(".cargo")
            .join("bin")
            .join("sccache");

        config.build.sccache = sccache_path;

        let config_path = std::env::current_dir().unwrap().join("turbo.toml");
        let config_file = toml::to_string(&config).unwrap();

        std::fs::write(config_path, config_file).unwrap();

        let disk = system.disks().get(0).unwrap();

        // ramdisk improvements are only found if the disk is a HDD and the program is using WSL
        if config.turbo && disk.type_() == DiskType::HDD && wsl::is_wsl() {
            // check if target_dir is not a symlink, if yes delete it
            if !target_dir.is_symlink() && target_dir.exists() {
                if let Err(err) = std::fs::remove_dir_all(target_dir.clone()) {
                    println!("{} {}", Red.paint("error: "), &err);
                    exit(1)
                }
            }

            if !turbo_dir.exists() {
                if let Err(err) = create_dir(turbo_dir.clone()) {
                    println!("{} {}", Red.paint("error: "), &err);
                    exit(1)
                }
            }

            if !target_dir.exists() {
                println!("💽 Creating Ramdisk");
                std::os::unix::fs::symlink(turbo_dir, target_dir).unwrap();
            }
        }
    }

    // https://doc.rust-lang.org/cargo/reference/config.html
    let cargo_manifest_dir = std::env::current_dir().unwrap().join(".cargo");

    std::fs::create_dir_all(&cargo_manifest_dir).unwrap();

    let config_toml = cargo_manifest_dir.join("config.toml");

    if !config_toml.exists() {
        std::fs::File::create(&config_toml).unwrap_or_else(|err| {
            println!(
                "{}: failed to create configuration files: {}",
                Red.paint("error"),
                err
            );
            exit(1);
        });
    }

    add_rustc_wrapper_and_target_configs(
        config_toml.to_str().unwrap(),
        config.build.sccache.to_str().unwrap(),
    );

    println!("🚀 {}", Green.paint("Turbo is ready!"));
}
