mod asdf;
mod commands;
mod config;

use std::error::Error;
use structopt::StructOpt;

pub fn run() -> Result<(), Box<dyn Error>> {
    match Cli::from_args().command {
        Command::Install(command) => command.run()?,
        Command::Config(command) => command.run()?,
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "asdfg", about = "Installing asdf packages from a YAML config.")]
pub struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Install(commands::Install),
    Config(commands::Config),
}
