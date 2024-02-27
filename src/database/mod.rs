use std::collections::BTreeMap;

use sqlx::{mysql::MySqlRow, sqlite::SqliteRow, Decode, Error, FromRow, Row, Type};

use self::model::{ColType, DbType};

pub mod model;
pub mod mysql;
pub mod sqlite;

pub enum DbRow {
    Sqlite(SqliteRow),
    Mysql(MySqlRow),
}

impl DbRow {
    fn get_sqlite_row(self) -> SqliteRow {
        match self {
            DbRow::Sqlite(t) => t,
            DbRow::Mysql(_) => panic!(),
        }
    }

    fn get_mysql_row(self) -> MySqlRow {
        match self {
            DbRow::Mysql(t) => t,
            DbRow::Sqlite(_) => panic!(),
        }
    }

    pub fn get<'r, T>(&'r self, idx: usize) -> Result<T, Error>
    where
        T: Decode<'r, <SqliteRow as Row>::Database>
            + Type<<SqliteRow as Row>::Database>
            + Decode<'r, <MySqlRow as Row>::Database>
            + Type<<MySqlRow as Row>::Database>,
    {
        match self {
            DbRow::Sqlite(row) => row.try_get::<'r, T, _>(idx),
            DbRow::Mysql(row) => row.try_get::<'r, T, _>(idx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conn {
    pub dbtype: DbType,
    pub sqlite: Option<sqlite::Sqlite>,
    pub mysql: Option<mysql::Mysql>,
    pub err: Option<String>,
}

impl Conn {
    pub fn new(dbtype: DbType, dbpath: &str) -> Self {
        match dbtype {
            DbType::Sqlite => {
                let sqlite_conn = sqlite::Sqlite::new(dbpath);
                let sqlite = futures::executor::block_on(sqlite_conn);

                Self {
                    dbtype,
                    sqlite: Some(sqlite.clone()),
                    mysql: None,
                    err: sqlite.connection.err(),
                }
            }
            DbType::Mysql => {
                let mysql_conn = mysql::Mysql::new(dbpath);
                let mysql = futures::executor::block_on(mysql_conn);

                Self {
                    dbtype,
                    sqlite: None,
                    mysql: Some(mysql.clone()),
                    err: mysql.connection.err(),
                }
            }
        }
    }

    pub async fn close(&self) {
        match self.dbtype {
            DbType::Sqlite => {
                self.sqlite.as_ref().unwrap().close().await;
            }
            DbType::Mysql => {
                self.mysql.as_ref().unwrap().close().await;
            }
        }
    }

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Result<Vec<DbRow>, String> {
        match self.dbtype {
            DbType::Sqlite => {
                let res = self.sqlite.as_ref().unwrap().query_all(query, args).await;
                match res {
                    Ok(rows) => Ok(rows.into_iter().map(DbRow::Sqlite).collect()),
                    Err(e) => Err(e),
                }
            }
            DbType::Mysql => {
                let res = self.mysql.as_ref().unwrap().query_all(query, args).await;
                match res {
                    Ok(rows) => Ok(rows.into_iter().map(DbRow::Mysql).collect()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub async fn query_one(&self, query: &str, args: Vec<ColType>) -> Result<DbRow, String> {
        match self.dbtype {
            DbType::Sqlite => {
                let res = self.sqlite.as_ref().unwrap().query_one(query, args).await;
                match res {
                    Ok(row) => Ok(DbRow::Sqlite(row)),
                    Err(e) => Err(e),
                }
            }
            DbType::Mysql => {
                let res = self.mysql.as_ref().unwrap().query_one(query, args).await;
                match res {
                    Ok(row) => Ok(DbRow::Mysql(row)),
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> Result<u64, String> {
        match self.dbtype {
            DbType::Sqlite => self.sqlite.as_ref().unwrap().execute(query, args).await,
            DbType::Mysql => self.mysql.as_ref().unwrap().execute(query, args).await,
        }
    }

    pub async fn query_all_with_type<T>(&self, query: &str) -> Result<Vec<T>, String>
    where
        T: for<'r> FromRow<'r, SqliteRow> + for<'r> FromRow<'r, MySqlRow> + Unpin + Send,
    {
        match self.dbtype {
            DbType::Sqlite => {
                self.sqlite
                    .as_ref()
                    .unwrap()
                    .query_all_with_type::<T>(query)
                    .await
            }
            DbType::Mysql => {
                self.mysql
                    .as_ref()
                    .unwrap()
                    .query_all_with_type::<T>(query)
                    .await
            }
        }
    }

    pub async fn query_one_with_type<T>(&self, query: &str) -> Result<T, String>
    where
        T: for<'r> FromRow<'r, SqliteRow> + for<'r> FromRow<'r, MySqlRow> + Unpin + Send,
    {
        match self.dbtype {
            DbType::Sqlite => {
                self.sqlite
                    .as_ref()
                    .unwrap()
                    .query_one_with_type::<T>(query)
                    .await
            }
            DbType::Mysql => {
                self.mysql
                    .as_ref()
                    .unwrap()
                    .query_one_with_type::<T>(query)
                    .await
            }
        }
    }

    pub fn parse_all(&self, rows: Vec<DbRow>) -> Result<Vec<BTreeMap<String, ColType>>, String> {
        match self.dbtype {
            DbType::Sqlite => self
                .sqlite
                .as_ref()
                .unwrap()
                .parse_all(rows.into_iter().map(|s| s.get_sqlite_row()).collect()),
            DbType::Mysql => self
                .mysql
                .as_ref()
                .unwrap()
                .parse_all(rows.into_iter().map(|s| s.get_mysql_row()).collect()),
        }
    }
}
