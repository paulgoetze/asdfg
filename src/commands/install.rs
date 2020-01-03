use crate::commands::utils::NONE;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Install asdf packages from the configured YAML file
pub struct Install {
    #[structopt(default_value = NONE)]
    /// The name of the package to install
    pub package: String,

    /// The version of the package to install
    #[structopt(default_value = NONE)]
    pub version: String,
}

impl Install {
    pub fn run(&self) {
        if self.package == NONE {
            install_all();
        } else if self.version == NONE {
            install_latest(&self.package);
        } else {
            install(&self.package, &self.version);
        }
    }
}

fn install_all() {
    println!("Installing all");
}

fn install_latest(package: &String) {
    println!("Installing latest {}", package);
}

fn install(package: &String, version: &String) {
    println!("Installing {} v{}", package, version);
}
