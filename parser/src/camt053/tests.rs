use crate::Camt053;
use crate::traits::FinancialDataRead;
use crate::traits::FinancialDataWrite;

use std::fs::File;

#[test]
fn test_parse() {
    let path = std::path::Path::new(r"test_data");
    let valid_file1 = File::open(path.join("valid1.camt053")).unwrap();
    let valid_file2 = File::open(path.join("valid2.camt053")).unwrap();
    let valid_file3 = File::open(path.join("valid3.camt053")).unwrap();

    assert!(Camt053::from_read(valid_file1).is_ok());
    assert!(Camt053::from_read(valid_file2).is_ok());
    assert!(Camt053::from_read(valid_file3).is_ok());

    let invalid_file1 = File::open(path.join("Invalid1.camt053")).unwrap();
    let invalid_file2 = File::open(path.join("Invalid2.camt053")).unwrap();
    let invalid_file3 = File::open(path.join("Invalid3.camt053")).unwrap();

    assert!(Camt053::from_read(invalid_file1).is_err());
    assert!(Camt053::from_read(invalid_file2).is_err());
    assert!(Camt053::from_read(invalid_file3).is_err());
}

#[test]
fn test_parse_camt053_fields() {
    let path = std::path::Path::new(r"test_data");
    let valid_file1 = File::open(path.join("valid1.camt053")).unwrap();

    let document = Camt053::from_read(valid_file1).expect("Failed to parse CAMT.053 XML");
    assert_eq!(
        document.bk_to_cstmr_stmt.grp_hdr.msg_id,
        Some("MSG123456789".to_string())
    );
    assert_eq!(
        document.bk_to_cstmr_stmt.grp_hdr.cre_dt_tm,
        Some("2023-10-05T14:30:00Z".to_string())
    );
    assert_eq!(document.bk_to_cstmr_stmt.stmts.len(), 1);

    let stmt = &document.bk_to_cstmr_stmt.stmts[0];
    assert_eq!(stmt.id, Some("STMT001".to_string()));
    assert_eq!(stmt.elctrnc_seq_nb, Some("1".to_string()));
    assert_eq!(stmt.cre_dt_tm, Some("2023-10-05T14:30:00Z".to_string()));

    assert!(stmt.fr_to_dt.is_some());
    let fr_to_dt = stmt.fr_to_dt.as_ref().unwrap();
    assert_eq!(fr_to_dt.fr_dt_tm, Some("2023-10-01T00:00:00Z".to_string()));
    assert_eq!(fr_to_dt.to_dt_tm, Some("2023-10-05T23:59:59Z".to_string()));

    assert!(stmt.acct.is_some());
    let acct = stmt.acct.as_ref().unwrap();
    assert_eq!(acct.ccy, Some("EUR".to_string()));
    assert_eq!(acct.name, Some("Test Account Name".to_string()));

    assert!(acct.id.is_some());
    let acct_id = acct.id.as_ref().unwrap();
    assert_eq!(acct_id.iban, Some("DE89370400440532013000".to_string()));

    assert_eq!(stmt.bal.len(), 2);

    let first_balance = &stmt.bal[0];
    assert!(first_balance.amt.is_some());
    let first_amt = first_balance.amt.as_ref().unwrap();
    assert_eq!(first_amt.currency, Some("EUR".to_string()));
    assert_eq!(first_amt.value, Some("1000.00".to_string()));
    assert_eq!(first_balance.cdt_dbt_ind, Some("CRDT".to_string()));

    let second_balance = &stmt.bal[1];
    assert!(second_balance.amt.is_some());
    let second_amt = second_balance.amt.as_ref().unwrap();
    assert_eq!(second_amt.value, Some("1500.50".to_string()));

    assert_eq!(stmt.ntry.len(), 2);

    let first_entry = &stmt.ntry[0];
    assert!(first_entry.amt.is_some());
    let first_entry_amt = first_entry.amt.as_ref().unwrap();
    assert_eq!(first_entry_amt.value, Some("100.00".to_string()));
    assert_eq!(first_entry.cdt_dbt_ind, Some("CRDT".to_string()));
    assert_eq!(first_entry.sts, Some("BOOK".to_string()));

    assert!(first_entry.bk_tx_cd.is_some());
    let first_bk_tx_cd = first_entry.bk_tx_cd.as_ref().unwrap();
    assert!(first_bk_tx_cd.domn.is_some());
    let domn = first_bk_tx_cd.domn.as_ref().unwrap();
    assert_eq!(domn.cd, Some("PMNT".to_string()));
    assert!(domn.fmly.is_some());
    let fmly = domn.fmly.as_ref().unwrap();
    assert_eq!(fmly.cd, Some("RCDT".to_string()));
    assert_eq!(fmly.sub_fmly_cd, Some("ESCT".to_string()));

    assert_eq!(first_entry.ntry_dtls.len(), 1);
    let first_ntry_details = &first_entry.ntry_dtls[0];
    assert_eq!(first_ntry_details.tx_dtls.len(), 1);

    let first_tx_details = &first_ntry_details.tx_dtls[0];
    assert!(first_tx_details.refs.is_some());
    let refs = first_tx_details.refs.as_ref().unwrap();
    assert_eq!(refs.end_to_end_id, Some("END2END123".to_string()));
    assert_eq!(refs.tx_id, Some("TXN123456".to_string()));

    assert!(first_tx_details.rltd_pties.is_some());
    let parties = first_tx_details.rltd_pties.as_ref().unwrap();
    assert!(parties.dbtr.is_some());
    assert_eq!(
        parties.dbtr.as_ref().unwrap().name,
        Some("John Debtor".to_string())
    );
    assert!(parties.cdtr.is_some());
    assert_eq!(
        parties.cdtr.as_ref().unwrap().name,
        Some("Jane Creditor".to_string())
    );

    assert!(first_tx_details.rmt_inf.is_some());
    let rmt_inf = first_tx_details.rmt_inf.as_ref().unwrap();
    assert_eq!(rmt_inf.ustrd, vec!["Invoice 12345", "Payment for services"]);

    let second_entry = &stmt.ntry[1];
    assert!(second_entry.amt.is_some());
    let second_entry_amt = second_entry.amt.as_ref().unwrap();
    assert_eq!(second_entry_amt.value, Some("50.00".to_string()));
    assert_eq!(second_entry.cdt_dbt_ind, Some("DBIT".to_string()));

    assert!(second_entry.bk_tx_cd.is_some());
    let second_bk_tx_cd = second_entry.bk_tx_cd.as_ref().unwrap();
    assert!(second_bk_tx_cd.prtry.is_some());
    let prtry = second_bk_tx_cd.prtry.as_ref().unwrap();
    assert_eq!(prtry.cd, Some("FEE".to_string()));
    assert_eq!(prtry.issr, Some("Bank".to_string()));
}

use std::path::Path;

#[test]
fn test_read_write() {
    // file paths: new file that will be created and valid camt053 file to compare
    let new_file_path = Path::new(r"test_data\test_write.camt053");
    let target_file_path = Path::new(r"test_data\valid1.camt053");
    // files
    let new_file = File::create(new_file_path).unwrap();
    let target_file = File::open(target_file_path).unwrap();
    // load valid camt053 file to struct (read tests suggest this operation is correct)
    // then serialize and write to new file
    let camt053_valid = Camt053::from_read(target_file).unwrap();
    let _ = camt053_valid.write_to(new_file).unwrap();
    // load new file and check that deserialization is correct
    let new_file = File::open(new_file_path).unwrap();
    let read_from_new_file = Camt053::from_read(new_file).unwrap();
    std::fs::remove_file(new_file_path).unwrap();

    // Can't compare two Camt053 structs directly:
    // Serde sometimes use None and sometimes empty string/struct
    // left: Some("")
    // right: None
    assert_eq!(
        read_from_new_file.bk_to_cstmr_stmt.grp_hdr.msg_id,
        camt053_valid.bk_to_cstmr_stmt.grp_hdr.msg_id
    );
    assert_eq!(
        read_from_new_file.bk_to_cstmr_stmt.grp_hdr.cre_dt_tm,
        camt053_valid.bk_to_cstmr_stmt.grp_hdr.cre_dt_tm
    );
    assert_eq!(
        read_from_new_file.bk_to_cstmr_stmt.stmts.len(),
        camt053_valid.bk_to_cstmr_stmt.stmts.len()
    );

    let stmt1 = &read_from_new_file.bk_to_cstmr_stmt.stmts[0];
    let stmt2 = &camt053_valid.bk_to_cstmr_stmt.stmts[0];
    assert_eq!(stmt1.id, stmt2.id,);
    assert_eq!(stmt1.elctrnc_seq_nb, stmt2.elctrnc_seq_nb);
    assert_eq!(stmt1.cre_dt_tm, stmt2.cre_dt_tm);

    let acct1 = stmt1.acct.as_ref().unwrap();
    let acct2 = stmt2.acct.as_ref().unwrap();
    assert_eq!(acct1.ccy, acct2.ccy);
    assert_eq!(acct1.name, acct2.name);

    let first_entry1 = &stmt1.ntry[0];
    let first_entry2 = &stmt2.ntry[0];
    let first_entry_amt1 = first_entry1.amt.as_ref().unwrap();
    let first_entry_amt2 = first_entry2.amt.as_ref().unwrap();
    assert_eq!(first_entry_amt1.value, first_entry_amt2.value);
    assert_eq!(first_entry1.cdt_dbt_ind, first_entry2.cdt_dbt_ind);
    assert_eq!(first_entry1.sts, first_entry2.sts);

    let acct_id1 = acct1.id.as_ref().unwrap();
    let acct_id2 = acct2.id.as_ref().unwrap();
    assert_eq!(acct_id1.iban, acct_id2.iban);

    let first_balance1 = &stmt1.bal[0];
    let first_balance2 = &stmt2.bal[0];
    let first_amt1 = first_balance1.amt.as_ref().unwrap();
    let first_amt2 = first_balance2.amt.as_ref().unwrap();
    assert_eq!(first_amt1.currency, first_amt2.currency);
    assert_eq!(first_amt1.value, first_amt2.value);
    assert_eq!(first_balance1.cdt_dbt_ind, first_balance2.cdt_dbt_ind);
}
