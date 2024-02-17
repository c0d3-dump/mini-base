use std::thread;

use axum_server::Handle;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::ListView;
use cursive::{
    views::{Dialog, EditView},
    Cursive,
};
use enum_iterator::all;

use crate::database;
use crate::database::model::DbType;
use crate::queries::model::Setup;
use crate::server::start_server;
use crate::tui::components;
use crate::tui::utils::{get_current_model, get_current_mut_model, get_data_from_refname};

use super::dashboard;

pub fn select_dbtype(s: &mut Cursive) {
    let on_select = |s: &mut Cursive, idx: &usize| {
        let optional_dbtype = all::<DbType>()
            .enumerate()
            .filter(|(i, _)| i == idx)
            .map(|(_, x)| x)
            .next();

        if let Some(dbtype) = optional_dbtype {
            setup_db_connection(s, dbtype);
        }
    };

    let dbtypes = all::<DbType>()
        .enumerate()
        .map(|(idx, dbtype)| (idx, dbtype.to_string()))
        .collect::<Vec<(usize, String)>>();

    let select = components::selector::select_component(dbtypes, "select_dbtype", on_select);

    s.add_layer(
        Dialog::new()
            .title("Databases")
            .content(select)
            .padding_lrtb(1, 1, 1, 0)
            .button("quit", Cursive::quit),
    );
}

fn setup_db_connection(s: &mut Cursive, dbtype: DbType) {
    let on_submit = move |s: &mut Cursive| {
        let dbpath = get_data_from_refname::<EditView>(s, "dbpath")
            .get_content()
            .to_string();

        let conn = database::Conn::new(dbtype.clone(), &dbpath);

        let ips = get_data_from_refname::<EditView>(s, "ips")
            .get_content()
            .to_string();

        let auth_secret = get_data_from_refname::<EditView>(s, "auth_secret")
            .get_content()
            .to_string();

        let storage_secret = get_data_from_refname::<EditView>(s, "storage_secret")
            .get_content()
            .to_string();

        let setup = Setup {
            dbpath: dbpath.clone(),
            ips: ips.clone(),
            auth_secret: auth_secret.clone(),
            storage_secret: storage_secret.clone(),
        };

        let model = get_current_mut_model(s);
        match model.jsondb.save_with_id(&setup, "setup") {
            Ok(_) => {}
            Err(e) => {
                log::error!("{:#?}", e);
            }
        }

        match conn.err {
            Some(e) => {
                s.add_layer(Dialog::info(e));
            }
            None => {
                let model = get_current_mut_model(s);
                model.conn = Some(conn);
                model.handle = Some(Handle::new());

                model.utils.ips = ips.split(',').map(|ip| ip.to_string()).collect();
                model.utils.auth_secret = auth_secret;
                model.utils.storage_secret = storage_secret;

                let model = get_current_model(s);
                thread::spawn(|| {
                    start_server(model);
                });

                s.pop_layer();
                s.pop_layer();

                dashboard::display_dashboard(s);
            }
        }
    };
    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let model = get_current_mut_model(s);
    let setup_data: Setup = match model.jsondb.get::<Setup>("setup") {
        Ok(data) => data,
        Err(e) => {
            log::error!("{:#?}", e);
            Setup {
                dbpath: "".to_string(),
                ips: "".to_string(),
                auth_secret: "".to_string(),
                storage_secret: "".to_string(),
            }
        }
    };

    let mut list = ListView::new();

    list.add_child(
        "DbPath",
        EditView::new()
            .content(setup_data.dbpath)
            .with_name("dbpath"),
    );
    list.add_child(
        "IPs",
        EditView::new().content(setup_data.ips).with_name("ips"),
    );
    list.add_child(
        "Auth Secret",
        EditView::new()
            .content(setup_data.auth_secret)
            .with_name("auth_secret"),
    );
    list.add_child(
        "Storage Secret",
        EditView::new()
            .content(setup_data.storage_secret)
            .with_name("storage_secret"),
    );

    s.add_layer(
        Dialog::new()
            .title("Setup")
            .content(list.scrollable().min_width(40))
            .padding_lrtb(1, 1, 1, 0)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
