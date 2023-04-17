use axum::{routing::get, Router, Server};

use crate::tui::model::Model;

// TODO: create some channel that can be accessed accross thread to start and stop server on demand

#[tokio::main]
pub async fn start_server(model: Model) {
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let server = Server::bind(&([127, 0, 0, 1], 3000).into()).serve(router.into_make_service());

    let graceful = server.with_graceful_shutdown(async {
        // rx.await.ok();
    });

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    // let (tx, rx) = tokio::sync::oneshot::channel::<()>();
}
