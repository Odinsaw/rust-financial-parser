use crate::CsvWrapper;
use crate::FinancialDataWrite;
use crate::ParserError;
use csv::WriterBuilder;

impl CsvWrapper {
    pub fn to_string(&self) -> Result<String, ParserError> {
        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .from_writer(Vec::new());

        for record in &self.0 {
            wtr.write_record(record)
                .map_err(|e| ParserError::Csv(e.to_string()))?;
        }

        wtr.flush()?;
        let bytes = wtr
            .into_inner()
            .map_err(|e| ParserError::Csv(e.to_string()))?;
        Ok(String::from_utf8(bytes).map_err(|e| ParserError::Csv(e.to_string()))?)
    }
}

impl FinancialDataWrite for CsvWrapper {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = self.to_string()?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}
