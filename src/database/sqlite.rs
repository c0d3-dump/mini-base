use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow},
    Column, Row, TypeInfo,
};

use super::model::{ColInfo, ColType};

#[derive(Debug, Clone)]
pub struct Sqlite {
    pub connection: SqlitePool,
}

impl Sqlite {
    pub async fn new() -> Self {
        let connection = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("test.db")
            .await
            .unwrap();
        Self { connection }
    }

    pub async fn query_all(&self, query: &str, args: Vec<ColType>) -> Vec<SqliteRow> {
        let mut q = sqlx::query(&query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                _ => panic!("Wrong datatype to bind"),
            };
        }

        q.fetch_all(&self.connection).await.unwrap()
    }

    pub async fn execute(&self, query: &str, args: Vec<ColType>) -> u64 {
        let mut q = sqlx::query(&query);

        for arg in args {
            q = match arg {
                ColType::Integer(t) => q.bind(t),
                ColType::String(t) => q.bind(t),
                _ => panic!("Wrong datatype to bind"),
            };
        }

        let out = q.execute(&self.connection).await.unwrap();

        out.rows_affected()
    }

    pub async fn get_table_info(&self, name: String) -> Vec<ColInfo> {
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
                dflt_value: row.get::<&str, _>(4).to_string(),
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
                        let t = row.get::<&str, _>(i);
                        ColType::String(t.to_string())
                    }
                    "INTEGER" => {
                        let t = row.get::<i64, _>(i);
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
