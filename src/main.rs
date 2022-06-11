use clap::Parser;

use std::fs;

mod cli;
mod game_engine;
mod v8_runtime;

use cli::{Cli, Commands};

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command.unwrap() {
        Commands::Run { file } => {
            let file = fs::read_to_string(file).unwrap();
            game_engine::GameEngine::start_game_engine(file);
        }
    }
}
