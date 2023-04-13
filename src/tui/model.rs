use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub db: Db,
    pub list: Vec<List>,
}

#[derive(Clone, Debug, Default)]
pub struct List {
    pub db: Db,
    pub label: String,
    pub query: String,
}

#[derive(Clone, Debug, Default)]
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
