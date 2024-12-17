pub mod commands;
pub mod handler;
pub mod tools;

use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use shuttle_serenity::ShuttleSerenity;

use crate::Bot;
use crate::Data;

use crate::bot::commands::*;
use crate::bot::handler::*;

type InteractionError = Box<dyn std::error::Error + Send + Sync>;
type InteractionContext<'a> = poise::Context<'a, Data, InteractionError>;

pub async fn start(bot: Bot) -> ShuttleSerenity {
    // Store all the slash command here. Make sure all module in commands.rs is properly connected.
    let commands_list = vec![ping(), fetch()];

    // Poise framework builder.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands_list,
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    if let FullEvent::Message { new_message } = event {
                        // Call the message listener when a new message event is triggered
                        on_message(ctx, new_message).await?;
                    }
                    if let FullEvent::Ready { data_about_bot } = event {
                        on_ready(ctx, data_about_bot).await?;
                    }
                    if let FullEvent::MessageDelete {
                        channel_id,
                        deleted_message_id,
                        guild_id,
                    } = event
                    {
                        on_delete(ctx, channel_id, deleted_message_id, guild_id).await?;
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // It best to use register_in_guild(ctx, &framework.options, GUILD_ID) for testing purpose.
                poise::builtins::register_in_guild(ctx, &framework.options().commands, bot.guild)
                    .await?;
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Serenity client builder.
    let client = ClientBuilder::new(&bot.token, GatewayIntents::all())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
