use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Failed to parse arguments: {0}")]
    ArgsError(String),
    #[error("Format parser error: {0}")]
    ParserError(String),
    #[error("Input/output error: {0}")]
    Io(#[from] std::io::Error),
}
