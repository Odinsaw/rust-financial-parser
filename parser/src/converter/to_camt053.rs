use crate::ParserError;
use crate::camt053::format::*;
use crate::mt940::format::*;
use chrono::NaiveDate;
use swift_mt_message::MT940StatementLine;
use swift_mt_message::SwiftField;

// --- Helper: parse balance ---
// It's easier to convert balance to swift string and parse it
fn parse_balance(field_swift: &str, tag: &str) -> Result<Balance, ParserError> {
    // Remove tags :60F:, :62F:, etc.
    let body = field_swift.get(5..).ok_or_else(|| {
        ParserError::Converter("Failed to parse balance string: field too short".to_string())
    })?;

    // Validate minimum length for required fields
    if body.len() < 10 {
        // 1 (C/D) + 6 (date) + 3 (currency) = minimum 10 chars
        return Err(ParserError::Converter(
            "Failed to parse balance string: insufficient data".to_string(),
        ));
    }

    let mut chars = body.chars();
    // Credit/Debit indicator
    let cdt_dbt_ind = match chars.next() {
        Some('C') => "CRDT",
        Some('D') => "DBIT",
        Some(other) => {
            return Err(ParserError::Converter(format!(
                "Invalid debit/credit indicator: '{}'",
                other
            )));
        }
        None => {
            return Err(ParserError::Converter(
                "Missing debit/credit indicator".to_string(),
            ));
        }
    };

    // Date: YYMMDD → YYYY-MM-DD
    let date_str: String = chars.by_ref().take(6).collect();
    if date_str.len() != 6 {
        return Err(ParserError::Converter(
            "Invalid date format: expected 6 characters".to_string(),
        ));
    }

    let date_iso = NaiveDate::parse_from_str(&date_str, "%y%m%d")
        .map_err(|e| ParserError::Converter(format!("Failed to parse date '{}': {}", date_str, e)))?
        .format("%Y-%m-%d")
        .to_string();

    // Currency (3 characters)
    let currency: String = chars.by_ref().take(3).collect();
    if currency.len() != 3 {
        return Err(ParserError::Converter(format!(
            "Invalid currency code: '{}'",
            currency
        )));
    }

    // Amount - process remaining characters
    let amount_str: String = chars.collect();
    if amount_str.is_empty() {
        return Err(ParserError::Converter("Missing amount".to_string()));
    }

    let amount_clean = amount_str.replace(",", ".");
    // Optional: validate that amount is a valid number
    amount_clean.parse::<f64>().map_err(|e| {
        ParserError::Converter(format!("Invalid amount format '{}': {}", amount_clean, e))
    })?;

    Ok(Balance {
        tp: Some(BalanceType {
            cd_or_prtry: Some(CodeOrProprietary {
                cd: Some(tag.into()),
                prtry: None,
            }),
        }),
        amt: Some(Amount {
            currency: Some(currency.to_string()),
            value: Some(amount_str.to_string()),
        }),
        cdt_dbt_ind: Some(cdt_dbt_ind.to_string()),
        dt: Some(DateAndDateTimeChoice {
            dt: Some(date_iso),
            dt_tm: None,
        }),
    })
}

// --- Helper: parse entry line ---
fn parse_entry(line: &MT940StatementLine) -> Result<Entry, ParserError> {
    let field61 = &line.field_61;

    let cdt_dbt = match field61.debit_credit_mark.as_str() {
        "C" => "CRDT",
        "D" => "DBIT",
        _ => "CRDT",
    };

    let amt_value = format!("{:.2}", field61.amount);
    let currency = "EUR".to_string();

    let rmt_info = line
        .field_86
        .as_ref()
        .map(|field_86| RemittanceInformation {
            ustrd: field_86.narrative.clone(),
            ..Default::default()
        });

    let tx_details = TransactionDetails {
        amt: Some(Amount {
            currency: Some(currency.clone()),
            value: Some(amt_value.clone()),
        }),
        rmt_inf: rmt_info,
        ..Default::default()
    };

    Ok(Entry {
        amt: Some(Amount {
            currency: Some(currency),
            value: Some(amt_value),
        }),
        cdt_dbt_ind: Some(cdt_dbt.into()),
        ntry_dtls: vec![EntryDetails {
            tx_dtls: vec![tx_details],
            ..Default::default()
        }],
        ..Default::default()
    })
}

impl From<&Mt940> for Result<Camt053, ParserError> {
    fn from(mt940: &Mt940) -> Self {
        let stmt = &mt940.statement;

        // --- GroupHeader ---
        let msg_id = stmt.field_20.reference.clone();
        let grp_hdr = GroupHeader {
            msg_id: Some(msg_id),
            ..Default::default()
        };

        // --- Account ---
        let acct_iban = stmt.field_25.authorisation.clone();
        let acct = Account {
            id: Some(AccountId {
                iban: Some(acct_iban),
                other: None,
            }),
            ccy: Some("EUR".into()),
            ..Default::default()
        };

        // --- Balances ---
        let mut balances = vec![];
        balances.push(parse_balance(&stmt.field_60f.to_swift_string(), "OPBD")?);
        balances.push(parse_balance(&stmt.field_62f.to_swift_string(), "CLBD")?);
        if let Some(f64bal) = &stmt.field_64 {
            balances.push(parse_balance(&f64bal.to_swift_string(), "CLAV")?);
        }
        if let Some(f65bal_list) = &stmt.field_65 {
            for f65bal in f65bal_list {
                balances.push(parse_balance(&f65bal.to_swift_string(), "FWAV")?);
            }
        }

        // --- Entries ---
        let entries: Vec<Entry> = stmt
            .statement_lines
            .iter()
            .map(|line| parse_entry(line))
            .collect::<Result<_, _>>()?;

        // --- Statement ---
        let statement = Statement {
            id: stmt.field_21.as_ref().map(|f| f.reference.clone()),
            acct: Some(acct),
            elctrnc_seq_nb: Some(
                stmt.field_28c
                    .to_swift_string()
                    .trim_start_matches(":28C:")
                    .to_string(),
            ),
            bal: balances,
            ntry: entries,
            ..Default::default()
        };
        // --- Camt053 ---
        Ok(Camt053 {
            bk_to_cstmr_stmt: BankToCustomerStatement {
                grp_hdr,
                stmts: vec![statement],
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::FinancialDataRead;
    use std::env;
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn test_convert_mt940_to_camt053() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = PathBuf::from(manifest_dir).join("test_data");
        let target_file_path = path.join("simple.mt940");
        let target_file = File::open(target_file_path).unwrap();
        let mt940_valid = Mt940::from_read(target_file).unwrap();

        let result: Result<Camt053, ParserError> = (&mt940_valid).into();
        let result = result.unwrap();

        // GroupHeader
        assert_eq!(
            result.bk_to_cstmr_stmt.grp_hdr.msg_id,
            Some("STAT202510210001".to_string()) // :20:STAT202510210001
        );

        // Statement
        let stmt = &result.bk_to_cstmr_stmt.stmts[0];
        assert_eq!(stmt.elctrnc_seq_nb, Some("1/1".to_string())); // :28C:00001/001

        // Account
        let acct = stmt.acct.as_ref().unwrap();
        let acct_id = acct.id.as_ref().unwrap();
        assert_eq!(acct_id.iban, Some("NL91ABNA0417164300".to_string())); // :25:NL91ABNA0417164300
        assert_eq!(acct.ccy, Some("EUR".to_string()));

        // Balances
        let bal1 = &stmt.bal[0]; // :60F:C251020EUR10000,00
        assert_eq!(
            bal1.tp.as_ref().unwrap().cd_or_prtry.as_ref().unwrap().cd,
            Some("OPBD".to_string())
        );
        assert_eq!(bal1.amt.as_ref().unwrap().currency, Some("EUR".to_string()));
        assert_eq!(
            bal1.amt.as_ref().unwrap().value,
            Some("10000,00".to_string())
        );
        assert_eq!(bal1.cdt_dbt_ind, Some("CRDT".to_string()));
        assert_eq!(bal1.dt.as_ref().unwrap().dt, Some("2025-10-20".to_string()));

        let bal2 = &stmt.bal[1]; // :62F:C251021EUR11239,50
        assert_eq!(
            bal2.tp.as_ref().unwrap().cd_or_prtry.as_ref().unwrap().cd,
            Some("CLBD".to_string())
        );
        assert_eq!(bal2.amt.as_ref().unwrap().currency, Some("EUR".to_string()));
        assert_eq!(
            bal2.amt.as_ref().unwrap().value,
            Some("11239,50".to_string())
        );
        assert_eq!(bal2.cdt_dbt_ind, Some("CRDT".to_string()));
        assert_eq!(bal2.dt.as_ref().unwrap().dt, Some("2025-10-21".to_string()));

        // Entry
        let entry = &stmt.ntry[0]; // :61:2510211021D250,00NTRFNONREF//BKNTRX0001
        assert_eq!(
            entry.amt.as_ref().unwrap().currency,
            Some("EUR".to_string())
        );
        assert_eq!(
            entry.amt.as_ref().unwrap().value,
            Some("250.00".to_string())
        );
        assert_eq!(entry.cdt_dbt_ind, Some("DBIT".to_string()));

        // TransactionDetails
        let tx_details = &entry.ntry_dtls[0].tx_dtls[0];
        assert_eq!(
            tx_details.amt.as_ref().unwrap().currency,
            Some("EUR".to_string())
        );
        assert_eq!(
            tx_details.amt.as_ref().unwrap().value,
            Some("250.00".to_string())
        );
        assert_eq!(
            tx_details.rmt_inf.as_ref().unwrap().ustrd[0],
            "Payment to supplier Roga i Kopyta Inv 1001".to_string() // :86:Payment to supplier Roga i Kopyta Inv 1001
        );
    }
}
