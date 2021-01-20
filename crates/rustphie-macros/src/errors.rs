use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub(crate) enum CommandsError {
    NoCommandsSpecified,
    ParseError(ParserTypeErrors)
}

impl Error for CommandsError {}

impl Display for CommandsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::NoCommandsSpecified => write!(f, "No commands specified in attribute, tip: add field `command = \"...\"`"),
            Self::ParseError(e) => write!(f, "failed to parse command attributes: {}", e)
        }
    }
}

#[derive(Debug)]
pub enum BasicErrors {
    CanBeUsedOnlyInStruct,
}

impl BasicErrors {
    pub fn compile_error(self) -> proc_macro::TokenStream {
        let display = format!("{}", self);
        proc_macro::TokenStream::from(
            quote::quote! {
                compile_error!(#display);
            }
        )
    }
}

impl Display for BasicErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::CanBeUsedOnlyInStruct => write!(f, "Can be used only in struct")
        }
    }
}

impl Error for BasicErrors {}

#[derive(Debug)]
pub(crate) enum ParserTypeErrors {
    UnknownParserType
}

impl Error for ParserTypeErrors {}

impl Display for ParserTypeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::UnknownParserType => write!(f, "Unknown parser type")
        }
    }
}
