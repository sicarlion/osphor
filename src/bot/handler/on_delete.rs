use poise::serenity_prelude::{ChannelId, Context, Error, GuildId, MessageId};

use crate::bot::tools::*;

// Called everytime a message is deleted.
pub async fn on_delete(
    _ctx: &Context,
    _channel_id: &ChannelId,
    message_id: &MessageId,
    guild_id: &Option<GuildId>,
) -> Result<(), Error> {
    if let Some(guild_id) = guild_id {
        logger::log_mark_deleted(&guild_id, &message_id)?;
    }
    Ok(())
}
