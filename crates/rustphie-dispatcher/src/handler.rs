use async_trait::async_trait;
use anyhow::Result;
use teloxide::types::Message;

#[async_trait]
pub trait Handler: HandlerClone + Send + Sync {
    async fn on_event(&self, event: Message) -> Result<()>;
}

pub trait HandlerClone {
    fn clone_box(&self) -> Box<dyn Handler>;
}

impl<T> HandlerClone for T
    where T: 'static + Handler + Clone
{
    fn clone_box(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub(super) struct InternalHandlerStruct(pub(super) Box<dyn Handler>);
