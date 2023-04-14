use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteRow},
    Column, Row, TypeInfo,
};

use super::model::ColType;

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

    pub fn parse_all(&self, rows: Vec<SqliteRow>) -> Vec<Vec<(String, ColType)>> {
        let mut table_data = vec![];

        for row in rows {
            let mut row_data: Vec<(String, ColType)> = vec![];

            for i in 0..row.len() {
                let row_value = match row.column(i).type_info().name() {
                    "NULL" => ColType::Null,
                    "TEXT" => {
                        let t = row.get::<&str, _>(i);
                        ColType::String(t.to_string())
                    }
                    "INTEGER" => {
                        let t = row.get::<i64, _>(i);
                        ColType::Integer(t)
                    }
                    _ => panic!(),
                };

                row_data.push((row.column(i).name().to_string(), row_value));
            }

            table_data.push(row_data);
        }

        table_data
    }
}
