use std::env::consts::OS;

use colored::Colorize;

pub fn install_linker(linker: &str) {
    match OS {
        "windows" => {
            if linker == "lld" {
                println!("🚄 lld - {} faster", "4x". bright_cyan());
            }
        }
        "linux" => {}
        "macos" => {}
        &_ => {}
    }
}
