use axum::{routing::get, Router};
use axum_server::Handle;
use std::net::SocketAddr;

use crate::tui::model::Model;

#[tokio::main]
pub async fn start_server(model: Model) {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // TODO: convert this to argument to this function and handle event from outside
    let handle = Handle::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
