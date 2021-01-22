use crate::errors::ParserTypeErrors;

const DEFAULT_SPLIT_DELIMITER: &str = "_";

#[derive(Clone)]
pub(crate) enum ParserType {
    Regex(String),
    Split(String),
}

impl ParserType {

    pub(crate) fn try_from(s: Option<String>, payload: Option<String>) -> Result<Option<Self>, ParserTypeErrors> {
        match match s { None => return Ok(None), Some(val) => val }.to_ascii_lowercase().as_str() {
            "regex" | "re" => { 
                if payload.is_none() {
                    return Err(ParserTypeErrors::CantFindRequiredData("regex".into()));
                };
                Ok(Some(ParserType::Regex(payload.unwrap())))
            },
            "split" => Ok(Some(ParserType::Split(payload.unwrap_or_else(|| DEFAULT_SPLIT_DELIMITER.into())))),
            _ => Ok(None)
        }
    }
}
