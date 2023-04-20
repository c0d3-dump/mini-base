use std::{thread, time::Duration};

use axum_server::Handle;
use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{Button, Dialog, LinearLayout, ListView, NamedView, ResizedView},
    Cursive,
};

use crate::{
    server,
    tui::{
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model},
    },
};

pub fn server_dashboard(_s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let mut layout = LinearLayout::new(Orientation::Vertical);

    let on_start_pressed = |s: &mut Cursive| {
        let model = get_current_model(s);

        let mut handle_model = get_current_mut_model(s);

        match handle_model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(3))),
            None => {}
        }

        handle_model.handle = Some(Handle::new());

        let handle = handle_model.handle.clone().unwrap();

        thread::spawn(move || {
            server::start_server(model.to_owned(), handle);
        });
    };

    let on_stop_pressed = |s: &mut Cursive| {
        let handle_model = get_current_mut_model(s);

        match handle_model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(5))),
            None => {
                s.add_layer(Dialog::info("server is not running..."));
            }
        }
    };

    layout.add_child(
        LinearLayout::new(Orientation::Horizontal)
            .child(Button::new("start/restart", on_start_pressed))
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
