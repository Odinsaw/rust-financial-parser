use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("MT940 parsing error: {0}")]
    Mt940(String),
    #[error("Input/Output error: {0}")]
    Io(String),
    #[error("Input format error: {0}")]
    InvalidInput(String),
}
