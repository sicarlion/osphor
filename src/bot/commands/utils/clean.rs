use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Exception, Log, Replies};

#[poise::command(
    slash_command,
    rename = "clean",
    description_localized("en-US", "PURGE ALL OF OBSTACLES")
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
        Ok(_) => Replies::say(&ctx, "TORN TO OBLIVION").await?,
        Err(why) => Replies::say(&ctx, &why.to_string()).await?
    }
    Ok(())
}
