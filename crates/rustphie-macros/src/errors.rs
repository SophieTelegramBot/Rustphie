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
    FailedToExtractParserType,
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
        match self {
            Self::CanBeUsedOnlyInStruct => write!(f, "Can be used only in struct"),
            Self::FailedToExtractParserType => write!(f, "Failed to determine the parser type, missing `parser` field!"),
        }
    }
}

impl Error for BasicErrors {}

#[derive(Debug)]
pub(crate) enum ParserTypeErrors {
    UnknownParserType,
    CantFindRequiredData(String),
}

impl Error for ParserTypeErrors {}

impl Display for ParserTypeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::UnknownParserType => write!(f, "Unknown parser type"),
            Self::CantFindRequiredData(t) => write!(f, "Can't find required data `{}`", t)
        }
    }
}

#[derive(Debug)]
pub(crate) enum CallbackDeriveErrors {
    NoPrefixGiven,
    ParseError(ParserTypeErrors),
}

impl Display for CallbackDeriveErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::NoPrefixGiven => write!(f, "No prefix were given!"),
            Self::ParseError(e) => write!(f, "Failed to extract parser type: {}", e),
        }
    }
}

impl Error for CallbackDeriveErrors {}
