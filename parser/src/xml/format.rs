use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Универсальный обёрточный тип для XML
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XmlWrapper(pub Value);
