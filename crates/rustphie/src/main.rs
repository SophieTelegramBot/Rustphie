use teloxide::Bot;
use config::Configuration;
use rustphie_dispatcher::Dispatcher;
use teloxide::dispatching::DispatcherHandlerRx;
use teloxide::types::Message;
use teloxide::prelude::{DispatcherHandlerRxExt, StreamExt, RequesterExt};
use teloxide::adaptors::AutoSend;
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::sync::Arc;

mod modules;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    teloxide::enable_logging!();
    // load config
    log::debug!("Loading configuration");
    let bot_config = Configuration::load_config()?;
    log::debug!("Successfully loaded configurations");

    // init bot
    log::debug!("Initiating bot");
    let bot = Bot::new(bot_config.token)
        .auto_send();
    log::info!("Successfully initiated bot");

    log::debug!("Initiating dispatcher");
    let mut dispatcher = Dispatcher::new();
    modules::register_mods(&mut dispatcher);
    log::debug!("Successfully registered all modules");

    Dispatcher::dispatch(dispatcher, bot)
        .await;
    Ok(())
}
