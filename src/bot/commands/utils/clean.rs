use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Exception, Log, Replies};

#[poise::command(
    slash_command,
    rename = "clean",
    description_localized("en-US", "Clean all log stored in the guild"),
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn clean(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    match Log::clean(&guild_id) {
        Ok(_) => {
            Replies::say(
                &ctx,
                "Log has been cleaned. Please initiate first message to continue with any command",
            )
            .await?
        }
        Err(why) => Replies::say(&ctx, &why.to_string()).await?,
    }
    Ok(())
}
