use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
use yaml_rust::{yaml, Yaml, YamlLoader};

pub const CONFIG_DIR: &str = ".asdfg";
pub const CONFIG_FILE: &str = "config.yaml";

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub versions: Vec<String>,
}

#[derive(Debug)]
pub struct YamlConfig {
    file: String,
}

impl YamlConfig {
    pub fn new(file: &str) -> YamlConfig {
        YamlConfig {
            file: String::from(file),
        }
    }

    pub fn parse(&self) -> Result<Vec<Package>, Box<dyn Error>> {
        let mut yaml = self.parse_yaml()?;

        let mut results = vec![];

        for entry in yaml.entries() {
            let name = entry.key().as_str().unwrap().to_string();
            let versions = self.parse_yaml_value(entry.get());
            results.push(Package { name, versions })
        }

        Ok(results)
    }

    fn parse_yaml(&self) -> Result<yaml::Hash, Box<dyn Error>> {
        let path = Path::new(&self.file);
        let content = fs::read_to_string(path)?;
        let docs = YamlLoader::load_from_str(&content)?;

        if docs.len() == 0 {
            println!("Config is empty. Open and edit it with `asdfg config open`");
            process::exit(1)
        }

        let doc = docs[0].clone();
        let hash = doc.into_hash().unwrap();

        Ok(hash)
    }

    fn parse_yaml_value(&self, value: &Yaml) -> Vec<String> {
        match value {
            Yaml::Array(value) => {
                let mut values = vec![];

                for item in value.iter() {
                    let mut v = self.parse_yaml_value(&item);
                    values.append(&mut v);
                }

                values
            }
            Yaml::String(value) => vec![value.to_string()],
            Yaml::Real(value) => vec![value.to_string()],
            Yaml::Integer(value) => vec![value.to_string()],
            _ => vec!["".to_string()],
        }
    }
}
