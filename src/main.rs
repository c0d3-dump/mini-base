mod database;
mod parser;
mod queries;
mod server;
mod tui;

#[tokio::main]
async fn main() {
    tui::run();
}
