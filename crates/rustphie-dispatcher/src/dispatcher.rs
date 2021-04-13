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
        let command = &upd.text()
            .expect("Got unexpected message type")
            .split_whitespace()
            .next()
            .unwrap() // not possible to be none.
            [1..];
        for handler in &self.command_handlers {
            if !handler.0.validate_command(command) { continue }
            handler.0.on_event(upd.clone()).await?
        }
        Ok(())
    }
}
