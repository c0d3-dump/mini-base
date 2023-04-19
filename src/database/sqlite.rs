use std::fs::File;

use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow},
    Column, Row, TypeInfo,
};

use super::model::{ColInfo, ColType};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pub connection: Option<SqlitePool>,
    pub err: Option<(String, String)>,
}

impl Default for Sqlite {
    fn default() -> Self {
        Self {
            connection: None,
            err: None,
        }
    }
}

impl Sqlite {
    pub fn new(file_path: &str) -> Self {
        match File::open(file_path) {
            Err(_) => match File::create(file_path) {
                Err(_) => {
                    return Self {
                        connection: None,
                        err: Some(("1".to_string(), "Error creating file".to_string())),
                    };
                }
                _ => {}
            },
            _ => {}
        }

        let opt_connection = futures::executor::block_on( SqlitePoolOptions::new().connect(file_path));

        match opt_connection {
            Ok(connection) => Self {
                connection: Some(connection),
                err: None,
            },
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

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Vec<SqliteRow> {
        let mut q = sqlx::query(&query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        q.fetch_all(conn).await.unwrap()
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> u64 {
        let mut q = sqlx::query(&query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
            };
        }

        let conn = match &self.connection {
            Some(conn) => conn,
            None => panic!("query all: error while getting connection string"),
        };

        let out = q.execute(conn).await.unwrap();

        out.rows_affected()
    }

    pub async fn get_table_info(&self, name: &str) -> Vec<ColInfo> {
        let q = format!("PRAGMA table_info({})", name);

        let rows = self.query_all(&q, vec![]).await;

        let mut info: Vec<ColInfo> = vec![];

        for row in rows {
            info.push(ColInfo {
                cid: row.get::<i64, _>(0),
                name: row.get::<&str, _>(1).to_string(),
                ctype: row.get::<&str, _>(2).to_string(),
                notnull: if row.get::<i8, _>(3) == 1 {
                    true
                } else {
                    false
                },
                dflt_value: row.get::<Option<String>, _>(4),
                pk: if row.get::<i8, _>(5) == 1 {
                    true
                } else {
                    false
                },
            });
        }

        info
    }

    pub fn parse_all(&self, rows: Vec<SqliteRow>) -> Vec<Vec<ColType>> {
        let mut table_data = vec![];

        for row in rows {
            let mut row_data: Vec<ColType> = vec![];

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "TEXT" => {
                        let t = row.get::<Option<String>, _>(i);
                        ColType::String(t)
                    }
                    "INTEGER" => {
                        let t = row.get::<Option<i64>, _>(i);
                        ColType::Integer(t)
                    }
                    _ => panic!("wrong type found!"),
                };

                row_data.push(row_value);
            }

            table_data.push(row_data);
        }

        table_data
    }
}
