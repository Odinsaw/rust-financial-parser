use crate::errors::ParserError;
use crate::traits::{FinancialDataRead, FinancialDataWrite};

use quick_xml::de::from_str;
use serde::Deserialize;

// Root document
#[derive(Debug, Deserialize)]
#[serde(rename = "Camt053")]
pub struct Camt053 {
    #[serde(rename = "BkToCstmrStmt")]
    pub bk_to_cstmr_stmt: Option<BankToCustomerStatement>,
}

#[derive(Debug, Deserialize)]
pub struct BankToCustomerStatement {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: Option<GroupHeader>,
    #[serde(rename = "Stmt", default)]
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Deserialize)]
pub struct GroupHeader {
    #[serde(rename = "MsgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Statement {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "ElctrncSeqNb")]
    pub elctrnc_seq_nb: Option<String>,
    #[serde(rename = "LglSeqNb")]
    pub lgl_seq_nb: Option<String>,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: Option<String>,
    #[serde(rename = "FrToDt")]
    pub fr_to_dt: Option<FromToDate>,
    #[serde(rename = "Acct")]
    pub acct: Option<Account>,
    #[serde(rename = "Bal", default)]
    pub bal: Vec<Balance>,
    #[serde(rename = "Ntry", default)]
    pub ntry: Vec<Entry>,
    #[serde(rename = "TxsSummry")]
    pub txs_summry: Option<TransactionSummary>,
}

#[derive(Debug, Deserialize)]
pub struct FromToDate {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: Option<String>,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "Id")]
    pub id: Option<AccountId>,
    #[serde(rename = "Ccy")]
    pub ccy: Option<String>,
    #[serde(rename = "Nm")]
    pub name: Option<String>,
    #[serde(rename = "Ownr")]
    pub owner: Option<Owner>,
    #[serde(rename = "Svcr")]
    pub servicer: Option<Servicer>,
}

#[derive(Debug, Deserialize)]
pub struct AccountId {
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    #[serde(rename = "Nm")]
    pub name: Option<String>,
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress>,
    #[serde(rename = "Id")]
    pub id: Option<OrgId>,
}

#[derive(Debug, Deserialize)]
pub struct OrgId {
    #[serde(rename = "Othr")]
    pub other: Option<OtherId>,
}

#[derive(Debug, Deserialize)]
pub struct OtherId {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "SchmeNm")]
    pub schme_nm: Option<SchemeName>,
}

#[derive(Debug, Deserialize)]
pub struct SchemeName {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostalAddress {
    #[serde(rename = "StrtNm")]
    pub street: Option<String>,
    #[serde(rename = "BldgNb")]
    pub building_number: Option<String>,
    #[serde(rename = "PstCd")]
    pub postal_code: Option<String>,
    #[serde(rename = "TwnNm")]
    pub town_name: Option<String>,
    #[serde(rename = "Ctry")]
    pub country: Option<String>,
    #[serde(rename = "AdrLine", default)]
    pub adr_lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Servicer {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: Option<FinancialInstitution>,
}

#[derive(Debug, Deserialize)]
pub struct FinancialInstitution {
    #[serde(rename = "BIC")]
    pub bic: Option<String>,
    #[serde(rename = "Nm")]
    pub name: Option<String>,
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress>,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    #[serde(rename = "Tp")]
    pub tp: BalanceType,
    #[serde(rename = "Amt")]
    pub amt_balance: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<String>,
    #[serde(rename = "Dt")]
    pub dt: Option<DateWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: CodeOrProprietary,
}

#[derive(Debug, Deserialize)]
pub struct CodeOrProprietary {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Amount {
    #[serde(rename = "$value")]
    pub value: f64,
    #[serde(rename = "@Ccy")]
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DateWrapper {
    #[serde(rename = "Dt")]
    pub dt: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    #[serde(rename = "NtryRef")]
    pub ntry_ref: Option<String>,
    #[serde(rename = "Amt")]
    pub amt_entry: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<String>,
    #[serde(rename = "Sts")]
    pub sts: Option<String>,
    #[serde(rename = "BookgDt")]
    pub bookg_dt: Option<DateWrapper>,
    #[serde(rename = "ValDt")]
    pub val_dt: Option<DateWrapper>,
    #[serde(rename = "AcctSvcrRef")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: Option<BkTxCd>,
    #[serde(rename = "AddtlInfInd")]
    pub addtl_inf_ind: Option<AddtlInfo>,
    #[serde(rename = "NtryDtls", default)]
    pub ntry_dtls: Vec<EntryDetails>,
}

#[derive(Debug, Deserialize)]
pub struct AddtlInfo {
    #[serde(rename = "MsgNmId")]
    pub msg_nm_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BkTxCd {
    #[serde(rename = "Domn")]
    pub domn: Option<Domain>,
    #[serde(rename = "Prtry")]
    pub prtry: Option<Proprietary>,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "Fmly")]
    pub fmly: Option<Fmly>,
}

#[derive(Debug, Deserialize)]
pub struct Fmly {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Proprietary {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "Issr")]
    pub issr: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EntryDetails {
    #[serde(rename = "TxDtls", default)]
    pub tx_dtls: Vec<TransactionDetails>,
    #[serde(rename = "Btch")]
    pub btch: Option<Batch>,
}

#[derive(Debug, Deserialize)]
pub struct Batch {
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetails {
    #[serde(rename = "Refs")]
    pub refs: Option<TransactionRefs>,
    #[serde(rename = "AmtDtls")]
    pub amt_dtls: Option<AmountDetails>,
    #[serde(rename = "RltdPties")]
    pub rltd_pties: Option<RelatedParties>,
    #[serde(rename = "RltdAgts")]
    pub rltd_agts: Option<RelatedAgents>,
    #[serde(rename = "RmtInf")]
    pub rmt_inf: Option<RemittanceInfo>,
    #[serde(rename = "RltdDts")]
    pub rltd_dts: Option<RelatedDates>,
    #[serde(rename = "AddtlTxInf")]
    pub addtl_tx_inf: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionRefs {
    #[serde(rename = "MsgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "TxId")]
    pub tx_id: Option<String>,
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id: Option<String>,
    #[serde(rename = "InstrId")]
    pub instr_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionSummary {
    #[serde(rename = "TtlNtries")]
    pub ttl_ntries: Option<TotalEntries>,
    #[serde(rename = "TtlCdtNtries")]
    pub ttl_cdt_ntries: Option<TotalCreditEntries>,
    #[serde(rename = "TtlDbtNtries")]
    pub ttl_dbt_ntries: Option<TotalDebitEntries>,
}

#[derive(Debug, Deserialize)]
pub struct TotalEntries {
    #[serde(rename = "NbOfNtries")]
    pub nb_of_ntries: Option<u32>,
    #[serde(rename = "TtlNetNtryAmt")]
    pub ttl_net_ntry_amt: Option<f64>,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TotalCreditEntries {
    #[serde(rename = "NbOfNtries")]
    pub nb_of_ntries: Option<u32>,
    #[serde(rename = "Sum")]
    pub sum: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TotalDebitEntries {
    #[serde(rename = "NbOfNtries")]
    pub nb_of_ntries: Option<u32>,
    #[serde(rename = "Sum")]
    pub sum: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AmountDetails {
    #[serde(rename = "InstdAmt")]
    pub instd_amt: Option<SingleAmount>,
    #[serde(rename = "TxAmt")]
    pub tx_amt: Option<SingleAmount>,
    #[serde(rename = "PrtryAmt")]
    pub prtry_amt: Option<ProprietaryAmount>,
}

#[derive(Debug, Deserialize)]
pub struct SingleAmount {
    #[serde(rename = "Amt")]
    pub amt: Amount,
}

#[derive(Debug, Deserialize)]
pub struct ProprietaryAmount {
    #[serde(rename = "Tp")]
    pub tp: Option<String>,
    #[serde(rename = "Amt")]
    pub amt: Option<Amount>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedParties {
    #[serde(rename = "Dbtr")]
    pub dbtr: Option<Party>,
    #[serde(rename = "Cdtr")]
    pub cdtr: Option<Party>,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: Option<AccountId>,
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: Option<AccountId>,
}

#[derive(Debug, Deserialize)]
pub struct Party {
    #[serde(rename = "Nm")]
    pub name: Option<String>,
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: Option<PostalAddress>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedAgents {
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: Option<FinancialInstitution>,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: Option<FinancialInstitution>,
}

#[derive(Debug, Deserialize)]
pub struct RemittanceInfo {
    #[serde(rename = "Ustrd", default)]
    pub ustrd: Vec<String>,
    #[serde(rename = "Strd", default)]
    pub strd: Vec<StructuredRemittance>,
}

#[derive(Debug, Deserialize)]
pub struct StructuredRemittance {
    #[serde(rename = "CdtrRefInf")]
    pub cdtr_ref_inf: Option<CreditorReference>,
}

#[derive(Debug, Deserialize)]
pub struct CreditorReference {
    #[serde(rename = "Tp")]
    pub tp: Option<CodeOrProprietary>,
    #[serde(rename = "Ref")]
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedDates {
    #[serde(rename = "AccptncDtTm")]
    pub accptnc_dt_tm: Option<String>,
}

impl Camt053 {
    pub fn from_string(data: &str) -> Result<Self, ParserError> {
        let data = data.replace('\u{00A0}', " ").replace('\u{FEFF}', "");
        let result: Camt053 = from_str(&data).map_err(|e| ParserError::Camt053(e.to_string()))?;
        Ok(result)
    }
}

impl FinancialDataRead for Camt053 {
    fn from_read<R: std::io::Read>(reader: R) -> Result<Self, ParserError> {
        let data = Self::read_to_string(reader).map_err(|e| ParserError::Camt053(e.to_string()))?;
        Self::from_string(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_parse_fields() {
        let path = std::path::Path::new(r"test_data");
        let valid_file1 = File::open(path.join("valid1.camt053")).unwrap();
        let doc = Camt053::from_read(valid_file1);
        assert!(doc.is_ok());
        let doc = doc.unwrap();

        let grp_hdr = doc
            .bk_to_cstmr_stmt
            .as_ref()
            .unwrap()
            .grp_hdr
            .as_ref()
            .unwrap();
        assert_eq!(grp_hdr.msg_id.as_deref(), Some("STMT20231020001"));
        assert_eq!(grp_hdr.cre_dt_tm.as_deref(), Some("2023-10-20T14:30:00"));

        let stmt = &doc.bk_to_cstmr_stmt.as_ref().unwrap().stmts[0];
        let acct = stmt.acct.as_ref().unwrap();
        assert_eq!(
            acct.id.as_ref().unwrap().iban.as_deref(),
            Some("NL91ABNA0417164300")
        );
        assert_eq!(acct.ccy.as_deref(), Some("EUR"));
        assert_eq!(
            acct.owner.as_ref().unwrap().name.as_deref(),
            Some("Example Company B.V.")
        );

        let bal_opbd = &stmt.bal[0];
        assert_eq!(bal_opbd.tp.cd_or_prtry.cd.as_deref(), Some("OPBD"));
        assert_eq!(bal_opbd.amt_balance.value, 1000.00);
        assert_eq!(bal_opbd.amt_balance.currency.as_deref(), Some("EUR"));

        let bal_clbd = &stmt.bal[1];
        assert_eq!(bal_clbd.tp.cd_or_prtry.cd.as_deref(), Some("CLBD"));
        assert_eq!(bal_clbd.amt_balance.value, 1250.50);
        assert_eq!(bal_clbd.amt_balance.currency.as_deref(), Some("EUR"));

        let entry = &stmt.ntry[0];
        assert_eq!(entry.amt_entry.value, 250.50);
        assert_eq!(entry.amt_entry.currency.as_deref(), Some("EUR"));
        assert_eq!(entry.cdt_dbt_ind.as_deref(), Some("CRDT"));
        assert_eq!(entry.sts.as_deref(), Some("BOOK"));

        let tx = &entry.ntry_dtls[0].tx_dtls[0];
        assert_eq!(tx.refs.as_ref().unwrap().msg_id.as_deref(), Some("MSG001"));
        assert_eq!(
            tx.refs.as_ref().unwrap().end_to_end_id.as_deref(),
            Some("E2E001")
        );
        assert_eq!(
            tx.rltd_pties
                .as_ref()
                .unwrap()
                .dbtr
                .as_ref()
                .unwrap()
                .name
                .as_deref(),
            Some("Customer Name")
        );
        assert_eq!(
            tx.rmt_inf.as_ref().unwrap().ustrd[0],
            "Invoice 12345 payment"
        );
    }
}
