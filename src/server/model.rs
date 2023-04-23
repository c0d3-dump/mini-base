use serde::{Deserialize, Serialize};

use crate::tui::model::Conn;

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
}
