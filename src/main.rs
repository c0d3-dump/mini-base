use flexi_logger::{Age, Cleanup, Criterion, Logger, Naming};

mod database;
mod parser;
mod queries;
mod server;
mod tui;

#[tokio::main(worker_threads = 2)]
async fn main() {
    Logger::try_with_env_or_str("info, error")
        .expect("Could not create Logger from environment :(")
        .log_to_file(flexi_logger::FileSpec::default().directory("logs"))
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        )
        .format(flexi_logger::opt_format)
        .start()
        .expect("failed to initialize logger!");

    tui::run();
}
