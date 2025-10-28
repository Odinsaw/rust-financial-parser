pub mod camt053;
pub mod converter;
pub mod csv;
pub mod errors;
pub mod mt940;
pub mod traits;
pub mod xml;

pub use camt053::format::Camt053;
pub use csv::format::CsvWrapper;
pub use errors::ParserError;
pub use mt940::format::Mt940;
pub use traits::{FinancialDataRead, FinancialDataWrite};
pub use xml::format::XmlWrapper;

#[derive(Debug, Clone, PartialEq)]
pub enum SupportedFormats {
    Csv,
    Xml,
    Camt053,
    Mt940,
}

impl std::str::FromStr for SupportedFormats {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mt940" => Ok(SupportedFormats::Mt940),
            "camt053" => Ok(SupportedFormats::Camt053),
            _ => Err(format!("Unknown format: {}. Use 'mt940' or 'camt053'", s)),
        }
    }
}
