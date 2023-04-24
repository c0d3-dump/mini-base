use std::{thread, time::Duration};

use axum_server::Handle;
use cursive::{
    align::Align,
    direction::Orientation,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, LinearLayout, ListView, NamedView, ResizedView, TextView},
    Cursive,
};

use crate::{
    server,
    tui::{
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model, get_data_from_refname},
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

        update_logs(s);
    };

    let on_stop_pressed = |s: &mut Cursive| {
        let handle_model = get_current_mut_model(s);

        match handle_model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(5))),
            None => {
                s.add_layer(Dialog::info("server is not running..."));
            }
        }

        clear_logs(s);
    };

    layout.add_child(
        LinearLayout::new(Orientation::Horizontal)
            .child(Button::new("start/restart", on_start_pressed))
            .child(Button::new("stop", on_stop_pressed)),
    );

    let logs = ListView::new().with_name("server_logs");
    layout.add_child(
        Dialog::new()
            .title("logs")
            .content(logs.scrollable())
            .full_screen(),
    );

    Dialog::new()
        .title("server")
        .content(layout)
        .full_screen()
        .with_name(Sidebar::SERVER.to_string())
}

fn update_logs(s: &mut Cursive) {
    let model = get_current_model(s);

    let mut logs = get_data_from_refname::<ListView>(s, "server_logs");
    for q in model.querylist {
        logs.add_child(
            q.label,
            TextView::new(q.exec_type.to_string()).align(Align::center_right()),
        );
    }
}

fn clear_logs(s: &mut Cursive) {
    let mut logs = get_data_from_refname::<ListView>(s, "server_logs");
    logs.clear();
}
