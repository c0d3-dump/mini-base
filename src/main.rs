use std::thread;

mod database;
mod parser;
mod server;
mod tui;

fn main() {
    tui::run();
}
