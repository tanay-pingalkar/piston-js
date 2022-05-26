use clap::Parser;

use std::fs;

mod cli;
mod runtime;
mod utils;

use cli::{Cli, Commands};
use runtime::Runtime;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command.unwrap() {
        Commands::Run => {
            let window_file = fs::read_to_string("example/window.p.js").unwrap();
            let frame_file = fs::read_to_string("example/frame.p.js").unwrap();
            let data_file = fs::read_to_string("example/data.p.js").unwrap();

            Runtime::new()
                .create_window(window_file)
                .init_data(data_file)
                .start_game_loop(frame_file);
        }
    }
}
