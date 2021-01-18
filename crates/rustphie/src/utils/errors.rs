use std::error::Error;

type PrefixedBotCommand = String;
type BotUsername = String;
type Regex = String;
type CommandArguments = String;

#[derive(Debug)]
pub enum ParseError {
    TooFewArguments {
        expected: usize,
        found: usize,
        message: String,
    },
    TooManyArguments {
        expected: usize,
        found: usize,
        message: String,
    },
    IncorrectFormat(Box<dyn Error + Send + Sync + 'static>),
    UnknownCommand(PrefixedBotCommand),
    WrongBotName(BotUsername),
    NoCapturesFound(Regex, CommandArguments),
}
