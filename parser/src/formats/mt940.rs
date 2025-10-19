use crate::errors::ParserError;
use crate::traits::{FinancialDataRead /*FinancialDataWrite*/};

use chrono::NaiveDate;
use chrono::{DateTime, Datelike, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Mt940 {
    pub basic_header: BasicHeaderBlock,
    pub application_header: String,  // depends on implementation
    pub user_header: Option<String>, // depends on implementation, may be skipped
    pub statement: Mt940Statement,
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Mt940Statement {
    pub reference: Option<String>,        // :20:
    pub account: Option<String>,          // :25:
    pub statement_number: Option<String>, // :28C:
    pub opening_balance: Option<Balance>, // :60F:
    pub transactions: Vec<Transaction>,
    pub closing_balance: Option<Balance>,    // :62F:
    pub other: HashMap<String, Vec<String>>, // reserved for other tags
}

impl Mt940Statement {
    pub fn from_string(input: &str) -> Result<Self, ParserError> {
        parse_mt940(input)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Balance {
    pub sign: char, // 'C' или 'D'
    pub date: NaiveDate,
    pub currency: Option<String>,
    pub amount: f64,
    pub raw: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub value_date: NaiveDate,
    pub entry_date: Option<NaiveDate>,
    pub dc_mark: String, // D, C, R/D etc
    pub amount: f64,
    pub funds_code: Option<String>,
    pub transaction_type_id: Option<String>,
    pub reference: Option<String>,
    pub supplementary: Option<String>,
    pub info_to_account_owner: Option<String>, // :86:
    pub raw: String,
}

fn parse_amount(s: &str) -> Result<f64, ParserError> {
    let normalized = s.trim().replace(',', ".").replace(' ', "");
    normalized
        .parse::<f64>()
        .map_err(|e| ParserError::InvalidInput(format!("amount parse {}: {}", s, e)))
}

fn parse_2digit_year(yy: u32) -> i32 {
    if yy <= 69 {
        (2000 + yy) as i32
    } else {
        (1900 + yy) as i32
    }
}

fn parse_date_yyMMdd(s: &str) -> Result<NaiveDate, ParserError> {
    if s.len() != 6 {
        return Err(ParserError::Mt940(format!("expected YYMMDD, got '{}'", s)));
    }
    let yy: u32 = s[0..2]
        .parse()
        .map_err(|_| ParserError::Mt940(s.to_string()))?;
    let mm: u32 = s[2..4]
        .parse()
        .map_err(|_| ParserError::Mt940(s.to_string()))?;
    let dd: u32 = s[4..6]
        .parse()
        .map_err(|_| ParserError::Mt940(s.to_string()))?;
    NaiveDate::from_ymd_opt(parse_2digit_year(yy), mm, dd)
        .ok_or_else(|| ParserError::Mt940(s.to_string()))
}

pub fn parse_mt940(input: &str) -> Result<Mt940Statement, ParserError> {
    let input = input.replace("\r\n", "\n").replace('\r', "\n");
    let mut tags: Vec<(String, String)> = Vec::new();

    let mut cur_tag: Option<String> = None;
    let mut cur_buf: Vec<String> = Vec::new();

    let tag_re = Regex::new(r"^:([^:]+):(.*)$").unwrap();

    for line in input.lines() {
        if let Some(caps) = tag_re.captures(line) {
            if let Some(t) = cur_tag.take() {
                tags.push((t, cur_buf.join("\n")));
                cur_buf.clear();
            }
            cur_tag = Some(caps[1].to_string());
            cur_buf.push(caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string());
        } else {
            if !cur_buf.is_empty() {
                cur_buf.push(line.to_string());
            }
        }
    }
    if let Some(t) = cur_tag.take() {
        tags.push((t, cur_buf.join("\n")));
    }

    let mut stmt = Mt940Statement {
        reference: None,
        account: None,
        statement_number: None,
        opening_balance: None,
        transactions: Vec::new(),
        closing_balance: None,
        other: HashMap::new(),
    };

    let mut push_other = |k: &str, v: String| {
        stmt.other
            .entry(k.to_string())
            .or_insert_with(Vec::new)
            .push(v);
    };

    let mut last_tx_index: Option<usize> = None;

    let re_61 = Regex::new(r"^(?P<valdate>\d{6})(?P<entry>\d{4})?(?P<dc>[DC])(?P<fundscode>[A-Z])?(?P<amount>[0-9,]+)(?P<rest>.*)$").unwrap();

    for (tag, content) in tags {
        match tag.as_str() {
            "20" => stmt.reference = Some(content),
            "25" => stmt.account = Some(content),
            "28" | "28C" => stmt.statement_number = Some(content),
            "60F" | "60M" => {
                let raw = content.clone();
                let re =
                    Regex::new(r"^(?P<sign>[CD])(?P<date>\d{6})(?P<cur>[A-Z]{3})?(?P<amt>[0-9,]+)")
                        .unwrap();
                if let Some(c) = re.captures(&content) {
                    let sign = c.name("sign").unwrap().as_str().chars().next().unwrap();
                    let date = parse_date_yyMMdd(c.name("date").unwrap().as_str())?;
                    let currency = c.name("cur").map(|m| m.as_str().to_string());
                    let amount = parse_amount(c.name("amt").unwrap().as_str())?;
                    stmt.opening_balance = Some(Balance {
                        sign,
                        date,
                        currency,
                        amount,
                        raw,
                    })
                } else {
                    return Err(ParserError::InvalidInput(format!(
                        "bad 60 tag: {}",
                        content
                    )));
                }
            }
            "61" => {
                let raw = content.clone();
                if let Some(c) = re_61.captures(&content.replace("\n", "")) {
                    let valdate = parse_date_yyMMdd(c.name("valdate").unwrap().as_str())?;
                    let entry = c.name("entry").map(|m| m.as_str());
                    let entry_date = if let Some(e) = entry {
                        if e.len() == 4 {
                            let month: u32 = e[0..2].parse().unwrap_or(0);
                            let day: u32 = e[2..4].parse().unwrap_or(0);
                            NaiveDate::from_ymd_opt(valdate.year(), month, day)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let dc_mark = c.name("dc").unwrap().as_str().to_string();
                    let funds_code = c.name("fundscode").map(|m| m.as_str().to_string());
                    let amount = parse_amount(c.name("amount").unwrap().as_str())?;
                    let rest = c.name("rest").map(|m| m.as_str().trim().to_string());

                    let mut transaction_type_id: Option<String> = None;
                    let mut reference: Option<String> = None;
                    let mut supplementary: Option<String> = None;

                    if let Some(r) = rest {
                        if r.contains("//") {
                            let parts: Vec<&str> = r.splitn(2, "//").collect();
                            if parts[0].len() >= 3 {
                                transaction_type_id = Some(parts[0].to_string());
                            }
                            reference =
                                Some(parts.get(1).map(|s| s.to_string()).unwrap_or_default());
                        } else {
                            let trimmed = r.trim();
                            if trimmed.len() >= 3 {
                                transaction_type_id = Some(trimmed[0..3].to_string());
                                supplementary = if trimmed.len() > 3 {
                                    Some(trimmed[3..].trim().to_string())
                                } else {
                                    None
                                };
                            } else if !trimmed.is_empty() {
                                supplementary = Some(trimmed.to_string());
                            }
                        }
                    }

                    let tx = Transaction {
                        value_date: valdate,
                        entry_date: entry_date,
                        dc_mark,
                        amount,
                        funds_code,
                        transaction_type_id,
                        reference,
                        supplementary,
                        info_to_account_owner: None,
                        raw,
                    };
                    stmt.transactions.push(tx);
                    last_tx_index = Some(stmt.transactions.len() - 1);
                } else {
                    return Err(ParserError::InvalidInput(format!(
                        "bad 61 tag: {}",
                        content
                    )));
                }
            }
            "86" | "86:" => {
                let text = content.trim().to_string();
                if let Some(ix) = last_tx_index {
                    if let Some(tx) = stmt.transactions.get_mut(ix) {
                        tx.info_to_account_owner = Some(match &tx.info_to_account_owner {
                            Some(existing) => format!("{}\n{}", existing, text),
                            None => text,
                        });
                    }
                } else {
                    push_other(&tag, content);
                }
            }
            "62F" | "62M" => {
                let raw = content.clone();
                let re =
                    Regex::new(r"^(?P<sign>[CD])(?P<date>\d{6})(?P<cur>[A-Z]{3})?(?P<amt>[0-9,]+)")
                        .unwrap();
                if let Some(c) = re.captures(&content) {
                    let sign = c.name("sign").unwrap().as_str().chars().next().unwrap();
                    let date = parse_date_yyMMdd(c.name("date").unwrap().as_str())?;
                    let currency = c.name("cur").map(|m| m.as_str().to_string());
                    let amount = parse_amount(c.name("amt").unwrap().as_str())?;
                    stmt.closing_balance = Some(Balance {
                        sign,
                        date,
                        currency,
                        amount,
                        raw,
                    })
                } else {
                    return Err(ParserError::InvalidInput(format!(
                        "bad 62 tag: {}",
                        content
                    )));
                }
            }
            other_tag => push_other(other_tag, content),
        }
    }

    Ok(stmt)
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
        let statement = Mt940Statement::from_string(
            &blocks[3]
                .clone()
                .ok_or(ParserError::Mt940("Missing Statement Block".to_string()))?,
        )?; // can't be skipped
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

// TODO
// impl FinancialDataWrite for Mt940Statement {
//     fn write_to<W: std::io::Write>(&self, writer: W) -> Result<()> {
//         let data = Mt940Parser::serialize(self)?;
//         let mut buffered = std::io::BufWriter::new(writer);
//         buffered
//             .write_all(data.as_bytes())
//             .map_err(crate::error::ParserError::Io)?;
//         buffered.flush().map_err(crate::error::ParserError::Io)?;
//         Ok(())
//     }
// }

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

    #[test]
    fn test_parse_sample() {
        let sample = ":20:REFERENCE123\n:25:NL12BANK0123456789\n:28C:00001/001\n:60F:C250930EUR12345,67\n:61:2509300930D123,45NTRFNONREF//ABC123\n:86:Payment for invoice 2025-09\n:61:2509290929C200,00NMSCREF123//987654\n:86:Salary for September\n:62F:C250930EUR12422,22\n";

        let stmt = Mt940Statement::from_string(sample).expect("Failed to parse statement");
        assert_eq!(stmt.reference.clone().unwrap(), "REFERENCE123");
        assert_eq!(stmt.account.clone().unwrap(), "NL12BANK0123456789");
        assert_eq!(stmt.transactions.len(), 2);
        assert!(stmt.opening_balance.is_some());
        assert!(stmt.closing_balance.is_some());

        println!("{}", serde_json::to_string_pretty(&stmt).unwrap());
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
    fn test_with_file_fields() {
        let path = std::path::Path::new(r"test_data");
        let valid_case1 = File::open(path.join("valid_simplified_fields.mt940")).unwrap();
        let mt940_valid1 = Mt940::from_read(valid_case1).unwrap();

        let target = Mt940 {
            basic_header: BasicHeaderBlock {
                application_identifier: "F".to_string(),
                service_identifier: "01".to_string(),
                lt_identifier: "GSCRUS30XXXX".to_string(),
                session_number: "3614".to_string(),
                sequence_number: "000002".to_string(),
            },
            application_header: "I940GSCRUS30XXXXN".to_string(),
            user_header: None,
            statement: Mt940Statement {
                reference: Some("15486025400".to_string()),
                account: Some("107048825".to_string()),
                statement_number: Some("49/2".to_string()),
                opening_balance: Some(Balance {
                    sign: 'C',
                    date: NaiveDate::from_ymd_opt(2025, 02, 18)
                        .expect("Failed to created data in test case"),
                    currency: Some("USD".to_string()),
                    amount: 2732398848.02,
                    raw: "C250218USD2732398848,02".to_string(), // 'C' - 2025-02-18 - USD - 2732398848.02
                }),
                transactions: vec!(Transaction {
                    value_date: NaiveDate::from_ymd_opt(2025, 02, 18)
                        .expect("Failed to created data in test case"),
                    entry_date: Some(NaiveDate::from_ymd_opt(2025, 02, 18)
                        .expect("Failed to created data in test case")),
                    dc_mark: "D".to_string(),
                    amount: 12.01,
                    funds_code: None,
                    transaction_type_id: Some("NTRFGSLNVSHSUTKWDR".to_string()),
                    reference: Some("GI2504900007841".to_string()),
                    supplementary: None,
                    info_to_account_owner: Some("/EREF/GSLNVSHSUTKWDR\n/CRNM/GOLDMAN SACHS BANK USA\n/CACT/107045863/CBIC/GSCRUS30XXX\n/REMI/USD Payment to Vendor\n/OPRP/Tag Payment".to_string()),
                    raw: "2502180218D12,01NTRFGSLNVSHSUTKWDR//GI2504900007841".to_string()}),
                closing_balance: Some(Balance {
                    sign: 'C',
                    date: NaiveDate::from_ymd_opt(2025, 02, 18)
                        .expect("Failed to created data in test case"),
                    currency: Some("USD".to_string()),
                    amount: 2937898.77,
                    raw: "C250218USD2937898,77\n-".to_string(), // 'C' - 2025-02-18 - USD - 2937898,77
                }),
                other: HashMap::new(),
            },
            footer: None,
        };

        assert_eq!(target, mt940_valid1);
    }
}
