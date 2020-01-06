use crate::config::{config_file, YamlConfig};
use std::error::Error;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Modify and show the asdfg YAML configuration
pub enum Config {
    /// Prints the asdfg config to the terminal
    List {
        /// The config file to use (defaults to ~/.asdfg/config.yaml)
        #[structopt(short, long)]
        config: Option<String>,
    },

    /// Opens the asdfg config in your default editor
    Open {
        /// The config file to use (defaults to ~/.asdfg/config.yaml)
        #[structopt(short, long)]
        config: Option<String>,
    },
}

impl Config {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Config::List { config } => list(config)?,
            Config::Open { config } => open(config)?,
        }

        Ok(())
    }
}

fn list(file: &Option<String>) -> Result<(), Box<dyn Error>> {
    let default_config = config_file(file);
    let path = file.as_ref().unwrap_or(&default_config);
    let config = YamlConfig::new(path);
    let packages = config.parse()?;

    let headline = format!("Configured packages from {}", path);
    eprintln!("{}\n", headline);

    for package in packages {
        let name = package.name;
        let versions = package.versions.join(", ");
        eprintln!(" - {}: {}", name, versions);
    }

    Ok(())
}

fn open(file: &Option<String>) -> Result<(), Box<dyn Error>> {
    let default_config = config_file(file);
    let path = file.as_ref().unwrap_or(&default_config);
    let basedir = Path::new(path).parent().unwrap();

    if !basedir.exists() {
        fs::create_dir_all(basedir)?;
    }

    if !Path::new(path).exists() {
        fs::File::create(path)?;
    }

    open::that(path)?;

    Ok(())
}
