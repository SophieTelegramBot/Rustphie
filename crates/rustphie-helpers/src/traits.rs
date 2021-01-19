use crate::ParseError;

/// Trait implementation used for parsing of command arguments
pub trait Command: Sized {
    /// :shrug: function will deserializes the command to implemented struct
    ///
    /// # Parameters
    ///
    /// `s`: _message text_ (including the command), eg: `/start some args here and here`
    /// `bot_username`: username of the bot, used to extract "mentioned" command, we dont intercept the command given to another bot
    fn parse<BotUsername: Into<String>>(s: &str, bot_username: BotUsername) -> Result<Self, ParseError>;
}
