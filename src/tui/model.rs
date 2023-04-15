use std::fmt;

use serde::{Deserialize, Serialize};

use crate::database::sqlite::Sqlite;

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub db: Db,
    pub conn: Conn,
    pub auth: Auth,
    pub rolelist: Vec<RoleList>,
    pub querylist: Vec<QueryList>,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum RoleAccess {
    #[default]
    NONE,
    READ,
    CREATE,
    DELETE,
    UPDATE,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct QueryList {
    pub label: String,
    pub exec_type: ExecType,
    pub role_access: Vec<RoleList>,
    pub query: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ExecType {
    #[default]
    QUERY,
    EXECUTION,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Auth {
    EMAIL {
        email: String,
        password: String,
    },
    #[default]
    None,
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
    STATS,
    AUTH,
    QUERY,
    DOCS,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::STATS => write!(f, "STATS"),
            Sidebar::AUTH => write!(f, "AUTH"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::DOCS => write!(f, "DOCS"),
        }
    }
}

impl Sidebar {}
