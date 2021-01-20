use crate::errors::ParserTypeErrors;

pub(crate) enum ParserType {
    Regex,
    Split,
}

impl ParserType {

    pub(crate) fn try_from(s: &str) -> Result<Self, ParserTypeErrors> {
        match s.to_ascii_lowercase().as_str() {
            "regex" | "re" => Ok(ParserType::Regex),
            "split" => Ok(ParserType::Split),
            _ => Err(ParserTypeErrors::UnknownParserType)
        }
    }
}
