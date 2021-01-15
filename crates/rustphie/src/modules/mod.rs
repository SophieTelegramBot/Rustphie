use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::requests::ResponseResult;
use teloxide::utils::command::BotCommand;

mod start;

#[derive(BotCommand)]
#[command(rename = "lowercase")]
pub enum Commands {
    Start,
}

pub async fn handle_updates(cx: UpdateWithCx<Message>, command: Commands) -> ResponseResult<()> {
    match command {
        Commands::Start => start::start_command(cx).await?,
    };
    Ok(())
}
