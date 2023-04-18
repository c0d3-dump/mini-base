use std::fmt;

use axum_server::Handle;
use serde::{Deserialize, Serialize};

use crate::database::sqlite::Sqlite;

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub db: Db,
    pub conn: Conn,
    pub auth: Vec<Auth>,
    pub rolelist: Vec<RoleList>,
    pub querylist: Vec<QueryList>,
    pub handle: Option<Handle>,
}

#[derive(Clone, Debug, Default)]
pub enum Conn {
    SQLITE(Sqlite),
    MYSQL,
    POSTGRES,
    #[default]
    None,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RoleList {
    pub label: String,
    pub approval_required: bool,
    pub role_access: Vec<RoleAccess>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum RoleAccess {
    #[default]
    NONE,
    READ,
    CREATE,
    DELETE,
    UPDATE,
}

impl fmt::Display for RoleAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoleAccess::NONE => write!(f, "None"),
            RoleAccess::READ => write!(f, "Read"),
            RoleAccess::CREATE => write!(f, "Create"),
            RoleAccess::DELETE => write!(f, "Delete"),
            RoleAccess::UPDATE => write!(f, "Update"),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct QueryList {
    pub label: String,
    pub exec_type: ExecType,
    pub role_access: Vec<String>,
    pub query: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ExecType {
    #[default]
    QUERY,
    EXECUTION,
}

impl fmt::Display for ExecType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecType::QUERY => write!(f, "Query"),
            ExecType::EXECUTION => write!(f, "Execution"),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Auth {
    #[default]
    EMAIL,
    GOOGLE,
    GITHUB,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Db {
    SQLITE {
        dbpath: String,
    },
    MYSQL {
        host: String,
        username: String,
        port: u16,
        password: String,
        database: Option<String>,
    },
    POSTGRES {
        host: String,
        username: String,
        port: u16,
        password: String,
        database: Option<String>,
    },
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub enum DbType {
    SQLITE,
    MYSQL,
    POSTGRES,
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbType::SQLITE => write!(f, "SQLITE"),
            DbType::MYSQL => write!(f, "MYSQL"),
            DbType::POSTGRES => write!(f, "POSTGRES"),
        }
    }
}

impl DbType {
    pub fn get_items(&self) -> Vec<&str> {
        match self {
            DbType::SQLITE => vec!["dbpath"],
            DbType::MYSQL => vec!["host", "username", "port", "password", "database"],
            DbType::POSTGRES => vec!["host", "username", "port", "password", "database"],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sidebar {
    AUTH,
    ROLE,
    QUERY,
    EDITOR,
    DOCS,
    SERVER,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::AUTH => write!(f, "AUTH"),
            Sidebar::ROLE => write!(f, "ROLE"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::EDITOR => write!(f, "EDITOR"),
            Sidebar::DOCS => write!(f, "DOCS"),
            Sidebar::SERVER => write!(f, "SERVER"),
        }
    }
}

impl Sidebar {}
