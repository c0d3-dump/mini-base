use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tui::model::{Conn, StorageAccess};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i64,
    pub email: String,
    pub token: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUser {
    pub id: i64,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub dbconn: Conn,
    pub curr_role: Vec<String>,
    pub default_role: String,
}

#[derive(Debug, Clone)]
pub struct StorageState {
    pub default_role: String,
    pub storage_access: HashMap<String, StorageAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub id: i64,
    pub filename: String,
    pub uniquename: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteFileSchema {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GetFileSchema {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenFile {
    pub unique_name: String,
}
