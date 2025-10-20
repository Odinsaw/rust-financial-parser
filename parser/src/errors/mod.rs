use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("MT940 parsing error: {0}")]
    Mt940(String),
    #[error("Camt053 parsing error: {0}")]
    Camt053(String),
    #[error("Input/Output error: {0}")]
    Io(String),
    #[error("Input format error: {0}")]
    InvalidInput(String),
    #[error("General error: {0}")]
    GeneralError(String),
}

impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        ParserError::Io(error.to_string())
    }
}

impl From<std::fmt::Error> for ParserError {
    fn from(error: std::fmt::Error) -> Self {
        ParserError::Io(error.to_string())
    }
}
