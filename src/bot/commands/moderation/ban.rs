use poise::serenity_prelude::{CreateEmbed, CreateMessage, Guild, Member};

use crate::{
    bot::{InteractionContext, InteractionError},
    tools::{Exception, Replies},
};

#[poise::command(
    slash_command,
    rename = "ban",
    description_localized("en-US", "Ban a member from the guild"),
    guild_only,
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn ban(
    ctx: InteractionContext<'_>,
    #[description = "Whom to ban?"] member: Member,
    #[description = "What is the reason?"] reason: String,
    #[description = "Can this member appeal this ban?"]
    #[min = 1]
    appeal: Option<bool>,
) -> Result<(), InteractionError> {
    // Retrieve the guild ID and ensure the command is in a guild
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            eprintln!("Command is not on guild.");
            return Ok(());
        }
    };

    // Clone the guild object before awaiting any futures
    let guild = match ctx.cache().guild(guild_id) {
        Some(guild) => guild.clone(),
        None => {
            eprintln!("Command is not on guild, somehow.");
            return Ok(());
        }
    };

    // Get bot user ID
    let bot_id = ctx.cache().current_user().id;

    // Retrieve bot [Member] data from the guild
    let bot_member = guild
        .member(&ctx.http(), bot_id)
        .await
        .unwrap()
        .into_owned();

    // Check for permissions
    if let Err(why) = check(&ctx, guild, bot_member, &member).await {
        return Ok(());
    };

    if let Ok(dm_channel) = member.user.create_dm_channel(&ctx.http()).await {
        dm_channel
            .send_message(&ctx.http(), CreateMessage::new().embed(CreateEmbed::new()
            .title(format!("Banned from {}", guild_id.name(&ctx).unwrap()))
            .description(format!("Your access to this guild has been revoked and future request to rejoin will be prevented due to the following reason:\n\n**{}**", &reason))))
        .await?;
    }

    // Ban
    match member.ban_with_reason(&ctx.http(), 0, &reason).await {
        Ok(_) => {
            let message = &format!("{} has been banned due to: {}", member.user.name, &reason);
            Replies::say(&ctx, message).await?
        }
        Err(why) => Replies::say(&ctx, &why.to_string()).await?,
    };

    Ok(())
}

pub async fn check(
    ctx: &InteractionContext<'_>,
    guild: Guild,
    bot: Member,
    target: &Member,
) -> Result<(), InteractionError> {
    let bot_role = guild.member_highest_role(&bot);
    let target_role = guild.member_highest_role(&target);

    // Case 1: Bot has no role, but target has one
    if bot_role.is_none() && target_role.is_some() {
        Replies::error(&ctx, Exception::MismatchedRoles).await?;
        return Err("Failed".into());
    }

    // Case 2: Bot has a role, but target has none
    if bot_role.is_some() && target_role.is_none() {
        Replies::error(&ctx, Exception::MismatchedRoles).await?;
        return Err("Failed".into());
    }

    // Case 3: Both has none, this good.
    if bot_role.is_none() && target_role.is_none() {
        return Ok(());
    }

    // Unwrap the value since we know that both of the value is_some
    let bot_role = bot_role.unwrap();
    let target_role = target_role.unwrap();

    // Compare that target member roles is lower than bot roles.
    if target_role.position < bot_role.position {
        return Ok(());
    } else {
        Replies::error(&ctx, Exception::RolesHierarchy).await?;
        return Err("Failed".into());
    }
}
