use std::{
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
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

    // TODO: create channel variable here and pass to start_server() function that can get state and act accordingly

    let on_restart_pressed = |s: &mut Cursive| {};

    let on_start_pressed = |s: &mut Cursive| {
        let t_model = get_current_model(s);
        let model = Arc::new(Mutex::new(t_model.clone()));

        // let mut handle_model = get_current_mut_model(s);

        // match t_model.handle {
        //     None => {
        //         handle_model.handle = Some(Handle::new());
        //     }
        //     Some(_) => todo!(),
        // }

        thread::spawn(move || {
            let m = model.lock().unwrap();
            server::start_server(m.to_owned());
        });
    };

    let on_stop_pressed = |s: &mut Cursive| {};

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
