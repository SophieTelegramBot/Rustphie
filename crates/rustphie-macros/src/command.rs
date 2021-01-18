use crate::attr::{Attr, CommandAttributes};

// todo: Ability to choose 'em
const COMMAND_PREFIX: char = '/';

pub struct CommandData {
    pub regex: Option<String>,
    pub name: String,
    pub prefix: char,
}

impl CommandData {
    pub fn try_from(attrs: &[Attr]) -> Result<Self, String> {
        let attrs = parse_attrs(attrs)?;

        let regex = attrs.regex;
        let name = attrs.command;
        Ok(Self { regex, name, prefix: COMMAND_PREFIX })
    }

    pub fn get_command(&self) -> String {
        self.prefix.to_string() + &self.name
    }
}

pub struct CommandAttrs {
    pub(crate) command: String,
    pub(crate) regex: Option<String>,
}

pub fn parse_attrs(attrs: &[Attr]) -> Result<CommandAttrs, String> {
    let mut regex = None;
    let mut command = None;

    for attr in attrs {
        match attr.name() {
            CommandAttributes::Regex => regex = Some(attr.value()),
            CommandAttributes::Command => command = Some(attr.value())
        }
    }

    Ok(CommandAttrs {
        regex,
        command: command.unwrap()
    })
}
