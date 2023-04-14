use std::{
    any::{self, Any},
    borrow::Borrow,
};

use async_std::{println, stream::StreamExt};
use futures::TryStreamExt;
use sqlx::{
    sqlite::{SqliteColumn, SqliteRow, SqliteTypeInfo},
    types::Json,
    Column, Database, Decode, Row, Type, TypeInfo,
};

use crate::database::model::ColType;

mod database;
mod tui;

#[async_std::main]
async fn main() {
    // tui::run()

    let pool = database::sqlite::Sqlite::new().await;
    let rows = pool
        .query_all(
            "DELETE FROM table_name WHERE id>?",
            vec![ColType::Integer(2)],
        )
        .await;

    let out = pool.parse_all(rows);

    println!("{:#?}", out).await;
}
