mod components;
mod jsondb;
mod layers;
pub mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    let model = futures::executor::block_on(jsondb::from_json());

    // thread::spawn(|| {
    //     server::start_server();
    // });

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
