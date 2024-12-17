use crate::bot::{InteractionContext, InteractionError};

#[derive(poise::ChoiceParameter)]
enum FetchOption {
    #[name = "Last"]
    Last,
}

// Fetch per guild messages log from the bot.
#[poise::command(slash_command, description_localized("en-US", "Fetches messages log from guild"))]
pub async fn fetch(
    ctx: InteractionContext<'_>,
    // Fetch the last item
    #[description = "Choose fetch filter option"] option: FetchOption,
) -> Result<(), InteractionError> {
    // Check the options
    match option {
        FetchOption::Last => {
            if let Err(why) = ctx.say("Picked last").await {
                eprintln!("[ERR] Cannot send messages. {why:?}")
            }
        }
    }

    Ok(())
}
