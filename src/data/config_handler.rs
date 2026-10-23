use std::{
    fs::{self, File},
    io::{
        ErrorKind::{self, NotFound},
        Read, Write,
    },
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    path: String,
    filename: String,
    data: Data,
}

impl Config {
    pub fn new(path: String, filename: String, data: Data) -> Self {
        Self {
            path,
            filename,
            data,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Data {
    inputs: Vec<Input>,
}

impl Data {
    pub fn new(inputs: Vec<Input>) -> Self {
        Self { inputs }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Input {
    client_id: String,
    name: String,
    description: String,
}

impl Input {
    pub fn new(client_id: String, name: String, description: String) -> Self {
        Self {
            client_id,
            name,
            description,
        }
    }
}

pub struct ConfigHandler {
    pub config: Config,
}

impl ConfigHandler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.config.path).join(&self.config.filename);

        self.config.data = serde_json::from_str(&std::fs::read_to_string(&path)?)?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = Path::new(&self.config.path);
        std::fs::create_dir_all(dir)?;

        let path = dir.join(&self.config.filename);
        let json = serde_json::to_string_pretty(&self.config.data)?;

        std::fs::write(path, json)?;

        Ok(())
    }

    pub fn data(&self) -> Data {
        self.config.data.clone()
    }
}
