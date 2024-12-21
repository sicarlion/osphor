use poise::serenity_prelude::GuildId;

use anyhow::Error;
use serde::Deserialize;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub features: Features,
    pub permissions: Permissions,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Features {
    pub MODERATION_ACTIONS: bool,
    pub GATEWAY_CHECKING: bool,
    pub CONTENT_FILTERING: bool,
    pub MESSAGE_LOGGING: bool,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    pub global_clip: bool,
    pub attenuate_perms: bool,
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
