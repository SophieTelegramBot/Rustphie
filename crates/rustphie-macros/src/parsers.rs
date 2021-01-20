use crate::errors::ParserTypeErrors;

const DEFAULT_SPLIT_DELIMITER: &str = "";

#[derive(Clone)]
pub(crate) enum ParserType {
    Regex,
    Split(String),
}

impl ParserType {

    pub(crate) fn try_from(s: &str, delim: Option<String>) -> Result<Self, ParserTypeErrors> {
        match s.to_ascii_lowercase().as_str() {
            "regex" | "re" => Ok(ParserType::Regex),
            "split" => Ok(ParserType::Split(delim.unwrap_or_else(|| DEFAULT_SPLIT_DELIMITER.into()))),
            _ => Err(ParserTypeErrors::UnknownParserType)
        }
    }
}
