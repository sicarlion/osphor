use poise::serenity_prelude::GuildId;

use anyhow::Error;
use serde::Deserialize;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub features: Features,
    pub permissions: Permissions,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub appeal_link: String,
    pub banned_words: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Features {
    pub moderation_action: bool,
    pub gateway_checking: bool,
    pub content_filtering: bool,
    pub message_logging: bool,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    pub attenuate_perms: bool,
    pub global_clip: bool,
}

impl Config {
    /// Get configuration data from the Guild ID provided. Will return struct containing all the value.
    pub fn get(guild_id: GuildId) -> Result<Config, Error> {
        let file_path = format!("./guild/{}/config.json", guild_id);
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let config: Config = serde_json::from_reader(reader)?;

        Ok(config)
    }
}
