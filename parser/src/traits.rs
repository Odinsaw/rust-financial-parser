use crate::errors::ParserError;
use std::io::{Read, Write};

/// Trait for reading financial data objects from input streams.
///
/// This trait defines a standard interface for deserializing financial data
/// (e.g., MT940, CAMT.053, CSV) from a source that implements [`std::io::Read`].
///
/// Implementations of this trait should parse the input and construct
/// a strongly typed representation of the financial data.
///
/// # Example
///
/// use mycrate::traits::FinancialDataRead;
/// use mycrate::Mt940;
///
/// let file = std::fs::File::open("statement.mt940")?;
/// let mt940 = Mt940::from_read(file)?;
pub trait FinancialDataRead: Sized {
    /// Creates a financial data object from a readable data source.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if the input data cannot be parsed successfully.
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError>;

    /// Reads the entire input stream into a string.
    ///
    /// This helper method provides a convenient way to read data before parsing.
    ///
    /// # Errors
    ///
    /// Returns a [`std::io::Error`] if reading from the stream fails.
    fn read_to_string<R: std::io::Read>(reader: R) -> Result<String, std::io::Error> {
        let mut buffered = std::io::BufReader::new(reader);
        let mut data = String::new();
        buffered.read_to_string(&mut data)?;
        Ok(data)
    }
}

/// Trait for writing financial data objects to output streams.
///
/// This trait defines a standard interface for serializing financial data
/// (e.g., MT940, CAMT.053, CSV) into any destination that implements [`std::io::Write`].
///
/// Implementations of this trait should take a structured data object and
/// produce the appropriate textual or binary output.
///
/// # Example
///
/// use mycrate::traits::FinancialDataWrite;
/// use mycrate::Mt940;
///
/// let mt940 = Mt940::new(...);
/// let mut file = std::fs::File::create("statement.mt940")?;
/// mt940.write_to(&mut file)?;
pub trait FinancialDataWrite {
    /// Writes the financial data object to the provided writable destination.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if serialization fails or if a write error occurs.
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError>;

    /// Writes a raw string to the provided writable destination.
    ///
    /// This helper method can be used to simplify implementations
    /// that output string-based representations of financial data.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if an I/O operation fails.
    fn write_string<W: std::io::Write>(writer: W, data: &str) -> Result<(), ParserError> {
        let mut buffered = std::io::BufWriter::new(writer);
        buffered.write_all(data.as_bytes())?;
        buffered.flush()?;
        Ok(())
    }
}
