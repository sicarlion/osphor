use anyhow::Error;
use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::bot::InteractionContext;

pub struct Replies {}
pub enum Exception {
    NotGuild,
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

    pub async fn error(ctx: &InteractionContext<'_>, err: Exception) -> Result<(), Error> {
        let error = match err {
            Exception::NotGuild => "Command is not executed in guild",
            Exception::Null => "This command cannot be executed"
        };

        ctx.send(CreateReply {
            embeds: vec![CreateEmbed::new().description(error)],
            ..Default::default()
        })
        .await?;

        return Ok(());
    }
}
