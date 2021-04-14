use anyhow::Result;
use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::requests::{ResponseResult, Request};
use rustphie_dispatcher::{Handler, CommandData};
use async_trait::async_trait;
use teloxide::adaptors::AutoSend;
use teloxide::Bot;

#[derive(Clone)]
pub(super) struct Start;

#[async_trait]
impl Handler for Start {
    async fn on_event(&self, event: &UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
        println!("{}", event.update.id);
        Ok(())
    }

    fn command(&self) -> CommandData {
        CommandData::Command(String::from("start"))
    }
}
