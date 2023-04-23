use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColType {
    Integer(Option<i64>),
    UnsignedInteger(Option<u64>),
    String(Option<String>),
    Bool(Option<bool>),
    Array(Vec<ColType>),
    Object(HashMap<String, ColType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColInfo {
    pub cid: i64,
    pub name: String,
    pub ctype: String,
    pub notnull: bool,
    pub dflt_value: Option<String>,
    pub pk: bool,
}
