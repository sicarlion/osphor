use crate::bot::{tools::OsphorLog, InteractionContext, InteractionError};

#[derive(poise::ChoiceParameter)]
enum FetchOption {
    #[name = "last"]
    Last,
    #[name = "all"]
    All,
}

// Fetch per guild messages log from the bot.
#[poise::command(
    slash_command,
    description_localized("en-US", "Fetches messages log from guild")
)]
pub async fn fetch(
    ctx: InteractionContext<'_>,
    // Fetch the last item
    #[description = "Choose the filter for fetching log"] option: FetchOption,
) -> Result<(), InteractionError> {
    // Check the options
    match option {
        FetchOption::Last => {
            if let None = ctx.guild_id() {
                if let Err(why) = ctx
                    .say("> You need to be on a Guild to use this command.")
                    .await
                {
                    eprintln!("[ERR] Cannot send replies. {why:?}")
                }
                return Ok(());
            }

            let guild_dir = format!("./guild/{}/messages.log", ctx.guild_id().unwrap());
            let last = match OsphorLog::fetch(&guild_dir)?.last() {
                Some(entry) => entry.print(),
                None => {
                    eprintln!("[ERR] No log entries found in the file: {}", guild_dir);
                    return Ok(());
                }
            };

            if let Err(why) = ctx.say(last).await {
                eprintln!("[ERR] Cannot send replies. {why:?}")
            }
        }
        FetchOption::All => {
            let guild_dir = format!("./guild/{}/messages.log", ctx.guild_id().unwrap());
            let osphor_log = OsphorLog::fetch(&guild_dir)?;
            let log = osphor_log.all();

            let mut all = String::new();

            for x in log {
                all.push_str(&x.print());
                all.push('\n');
            }

            ctx.say(all).await?;
        }
    }

    Ok(())
}
