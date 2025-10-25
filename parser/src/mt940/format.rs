use serde::{Deserialize, Serialize};
use swift_mt_message::messages;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Mt940 {
    pub basic_header: BasicHeaderBlock,
    pub application_header: String,  // depends on implementation
    pub user_header: Option<String>, // depends on implementation, may be skipped
    pub statement: messages::MT940,
    pub footer: Option<String>, // depends on implementation, may be skipped
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BasicHeaderBlock {
    pub application_identifier: String,
    pub service_identifier: String,
    pub lt_identifier: String,
    pub session_number: String,
    pub sequence_number: String,
}
