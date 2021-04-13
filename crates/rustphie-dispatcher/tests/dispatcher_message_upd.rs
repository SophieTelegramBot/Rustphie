
#[cfg(test)]
mod dispatcher_message_update {
    use rustphie_dispatcher::{Dispatcher, Handler};
    use teloxide::types::{Message, MessageKind, Chat, ChatKind, ChatPrivate, MessageNewChatTitle};
    use anyhow::{Result, anyhow};
    use async_trait::async_trait;
    use futures::executor;

    #[derive(Clone)]
    struct Handler1;

    #[async_trait]
    impl Handler for Handler1 {
        async fn on_event(&self, event: Message) -> Result<()> {
            if event.id == 0 { Ok(()) } else { Err(anyhow!("test failed")) }
        }
    }

    #[test]
    fn test_dispatcher_message_upd() {
        let mut dispatcher = Dispatcher::new();

        // add event handler
        dispatcher.add_message_handler(Box::new(Handler1));

        // fake message upd
        let msg = Message::new(
            0,
            0,
            Chat::new(
                0,
                ChatKind::Private(ChatPrivate::new())),
            MessageKind::NewChatTitle(MessageNewChatTitle::new("test"))
        );

        // propagate
        let result = executor::block_on(dispatcher.propagate_message_update(msg));
        assert!(result.is_ok())
    }
}
