use anyhow::Result;
use teloxide::types::Message;
use crate::handler::{InternalHandlerStruct, Handler};

pub struct Dispatcher {
    command_handlers: Vec<InternalHandlerStruct>
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            command_handlers: Vec::new(),
        }
    }

    pub fn add_message_handler(&mut self, handler: Box<dyn Handler>) {
        let handler = InternalHandlerStruct(handler);
        self.command_handlers.push(handler);
    }

    pub async fn propagate_message_update(&self, upd: Message) -> Result<()> {
        for handler in &self.command_handlers {
            handler.0.on_event(upd.clone()).await?
        }
        Ok(())
    }
}
