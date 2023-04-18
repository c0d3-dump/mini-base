use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use axum_server::Handle;
use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Button, Dialog, EditView, LinearLayout, ListView, NamedView, ResizedView},
    Cursive,
};

use crate::{
    server,
    tui::{
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model},
    },
};

pub fn server_dashboard(s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let mut layout = LinearLayout::new(Orientation::Vertical);

    let on_restart_pressed = |s: &mut Cursive| {};

    let on_start_pressed = |s: &mut Cursive| {
        let t_model = get_current_model(s);
        let model = Arc::new(Mutex::new(t_model.clone()));

        let mut handle_model = get_current_mut_model(s);
        handle_model.handle = Some(Handle::new());

        let handle = Arc::new(Mutex::new(handle_model.handle.clone().unwrap()));

        thread::spawn(move || {
            let m = model.lock().unwrap();
            let h = handle.lock().unwrap();
            server::start_server(m.to_owned(), h.to_owned());
        });
    };

    let on_stop_pressed = |s: &mut Cursive| {
        let handle_model = get_current_mut_model(s);

        match handle_model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(5))),
            None => (),
        }
    };

    layout.add_child(
        LinearLayout::new(Orientation::Horizontal)
            .child(Button::new("start", on_start_pressed))
            .child(Button::new("restart", on_restart_pressed))
            .child(Button::new("stop", on_stop_pressed)),
    );

    let logs = ListView::new();
    layout.add_child(logs);

    Dialog::new()
        .title("server")
        .content(layout)
        .full_screen()
        .with_name(Sidebar::SERVER.to_string())
}
