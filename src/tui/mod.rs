mod components;
mod jsondb;
mod layers;
pub mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    let model = jsondb::from_json();

    app.set_user_data(model);

    layers::setup_db::select_dbtype(&mut app);

    app.run();
}
