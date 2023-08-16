mod database;
mod queries;
// mod parser;
// mod server;
mod tui;

#[tokio::main]
async fn main() {
    tui::run();
}
