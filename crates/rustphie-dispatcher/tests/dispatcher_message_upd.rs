
#[cfg(test)]
mod dispatcher_message_update {
    use rustphie_dispatcher::{Dispatcher, Handler, CommandData};
    use teloxide::types::{Message, MessageKind, Chat, ChatKind, ChatPrivate, MessageCommon, ForwardKind, ForwardOrigin, MediaKind, MediaText};
    use anyhow::{Result, anyhow};
    use async_trait::async_trait;
    use futures::executor;

    #[derive(Clone)]
    struct Handler1;

    #[async_trait]
    impl Handler for Handler1 {
        async fn on_event(&self, _: Message) -> Result<()> {
            Err(anyhow!("error"))
            // throwing error will help test fn to know event has been propagated successfully
        }

        fn command(&self) -> CommandData {
            CommandData::Command(String::from("command"))
        }
    }

    #[test]
    fn test_dispatcher_propagation() {
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
            MessageKind::Common(
                MessageCommon::new(
                    ForwardKind::Origin(ForwardOrigin::new()),
                    MediaKind::Text(MediaText::new("/command", Vec::new()))
                )
            )
        );

        // propagate
        let result = executor::block_on(dispatcher.propagate_message_update(msg));
        // if success; event listener would return error
        assert!(result.is_err())
    }
}
