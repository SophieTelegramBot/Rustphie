use crate::errors::ParserTypeErrors;

const DEFAULT_SPLIT_DELIMITER: &str = "";

#[derive(Clone)]
pub(crate) enum ParserType {
    Regex(String),
    Split(String),
}

impl ParserType {

    pub(crate) fn try_from(s: &str, payload: Option<String>) -> Result<Self, ParserTypeErrors> {
        match s.to_ascii_lowercase().as_str() {
            "regex" | "re" => { 
                if payload.is_none() {
                    return Err(ParserTypeErrors::CantFindRequiredData("regex".into()));
                };
                Ok(ParserType::Regex(payload.unwrap())) 
            },
            "split" => Ok(ParserType::Split(payload.unwrap_or_else(|| DEFAULT_SPLIT_DELIMITER.into()))),
            _ => Err(ParserTypeErrors::UnknownParserType)
        }
    }
}

#[derive(Clone)]
pub(crate) struct ParserPayloadData {
    pub(crate) parser_type: ParserType,
}
