use crate::FinancialDataRead;
use crate::XmlWrapper;
use crate::ParserError;
use serde::Deserialize;
use std::io::Read;

impl XmlWrapper {
    pub fn from_string(s: &str) -> Result<Self, ParserError> {
        Ok(XmlWrapper(s.to_string()))
    }

    pub fn parse_xml(&self) -> Result<serde_json::Value, ParserError> {
        if self.0.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        quick_xml::de::from_str(&self.0).map_err(|e| ParserError::Xml(e.to_string()))
    }
}

impl FinancialDataRead for XmlWrapper {
    fn from_read<R: Read>(reader: R) -> Result<Self, ParserError> {
        let mut data = String::new();
        std::io::BufReader::new(reader)
            .read_to_string(&mut data)
            .map_err(|e| ParserError::Xml(e.to_string()))?;
        Ok(XmlWrapper(data))
    }
}

impl<'de> Deserialize<'de> for XmlWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(XmlWrapper(s))
    }
}
