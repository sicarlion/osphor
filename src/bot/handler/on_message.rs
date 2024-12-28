use crate::tools::{Exception, Log, Replies};

use anyhow::Error;
use poise::serenity_prelude::{Context, GuildId, Message};

// Called everytime a message is casted.
pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), Error> {
    let commands = vec!["clip"];

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    if msg.author.bot {
        return Ok(());
    }

    if msg.content.to_lowercase() == commands[0] {
        clip(&ctx, &msg, guild_id).await?;
    }

    for x in &commands {
        if msg.content.to_lowercase() == *x {
            return Ok(());
        }
    }

    Ok(())
}

pub async fn clip(ctx: &Context, msg: &Message, guild_id: GuildId) -> Result<(), Error> {
    let guild_dir = format!("./guild/{}/messages.log", guild_id);
    let last = match Log::fetch(&guild_dir)?.last() {
        Some(entry) => entry.print(),
        None => {
            Replies::raw_error(&ctx, &msg, Exception::EmptyLog).await?;
            return Ok(());
        }
    };

    Replies::raw_send(&ctx, &msg, &last).await?;
    Ok(())
}
