use crate::config::{YamlConfig, CONFIG_FILE};
use std::error::Error;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Modify and show the asdfg YAML configuration
pub enum Config {
    /// Prints the asdfg config to the terminal
    List,

    /// Opens the asdfg config in your default editor
    Open,
}

impl Config {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Config::List => list(CONFIG_FILE)?,
            Config::Open => open(CONFIG_FILE)?,
        }

        Ok(())
    }
}

pub fn list(file: &str) -> Result<(), Box<dyn Error>> {
    let config = YamlConfig::new(file);
    let packages = config.parse()?;

    let headline = format!("Configured packages from {}", file);
    eprintln!("{}\n", headline);

    for package in packages {
        let name = package.name;
        let versions = package.versions.join(", ");
        eprintln!(" - {}: {}", name, versions);
    }

    Ok(())
}

pub fn open(file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let basedir = path.parent().unwrap();

    // TODO: use absolute path, right now it creates a relative path
    fs::create_dir_all(basedir)?;
    fs::File::open(path)?;
    open::that(file)?;

    Ok(())
}
