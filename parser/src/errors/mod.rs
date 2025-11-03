use thiserror::Error;

/// Represents all possible errors that can occur during parsing and data conversion.
///
/// This enum consolidates various error types produced by parsers for supported formats
/// (e.g., MT940, CAMT.053, CSV, XML), as well as general I/O and conversion issues.
/// Each variant contains a descriptive error message for debugging and logging purposes.
#[derive(Error, Debug)]
pub enum ParserError {
    /// An error that occurred while parsing an **MT940** file.
    ///
    /// Contains a description of the parsing issue.
    #[error("MT940 parsing error: {0}")]
    Mt940(String),

    /// An error that occurred while parsing a **CAMT.053** file.
    ///
    /// Contains a description of the parsing issue.
    #[error("Camt053 parsing error: {0}")]
    Camt053(String),

    /// An **input/output (I/O)** error, such as a failed file read or write operation.
    #[error("Input/Output error: {0}")]
    Io(String),

    /// An error indicating that the input data was malformed or did not match the expected format.
    #[error("Input format error: {0}")]
    InvalidInput(String),

    /// A **general, non-specific error** that does not fall under other categories.
    #[error("General error: {0}")]
    GeneralError(String),

    /// An error that occurred while parsing a **CSV** file.
    #[error("CSV parsing error: {0}")]
    Csv(String),

    /// An error that occurred while parsing an **XML** file.
    #[error("XML parsing error: {0}")]
    Xml(String),

    /// An error that occurred while **converting data between formats**.
    #[error("Format conversion error: {0}")]
    Converter(String),
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
