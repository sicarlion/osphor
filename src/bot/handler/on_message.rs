use crate::tools::{Exception, Log, Replies};

use poise::serenity_prelude::{Context, Message};
use anyhow::Error;

// Called everytime a message is casted.
pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), Error> {

    let commands = vec!["clip"];

    if let Some(guild_id) = msg.guild_id {
        if msg.author.bot {
            return Ok(());
        } 

        if msg.content.to_lowercase() == commands[0] {
            let guild_id = match msg.guild_id {
                Some(id) => id,
                None => {
                    Replies::raw_error(&ctx, msg, Exception::NotGuild).await?;
                    return Ok(());
                }
            };

            let guild_dir = format!("./guild/{}/messages.log", guild_id);
            let last = match Log::fetch(&guild_dir)?.last() {
                Some(entry) => entry.print(),
                None => {
                    Replies::raw_error(&ctx, msg, Exception::EmptyLog).await?;
                    return Ok(());
                }
            };

            Replies::raw_send(&ctx, &msg, &last).await?
        }

        for x in &commands {
            if msg.content.to_lowercase() == *x {
                return Ok(());
            }
        }
        
        Log::log(&msg)?;
    }

    Ok(())
}
