use std::{collections::HashMap, fs::File};

use chrono::{DateTime, Local, NaiveTime};
use sqlx::{
    query_as,
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow},
    Column, Error, FromRow, Row, TypeInfo,
};

use super::model::ColType;

#[derive(Debug)]
pub struct Sqlite {
    pub connection: Result<SqlitePool, Error>,
}

impl Clone for Sqlite {
    fn clone(&self) -> Self {
        Self {
            connection: match self.connection {
                Ok(conn) => Ok(conn.clone()),
                Err(e) => Err(e),
            },
        }
    }
}

impl Sqlite {
    #[tokio::main]
    pub async fn new(dbpath: &str) -> Self {
        match File::open(dbpath) {
            Err(_) => match File::create(dbpath) {
                Err(_) => {
                    return Self {
                        connection: Err(Error::Protocol("Error creating file".to_string())),
                    };
                }
                _ => {}
            },
            _ => {}
        }

        let opt_connection = SqlitePoolOptions::new().connect(dbpath).await;

        match opt_connection {
            Ok(connection) => {
                let query = "
                    CREATE TABLE IF NOT EXISTS
                        roles (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR(255) NOT NULL,
                            is_default TINYINT(1) NOT NULL DEFAULT 0,
                            can_read TINYINT(1) NOT NULL DEFAULT 0,
                            can_write TINYINT(1) NOT NULL DEFAULT 0,
                            can_delete TINYINT(1) NOT NULL DEFAULT 0
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        users (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            email VARCHAR(100) UNIQUE NOT NULL,
                            password VARCHAR(255) NOT NULL,
                            role_id INTEGER,
                            FOREIGN KEY (role_id) REFERENCES roles (id)
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        storage (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            file_name VARCHAR(255) NOT NULL,
                            unique_name VARCHAR(36) NOT NULL,
                            uploaded_by INTEGER NOT NULL,
                            FOREIGN KEY (uploaded_by) REFERENCES users (id)
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        queries (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR(255) NOT NULL,
                            exec_type VARCHAR(50) NOT NULL CHECK (exec_type IN ('fetch', 'execute')),
                            query TEXT
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        role_access (
                            role_id INTEGER NOT NULL,
                            query_id INTEGER NOT NULL,
                            FOREIGN KEY (role_id) REFERENCES roles (id),
                            FOREIGN KEY (query_id) REFERENCES queries (id)
                        );
                    ";

                let q = sqlx::query(query);
                match q.execute(&connection).await {
                    Ok(_) => Self {
                        connection: Ok(connection),
                    },
                    Err(e) => Self { connection: Err(e) },
                }
            }
            Err(e) => Self { connection: Err(e) },
        }
    }

    pub async fn query_all(
        &self,
        query: &str,
        args: Vec<ColType>,
    ) -> Result<Vec<SqliteRow>, Error> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => return Err(Error::Protocol("wrong type".to_string())),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.as_database_error().unwrap().message()),
        };

        match q.fetch_all(conn).await {
            Ok(out) => Ok(out),
            Err(e) => Err(e),
        }
    }

    pub async fn query_one(&self, query: &str, args: Vec<ColType>) -> Result<SqliteRow, Error> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => return Err(Error::Protocol("wrong type".to_string())),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.as_database_error().unwrap().message()),
        };

        match q.fetch_one(conn).await {
            Ok(out) => Ok(out),
            Err(e) => Err(e),
        }
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> Result<u64, Error> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => return Err(Error::Protocol("wrong type".to_string())),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.as_database_error().unwrap().message()),
        };

        match q.execute(conn).await {
            Ok(out) => Ok(out.rows_affected()),
            Err(e) => Err(e),
        }
    }

    pub async fn query_all_with_type<T>(&self, query: &str) -> Result<Vec<T>, Error>
    where
        T: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.as_database_error().unwrap().message()),
        };

        let r_out: Result<Vec<T>, Error> = query_as(&query).fetch_all(conn).await;

        match r_out {
            Ok(out) => Ok(out),
            Err(e) => Err(e),
        }
    }

    pub async fn query_one_with_type<T>(&self, query: &str) -> Result<T, Error>
    where
        T: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
    {
        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e.as_database_error().unwrap().message()),
        };

        let r_out: Result<T, Error> = query_as(&query).fetch_one(conn).await;

        match r_out {
            Ok(out) => Ok(out),
            Err(e) => Err(e),
        }
    }

    pub fn parse_all(&self, rows: Vec<SqliteRow>) -> Result<Vec<HashMap<String, ColType>>, Error> {
        let mut table_data = vec![];

        for row in rows {
            let mut map: HashMap<String, ColType> = HashMap::new();

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "TEXT" | "VARCHAR" => {
                        let t = row.get::<Option<String>, _>(i);
                        ColType::String(t)
                    }
                    "INTEGER" => {
                        let t = row.get::<Option<i64>, _>(i);
                        ColType::Integer(t)
                    }
                    "REAL" | "NUMERIC" => {
                        let t = row.get::<Option<f64>, _>(i);
                        ColType::Real(t)
                    }
                    "BOOLEAN" => {
                        let t = row.get::<Option<bool>, _>(i);
                        ColType::Bool(t)
                    }
                    "DATETIME" => {
                        let t = row.get::<Option<DateTime<Local>>, _>(i);
                        ColType::Datetime(t)
                    }
                    "DATE" => {
                        let t = row.get::<Option<DateTime<Local>>, _>(i);
                        ColType::Date(t)
                    }
                    "TIME" => {
                        let t = row.get::<Option<NaiveTime>, _>(i);
                        ColType::Time(t)
                    }
                    _ => return Err(Error::Protocol("wrong type".to_string())),
                };

                map.insert(row.column(i).name().to_string(), row_value);
            }

            table_data.push(map);
        }

        Ok(table_data)
    }
}
