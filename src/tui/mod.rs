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

    app.set_user_data(model.clone());

    match model.conn {
        model::Conn::None => {
            layers::setup_db::select_dbtype(&mut app);
        }
        _ => {
            layers::dashboard::display_dashboard(&mut app);
        }
    }

    app.run();
}
