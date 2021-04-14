use anyhow::Result;
use teloxide::types::Message;
use crate::handler::{InternalHandlerStruct, Handler};
use teloxide::dispatching::UpdateWithCx;
use teloxide::adaptors::AutoSend;
use teloxide::Bot;
use teloxide::prelude::{DispatcherHandlerRx, DispatcherHandlerRxExt, StreamExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use std::sync::Arc;

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

    async fn propagate_message_update(&self, upd: UpdateWithCx<AutoSend<Bot>, Message>, text: String) -> Result<()> {
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

    pub async fn dispatch(dispatcher: Self, bot: AutoSend<Bot>,) {
        let self_arc = Arc::new(dispatcher);
        log::debug!("Registering dispatcher with teloxide");
        let listener = teloxide::dispatching::Dispatcher::new(bot)
            .messages_handler(move |rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
                UnboundedReceiverStream::new(rx).text_messages().for_each_concurrent(
                    None,
                    move |(cx, cmd)| {
                        let dispatcher = Arc::clone(&self_arc);

                        async move {
                            dispatcher.propagate_message_update(cx, cmd).await;
                        }
                    }
                )
            });
        log::info!("Successfully registered dispatcher with teloxide listener");
        log::debug!("Listening for updates");
        listener
            .dispatch()
            .await;
    }
}
