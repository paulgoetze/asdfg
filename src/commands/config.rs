use crate::config::{config_file, YamlConfig};
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
        let path = config_file();

        match self {
            Config::List => list(&path)?,
            Config::Open => open(&path)?,
        }

        Ok(())
    }
}

fn list(path: &String) -> Result<(), Box<dyn Error>> {
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

fn open(path: &String) -> Result<(), Box<dyn Error>> {
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
