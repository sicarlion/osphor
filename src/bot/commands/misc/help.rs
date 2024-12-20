use crate::bot::{InteractionContext, InteractionError};
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

pub struct Help {}

impl Help {
    #[poise::command(
        slash_command,
        rename = "help",
        description_localized("en-US", "Print all commands and setup information")
    )]
    pub async fn new(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
        ctx.send(CreateReply {
            embeds: vec![
                CreateEmbed::new()
                    .title("Help")
                    .description("Guide Book for using Osphor")
                    .fields(vec![
                        ("fetch", "Retrieve message log from this guild", false),
                        ("ping", "Check connection status with the Bot", true),
                        ("help", "Print this menu", true)
                    ])
            ],
            ..Default::default()
        })
        .await?;
        Ok(())
    }
}
