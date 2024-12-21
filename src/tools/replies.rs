use crate::bot::InteractionContext;

use poise::{serenity_prelude::{Context, CreateEmbed, CreateMessage, Message}, CreateReply};

use anyhow::Error;

pub struct Replies {}
pub enum Exception {
    NotGuild,
    EmptyLog,
    Null
}

impl Replies {
    pub async fn say(ctx: &InteractionContext<'_>, msg: &str) -> Result<(), Error> {
        ctx.send(CreateReply {
            embeds: vec![CreateEmbed::new().description(msg)],
            ..Default::default()
        })
        .await?;

        return Ok(());
    }

    pub async fn embed(ctx: &InteractionContext<'_>, embed: CreateEmbed) -> Result<(), Error> {
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;

        return Ok(());
    }

    pub async fn send(ctx: &InteractionContext<'_>, msg: &str) -> Result<(), Error> {
        ctx.channel_id().send_message(&ctx.http(), CreateMessage::new().embed(
            CreateEmbed::new().description(msg)
        )).await?;
        return Ok(());
    }

    pub async fn raw_send(ctx: &Context, msg: &Message, content: &str) -> Result<(), Error> {
        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(
            CreateEmbed::new().description(content)
        )).await?;
        return Ok(());
    }

    pub async fn error(ctx: &InteractionContext<'_>, err: Exception) -> Result<(), Error> {
        let error = match err {
            Exception::NotGuild => "Command is not executed in guild",
            Exception::EmptyLog => "Log file is empty, please initiate first message",
            Exception::Null => "This command cannot be executed"
        };

        ctx.send(CreateReply {
            embeds: vec![CreateEmbed::new().description(error)],
            ..Default::default()
        })
        .await?;

        return Ok(());
    }
 
    pub async fn raw_error(ctx: &Context, msg: &Message, err: Exception) -> Result<(), Error> {
        let error = match err {
            Exception::NotGuild => "Command is not executed in guild",
            Exception::EmptyLog => "Log file is empty, please initiate first message",
            Exception::Null => "This command cannot be executed"
        };

        msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(
            CreateEmbed::new().description(error)
        )).await?;

        return Ok(());
    }
}
