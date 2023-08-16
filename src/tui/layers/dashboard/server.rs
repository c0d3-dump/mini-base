use std::{thread, time::Duration};

use axum_server::Handle;
use cursive::{
    align::Align,
    direction::Orientation,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, LinearLayout, ListView, NamedView, ResizedView, TextView},
    Cursive,
};

use crate::tui::{
    model::Sidebar,
    utils::{get_current_model, get_current_mut_model, get_data_from_refname},
};

pub fn server_dashboard(_s: &mut Cursive) -> NamedView<ResizedView<Dialog>> {
    let mut layout = LinearLayout::new(Orientation::Vertical);

    let on_start_pressed = |s: &mut Cursive| {
        let model = get_current_mut_model(s);

        match model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(3))),
            None => {}
        }

        model.handle = Some(Handle::new());

        // let handle = model.handle.clone().unwrap();

        // thread::spawn(move || {
        //     server::start_server(model.to_owned(), handle);
        // });

        update_apis(s);
    };

    let on_stop_pressed = |s: &mut Cursive| {
        let handle_model = get_current_mut_model(s);

        match handle_model.clone().handle {
            Some(h) => h.graceful_shutdown(Some(Duration::from_secs(5))),
            None => {
                s.add_layer(Dialog::info("server is not running..."));
            }
        }

        clear_apis(s);
    };

    layout.add_child(
        LinearLayout::new(Orientation::Horizontal)
            .child(Button::new("start/restart", on_start_pressed))
            .child(Button::new("stop", on_stop_pressed)),
    );

    let apis = ListView::new().with_name("server_apis");
    layout.add_child(
        Dialog::new()
            .title("apis")
            .content(apis.scrollable())
            .full_screen(),
    );

    Dialog::new()
        .title("server")
        .content(layout)
        .full_screen()
        .with_name(Sidebar::SERVER.to_string())
}

fn update_apis(s: &mut Cursive) {
    let model = get_current_model(s);

    let mut apis = get_data_from_refname::<ListView>(s, "server_apis");
    apis.clear();
    apis.add_child(
        "/auth/login",
        TextView::new("Auth").align(Align::center_right()),
    );
    apis.add_child(
        "/auth/signup",
        TextView::new("Auth").align(Align::center_right()),
    );
    apis.add_child(
        "/auth/logout",
        TextView::new("Auth").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/get",
        TextView::new("Storage").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/upload",
        TextView::new("Storage").align(Align::center_right()),
    );
    apis.add_child(
        "/storage/delete",
        TextView::new("Storage").align(Align::center_right()),
    );
    // for q in model.querylist {
    //     apis.add_child(
    //         format!("/api/{}", q.label),
    //         TextView::new(q.exec_type.to_string()).align(Align::center_right()),
    //     );
    // }
}

fn clear_apis(s: &mut Cursive) {
    let mut apis = get_data_from_refname::<ListView>(s, "server_apis");
    apis.clear();
}
