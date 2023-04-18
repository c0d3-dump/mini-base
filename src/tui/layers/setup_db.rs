use cursive::view::Nameable;
use cursive::{
    views::{Dialog, EditView},
    Cursive,
};

use crate::tui::model::{Conn, Db, DbType, Model};
use crate::tui::utils::get_current_model;
use crate::{
    database,
    tui::{components, jsondb, utils},
};

use super::dashboard;

pub fn select_dbtype(s: &mut Cursive) {
    let on_select = |s: &mut Cursive, idx: &usize| {
        let dbtype = match idx {
            0 => DbType::SQLITE,
            1 => DbType::MYSQL,
            2 => DbType::POSTGRES,
            _ => panic!("error: dbtype selection"),
        };

        setup_db_connection(s, dbtype);
    };

    let dbtype = vec![
        DbType::SQLITE.to_string(),
        DbType::MYSQL.to_string(),
        DbType::POSTGRES.to_string(),
    ];

    let select = components::selector::select_component(dbtype, "select_dbtype", on_select);

    s.add_layer(
        Dialog::new()
            .title("select database type")
            .content(select)
            .button("quit", Cursive::quit),
    );
}

fn setup_db_connection(s: &mut Cursive, dbtype: DbType) {
    let items = dbtype
        .get_items()
        .into_iter()
        .map(|i| {
            // TODO: prefill content when local storage is setup
            let v = EditView::new().with_name(i);
            (i.to_string(), v)
        })
        .collect();

    let on_submit = move |s: &mut Cursive| {
        let conn = match dbtype {
            DbType::SQLITE => {
                let dbpath = utils::get_data_from_refname::<EditView>(s, "dbpath")
                    .get_content()
                    .to_string();

                let conn = database::sqlite::Sqlite::new(&dbpath);

                (dbpath, Conn::SQLITE(conn))
            }
            DbType::MYSQL => {
                let host = utils::get_data_from_refname::<EditView>(s, "host")
                    .get_content()
                    .to_string();
                let username = utils::get_data_from_refname::<EditView>(s, "username")
                    .get_content()
                    .to_string();
                let port = utils::get_data_from_refname::<EditView>(s, "port")
                    .get_content()
                    .parse::<u16>()
                    .unwrap();
                let password = utils::get_data_from_refname::<EditView>(s, "password")
                    .get_content()
                    .to_string();
                let database = utils::get_data_from_refname::<EditView>(s, "database")
                    .get_content()
                    .to_string();
                panic!();
                // Db::MYSQL {
                //     host,
                //     username,
                //     port,
                //     password,
                //     database: Some(database),
                // }
            }
            DbType::POSTGRES => {
                let host = utils::get_data_from_refname::<EditView>(s, "host")
                    .get_content()
                    .to_string();
                let username = utils::get_data_from_refname::<EditView>(s, "username")
                    .get_content()
                    .to_string();
                let port = utils::get_data_from_refname::<EditView>(s, "port")
                    .get_content()
                    .parse::<u16>()
                    .unwrap();
                let password = utils::get_data_from_refname::<EditView>(s, "password")
                    .get_content()
                    .to_string();
                let database = utils::get_data_from_refname::<EditView>(s, "database")
                    .get_content()
                    .to_string();
                panic!();
                // Db::POSTGRES {
                //     host,
                //     username,
                //     port,
                //     password,
                //     database: Some(database),
                // }
            }
        };

        let (dbpath, conn) = conn;
        s.with_user_data(|m: &mut Model| {
            m.db = Db::SQLITE { dbpath };
            m.conn = conn;
        });

        let model = get_current_model(s);
        jsondb::to_json(model);

        s.pop_layer();
        s.pop_layer();
        dashboard::display_dashboard(s);
    };
    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let connection = components::list::list_component(items);

    s.add_layer(
        Dialog::new()
            .title("add database values")
            .content(connection)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
