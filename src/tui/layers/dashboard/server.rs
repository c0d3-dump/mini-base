use std::rc::Rc;

use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, TextArea},
    Cursive, View,
};

use crate::tui::{
    components,
    model::Sidebar,
    utils::{
        get_current_model, get_current_mut_model, get_data_from_refname, update_query_with_model,
    },
};

pub fn server_dashboard(s: &mut Cursive) -> impl View {
    Dialog::new()
        .title("server")
        .full_screen()
        .with_name(Sidebar::SERVER.to_string())
}
