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

    // let exec = pool.execute("DELETE FROM user", vec![]).await;

    // println!("exec {:?}", exec).await;

    // let exec = pool
    //     .execute(
    //         "INSERT INTO user (content) VALUES (?); INSERT INTO user (content) VALUES (?);",
    //         vec![
    //             ColType::String("Hello".to_string()),
    //             ColType::String("Hi".to_string()),
    //         ],
    //     )
    //     .await;

    // println!("exec {:?}", exec).await;

    let rows = pool.query_all("SELECT * FROM user", vec![]).await;

    let out = pool.parse_all(rows);

    // let out = pool.get_table_info("user").await;

    println!("{:#?}", out).await;
}
