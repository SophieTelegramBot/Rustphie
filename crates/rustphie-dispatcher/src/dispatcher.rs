use anyhow::Result;
use teloxide::types::Message;
use crate::handler::{InternalHandlerStruct, Handler};
use teloxide::dispatching::UpdateWithCx;
use teloxide::adaptors::AutoSend;
use teloxide::Bot;

pub struct Dispatcher {
    command_handlers: Vec<InternalHandlerStruct>
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            command_handlers: Vec::new(),
        }
    }

    pub fn add_message_handler<T: 'static>(&mut self, handler: Box<T>)
        where
            T: Handler
    {
        let handler = InternalHandlerStruct(handler);
        self.command_handlers.push(handler);
        if cfg!(debug_assertions) {
            let type_name = std::any::type_name::<T>();
            log::debug!("Registered {} module successfully", type_name)
        }
    }

    pub async fn propagate_message_update(&self, upd: UpdateWithCx<AutoSend<Bot>, Message>, text: String) -> Result<()> {
        log::debug!(
            "Received update. ID: {update_id}",
            update_id=upd.update.id,
        );
        let command = &text
            .split_whitespace()
            .next()
            .unwrap() // not possible to be none.
            [1..];
        for handler in &self.command_handlers {
            if !handler.0.validate_command(command) { continue }
            handler.0.on_event(&upd).await?
        }
        Ok(())
    }
}
