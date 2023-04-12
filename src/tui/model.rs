#[derive(Clone, Debug, Default)]
pub struct Model {
    pub dbtype: Vec<String>,
    pub db: Db,
    pub list: Vec<List>,
}

#[derive(Clone, Debug, Default)]
pub struct List {
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
