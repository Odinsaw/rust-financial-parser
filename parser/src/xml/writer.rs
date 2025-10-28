use crate::FinancialDataWrite;
use crate::XmlWrapper;
use crate::ParserError;
use serde::Serialize;
use std::io::Write;

impl XmlWrapper {
    pub fn to_string(&self) -> Result<String, ParserError> {
        Ok(self.0.clone())
    }
}

impl FinancialDataWrite for XmlWrapper {
    fn write_to<W: Write>(&self, writer: W) -> Result<(), ParserError> {
        std::io::BufWriter::new(writer)
            .write_all(self.0.as_bytes())
            .map_err(|e| ParserError::Xml(e.to_string()))?;
        Ok(())
    }
}

impl Serialize for XmlWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}
