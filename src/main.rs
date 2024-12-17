mod bot;

use anyhow::Context;
use poise::serenity_prelude::GuildId;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

use bot::start;

// User Interaction Data will be stored temporarily here
struct Data {}

// Struct for general data about the bot
struct Bot {
    token: String,
    guild: GuildId,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let bot = Bot {
        // Get token from shuttle environment. Can be changed to plain string or any system .env
        token: secret_store
            .get("BOT_TOKEN")
            .context("[ERR] 'BOT_TOKEN' was not found")?,
        guild: GuildId::new(
            secret_store
                .get("TEST_GUILD")
                .context("[ERR] 'TEST_GUILD' was not found")?
                .parse()
                .expect("[ERR] Cannot convert TEST_GUILD to u64. Are you sure it's containing number?"),
        ),
    };

    // Start the logic on bot.rs
    start(bot).await
}
