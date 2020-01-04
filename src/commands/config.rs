use crate::config::{YamlConfig, CONFIG_DIR, CONFIG_FILE};
use dirs;
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
        let dir = Path::new(CONFIG_DIR);
        let file = Path::new(CONFIG_FILE);
        let path = dir.join(file);

        match self {
            Config::List => list(&path)?,
            Config::Open => open(&path)?,
        }

        Ok(())
    }
}

fn list(path: &Path) -> Result<(), Box<dyn Error>> {
    let config_file = config_file(path);
    let config = YamlConfig::new(&config_file);
    let packages = config.parse()?;

    let headline = format!("Configured packages from {}", config_file);
    eprintln!("{}\n", headline);

    for package in packages {
        let name = package.name;
        let versions = package.versions.join(", ");
        eprintln!(" - {}: {}", name, versions);
    }

    Ok(())
}

fn open(path: &Path) -> Result<(), Box<dyn Error>> {
    let config_file = config_file(path);
    let basedir = Path::new(&config_file).parent().unwrap();

    if !basedir.exists() {
        fs::create_dir_all(basedir)?;
    }

    if !Path::new(&config_file).exists() {
        fs::File::create(&config_file)?;
    }

    open::that(config_file)?;

    Ok(())
}

fn config_file(path: &Path) -> String {
    let home = dirs::home_dir().unwrap();
    let config_file = home.join(Path::new(path));
    let config_file = config_file.to_str().unwrap().to_string();

    println!("File: {:?}", config_file);
    config_file
}
