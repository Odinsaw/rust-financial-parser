use crate::FinancialDataRead;
use crate::ParserError;
use crate::XmlWrapper;
use quick_xml::de::from_str;
use serde_json::Value;

impl XmlWrapper {
    pub fn from_string(s: &str) -> Result<Self, ParserError> {
        let value: Value = from_str(s).map_err(|e| ParserError::Xml(e.to_string()))?;
        Ok(XmlWrapper(value))
    }
}

impl FinancialDataRead for XmlWrapper {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Xml(e.to_string()))?;
        let result = Self::from_string(&data)?;
        Ok(result)
    }
}
