use crate::bot::{InteractionContext, InteractionError};

use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

#[poise::command(
    slash_command,
    rename = "help",
    description_localized("en-US", "Print all commands and setup information")
)]
pub async fn help(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    ctx.send(CreateReply {
        embeds: vec![
            CreateEmbed::new()
                .title("Guide book for using Osphor")
                .description("This tutorial will walkthrough on how to setup Osphor on your setup")
                .fields(vec![
                    ("Jawa", "adalah kuncinya", false)
                ])
        ],
        ..Default::default()
    })
    .await?;
    Ok(())
}
