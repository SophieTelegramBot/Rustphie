use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::requests::{ResponseResult, Request};

pub async fn start_command(message: UpdateWithCx<Message>) -> ResponseResult<()> {
    message.reply_to("Hey there!").send().await?;
    Ok(())
}
