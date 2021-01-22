use crate::attr::{Attr, CommandAttributes};
use crate::errors::CommandsError;
use crate::parsers::ParserType;

// todo: Ability to choose 'em
const COMMAND_PREFIX: char = '/';

#[derive(Clone)]
pub struct CommandData {
    pub(crate) name: String,
    pub(crate) prefix: char,
    pub(crate) parser_type: Option<ParserType>,
}

impl CommandData {
    pub(crate) fn try_from(attrs: &[Attr<CommandAttributes>]) -> Result<Self, CommandsError> {
        let attrs = parse_attrs(attrs)?;

        let name = attrs.command;
        Ok(Self { name, prefix: COMMAND_PREFIX, parser_type: attrs.parser })
    }

    pub(crate) fn get_command(&self) -> String {
        self.prefix.to_string() + &self.name
    }
}

pub struct CommandAttrs {
    pub(crate) command: String,
    pub(crate) parser: Option<ParserType>,
}

pub(crate) fn parse_attrs(attrs: &[Attr<CommandAttributes>]) -> Result<CommandAttrs, CommandsError> {
    let mut command = None;
    let mut parser = None;
    let mut data = None;

    for attr in attrs {
        match attr.name() {
            CommandAttributes::Command => command = Some(attr.value()),
            CommandAttributes::Parser => parser = Some(attr.value()),
            CommandAttributes::Data => data = Some(attr.value()),
        }
    }

    let command = command.ok_or(CommandsError::NoCommandsSpecified)?;
    let parser = ParserType::try_from(parser, data)
        .map_err(CommandsError::ParseError)?;

    Ok(CommandAttrs {
        command,
        parser,
    })
}
