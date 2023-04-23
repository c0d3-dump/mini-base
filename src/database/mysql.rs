use std::collections::HashMap;

use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions, MySqlRow},
    Column, Row, TypeInfo,
};

use super::model::ColType;

#[derive(Debug, Clone, Default)]
pub struct Mysql {
    pub connection: Option<MySqlPool>,
    pub err: Option<(String, String)>,
}

impl Mysql {
    #[tokio::main]
    pub async fn new(dbpath: &str) -> Self {
        let opt_connection = MySqlPoolOptions::new().connect(dbpath).await;

        match opt_connection {
            Ok(connection) => {
                let query = "
                CREATE TABLE
                    IF NOT EXISTS users(
                        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                        email VARCHAR(100) UNIQUE NOT NULL,
                        password VARCHAR(255) NOT NULL,
                        role VARCHAR(20)
                    );";

                let q = sqlx::query(query);
                let _ = q.execute(&connection).await.unwrap();

                Self {
                    connection: Some(connection),
                    err: None,
                }
            }
            Err(err) => {
                let code = err.as_database_error().unwrap().code().unwrap().to_string();
                let msg = err.as_database_error().unwrap().message().to_string();

                Self {
                    connection: None,
                    err: Some((code, msg)),
                }
            }
        }
    }

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Vec<MySqlRow> {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::UnsignedInteger(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Array(_) => todo!(),
                ColType::Object(_) => todo!(),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        q.fetch_all(conn).await.unwrap()
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> u64 {
        let mut q = sqlx::query(query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::UnsignedInteger(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                ColType::Bool(t) => q.bind(t),
                ColType::Array(_) => todo!(),
                ColType::Object(_) => todo!(),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        let out = q.execute(conn).await.unwrap();

        out.rows_affected()
    }

    pub fn parse_all(&self, rows: Vec<MySqlRow>) -> Vec<HashMap<String, ColType>> {
        let mut table_data = vec![];

        for row in rows {
            let mut map: HashMap<String, ColType> = HashMap::new();

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "TEXT" | "VARCHAR" => {
                        let t = row.get::<Option<String>, _>(i);
                        ColType::String(t)
                    }
                    "INTEGER" | "INT" | "BIGINT" => {
                        let t = row.get::<Option<i64>, _>(i);
                        ColType::Integer(t)
                    }
                    "BIGINT UNSIGNED" => {
                        let t = row.get::<Option<u64>, _>(i);
                        ColType::UnsignedInteger(t)
                    }
                    _ => panic!("wrong type found!"),
                };

                map.insert(row.column(i).name().to_string(), row_value);
            }

            table_data.push(map);
        }

        table_data
    }
}
