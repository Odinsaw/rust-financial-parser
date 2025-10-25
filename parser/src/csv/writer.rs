use crate::CsvWrapper;
use crate::FinancialDataWrite;
use crate::ParserError;

impl FinancialDataWrite for CsvWrapper {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = self.to_string()?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}
