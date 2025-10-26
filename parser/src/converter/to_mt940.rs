use crate::ParserError;
use crate::camt053::format::*;
use crate::mt940::format::*;
use chrono::Datelike;
use swift_mt_message::MT940StatementLine;
use swift_mt_message::SwiftField;

/*
:20: (Field20) — GrpHdr.MsgId или Stmt.Id.
:21:(Field21) — Stmt.Id или Stmt.ElctrncSeqNb
:25: (Field25) — IBAN из Statement.Acct.Id.IBAN либо Othr.Id.
:28C: — Statement.Id / Statement.ElctrncSeqNb (если есть).
:60F: (Opening balance) — take Balance with OPBD/OP/OPP in Balance.Tp.CdOrPrtry.Cd or first bal entry.
:62F: (Closing balance) — take Balance with CLBD/CL or last bal entry.
Lines :61: (statement lines) — Entry.
field_64 — Bal with CLAV (Closing Available Balance).
field_65 — Bal with FWAV (Forward Available Balance).
*/

fn find_balance<'a>(stmt: &'a Statement, tag: &'a str) -> Result<&'a Balance, ParserError> {
    Ok(stmt
        .bal
        .iter()
        .find(|b| {
            b.tp.as_ref()
                .and_then(|tp| tp.cd_or_prtry.as_ref())
                .and_then(|cd_prtry| cd_prtry.cd.as_deref())
                == Some(tag)
        })
        .ok_or(ParserError::Converter(format!(
            "Failed to parse Balance: {}",
            tag
        )))?)
}

fn format_mt940_balance_line(bal: &Balance) -> String {
    let cdt_dbt = match bal.cdt_dbt_ind.as_deref().unwrap_or("C") {
        "C" | "CRDT" => "C",
        "D" | "DBIT" => "D",
        _other => "",
    };

    let date_str = bal
        .dt
        .as_ref()
        .and_then(|d| d.dt.clone().or_else(|| d.dt_tm.clone()))
        .unwrap_or("0000-00-00".into());

    let date_formatted = if let Ok(date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
    {
        format!(
            "{:02}{:02}{:02}",
            date.year() % 100,
            date.month(),
            date.day()
        )
    } else {
        "000000".into()
    };

    let currency = bal
        .amt
        .as_ref()
        .and_then(|a| a.currency.clone())
        .unwrap_or("EUR".into());
    let amount = bal
        .amt
        .as_ref()
        .and_then(|a| a.value.clone())
        .unwrap_or("0.0".into());

    format!("{}{}{}{}", cdt_dbt, date_formatted, currency, amount)
}

impl From<&Camt053> for Result<Vec<Mt940>, ParserError> {
    fn from(camt: &Camt053) -> Self {
        let mut result = vec![];

        let msg_id = camt
            .bk_to_cstmr_stmt
            .grp_hdr
            .msg_id
            .clone()
            .unwrap_or_default();

        for stmt in &camt.bk_to_cstmr_stmt.stmts {
            let field_20 = swift_mt_message::fields::Field20::parse(&msg_id)
                .map_err(|e| ParserError::Converter(e.to_string()))?;

            let field_21 = stmt.id.clone().unwrap_or_default();
            let field_21 = Some(
                swift_mt_message::fields::Field21NoOption::parse(&field_21)
                    .map_err(|e| ParserError::Converter(e.to_string()))?,
            );

            let field_25 = stmt
                .acct
                .as_ref()
                .and_then(|a| a.id.as_ref())
                .and_then(|id| id.iban.clone().or(id.other.as_ref()?.id.clone()))
                .unwrap_or_default();
            let field_25 = swift_mt_message::fields::Field25NoOption::parse(&field_25)
                .map_err(|e| ParserError::Converter(e.to_string()))?;

            let field_28c = {
                let seq = stmt
                    .elctrnc_seq_nb
                    .clone()
                    .or(stmt.lgl_seq_nb.clone())
                    .unwrap_or_default();

                let digits: String = seq.chars().filter(|c| c.is_ascii_digit()).collect();
                let statement_number = if digits.len() > 5 {
                    digits[digits.len() - 5..].to_string()
                } else {
                    digits
                };

                let line = format!("{}/1", statement_number);
                swift_mt_message::fields::Field28C::parse(&line)
                    .map_err(|e| ParserError::Converter(e.to_string()))?
            };

            let line_60f = format_mt940_balance_line(find_balance(stmt, "OPBD")?);
            let field_60f = swift_mt_message::fields::Field60F::parse(&line_60f)
                .map_err(|e| ParserError::Converter(e.to_string()))?;

            let line_62f = format_mt940_balance_line(find_balance(stmt, "CLBD")?);
            let field_62f = swift_mt_message::fields::Field62F::parse(&line_62f)
                .map_err(|e| ParserError::Converter(e.to_string()))?;

            let statement_lines: Vec<MT940StatementLine> = stmt
                .ntry
                .iter()
                .filter_map(|entry| {
                    let amt = entry.amt.as_ref()?.value.as_ref()?;
                    let line_61_str = format!("{}", amt);

                    let field_61 = swift_mt_message::fields::Field61::parse(&line_61_str).ok()?;

                    let field_86 =
                        Some(swift_mt_message::fields::Field86::parse("TRANSACTION DETAIL").ok()?);

                    Some(MT940StatementLine { field_61, field_86 })
                })
                .collect();

            let line_64 = format_mt940_balance_line(find_balance(stmt, "CLAV")?);
            let field_64 = Some(
                swift_mt_message::fields::Field64::parse(&line_64)
                    .map_err(|e| ParserError::Converter(e.to_string()))?,
            );

            let field_65: Option<Vec<swift_mt_message::fields::Field65>> = {
                let list: Vec<_> = stmt
                    .bal
                    .iter()
                    .filter(|b| {
                        b.tp.as_ref()
                            .and_then(|tp| tp.cd_or_prtry.as_ref()?.cd.as_ref())
                            .map(|cd| cd == "FWAV")
                            .unwrap_or(false)
                    })
                    .filter_map(|bal| {
                        let line = format_mt940_balance_line(&bal);
                        swift_mt_message::fields::Field65::parse(&line).ok()
                    })
                    .collect();

                if list.is_empty() { None } else { Some(list) }
            };

            let statement = swift_mt_message::MT940 {
                field_20,
                field_21,
                field_25,
                field_28c,
                field_60f,
                statement_lines,
                field_62f,
                field_64,
                field_65,
            };

            result.push(Mt940 {
                basic_header: BasicHeaderBlock::default(),
                application_header: String::default(),
                user_header: None,
                statement: statement,
                footer: None,
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::FinancialDataRead;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_convert_camt053_to_mt940() {
        let target_file_path = Path::new(r"test_data\valid1.camt053");
        let target_file = File::open(target_file_path).unwrap();
        let camt053_valid = Camt053::from_read(target_file).unwrap();

        let expected_string = "{1:}{2:}\r\n{4::20:MSG123456789\r\n:21:STMT001\r\n:25:/\r\n:28C:1/1\r\n:60F:C231005EUR1000,00\r\n:62F:C231005EUR1500,50\r\n:64:C251026EUR1150,00\r\n-}\r\n".to_string();
        let result: Result<Vec<Mt940>, ParserError> = (&camt053_valid).into();
        let result = result.unwrap();
        let mt940_str = result[0].to_string();
        assert_eq!(mt940_str.unwrap(), expected_string);
    }
}
