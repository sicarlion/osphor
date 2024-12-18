use poise::serenity_prelude::{Context, Error, Message};

use crate::bot::tools::OsphorLog;

// Called everytime a message is casted.
pub async fn on_message(ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Some(guild_id) = msg.guild_id {
        OsphorLog::log(&msg).unwrap_or_else(|why| eprintln!("[ERR] Cannot log message. {why:?}"))
    }

    Ok(())
}
