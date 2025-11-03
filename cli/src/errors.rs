use thiserror::Error;

/// Represents all possible errors that can occur within the command-line interface (CLI) layer.
///
/// This enum consolidates parsing, I/O, and conversion errors that may occur
/// when handling user input or invoking format conversion logic.
/// It provides human-readable messages suitable for displaying to end users.
#[derive(Error, Debug)]
pub enum CliError {
    /// An error that occurred while parsing command-line arguments.
    ///
    /// Typically returned when user-provided flags or options are invalid or missing.
    #[error("Failed to parse arguments: {0}")]
    ArgsError(String),

    /// An error related to format parsing or validation.
    ///
    /// Wraps parsing failures originating from format-specific parsers (e.g., MT940, CAMT.053).
    #[error("Format parser error: {0}")]
    ParserError(String),

    /// An input/output (I/O) error that occurred during file or stream operations.
    ///
    /// Automatically converted from [`std::io::Error`].
    #[error("Input/output error: {0}")]
    Io(#[from] std::io::Error),

    /// A general error that occurred during format conversion.
    ///
    /// Used when an error cannot be categorized more specifically.
    #[error("Format conversion: {0}")]
    ConversionError(String),
}

impl From<anyhow::Error> for CliError {
    /// Converts a generic [`anyhow::Error`] into a [`CliError::ConversionError`].
    ///
    /// This allows functions using `anyhow::Result` to integrate seamlessly with
    /// the CLI’s error-handling model.
    fn from(err: anyhow::Error) -> Self {
        CliError::ConversionError(err.to_string())
    }
}
