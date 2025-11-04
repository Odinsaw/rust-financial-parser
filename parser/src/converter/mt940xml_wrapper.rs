use crate::mt940::format::*;
use serde::{Deserialize, Serialize};
use swift_mt_message::fields::*;

/// This wrapper is needed beceause XML tags are incompatible with CAMT940 fields names
#[derive(Deserialize, Serialize)]
pub(crate) struct Mt940Xml {
    pub(crate) basic_header: BasicHeaderBlock,
    pub(crate) application_header: String,
    pub(crate) user_header: Option<String>,
    pub(crate) statement: MT940XmlStatement,
    pub(crate) footer: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MT940XmlStatement {
    #[serde(rename = "field20")]
    pub(crate) field_20: Field20,

    #[serde(rename = "field21")]
    pub(crate) field_21: Option<Field21NoOption>,

    #[serde(rename = "field25")]
    pub(crate) field_25: Field25NoOption,

    #[serde(rename = "field28C")]
    pub(crate) field_28c: Field28C,

    #[serde(rename = "field60F")]
    pub(crate) field_60f: Field60F,

    #[serde(rename = "transactions")]
    pub(crate) statement_lines: Vec<MT940StatementLineXml>,

    #[serde(rename = "field62F")]
    pub(crate) field_62f: Field62F,

    #[serde(rename = "field64")]
    pub(crate) field_64: Option<Field64>,

    #[serde(rename = "field65")]
    pub(crate) field_65: Option<Vec<Field65>>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MT940StatementLineXml {
    #[serde(rename = "field61")]
    pub(crate) field_61: Field61,
    #[serde(rename = "field86")]
    pub(crate) field_86: Option<Field86>,
}
