use crate::FinancialDataRead;
use crate::ParserError;
use crate::mt940::format::{BasicHeaderBlock, Mt940};
use regex::Regex;
use std::fmt::Write;
use swift_mt_message::messages;

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
fn split_to_blocks(data: &str) -> Result<Vec<Option<String>>, ParserError> {
    let mut result = vec![None; 5];

    let re = Regex::new(r"\{(\d):([^}]*)\}").map_err(|e| ParserError::Mt940(e.to_string()))?;

    for caps in re.captures_iter(data) {
        if let (Some(num), Some(data)) = (caps.get(1), caps.get(2)) {
            if let Ok(idx) = num.as_str().parse::<usize>() {
                if (1..=5).contains(&idx) {
                    result[idx - 1] = Some(data.as_str().to_string());
                }
            }
        }
    }

    Ok(result)
}

impl Mt940 {
    pub fn to_string(&self) -> Result<String, ParserError> {
        let mut msg = String::new();

        write!(
            msg,
            "{{1:{}}}{{2:{}}}\r\n",
            self.basic_header.to_string(),
            self.application_header
        )?;

        if let Some(ref uh) = self.user_header {
            write!(msg, "{{3:{}}}\r\n", uh)?;
        }

        write!(msg, "{{4:{}\r\n", self.statement.to_mt_string())?;
        write!(msg, "-}}\r\n")?;

        if let Some(ref footer) = self.footer {
            write!(msg, "{{5:{}}}", footer)?;
        }

        Ok(msg)
    }
}

// TODO remove clone()
impl FinancialDataRead for Mt940 {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Mt940(e.to_string()))?;
        let blocks = split_to_blocks(&data)?;

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
