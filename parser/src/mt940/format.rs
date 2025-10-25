use serde::{Deserialize, Deserializer, Serialize, Serializer};
use swift_mt_message::messages;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Mt940 {
    pub basic_header: BasicHeaderBlock,
    pub application_header: String,  // depends on implementation
    pub user_header: Option<String>, // depends on implementation, may be skipped
    pub statement: messages::MT940,
    pub footer: Option<String>, // depends on implementation, may be skipped
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicHeaderBlock {
    pub application_identifier: String,
    pub service_identifier: String,
    pub lt_identifier: String,
    pub session_number: String,
    pub sequence_number: String,
}

// custom serialization for BasicHeaderBlock
impl Serialize for BasicHeaderBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Реализация Deserialize
impl<'de> Deserialize<'de> for BasicHeaderBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BasicHeaderBlock::from_string(&s).map_err(serde::de::Error::custom)
    }
}
