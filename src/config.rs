use serde::{de::Error, Deserialize, Deserializer};
use serenity::all::{ActivityType, OnlineStatus};
use std::{collections::HashMap, fs};

use anyhow::{Context, Result};

#[derive(Deserialize, Debug, Clone)]
pub struct Presence {
    #[serde(deserialize_with = "status_from_string")]
    pub status: OnlineStatus,
    #[serde(deserialize_with = "activity_type_from_string")]
    pub activity: ActivityType,
    pub description: String,
}

fn status_from_string<'de, D>(deserializer: D) -> Result<OnlineStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    if s == "online" {
        Ok(OnlineStatus::Online)
    } else if s == "idle" {
        Ok(OnlineStatus::Idle)
    } else if s == "dnd" {
        Ok(OnlineStatus::DoNotDisturb)
    } else if s == "invisible" {
        Ok(OnlineStatus::Invisible)
    } else if s == "offline" {
        Ok(OnlineStatus::Offline)
    } else {
        Err(D::Error::custom("Failed to deserialize status"))
    }
}

fn activity_type_from_string<'de, D>(deserializer: D) -> Result<ActivityType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    if s == "playing" {
        Ok(ActivityType::Playing)
    } else if s == "watching" {
        Ok(ActivityType::Watching)
    } else if s == "listening" {
        Ok(ActivityType::Listening)
    } else if s == "streaming" {
        Ok(ActivityType::Streaming)
    } else {
        Err(D::Error::custom("Failed to deserialize activity type"))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextCommand {
    pub name: String,
    pub response: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextCommands {
    pub prefix: char,
    pub commands: Vec<TextCommand>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlashCommand {
    pub name: String,
    pub description: String,
    pub response: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SlashCommands {
    pub commands: Vec<SlashCommand>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Commands {
    pub text: Option<TextCommands>,
    pub slash: Option<SlashCommands>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub guild_id: u64,
    pub responses: Option<HashMap<String, String>>,
    pub presence: Option<Presence>,
    pub commands: Option<Commands>,
}

impl serenity::prelude::TypeMapKey for Config {
    type Value = Config;
}

impl Config {
    pub fn load() -> Result<Config> {
        let config_file =
            fs::read_to_string("config.yaml").context("Failed to open config.yaml")?;

        Ok(serde_yaml::from_str(&config_file)?)
    }
}
