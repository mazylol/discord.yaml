use serde::Deserialize;
use std::{collections::HashMap, fs};

use anyhow::{Result, Context};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub responses: HashMap<String, String>,
}

impl serenity::prelude::TypeMapKey for Config {
    type Value = Config;
}

impl Config {
    pub fn load() -> Result<Config> {
       let config_file = fs::read_to_string("config.yaml").context("Failed to open config.yaml")?;

        Ok(serde_yaml::from_str(&config_file)?)
    }
}
