mod database;
mod parser;
mod queries;
mod server;
mod tui;

#[tokio::main(worker_threads = 2)]
async fn main() {
    tui::run();
}
