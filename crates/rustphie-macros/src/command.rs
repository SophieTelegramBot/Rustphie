use crate::attr::{Attr, CommandAttributes};
use crate::errors::CommandsError;
use crate::parsers::{ParserPayloadData, ParserType};

// todo: Ability to choose 'em
const COMMAND_PREFIX: char = '/';

#[derive(Clone)]
pub struct CommandData {
    pub(crate) regex: Option<String>,
    pub(crate) name: String,
    pub(crate) prefix: char,
    pub(crate) parser_type: ParserType,
}

impl CommandData {
    pub(crate) fn try_from(attrs: &[Attr<CommandAttributes>]) -> Result<Self, CommandsError> {
        let attrs = parse_attrs(attrs)?;

        let regex = attrs.regex;
        let name = attrs.command;
        Ok(Self { regex, name, prefix: COMMAND_PREFIX, parser_type: attrs.parser })
    }

    pub(crate) fn get_command(&self) -> String {
        self.prefix.to_string() + &self.name
    }
}

pub struct CommandAttrs {
    pub(crate) command: String,
    pub(crate) regex: Option<String>,
    pub(crate) parser: ParserType,
}

pub(crate) fn parse_attrs(attrs: &[Attr<CommandAttributes>]) -> Result<CommandAttrs, CommandsError> {
    let mut regex = None;
    let mut command = None;
    let mut parser = None;
    let mut separator = None;

    for attr in attrs {
        match attr.name() {
            CommandAttributes::Regex => regex = Some(attr.value()),
            CommandAttributes::Command => command = Some(attr.value()),
            CommandAttributes::Parser => parser = Some(attr.value()),
            CommandAttributes::Separator => separator = Some(attr.value()),
        }
    }

    // We dont want both `parser` and `regex` fields to be included
    // sometimes user can pass different parser other parser other than regex.
    // currently we pretends the `parser` field doesnt even exists if both `regex` and `parser` fields are given!
    // kinda hacky, we should inform the user about this?
    // TODO: Inform user about ignoring `parser` field when both regex and parser fields are given
    if regex.is_some() && parser.is_some() {
        parser = None;
    }

    let command = command.ok_or(CommandsError::NoCommandsSpecified)?;
    let parser = if parser.is_some() {
        let __unwrapped_parser_input = parser.unwrap();
        ParserType::try_from(__unwrapped_parser_input.as_str(), separator)
    } else {
        ParserType::try_from("regex", regex.clone())
    }
        .map_err(CommandsError::ParseError)?;

    Ok(CommandAttrs {
        regex,
        command,
        parser,
    })
}

impl Into<ParserPayloadData> for CommandData {
    fn into(self) -> ParserPayloadData {
        ParserPayloadData {
            parser_type: self.parser_type
        }
    }
}
