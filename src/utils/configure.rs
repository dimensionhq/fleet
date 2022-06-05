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

use std::env::consts::OS;

use colored::Colorize;

/// Installs a linker
///
/// Installs the specified linker using the appropriate
/// package manager for the user operating system
///
/// # Panics
/// Can panic if unable to install clang and lld
pub fn install_linker(linker: &str) {
    match OS {
        "windows" => {
            if linker.contains("lld") {
                // LLD ships with Rust:
                println!("🚄 lld {}", "enabled".bright_green());
            }
        }
        "linux" => {
            if linker.contains("mold") {
                println!("🚀 mold {}", "enabled".bright_green());
            }

            if linker.contains("lld") {
                println!(
                    "{} {}",
                    "$".bright_black(),
                    "sudo apt install clang lld".bright_yellow()
                );

                // TODO: allow for user confirmation on whether or not they want to run the command
                // TODO: add distro support
                println!("{:?}", sys_info::linux_os_release());

                let result = std::process::Command::new("bash")
                    .arg("-c")
                    .arg("sudo apt install clang lld")
                    .status()
                    .expect("Failed to install clang + lld");

                if result.success() {
                    println!("🚄 lld {}", "enabled".bright_green());
                }
            }
        }
        "macos" => {
            if linker.contains("zld") {
                println!(
                    "{} {}",
                    "$".bright_black(),
                    "brew install michaeleisel/zld/zld".bright_yellow()
                );

                // TODO: allow for user confirmation on whether or not they want to run the command
                // Spawn the command
                let result = std::process::Command::new("bash")
                    .arg("-c")
                    .arg("brew install michaeleisel/zld/zld")
                    .status()
                    .unwrap();

                if result.success() {
                    println!("🚀 zld {}", "enabled".bright_green());
                }
            }

            if linker.contains("lld") {
                println!(
                    "{} {}",
                    "$".bright_black(),
                    "sudo apt install clang lld".bright_yellow()
                );

                // TODO: allow for user confirmation on whether or not they want to run the command
                let result = std::process::Command::new("bash")
                    .arg("-c")
                    .arg("sudo apt install clang lld")
                    .status()
                    .unwrap();

                if result.success() {
                    println!("🚄 lld {}", "enabled".bright_green());
                }
            }
        }
        &_ => {}
    }
}
