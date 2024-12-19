use poise::serenity_prelude::{Context, Error, Message};

use crate::tools::OsphorLog;

// Called everytime a message is casted.
pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Some(guild_id) = msg.guild_id {
        if msg.author.bot {
            return Ok(())
        }
        OsphorLog::log(&msg)?
    }

    Ok(())
}
