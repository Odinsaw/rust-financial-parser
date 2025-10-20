use crate::errors::ParserError;
use std::io::{Read, Write};

pub trait FinancialDataRead: Sized {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError>;

    fn read_to_string<R: std::io::Read>(reader: R) -> Result<String, std::io::Error> {
        let mut buffered = std::io::BufReader::new(reader);
        let mut data = String::new();
        buffered.read_to_string(&mut data)?;
        Ok(data)
    }
}

pub trait FinancialDataWrite {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError>;

    fn write_string<W: std::io::Write>(writer: W, data: &str) -> Result<(), ParserError> {
        let mut buffered = std::io::BufWriter::new(writer);
        buffered.write_all(data.as_bytes())?;
        buffered.flush()?;
        Ok(())
    }
}
