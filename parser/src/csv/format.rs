use csv::StringRecord;

#[derive(Debug, PartialEq)]
pub struct CsvWrapper(pub Vec<StringRecord>);
