use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Config, Replies, Exception};
use poise::serenity_prelude::CreateEmbed;

#[poise::command(
    slash_command,
    rename = "setup",
    description_localized("en-US", "Open configuration editor")
)]
pub async fn setup(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    let config = Config::get(guild_id)?;

    Replies::embed(&ctx, CreateEmbed::new()
        .title("Configuration Editor")
        .description("Setup all the features available on your guild")
        .fields(vec![
            (
                "Features", format!(
                    "`{}`: {}\n`{}`: {}\n`{}`: {}\n`{}`: {}", 
                    "MODERATION_ACTIONS", "not implemented",
                    "GATEWAY_CHECKING", "not implemented",
                    "CONTENT_FILTERING", "not implemented",
                    "MESSAGES_LOGGING", config.features.MESSAGE_LOGGING,
                ), 
                false
            ),
            (
                "Permissions", format!(
                    "`{}`: {}\n`{}`: {}", 
                    "global_clip", config.permissions.global_clip,
                    "attenuate_perms", config.permissions.attenuate_perms,
                ), 
                false
            )
        ])
    )
    .await?;
    Ok(())
}
