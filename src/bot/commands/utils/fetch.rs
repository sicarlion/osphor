use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Exception, Log, Replies};

// Fetch per guild messages log from the bot.
#[poise::command(
    slash_command,
    rename = "fetch",
    description_localized("en-US", "Fetches messages log from guild"),
    subcommands("last", "all"),
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn fetch(
    ctx: InteractionContext<'_>,
) -> Result<(), InteractionError> {
    Replies::say(&ctx, "Jawa").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "last",
    description_localized("en-US", "Fetch the last message sent on the guild")
)]
pub async fn last(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    let guild_dir = format!("./guild/{}/messages.log", guild_id);
    let last = match Log::fetch(&guild_dir)?.last() {
        Some(entry) => entry.print(),
        None => {
            eprintln!("[ERR] No log entries found in the file: {}", guild_dir);
            return Ok(());
        }
    };

    Replies::say(&ctx, &last).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "all",
    description_localized("en-US", "Fetch all the messages stored on the guild log")
)]
pub async fn all(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    let guild_dir = format!("./guild/{}/messages.log", guild_id);
    let osphor_log = Log::fetch(&guild_dir)?;
    let log = osphor_log.all();

    let mut all = String::new();

    for x in log {
        all.push_str(&x.print());
        all.push('\n');
    }

    Replies::say(&ctx, &all).await?;
    Ok(())
}
