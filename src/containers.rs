
use std::io::prelude::*;
use std::fs::File;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};

const DOCKER_ROOT: &'static str = "/home/q/docker";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {

    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "NetworkSettings")]
    net_setting: NetworkSettings,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            id: String::new(),
            name: String::new(),
            net_setting: NetworkSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkSettings {
    
    #[serde(rename = "Bridge")]
    bridge: String,

    #[serde(rename = "SandboxKey")]
    sandbox_key: String,
}

impl Default for NetworkSettings {
    fn default() -> Self {
         NetworkSettings {
             bridge: String::new(),
             sandbox_key: String::new(),
         }
    }
}

impl NetworkSettings {
    pub fn get_sandbox_key(&self) -> String {
        self.sandbox_key.clone()
    }
}

impl Config {
    fn from_str(data: &str) -> serde_json::Result<Self> {
        serde_json::from_str(data)
    }

    pub fn get_sandbox_key(&self) -> String {
        self.net_setting.get_sandbox_key()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    name: String,
    config: Config
}


impl Container {
    pub fn new() -> Self {
        Container{
            name: String::new(),
            config: Config::default(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn name<'a>(&mut self, name: &'a str) {
        self.name = String::from(name);
    }

    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    pub fn config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn read_config(&mut self, path: &str) {
        if let Ok(mut file) = File::open(&path) {

            let mut buffer = String::new();
            if let Ok(_) = file.read_to_string(&mut buffer) {
                if let Ok(config) = Config::from_str(&buffer) {
                    self.name(&config.name);
                    self.config(config);
                }
            }
        }
    }
}


impl ToString for Container {
    fn to_string(&self) -> String {
        format!("ID: {}, Name: {}", 
                String::from(&self.get_config().get_id()[..12]),
                self.get_name())
    }
}


fn walk() -> Vec<String> {
    let container_path = format!("{}/containers", DOCKER_ROOT);
    let mut paths: Vec<String> = Vec::new();

    for e in WalkDir::new(&container_path)
            .into_iter()
            .filter_map(|e| e.ok()) {

        if e.metadata().unwrap().is_file() {
            let path = e.path();
            if path.ends_with("config.json") || path.ends_with("config.v2.json") {
                if let Some(p) = path.to_str() {
                    paths.push(String::from(p));
                }
            }
        }
    }

    paths
}


pub fn containers() -> Vec<Container> {
   let mut cs: Vec<Container> = Vec::new();
   for path in walk() {
      let mut container = Container::new();
      container.read_config(&path);
      cs.push(container)
   }
   cs
}
