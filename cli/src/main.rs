use crate::commands::Commands;

pub mod commands;
#[tokio::main]
async fn main() {
    let app = utils::app::App::new();
    Commands::run(app);
}
