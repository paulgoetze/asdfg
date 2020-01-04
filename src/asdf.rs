use crate::config;
use std::error::Error;
use std::process;

const BASE_CMD: &str = "asdf";

pub fn add_plugin(name: &str) -> Result<(), Box<dyn Error>> {
    let desc = format!("Adding asdf plugin: {}", name);
    let error = format!("Plugin {} could not be installed.", name);
    run_cmd(&["plugin add", name], &desc, &error)?;

    Ok(())
}

pub fn install_package(package: &config::Package) -> Result<(), Box<dyn Error>> {
    for version in &package.versions {
        let desc = format!("Installing {} ({})", package.name, version);
        let error = format!("{} v{} could not be installed.", package.name, version);
        run_cmd(&["install", &package.name, &version], &desc, &error)?;
    }

    Ok(())
}

fn run_cmd(args: &[&str], desc: &str, error: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("{}", desc);

    let output = process::Command::new(BASE_CMD)
        .args(args)
        .output()
        .expect(error);

    if !output.stdout.is_empty() {
        eprintln!("{}", String::from_utf8(output.stdout).unwrap());
    }

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
    }

    Ok(())
}
