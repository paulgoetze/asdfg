use crate::asdf;
use crate::config;
use colored::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Install asdf packages from the configured YAML file
pub struct Install {
    #[structopt()]
    /// The name of the package to install
    pub package: Option<String>,

    /// The config file to use (defaults to ~/.asdfg/config.yaml)
    #[structopt(short, long)]
    config: Option<String>,
}

impl Install {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match &self.package {
            Some(package) => install_by_name(&package, &self.config)?,
            None => install_all(&self.config)?,
        }

        Ok(())
    }
}

fn install_all(config_file: &Option<String>) -> Result<(), Box<dyn Error>> {
    let packages = config::load(config_file)?;

    for package in packages {
        install(&package)?;
    }

    Ok(())
}

fn install_by_name(name: &String, config_file: &Option<String>) -> Result<(), Box<dyn Error>> {
    let packages = config::load(config_file)?;
    let packages = packages
        .iter()
        .filter(|package| &package.name == name)
        .collect::<Vec<_>>();

    for package in packages {
        install(&package)?;
    }

    Ok(())
}

fn install(package: &config::Package) -> Result<(), Box<dyn Error>> {
    eprintln!("{}", "-----------------------------------".blue().bold());
    eprintln!("{}", package.name.to_uppercase().blue().bold());
    eprintln!("{}", "-----------------------------------".blue().bold());

    asdf::add_plugin(&package.name)?;
    asdf::install_package(package)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
