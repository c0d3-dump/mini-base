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
            _ => panic!("error: dbtype selection"),
        };

        setup_db_connection(s, dbtype);
    };

    let dbtype = vec![DbType::SQLITE.to_string(), DbType::MYSQL.to_string()];

    let select = components::selector::select_component(dbtype, "select_dbtype", on_select);

    s.add_layer(
        Dialog::new()
            .title("select database type")
            .content(select)
            .button("quit", Cursive::quit),
    );
}

fn setup_db_connection(s: &mut Cursive, dbtype: DbType) {
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
                let dbpath = utils::get_data_from_refname::<EditView>(s, "dbpath")
                    .get_content()
                    .to_string();

                let conn = database::mysql::Mysql::new(&dbpath);

                (dbpath, Conn::MYSQL(conn))
            }
        };

        let (dbpath, conn) = conn;
        match &conn {
            Conn::SQLITE(c) => match &c.err {
                Some(err) => s.add_layer(Dialog::info(err)),
                None => {
                    s.with_user_data(|m: &mut Model| {
                        m.db = Db::SQLITE { dbpath };
                        m.conn = conn;
                    });

                    let model = get_current_model(s);
                    jsondb::to_json(model);

                    s.pop_layer();
                    s.pop_layer();
                    dashboard::display_dashboard(s);
                }
            },
            Conn::MYSQL(c) => match &c.err {
                Some(err) => s.add_layer(Dialog::info(err)),
                None => {
                    s.with_user_data(|m: &mut Model| {
                        m.db = Db::MYSQL { dbpath };
                        m.conn = conn;
                    });

                    let model = get_current_model(s);
                    jsondb::to_json(model);

                    s.pop_layer();
                    s.pop_layer();
                    dashboard::display_dashboard(s);
                }
            },
            Conn::None => panic!(),
        }
    };
    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let dbpath_view = EditView::new().with_name("dbpath");

    s.add_layer(
        Dialog::new()
            .title("add database values")
            .content(dbpath_view)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}
