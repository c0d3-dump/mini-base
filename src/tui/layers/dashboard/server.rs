use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, NamedView, ResizedView},
    Cursive,
};

use crate::tui::model::Sidebar;

pub fn server_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    Dialog::new()
        .title("server")
        .full_screen()
        .with_name(Sidebar::SERVER.to_string())
}
