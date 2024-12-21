use crate::bot::{InteractionContext, InteractionError};
use crate::tools::Config;
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

pub struct Test {}

impl Test {
    #[poise::command(
        slash_command,
        rename = "test",
        description_localized("en-US", "Test command")
    )]
    pub async fn new(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
        let guild_id = match ctx.guild_id() {
            Some(id) => id,
            None => {
                eprintln!("[ERR] Command not executed in a guild.");
                return Ok(());
            }
        };

        let config = Config::get(guild_id)?;

        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::new().description(format!("{}", config.features.MODERATION_ACTIONS))
            ],
            ..Default::default()
        })
        .await?;
        Ok(())
    }
}
