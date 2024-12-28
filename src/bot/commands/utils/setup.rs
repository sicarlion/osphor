use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Config, Exception, Replies};

use poise::serenity_prelude::CreateEmbed;

#[poise::command(
    slash_command,
    rename = "setup",
    description_localized("en-US", "Open configuration editor"),
    guild_only
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
    Replies::say(&ctx, &format!("{:?}", config)).await?;

    Replies::embed(
        &ctx,
        CreateEmbed::new()
            .title("Configuration Editor")
            .description("Setup all the features available on your guild")
            .fields(vec![
                (
                    "General",
                    format!(
                        "`{}`: {}\n`{}`: {:?}",
                        "appeal_link",
                        config.general.appeal_link,
                        "banned_words",
                        config.general.banned_words,
                    ),
                    false,
                ),
                (
                    "Features",
                    format!(
                        "**{}**: {}\n**{}**: {}\n**{}**: {}\n**{}**: {}",
                        "Moderation Action",
                        config.features.moderation_action,
                        "Gateway Checking",
                        "not implemented",
                        "Content Filtering",
                        "not implemented",
                        "Messages Logging",
                        config.features.message_logging,
                    ),
                    false,
                ),
                (
                    "Permissions",
                    format!(
                        "`{}`: {}\n`{}`: {}",
                        "global_clip",
                        config.permissions.global_clip,
                        "attenuate_perms",
                        config.permissions.attenuate_perms,
                    ),
                    false,
                ),
            ]),
    )
    .await?;
    Ok(())
}
