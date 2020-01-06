use dirs;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use yaml_rust::{yaml, Yaml, YamlLoader};

const CONFIG_DIR: &str = ".asdfg";
const CONFIG_FILE: &str = "config.yaml";

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
    pub fn new(file: &String) -> YamlConfig {
        YamlConfig { file: file.clone() }
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

pub fn config_file(path: &Option<String>) -> String {
    let config_file = match path {
        Some(s) => PathBuf::from(s),
        _ => {
            let dir = Path::new(CONFIG_DIR);
            let file = Path::new(CONFIG_FILE);
            let default_path = dir.join(file);
            let home = dirs::home_dir().unwrap();
            home.join(Path::new(&default_path))
        }
    };

    let config_file = config_file.to_str().unwrap().to_string();

    eprintln!("Using config: {}\n", config_file);
    config_file
}

pub fn load(file: &Option<String>) -> Result<Vec<Package>, Box<dyn Error>> {
    let file = config_file(file);
    YamlConfig::new(&file).parse()
}
