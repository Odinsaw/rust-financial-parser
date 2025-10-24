use crate::errors::ParserError;
use crate::traits::{FinancialDataRead, FinancialDataWrite};

use regex::Regex;
use std::fmt::Write;

use swift_mt_message::messages;

#[derive(Debug, Clone, PartialEq)]
pub struct Mt940 {
    pub basic_header: BasicHeaderBlock,
    pub application_header: String,  // depends on implementation
    pub user_header: Option<String>, // depends on implementation, may be skipped
    pub statement: messages::MT940,
    pub footer: Option<String>, // depends on implementation, may be skipped
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicHeaderBlock {
    application_identifier: String,
    service_identifier: String,
    lt_identifier: String,
    session_number: String,
    sequence_number: String,
}

impl BasicHeaderBlock {
    pub fn from_string(data: &str) -> Result<Self, ParserError> {
        // This len is fixed
        if data.len() != 25 {
            return Err(ParserError::Mt940(
                "Invalid Basic Header Block length!".to_string(),
            ));
        }

        Ok(Self {
            application_identifier: data[0..1].to_string(),
            service_identifier: data[1..3].to_string(),
            lt_identifier: data[3..15].to_string(),
            session_number: data[15..19].to_string(),
            sequence_number: data[19..25].to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.application_identifier,
            self.service_identifier,
            self.lt_identifier,
            self.session_number,
            self.sequence_number
        )
    }
}

// MT940 Can have up to 5 blocks, looking like this: e.g. {1:...}{2:...}{4:...}
fn split_to_blocks(data: &str) -> Vec<Option<String>> {
    let mut result = vec![None; 5];

    let re = Regex::new(r"\{(\d):([^}]*)\}").unwrap();

    for caps in re.captures_iter(data) {
        if let (Some(num), Some(data)) = (caps.get(1), caps.get(2)) {
            if let Ok(idx) = num.as_str().parse::<usize>() {
                if (1..=5).contains(&idx) {
                    result[idx - 1] = Some(data.as_str().to_string());
                }
            }
        }
    }

    result
}

impl Mt940 {
    pub fn to_string(&self) -> Result<String, ParserError> {
        let mut msg = String::new();

        let _ = write!(
            msg,
            "{{1:{}}}{{2:{}}}\r\n",
            self.basic_header.to_string(),
            self.application_header
        )?;

        if let Some(ref uh) = self.user_header {
            let _ = write!(msg, "{{3:{}}}\r\n", uh)?;
        }

        let _ = write!(msg, "{{4:{}\r\n", self.statement.to_mt_string())?;
        let _ = write!(msg, "-}}\r\n")?;

        if let Some(ref footer) = self.footer {
            let _ = write!(msg, "{{5:{}}}", footer)?;
        }

        Ok(msg)
    }
}

// TODO remove clone()
impl FinancialDataRead for Mt940 {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Mt940(e.to_string()))?;
        let blocks = split_to_blocks(&data);

        let basic_header = BasicHeaderBlock::from_string(
            &blocks[0]
                .clone()
                .ok_or(ParserError::Mt940("Missing Basic Header Block".to_string()))?,
        )?; // can't be skipped
        let application_header = blocks[1].clone().ok_or(ParserError::Mt940(
            "Missing Application Header Block".to_string(),
        ))?; // can't be skipped
        let user_header = blocks[2].clone(); // can be skipped
        let statement = messages::MT940::parse_from_block4(&blocks[3].clone().unwrap_or_default())
            .map_err(|e| ParserError::Mt940(e.to_string()))?; // can't be skipped
        let footer = blocks[4].clone(); // can be skipped

        Ok(Mt940 {
            basic_header: basic_header,
            application_header: application_header,
            user_header: user_header,
            statement: statement,
            footer: footer,
        })
    }
}

impl FinancialDataWrite for Mt940 {
    fn write_to<W: std::io::Write>(&self, writer: W) -> Result<(), ParserError> {
        let data = Mt940::to_string(self)?;
        Self::write_string(writer, &data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_header_constructor() {
        let header_str = String::from("F01GSCRUS30XXXX3614000002");
        let header_str_bigger = String::from("F01GSCRUS30XXXX36140000021");
        let header_str_less = String::from("F01GSCRUS30XXXX361400000");

        let target = BasicHeaderBlock {
            application_identifier: "F".to_string(),
            service_identifier: "01".to_string(),
            lt_identifier: "GSCRUS30XXXX".to_string(),
            session_number: "3614".to_string(),
            sequence_number: "000002".to_string(),
        };

        assert_eq!(
            target,
            BasicHeaderBlock::from_string(&header_str).expect("Failed to parse header")
        );
        assert!(BasicHeaderBlock::from_string(&header_str_bigger).is_err());
        assert!(BasicHeaderBlock::from_string(&header_str_less).is_err());
    }

    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_with_file() {
        let path = std::path::Path::new(r"test_data");
        let valid_case1 = File::open(path.join("valid1.mt940")).unwrap();
        let valid_case2 = File::open(path.join("valid2.mt940")).unwrap();
        let invalid_case1 = File::open(path.join("invalid1.mt940")).unwrap();
        let invalid_case2 = File::open(path.join("invalid2.mt940")).unwrap();

        let mt940_valid1 = Mt940::from_read(valid_case1);
        let mt940_valid2 = Mt940::from_read(valid_case2);
        let mt940_invalid1 = Mt940::from_read(invalid_case1);
        let mt940_invalid2 = Mt940::from_read(invalid_case2);

        assert!(mt940_valid1.is_ok());
        assert!(mt940_valid2.is_ok());
        assert!(mt940_invalid1.is_err());
        assert!(mt940_invalid2.is_err());
        assert_ne!(mt940_valid1.unwrap(), mt940_valid2.unwrap());
    }

    #[test]
    fn test_read_write() {
        // file paths: new file that will be created and valid mt940 file to compare
        let new_file_path = Path::new(r"test_data\test_write.mt940");
        let target_file_path = Path::new(r"test_data\valid1.mt940");
        // files
        let new_file = File::create(new_file_path).unwrap();
        let target_file = File::open(target_file_path).unwrap();
        // load valid mt940 file to struct (read tests suggest this operation is correct)
        // then serialize and write to new file
        let mt940_valid = Mt940::from_read(target_file).unwrap();
        let _ = mt940_valid.write_to(new_file).unwrap();
        // load new file and check that deserialization is correct
        let new_file = File::open(new_file_path).unwrap();
        let read_from_new_file = Mt940::from_read(new_file).unwrap();
        std::fs::remove_file(new_file_path).unwrap();
        assert_eq!(read_from_new_file, mt940_valid);
    }
}
