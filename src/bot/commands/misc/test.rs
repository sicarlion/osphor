use crate::bot::{InteractionContext, InteractionError};
use crate::tools::Replies;

#[poise::command(
    slash_command,
    rename = "test",
    description_localized("en-US", "Test command")
)]
pub async fn test(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    Replies::say(&ctx, "Not implemented").await?;
    Ok(())
}
