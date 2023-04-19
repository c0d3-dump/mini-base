use database::{model::ColType, sqlite::Sqlite};

mod database;
mod parser;
mod server;
mod tui;

// #[tokio::main]
 fn main() {
    tui::run();

    // let conn = Sqlite::new("test.db");

    // let rows = conn
    //     .query_all(
    //         "SELECT * FROM user WHERE name=?",
    //         vec![ColType::String(Some("b2b".to_owned()))],
    //     )
    //     .await;

    // let out = conn.parse_all(rows);

    // println!("{}", serde_json::to_string(&out).unwrap())
}
