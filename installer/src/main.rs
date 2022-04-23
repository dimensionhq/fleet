use owo_colors::OwoColorize;
use std::process::Command;

pub fn install_sccache() {
    println!("{}", "> cargo install sccache".bright_cyan());

    // Install sccache
    let status = Command::new("cargo")
        .arg("install")
        .arg("sccache")
        .status()
        .unwrap();

    if status.success() {
        println!("{}", "Installed SCCache".bright_green());
    } else {
        println!("{}", "Failed to install SCCache".bright_red());
    }
}

pub fn set_compiler_nightly() {
    println!("{}", "> rustup default nightly".bright_cyan());

    // Set nightly compiler
    let status = Command::new("rustup")
        .arg("default")
        .arg("nightly")
        .status()
        .unwrap();

    if status.success() {
        println!(
            "{} {} {}",
            "Switched to".bright_green(),
            "nightly".bright_blue(),
            "compiler".bright_green(),
        );
    } else {
        println!("{}", "Failed to set nightly compiler".bright_red());
    }
}

pub fn install_fleet() {
    println!("{}", "> cargo install fleet-rs".bright_cyan());

    // Install Fleet
    let status = Command::new("cargo")
        .arg("install")
        .arg("fleet-rs")
        .status()
        .unwrap();

    if status.success() {
        println!("{}", "Installed Fleet".bright_green());
    } else {
        println!("{}", "Failed to install Fleet".bright_red());
    }
}

fn main() {
    println!("🚀 Installing Fleet");

    match std::env::consts::OS {
        "macos" => {
            println!("💻 Installing for MacOS");

            // Install zld, sccache
            let status = Command::new("brew").arg("install").arg("michaeleisel/zld/zld").status();

            if status.is_ok() {
                println!("{}", "Installed zld".bright_green());
            } else {
                println!("{}", "Failed to install zld".bright_red());
            }

            set_compiler_nightly();
	    install_sccache();
            install_fleet();

            println!("{}", "Installation complete".bright_green());
        }
        "linux" => {
            println!("💻 Installing for Linux");

            println!("{}", "> sudo apt install lld clang".bright_cyan());

            // Install lld, clang, sccache
            let status = Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("lld")
                .arg("clang")
                .arg("-y")
                .status()
                .unwrap();

            if status.success() {
                println!(
                    "{}{}",
                    "Installed ".bright_green(),
                    "lld, clang".bright_yellow()
                );
            } else {
                println!("{}", "Failed to install lld, clang".bright_red());
            }

            set_compiler_nightly();
            install_sccache();
            install_fleet();

            println!("{}", "Installation complete".bright_green());
        }
        "windows" => {
            println!("💻 Installing for Windows");

            let _ = enable_ansi_support::enable_ansi_support();

            set_compiler_nightly();
            install_sccache();
            install_fleet();

            println!("{}", "Installation complete".bright_green());
        }
        _ => {
            println!("OS Not Supported");
        }
    }
}
