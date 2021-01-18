use teloxide::Bot;

use config::Configuration;

mod modules;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    teloxide::enable_logging!();
    // load config
    log::debug!("Loading configuration");
    let bot_config = Configuration::load_config()?;
    log::debug!("Successfully loaded configurations");

    // init bot
    log::debug!("Initiating bot");
    let bot = Bot::builder()
        .token(bot_config.token)
        .build();

    // todo: bot_name? static lifetime wth???
    Ok(teloxide::commands_repl(bot, "", modules::handle_updates).await)
}
