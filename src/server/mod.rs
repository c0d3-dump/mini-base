use axum::{routing::get, Router};
use axum_server::Handle;
use std::{net::SocketAddr, sync::mpsc::Receiver};

use crate::tui::model::Model;

#[tokio::main]
pub async fn start_server(model: Model, handle: Handle) {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
