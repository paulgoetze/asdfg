use crate::asdf;
use crate::commands::utils::NONE;
use crate::config;
use colored::*;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Install asdf packages from the configured YAML file
pub struct Install {
    #[structopt(default_value = NONE)]
    /// The name of the package to install
    pub package: String,
}

impl Install {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.package == NONE {
            install_all()?;
        } else {
            install_by_name(&self.package)?;
        }

        Ok(())
    }
}

fn install_all() -> Result<(), Box<dyn Error>> {
    let packages = config::load()?;

    for package in packages {
        install(&package)?;
    }

    Ok(())
}

fn install_by_name(name: &String) -> Result<(), Box<dyn Error>> {
    let packages = config::load()?;
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
