use std::env::consts::OS;

use colored::Colorize;

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
        "macos" => {
            if linker.contains("zld") {
                println!(
                    "{} {}",
                    "$".bright_black(),
                    "brew install michaeleisel/zld/zld".bright_yellow()
                );

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
