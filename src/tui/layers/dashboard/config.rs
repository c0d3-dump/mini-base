use std::{thread, time::Duration};

use axum_server::Handle;
use cursive::{
    view::{Nameable, Scrollable},
    views::{Dialog, EditView, ListView, NamedView},
    Cursive,
};

use crate::{
    queries::model::Config,
    server::start_server,
    tui::{
        model::Sidebar,
        utils::{get_current_model, get_current_mut_model, get_data_from_refname},
    },
};

pub fn config_dashboard(s: &mut Cursive) -> NamedView<Dialog> {
    let mut list = ListView::new();

    let model = get_current_mut_model(s);
    let config_data: Config = match model.jsondb.get::<Config>("config") {
        Ok(data) => data,
        Err(e) => {
            log::error!("{:#?}", e);
            Config {
                ips: "".to_string(),
                auth_secret: "".to_string(),
                storage_secret: "".to_string(),
            }
        }
    };

    list.add_child(
        "IPs",
        EditView::new()
            .on_edit(on_data_changes)
            .content(config_data.ips)
            .with_name("ips"),
    );
    list.add_child(
        "Auth Secret",
        EditView::new()
            .on_edit(on_data_changes)
            .content(config_data.auth_secret)
            .with_name("auth_secret"),
    );
    list.add_child(
        "Storage Secret",
        EditView::new()
            .on_edit(on_data_changes)
            .content(config_data.storage_secret)
            .with_name("storage_secret"),
    );

    Dialog::new()
        .title("Config")
        .content(list.with_name("config").scrollable())
        .padding_lrtb(1, 1, 1, 0)
        .with_name(Sidebar::Config.to_string())
}

fn on_data_changes<'a, 'b>(s: &'a mut Cursive, _text: &'b str, _cursor: usize) {
    let model = get_current_mut_model(s);
    model.temp.restart_required = true;

    let mut dialog = get_data_from_refname::<Dialog>(s, &Sidebar::Config.to_string());
    dialog.clear_buttons();
    dialog.add_button("save", on_save);
}

fn on_save(s: &mut Cursive) {
    let model = get_current_mut_model(s);
    match &model.handle {
        Some(h) => h.graceful_shutdown(Some(Duration::from_secs(3))),
        None => {}
    }

    let ips = get_data_from_refname::<EditView>(s, "ips")
        .get_content()
        .to_string();

    let auth_secret = get_data_from_refname::<EditView>(s, "auth_secret")
        .get_content()
        .to_string();

    let storage_secret = get_data_from_refname::<EditView>(s, "storage_secret")
        .get_content()
        .to_string();

    let config = Config {
        ips: ips.clone(),
        auth_secret: auth_secret.clone(),
        storage_secret: storage_secret.clone(),
    };

    let model = get_current_mut_model(s);
    match model.jsondb.save_with_id(&config, "config") {
        Ok(_) => {}
        Err(e) => {
            log::error!("{:#?}", e);
        }
    }

    model.handle = Some(Handle::new());
    model.utils.ips = ips.split(',').map(|ip| ip.to_string()).collect();
    model.utils.auth_secret = auth_secret;
    model.utils.storage_secret = storage_secret;

    let model = get_current_model(s);
    thread::spawn(|| {
        start_server(model);
    });

    let mut dialog = get_data_from_refname::<Dialog>(s, &Sidebar::Config.to_string());
    dialog.clear_buttons();
}
