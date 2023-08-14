#[derive(Debug, Clone)]
pub struct Offset {
    pub user: i64,
    pub query: i64,
    pub storage: i64,
}

#[derive(Debug, Clone)]
pub struct SearchTerm {
    pub user: String,
    pub query: String,
    pub storage: String,
}

#[derive(Debug, Clone)]
pub struct Index {
    pub role: i64,
    pub user: i64,
    pub query: i64,
    pub storage: i64,
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
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Storage {
    pub id: i64,
    pub file_name: String,
    pub unique_name: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Query {
    pub id: i64,
    pub name: String,
    pub exec_type: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RoleAccess {
    pub role_id: i64,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QueryString {
    pub query: String,
}
