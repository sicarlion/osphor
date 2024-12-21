use poise::serenity_prelude::{ChannelId, Context, Error, GuildId, MessageId};

use crate::tools::Log;

// Called everytime a message is deleted.
pub async fn on_delete(
    _ctx: &Context,
    channel_id: &ChannelId,
    message_id: &MessageId,
    guild_id: &Option<GuildId>,
) -> Result<(), Error> {
    if let Some(guild_id) = guild_id {
        Log::mark_deleted(&guild_id, &channel_id, &message_id)?;
    }
    Ok(())
}
