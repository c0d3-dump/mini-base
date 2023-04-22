use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    // pub role: String,
    // pub is_verified: bool,
    // pub created_at: Option<DateTime<Utc>>,
    // pub updated_at: Option<DateTime<Utc>>,
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
    // pub role: String,
    // pub is_verified: bool,
    // pub created_at: Option<DateTime<Utc>>,
    // pub updated_at: Option<DateTime<Utc>>,
}
