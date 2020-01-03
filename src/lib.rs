mod commands;
use structopt::StructOpt;

pub fn run() {
    match Cli::from_args().cmd {
        Command::Install(install) => install.run(),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "asdfg", about = "Installing asdf packages from a YAML config.")]
pub struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Install(commands::Install),
}
