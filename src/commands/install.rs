use crate::commands::utils::NONE;
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
            install_package(&self.package)?;
        }

        Ok(())
    }
}

fn install_all() -> Result<(), Box<dyn Error>> {
    println!("Installing all");
    Ok(())
}

fn install_package(package: &String) -> Result<(), Box<dyn Error>> {
    println!("Installing all versions for package {}", package);
    Ok(())
}

#[cfg(test)]
mod tests {}
