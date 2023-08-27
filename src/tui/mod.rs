use crate::queries::Model;

mod components;
mod layers;
pub mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    let model = Model::default();

    app.set_user_data(model);

    layers::setup_db::select_dbtype(&mut app);

    app.run();
}
