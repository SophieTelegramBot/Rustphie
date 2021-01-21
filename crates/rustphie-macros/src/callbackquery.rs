use crate::attr::{Attr, CallbackQueryAttributes};
use crate::errors::CallbackDeriveErrors;
use crate::parsers::{ParserPayloadData, ParserType};

#[derive(Clone)]
pub(crate) struct CallbackDeriveData {
    pub(crate) prefix: String
}

impl CallbackDeriveData {
    pub(crate) fn try_from(attrs: &[Attr<CallbackQueryAttributes>]) -> Result<Self, CallbackDeriveErrors> {
        let attrs = parse_attrs(attrs)?;

        let prefix = attrs.prefix;
        Ok(
            Self {
                prefix
            }
        )
    }
}

pub struct CallbackDeriveAttrs {
    pub(crate) prefix: String,
}

pub(crate) fn parse_attrs(attrs: &[Attr<CallbackQueryAttributes>]) -> Result<CallbackDeriveAttrs, CallbackDeriveErrors> {
    let mut prefix = None;

    for attr in attrs {
        match attr.name() {
            CallbackQueryAttributes::Prefix => prefix = Some(attr.value()),
        }
    }
    if prefix.is_none() {
        return Err(CallbackDeriveErrors::NoPrefixGiven)
    }

    Ok(
        CallbackDeriveAttrs {
            prefix: prefix.unwrap(),
        }
    )
}

impl Into<ParserPayloadData> for CallbackDeriveData {
    fn into(self) -> ParserPayloadData {
        ParserPayloadData {
            parser_type: ParserType::Split("_".into()),
        }
    }
}
