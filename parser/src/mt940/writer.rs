use crate::FinancialDataWrite;
use crate::Mt940;
use crate::ParserError;

impl FinancialDataWrite for Mt940 {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = Mt940::to_string(self)?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}
