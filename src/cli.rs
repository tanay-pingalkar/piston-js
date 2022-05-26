use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "tanay pingalkar <tanaydpingalkar@gmail.com>", version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "to run game `piston-js run")]
    Run
}

