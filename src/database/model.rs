use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColType {
    Integer(Option<i64>),
    Real(Option<f64>),
    UnsignedInteger(Option<u64>),
    String(Option<String>),
    Bool(Option<bool>),
    Date(Option<DateTime<Local>>),
    Time(Option<NaiveTime>),
    Datetime(Option<DateTime<Local>>),
    Json(Option<Json<HashMap<String, ColType>>>),
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
