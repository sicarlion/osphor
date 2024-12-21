use crate::bot::{InteractionContext, InteractionError};
use crate::tools::{Exception, Log, Replies};

// Fetch per guild messages log from the bot.
#[poise::command(
    slash_command,
    rename = "fetch",
    description_localized("en-US", "Fetches messages log from guild"),
    subcommands("last", "peek", "all"),
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
            Replies::error(&ctx, Exception::EmptyLog).await?;
            return Ok(());
        }
    };

    Replies::say(&ctx, &last).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "peek",
    description_localized("en-US", "Peek some of the recent messages sent on the guild")
)]
pub async fn peek(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    let temp_replies = ctx.defer();

    let guild_dir = format!("./guild/{}/messages.log", guild_id);
    let osphor_log = Log::fetch(&guild_dir)?;
    let log = osphor_log.all();

    let mut peek = Vec::new();
    for x in log {
        peek.push(x.print());
    }
    peek.reverse();

    let mut chunks = String::new();

    for line in peek {
        // If adding the line exceeds the limit, end it
        if chunks.len() + line.len() + 1 > 2000 {
            break;
        }
        chunks = format!("{}\n{}", line, chunks); // Prepend the line
    }

    if chunks.is_empty() {
        Replies::error(&ctx, Exception::EmptyLog).await?;
        return Ok(())
    }

    // Send each chunk as a separate message 
    Replies::say(&ctx, &chunks).await?;

    Ok(())
}

#[poise::command(
    slash_command,
    rename = "all",
    description_localized("en-US", "Retrieve all the messages log stored on the guild")
)]
pub async fn all(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            Replies::error(&ctx, Exception::NotGuild).await?;
            return Ok(());
        }
    };

    let temp_replies = ctx.defer();

    let guild_dir = format!("./guild/{}/messages.log", guild_id);
    let osphor_log = Log::fetch(&guild_dir)?;
    let log = osphor_log.all();

    let mut all = String::new();
    for x in log {
        all.push_str(&x.print());
        all.push('\n');
    }

    // Split into chunks by line to prevent 2000-character limit
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for line in all.lines() {
        // If adding the line exceeds the limit, push the current chunk and start a new one
        if current_chunk.len() + line.len() + 1 > 2000 {
            chunks.push(current_chunk);
            current_chunk = String::new();
        }
        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    // Push the last chunk if it exists
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    // Send each chunk as a separate message
    for chunk in chunks {
        Replies::send(&ctx, &chunk).await?;
    }
    
    Replies::say(&ctx, "User have requested full log fetch").await?;

    Ok(())
}
