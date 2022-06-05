/*
 *
 *    Copyright 2021 Fleet Contributors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

use crate::core::config::cargo::add_rustc_wrapper_and_target_configs;
use ansi_term::Colour::{Green, Red};
use std::{
    path::{self, PathBuf},
    process::{exit, Command},
};

/// Unwraps a item of Option<PathBuf> and returns the path as a String in Option<String>
fn string_path_unwrap(path: Option<PathBuf>) -> Option<String> {
    path.map(|path| path.to_str().unwrap().to_string())
}

#[cfg(unix)]
use sysinfo::{DiskExt, DiskType, RefreshKind, System, SystemExt};

/// If the `./.cargo/config.toml` doesn't exist, it is created.
///
/// The application config is written onto the `./.cargo/config.toml`.
///
/// Ramdisk improvements are applied if the disk is a HDD and the program is using WSL
///
///  # Panics
/// Can panic if cannot get `dirs::home_dir`
pub fn enable_fleet(app: crate::cli::app::App) {
    let cargo_toml = path::Path::new("./Cargo.toml");

    if !cargo_toml.exists() {
        if let Err(cmd) = Command::new("cargo").arg("init").status() {
            eprintln!("{}: failed to run cargo init: {}", Red.paint("error"), cmd);
            exit(1);
        }
    }

    let config = app.config;
    let os = std::env::consts::OS;

    if os != "windows" {
        // ramdisk improvements are only found if the disk is a HDD and the program is using WSL
        #[cfg(target_os = "linux")]
        {
            let refresh_kind = RefreshKind::new();
            let disks = refresh_kind.with_disks_list();
            let system = System::new_with_specifics(disks);
            let disk = system.disks().get(0).unwrap();

            if disk.type_() == DiskType::HDD || wsl::is_wsl() {
                let ramdisk_dir = path::Path::new("/dev/shm");
                let fleet_dir = ramdisk_dir.join(&config.fleet_id);
                let target_dir = std::env::current_dir().unwrap().join("target");

                // check if target_dir is not a symlink, if yes delete it
                if !target_dir.is_symlink() && target_dir.exists() {
                    if let Err(err) = std::fs::remove_dir_all(target_dir.clone()) {
                        eprintln!("{} {}", Red.paint("error: "), &err);
                        exit(1)
                    }
                }

                if !fleet_dir.exists() {
                    if let Err(err) = std::fs::create_dir(fleet_dir.clone()) {
                        eprintln!("{} {}", Red.paint("error: "), &err);
                        exit(1)
                    }
                }

                if !target_dir.exists() {
                    println!("💽 Creating Ramdisk");
                    std::os::unix::fs::symlink(fleet_dir, target_dir).unwrap();
                }
            }
        }
    }

    // https://doc.rust-lang.org/cargo/reference/config.html
    let cargo_manifest_dir = std::env::current_dir().unwrap().join(".cargo");

    std::fs::create_dir_all(&cargo_manifest_dir).unwrap();

    let config_toml = cargo_manifest_dir.join("config.toml");
    let config_no_toml = cargo_manifest_dir.join("config");

    if !config_toml.exists() && !config_no_toml.exists() {
        std::fs::File::create(&config_toml).unwrap_or_else(|err| {
            eprintln!(
                "{}: failed to create configuration files: {}",
                Red.paint("error"),
                err
            );
            exit(1);
        });
    }

    add_rustc_wrapper_and_target_configs(
        if config_toml.exists() {
            config_toml.to_str().unwrap()
        } else {
            config_no_toml.to_str().unwrap()
        },
        string_path_unwrap(config.build.sccache),
        string_path_unwrap(config.build.clang),
        string_path_unwrap(config.build.lld),
        string_path_unwrap(config.build.zld),
    );

    println!("🚀 {}", Green.paint("Fleet is ready!"));
}
