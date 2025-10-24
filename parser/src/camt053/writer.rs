use crate::Camt053;
use crate::FinancialDataWrite;
use crate::ParserError;

use quick_xml::se::to_string;

impl FinancialDataWrite for Camt053 {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = to_string(self).map_err(|e| ParserError::Camt053(e.to_string()))?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}
