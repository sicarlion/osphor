pub mod commands;
pub mod handler;

use crate::Bot;
use crate::Data;

use crate::bot::commands::*;
use crate::bot::handler::*;

use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use shuttle_serenity::ShuttleSerenity;

pub type InteractionError = Box<dyn std::error::Error + Send + Sync>;
pub type InteractionContext<'a> = poise::Context<'a, Data, InteractionError>;

impl Bot {
    pub async fn start(bot: Bot) -> ShuttleSerenity {
        // Store all the slash command here. Make sure all module in commands.rs is properly connected.
        let commands_list = vec![ping(), help(), setup(), test(), fetch(), clean()];

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
                            // Call the ready listener when bot is ready
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
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        bot.guild,
                    )
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
}
