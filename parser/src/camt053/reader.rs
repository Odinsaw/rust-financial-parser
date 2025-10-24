use crate::Camt053;
use crate::FinancialDataRead;
use crate::ParserError;

use quick_xml::de::from_str;

impl FinancialDataRead for Camt053 {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Camt053(e.to_string()))?;
        let result: Camt053 = from_str(&data).map_err(|e| ParserError::Camt053(e.to_string()))?;
        Ok(result)
    }
}
