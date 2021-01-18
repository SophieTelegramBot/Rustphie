use super::ParseError;

/// TODO
pub trait Command: Sized {
    /// TODO
    fn parse<BotUsername: Into<String>>(s: &str, bot_username: BotUsername) -> Result<Self, ParseError>;
}
