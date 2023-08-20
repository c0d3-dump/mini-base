use flexi_logger::Logger;

use crate::queries::Model;

mod components;
mod layers;
pub mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    Logger::try_with_env_or_str("info, error")
        .expect("Could not create Logger from environment :(")
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory("logs")
                .suppress_timestamp(),
        )
        .format(flexi_logger::opt_format)
        .start()
        .expect("failed to initialize logger!");

    app.set_theme(style::get_theme());

    let model = Model::default();

    app.set_user_data(model);

    layers::setup_db::select_dbtype(&mut app);

    app.run();
}
