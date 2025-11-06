use csv::StringRecord;

#[derive(Debug, PartialEq)]
pub(crate) struct CsvWrapper(pub(crate) Vec<StringRecord>);
