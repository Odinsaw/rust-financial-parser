use crate::CsvWrapper;
use crate::FinancialDataRead;
use crate::ParserError;
use csv::ReaderBuilder;

impl CsvWrapper {
    fn from_string(s: &str) -> Result<Self, ParserError> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(s.as_bytes());

        let mut records = Vec::new();
        for result in rdr.records() {
            records.push(result.map_err(|e| ParserError::Csv(e.to_string()))?)
        }

        let headers = rdr
            .headers()
            .map_err(|e| ParserError::Csv(e.to_string()))?
            .clone();
        records.insert(0, headers);

        Ok(CsvWrapper(records))
    }
}

impl FinancialDataRead for CsvWrapper {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Csv(e.to_string()))?;
        let result = Self::from_string(&data)?;
        Ok(result)
    }
}
