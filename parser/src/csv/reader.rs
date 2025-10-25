use crate::CsvWrapper;
use crate::FinancialDataRead;
use crate::ParserError;

impl FinancialDataRead for CsvWrapper {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Csv(e.to_string()))?;
        let result = Self::from_string(&data)?;
        Ok(result)
    }
}
