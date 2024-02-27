use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Offset {
    pub user: i64,
    pub storage: i64,
}

#[derive(Debug, Clone)]
pub struct SearchTerm {
    pub user: String,
    pub storage: String,
}

#[derive(Debug, Clone)]
pub struct Temp {
    pub query_access: Vec<QueryAccess>,
    pub query_string: String,
    pub query_written: bool,
    pub query_access_update: bool,
    pub selected_role_access_id: Option<i64>,
    pub restart_required: bool,
    pub up_migration_string: String,
    pub down_migration_string: String,
    pub up_migration_written: bool,
    pub down_migration_written: bool,
    pub editor_popup_active: bool,
    pub webhook_query: Vec<WebhookQuery>,
    pub webhook_query_update: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub is_default: bool,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoleName {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoleAccess {
    pub role_id: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoleAccess {
    pub role_id: i64,
    pub name: String,
    pub is_selected: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Migration {
    pub id: i64,
    pub name: String,
    pub up_query: String,
    pub down_query: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MigrationName {
    pub id: i64,
    pub name: String,
    pub executed: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MigrationUp {
    pub id: i64,
    pub up_query: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MigrationDown {
    pub id: i64,
    pub down_query: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserStorage {
    pub id: i64,
    pub role_id: Option<i64>,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserId {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub role_id: Option<i64>,
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Query {
    pub id: i64,
    pub name: String,
    pub exec_type: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QueryName {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QueryAccess {
    pub id: i64,
    pub name: String,
    pub has_access: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QueryString {
    pub query: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WebhookName {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub exec_type: String,
    pub action: String,
    pub url: String,
    pub args: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WebhookQuery {
    pub id: i64,
    pub name: String,
    pub is_connected: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DefaultRole {
    pub role: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Storage {
    pub id: i64,
    pub file_name: String,
    pub unique_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Setup {
    pub dbpath: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ips: String,
    pub auth_secret: String,
    pub storage_secret: String,
}
