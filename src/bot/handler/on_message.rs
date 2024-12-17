use poise::serenity_prelude::{Context, Error, Message};

use crate::bot::tools::*;

// Called everytime a message is casted.
pub async fn on_message(_ctx: &Context, msg: &Message) -> Result<(), Error> {
    if let Some(guild_id) = msg.guild_id {
        logger::log_message(&guild_id, &msg.id, &msg.author.name, &msg.content)?;
    }

    Ok(())
}
