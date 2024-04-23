use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub responses: HashMap<String, String>,
}

impl serenity::prelude::TypeMapKey for Config {
    type Value = Config;
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
       let config_file = fs::read_to_string("config.yaml")?;

        Ok(serde_yaml::from_str(&config_file)?)
    }
}
