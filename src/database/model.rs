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

#[derive(Debug, Clone, Default)]
pub enum DbType {
    #[default]
    SQLITE,
    MYSQL,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub is_default: bool,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role_id: Option<i64>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Storage {
    pub id: i64,
    pub file_name: String,
    pub unique_name: String,
    pub uploaded_by: i64,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Query {
    pub id: i64,
    pub name: String,
    pub exec_type: String,
    pub query: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct RoleAccess {
    pub role_id: i64,
    pub query_id: i64,
}
