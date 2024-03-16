use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveTime};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    Json(Option<String>),
    Array(Option<Vec<ColType>>),
    Object(Option<HashMap<String, Box<ColType>>>),
}

impl ColType {
    pub fn get_col_type_from_value(val: Value) -> ColType {
        match val {
            Value::Null => ColType::Bool(None),
            Value::Bool(t) => ColType::Bool(Some(t)),
            Value::Number(t) => ColType::Real(t.as_f64()),
            Value::String(t) => ColType::String(Some(t)),
            Value::Array(t) => ColType::Array(Some(
                t.into_iter()
                    .map(Self::get_col_type_from_value)
                    .collect(),
            )),
            Value::Object(t) => {
                let mut map = HashMap::new();
                t.into_iter().for_each(|(k, v)| {
                    map.insert(k, Box::new(Self::get_col_type_from_value(v)));
                });
                ColType::Object(Some(map))
            }
        }
    }
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

#[derive(Debug, Clone, Default, PartialEq, Sequence)]
pub enum DbType {
    #[default]
    Sqlite,
    Mysql,
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbType::Sqlite => write!(f, "SQLITE"),
            DbType::Mysql => write!(f, "MYSQL"),
        }
    }
}
