use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveTime};
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions, MySqlRow},
    types::Json,
    Column, Row, TypeInfo,
};

use super::model::ColType;

#[derive(Debug, Clone, Default)]
pub struct Mysql {
    pub connection: Option<MySqlPool>,
    pub err: Option<String>,
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
                        id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
                        email VARCHAR(100) UNIQUE NOT NULL,
                        password VARCHAR(255) NOT NULL,
                        role VARCHAR(20)
                    );
                
                CREATE TABLE 
                    IF NOT EXISTS storage(
                        id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
                        filename VARCHAR(255) NOT NULL,
                        uniquename VARCHAR(36) NOT NULL,
                        uploaded_by INTEGER NOT NULL,
                        FOREIGN KEY (uploaded_by) REFERENCES users(id)
                    );";

                let q = sqlx::query(query);
                match q.execute(&connection).await {
                    Ok(_) => Self {
                        connection: Some(connection),
                        err: None,
                    },
                    Err(e) => Self {
                        connection: None,
                        err: Some(e.as_database_error().unwrap().message().to_string()),
                    },
                }
            }
            Err(err) => match err.as_database_error() {
                Some(msg) => Self {
                    connection: None,
                    err: Some(msg.message().to_string()),
                },
                None => Self {
                    connection: None,
                    err: Some("something went wrong!".to_string()),
                },
            },
        }
    }

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Vec<MySqlRow> {
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
                _ => panic!("wrong type"),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        match q.fetch_all(conn).await {
            Ok(out) => out,
            Err(_) => vec![],
        }
    }

    pub async fn query_one(&self, query: &str, args: Vec<ColType>) -> Option<MySqlRow> {
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
                _ => panic!("wrong type"),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        match q.fetch_one(conn).await {
            Ok(out) => Some(out),
            Err(_) => None,
        }
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> u64 {
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
                _ => panic!("wrong type"),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("execute: error while getting connection string"),
        };

        match q.execute(conn).await {
            Ok(out) => out.rows_affected(),
            Err(_) => 0,
        }
    }

    pub fn parse_all(&self, rows: Vec<MySqlRow>) -> Vec<HashMap<String, ColType>> {
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
                    _ => panic!("wrong type found!"),
                };

                map.insert(row.column(i).name().to_string(), row_value);
            }

            table_data.push(map);
        }

        table_data
    }
}
