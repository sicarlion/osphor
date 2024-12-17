use poise::serenity_prelude::{ActivityData, Context, Error, Ready};

// Called whenever the client is initialized for the first time and is ready.
pub async fn on_ready(ctx: &Context, client: &Ready) -> Result<(), Error> {
    println!("[LOG] {} is online!", client.user.display_name());
    ctx.shard
        .set_activity(Some(ActivityData::watching("0s and 1s")));
    Ok(())
}
