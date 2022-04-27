use colored::Colorize;
use console::Emoji;
use utils::{app::App, VERSION};

pub enum Arguments {
    Run,
    Init,
    Build,
}

pub struct Commands {}

impl Commands {
    pub fn help() -> String {
        format!(
            "{} v{}
Dimension <{}>
The blazing fast build tool for Rust {}

{}: {} [SUBCOMMAND] [OPTIONS]

{}: 
    -h, --help       Display help menu
    -v, --version    Display version

{}:
    build            Build the project
    init             Initialize the project
    run              Run the project
    watch            Run the project with hot reloading enabled
",
            "fleet".green().bold(),
            VERSION,
            "team@dimension.dev".bold(),
            Emoji("🦀", ""),
            "USAGE".yellow(),
            "fleet".green(),
            "OPTIONS".yellow(),
            "SUBCOMMANDS".yellow(),
        )
    }
    pub fn run(app: App) {
        if app.args.len() > 1 {
            let arg = &app.args[1];

            match arg.as_str() {
                "run" => {
                    println!("run");
                }
                "init" => {
                    println!("init");
                }
                "build" => {
                    println!("build");
                }
                _ => {
                    println!("{}", Commands::help());
                }
            }
        } else {
            println!("{}", Commands::help());
        }
    }
}
