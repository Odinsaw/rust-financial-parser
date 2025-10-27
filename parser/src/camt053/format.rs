use serde::{Deserialize, Serialize};

// Root document
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename = "Document")]
pub struct Camt053 {
    #[serde(rename = "BkToCstmrStmt")]
    pub bk_to_cstmr_stmt: BankToCustomerStatement,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BankToCustomerStatement {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader,
    #[serde(rename = "Stmt", default)]
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct GroupHeader {
    #[serde(rename = "MsgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
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
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FromToDate {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: Option<String>,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Account {
    #[serde(rename = "Id")]
    pub id: Option<AccountId>,
    #[serde(rename = "Ccy")]
    pub ccy: Option<String>,
    #[serde(rename = "Nm")]
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct AccountId {
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
    #[serde(rename = "Othr")]
    pub other: Option<GenericAccountIdentification>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct GenericAccountIdentification {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Balance {
    #[serde(rename = "Tp")]
    pub tp: Option<BalanceType>,
    #[serde(rename = "Amt")]
    pub amt: Option<Amount>,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<String>,
    #[serde(rename = "Dt")]
    pub dt: Option<DateAndDateTimeChoice>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: Option<CodeOrProprietary>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct CodeOrProprietary {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "Prtry")]
    pub prtry: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Amount {
    #[serde(rename = "@Ccy")]
    pub currency: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct DateAndDateTimeChoice {
    #[serde(rename = "Dt")]
    pub dt: Option<String>,
    #[serde(rename = "DtTm")]
    pub dt_tm: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Entry {
    #[serde(rename = "Amt")]
    pub amt: Option<Amount>,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: Option<String>,
    #[serde(rename = "Sts")]
    pub sts: Option<String>,
    #[serde(rename = "BookgDt")]
    pub bookg_dt: Option<DateAndDateTimeChoice>,
    #[serde(rename = "ValDt")]
    pub val_dt: Option<DateAndDateTimeChoice>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: Option<BankTransactionCode>,
    #[serde(rename = "NtryDtls", default)]
    pub ntry_dtls: Vec<EntryDetails>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BankTransactionCode {
    #[serde(rename = "Domn")]
    pub domn: Option<BankTransactionCodeStructure>,
    #[serde(rename = "Prtry")]
    pub prtry: Option<ProprietaryBankTransactionCode>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BankTransactionCodeStructure {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "Fmly")]
    pub fmly: Option<BankTransactionCodeFamily>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct BankTransactionCodeFamily {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ProprietaryBankTransactionCode {
    #[serde(rename = "Cd")]
    pub cd: Option<String>,
    #[serde(rename = "Issr")]
    pub issr: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct EntryDetails {
    #[serde(rename = "TxDtls", default)]
    pub tx_dtls: Vec<TransactionDetails>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct TransactionDetails {
    #[serde(rename = "Refs")]
    pub refs: Option<TransactionReferences>,
    #[serde(rename = "Amt")]
    pub amt: Option<Amount>,
    #[serde(rename = "RltdPties")]
    pub rltd_pties: Option<RelatedParties>,
    #[serde(rename = "RmtInf")]
    pub rmt_inf: Option<RemittanceInformation>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct TransactionReferences {
    #[serde(rename = "MsgId")]
    pub msg_id: Option<String>,
    #[serde(rename = "AcctSvcrRef")]
    pub acct_svcr_ref: Option<String>,
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id: Option<String>,
    #[serde(rename = "InstrId")]
    pub instr_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Option<String>,
    #[serde(rename = "TxId")]
    pub tx_id: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RelatedParties {
    #[serde(rename = "Dbtr")]
    pub dbtr: Option<Party>,
    #[serde(rename = "Cdtr")]
    pub cdtr: Option<Party>,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: Option<AccountIdentification>,
    #[serde(rename = "CdtrAcct")]
    pub cdtr_acct: Option<AccountIdentification>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Party {
    #[serde(rename = "Nm")]
    pub name: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct AccountIdentification {
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
    #[serde(rename = "Othr")]
    pub other: Option<GenericAccountIdentification>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct RemittanceInformation {
    #[serde(rename = "Ustrd", default)]
    pub ustrd: Vec<String>,
}
