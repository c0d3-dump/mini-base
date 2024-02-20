use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveTime};
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions, MySqlRow},
    query_as,
    types::Json,
    Column, Error, FromRow, Row, TypeInfo,
};

use super::model::ColType;

#[derive(Debug, Clone)]
pub struct Mysql {
    pub connection: Result<MySqlPool, String>,
}

impl Mysql {
    pub async fn new(dbpath: &str) -> Self {
        let opt_connection = MySqlPoolOptions::new().connect(dbpath).await;

        match opt_connection {
            Ok(connection) => {
                let query = "
                    CREATE TABLE IF NOT EXISTS
                        roles (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR(255) UNIQUE NOT NULL,
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
                            unique_name VARCHAR(36) UNIQUE NOT NULL,
                            uploaded_by INTEGER NOT NULL,
                            FOREIGN KEY (uploaded_by) REFERENCES users (id)
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        queries (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR(255) UNIQUE NOT NULL,
                            exec_type VARCHAR(50) NOT NULL DEFAULT 'get' CHECK (exec_type IN ('get', 'post', 'delete', 'put')),
                            query TEXT DEFAULT ''
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        role_access (
                            role_id INTEGER NOT NULL,
                            query_id INTEGER NOT NULL,
                            FOREIGN KEY (role_id) REFERENCES roles (id),
                            FOREIGN KEY (query_id) REFERENCES queries (id) ON DELETE CASCADE,
                            UNIQUE (role_id, query_id)
                        );
                    
                    CREATE TABLE IF NOT EXISTS
                        migrations (
                            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                            name VARCHAR(255) UNIQUE NOT NULL,
                            up_query TEXT DEFAULT '',
                            down_query TEXT DEFAULT '',
                            executed TINYINT(1) DEFAULT 0
                        );
                    ";

                let q = sqlx::query(query);
                match q.execute(&connection).await {
                    Ok(_) => Self {
                        connection: Ok(connection),
                    },
                    Err(e) => Self {
                        connection: Err(e.to_string()),
                    },
                }
            }
            Err(e) => Self {
                connection: Err(e.to_string()),
            },
        }
    }

    pub async fn close(&self) {
        match &self.connection {
            Ok(conn) => {
                conn.close().await;
            }
            Err(_) => {}
        }
    }

    pub async fn query_all(
        &self,
        query: &str,
        args: Vec<ColType>,
    ) -> Result<Vec<MySqlRow>, String> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::UnsignedInteger(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => return Err("wrong type".to_string()),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        match q.fetch_all(conn).await {
            Ok(out) => Ok(out),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn query_one(&self, query: &str, args: Vec<ColType>) -> Result<MySqlRow, String> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::UnsignedInteger(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Date(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                _ => return Err("wrong type".to_string()),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        match q.fetch_one(conn).await {
            Ok(out) => Ok(out),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> Result<u64, String> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::Real(t) => q.bind(t),
                ColType::UnsignedInteger(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Time(t) => q.bind(t),
                ColType::Datetime(t) => q.bind(t),
                ColType::Json(t) => q.bind(t),
                _ => return Err("wrong type".to_string()),
            };
        }

        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        match q.execute(conn).await {
            Ok(out) => Ok(out.rows_affected()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn query_all_with_type<T>(&self, query: &str) -> Result<Vec<T>, String>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Unpin + Send,
    {
        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        let r_out: Result<Vec<T>, Error> = query_as(query).fetch_all(conn).await;

        match r_out {
            Ok(out) => Ok(out),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn query_one_with_type<T>(&self, query: &str) -> Result<T, String>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Unpin + Send,
    {
        let conn = match &self.connection {
            Ok(conn) => conn,
            Err(e) => panic!("{}", e),
        };

        let r_out: Result<T, Error> = query_as(query).fetch_one(conn).await;

        match r_out {
            Ok(out) => Ok(out),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn parse_all(&self, rows: Vec<MySqlRow>) -> Result<Vec<HashMap<String, ColType>>, String> {
        let mut table_data = vec![];

        for row in rows {
            let mut map: HashMap<String, ColType> = HashMap::new();

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "TEXT" | "VARCHAR" | "ENUM" | "TINYTEXT" | "CHAR" => {
                        let t = row.get::<Option<String>, _>(i);
                        ColType::String(t)
                    }
                    "INTEGER" | "INT" | "BIGINT" | "TINYINT" | "SMALLINT" | "MEDIUMINT"
                    | "DECIMAL" => {
                        let t = row.get::<Option<i64>, _>(i);
                        ColType::Integer(t)
                    }
                    "BIGINT UNSIGNED" | "TINYINT UNSIGNED" | "SMALLINT UNSIGNED"
                    | "INT UNSIGNED" | "MEDIUMINT UNSIGNED" | "TIMESTAMP" => {
                        let t = row.get::<Option<u64>, _>(i);
                        ColType::UnsignedInteger(t)
                    }
                    "FLOAT" | "DOUBLE" => {
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
                    "TIME" => {
                        let t = row.get::<Option<NaiveTime>, _>(i);
                        ColType::Time(t)
                    }
                    "JSON" => {
                        let t = row.get::<Option<Json<HashMap<String, ColType>>>, _>(i);
                        ColType::Json(t)
                    }
                    _ => return Err("wrong type".to_string()),
                };

                map.insert(row.column(i).name().to_string(), row_value);
            }

            table_data.push(map);
        }

        Ok(table_data)
    }
}
