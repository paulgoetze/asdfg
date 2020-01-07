use crate::config;
use colored::*;
use std::error::Error;
use std::process;

const BASE_CMD: &str = "asdf";
const VERSION_LATEST: &str = "latest";

pub fn add_plugin(name: &str) -> Result<(), Box<dyn Error>> {
    let desc = format!("Adding asdf plugin");
    let error = format!("Plugin {} could not be installed.", name).red();
    run_cmd(&["plugin add", name], &desc, &error)?;
    eprintln!("---");

    Ok(())
}

pub fn install_package(package: &config::Package) -> Result<(), Box<dyn Error>> {
    for version in &package.versions {
        let name = &package.name;
        install_version(name, version)?;
        set_global(name, version)?;
        reshim(name)?;
    }

    Ok(())
}

fn install_version(name: &String, version: &String) -> Result<(), Box<dyn Error>> {
    let desc = format!("Installing {} ({})", name, version);
    let error = format!("{} v{} could not be installed", name, version).red();
    run_cmd(&["install", &name, &version], &desc, &error)?;

    Ok(())
}

fn set_global(name: &String, version: &String) -> Result<(), Box<dyn Error>> {
    let mut final_version = &version.clone();
    let available_versions = run_cmd_silent(&["list", name])?;
    let last_version = available_versions.last().unwrap();

    if final_version == VERSION_LATEST {
        final_version = last_version;
    }

    let desc = format!("Setting global {} version to {}", name, final_version);
    let error = format!("Failed to use global {} v{}", name, final_version);
    run_cmd(&["global", &name, &final_version], &desc, &error)?;

    Ok(())
}

fn reshim(name: &String) -> Result<(), Box<dyn Error>> {
    let error = format!("Failed to reshim {}", name);
    run_cmd(&["reshim", &name], "", &error)?;

    Ok(())
}

fn run_cmd(args: &[&str], desc: &str, error: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("{}", desc);

    let output = process::Command::new(BASE_CMD)
        .args(args)
        .output()
        .expect(error);

    if !output.stdout.is_empty() {
        eprintln!("{}", String::from_utf8(output.stdout).unwrap().green());
    }

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap().yellow());
    }

    Ok(())
}

fn run_cmd_silent(args: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    let output = process::Command::new(BASE_CMD).args(args).output().unwrap();

    if !output.stdout.is_empty() {
        return Ok(String::from_utf8(output.stdout)
            .unwrap()
            .split("\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() || s.starts_with("--"))
            .collect::<Vec<_>>());
    }

    if !output.stderr.is_empty() {
        return Ok(vec![String::from_utf8(output.stderr).unwrap()]);
    }

    Ok(vec![])
}
