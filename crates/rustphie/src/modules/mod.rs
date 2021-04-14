use teloxide::dispatching::UpdateWithCx;
use teloxide::types::Message;
use teloxide::requests::ResponseResult;
use rustphie_dispatcher::Dispatcher;

mod start;

pub fn register_mods(dp: &mut Dispatcher) {
    log::debug!("Registering modules");
    dp.add_message_handler(Box::new(start::Start))
}
