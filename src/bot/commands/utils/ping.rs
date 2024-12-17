use crate::bot::{InteractionContext, InteractionError};
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use std::time::Instant;

#[poise::command(slash_command)]
pub async fn ping(ctx: InteractionContext<'_>) -> Result<(), InteractionError> {
    // Record the start time
    let start_time = Instant::now();

    // Send the initial response
    ctx.say("Eh?").await?;

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed().as_millis();

    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new().description(format!("Pong! It took {}ms!", elapsed_time))],
        ..Default::default()
    })
    .await?;

    Ok(())
}
