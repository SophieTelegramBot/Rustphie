use crate::attr::{Attr, CallbackQueryAttributes};
use crate::errors::CallbackDeriveErrors;
use crate::parsers::ParserType;

#[derive(Clone)]
pub(crate) struct CallbackDeriveData {
    pub(crate) prefix: String,
    pub(crate) parser: Option<ParserType>,
}

impl CallbackDeriveData {
    pub(crate) fn try_from(attrs: &[Attr<CallbackQueryAttributes>]) -> Result<Self, CallbackDeriveErrors> {
        let attrs = parse_attrs(attrs)?;

        let prefix = attrs.prefix;
        let parser = attrs.parser;
        Ok(
            Self {
                prefix,
                parser,
            }
        )
    }
}

pub struct CallbackDeriveAttrs {
    pub(crate) prefix: String,
    pub(crate) parser: Option<ParserType>,
}

pub(crate) fn parse_attrs(attrs: &[Attr<CallbackQueryAttributes>]) -> Result<CallbackDeriveAttrs, CallbackDeriveErrors> {
    let mut prefix = None;
    let mut delim = None;

    for attr in attrs {
        match attr.name() {
            CallbackQueryAttributes::Prefix => prefix = Some(attr.value()),
            CallbackQueryAttributes::Delimiter => delim = Some(attr.value()),
        }
    }
    if prefix.is_none() {
        return Err(CallbackDeriveErrors::NoPrefixGiven)
    }

    let parser = ParserType::try_from(Some("split".into()), delim)
        .map_err(CallbackDeriveErrors::ParseError)?;

    Ok(
        CallbackDeriveAttrs {
            prefix: prefix.unwrap(),
            parser,
        }
    )
}
