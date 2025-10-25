use crate::ParserError;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};

#[derive(Debug, PartialEq)]
pub struct CsvWrapper(pub Vec<StringRecord>);

impl CsvWrapper {
    pub fn from_string(s: &str) -> Result<Self, ParserError> {
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
