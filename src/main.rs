use std::thread;

use axum_server::Handle;

mod database;
mod queries;
// mod parser;
mod server;
mod tui;

#[tokio::main]
async fn main() {
    let model = queries::Model {
        conn: None,
        handle: Some(Handle::new()),
        offset: queries::model::Offset {
            user: 0,
            storage: 0,
        },
        temp: queries::model::Temp {
            query_access: vec![],
            query_string: "".to_string(),
        },
    };

    thread::spawn(|| {
        server::start_server(model);
    });

    tui::run();
}
