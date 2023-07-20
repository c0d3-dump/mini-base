use std::fmt;

use axum_server::Handle;
use serde::{Deserialize, Serialize};

use crate::database::mysql::Mysql;
use crate::database::sqlite::Sqlite;

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub db: Db,
    pub conn: Conn,
    pub rolelist: Vec<String>,
    pub default_role: String,
    pub querylist: Vec<QueryList>,
    pub handle: Option<Handle>,
    pub offset: usize,
}

#[derive(Clone, Debug, Default)]
pub enum Conn {
    SQLITE(Sqlite),
    MYSQL(Mysql),
    #[default]
    None,
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
pub enum Db {
    SQLITE {
        dbpath: String,
    },
    MYSQL {
        dbpath: String,
    },
    #[default]
    None,
}

#[derive(Debug, Clone)]
pub enum DbType {
    SQLITE,
    MYSQL,
}

impl fmt::Display for DbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbType::SQLITE => write!(f, "SQLITE"),
            DbType::MYSQL => write!(f, "MYSQL"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Sidebar {
    AUTH,
    ROLE,
    QUERY,
    EDITOR,
    SERVER,
}

impl fmt::Display for Sidebar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sidebar::AUTH => write!(f, "AUTH"),
            Sidebar::ROLE => write!(f, "ROLE"),
            Sidebar::QUERY => write!(f, "QUERY"),
            Sidebar::EDITOR => write!(f, "EDITOR"),
            Sidebar::SERVER => write!(f, "SERVER"),
        }
    }
}

impl Sidebar {}
