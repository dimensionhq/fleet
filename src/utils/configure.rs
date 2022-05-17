use std::env::consts::OS;

use colored::Colorize;

pub fn install_linker(linker: &str) {
    match OS {
        "windows" => {
            if linker == "lld" {
                // LLD ships with Rust:
                println!("🚄 lld {}", "enabled".bright_green());
            }
        }
        "linux" => {}
        "macos" => {}
        &_ => {}
    }
}
