use cursive::view::Nameable;
use cursive::{
    views::{Dialog, EditView},
    Cursive,
};
use enum_iterator::all;

use crate::database;
use crate::database::model::DbType;
use crate::tui::components;
use crate::tui::utils::{get_current_mut_model, get_data_from_refname};

use super::dashboard;

pub fn select_dbtype(s: &mut Cursive) {
    let on_select = |s: &mut Cursive, idx: &usize| {
        let optional_dbtype = all::<DbType>()
            .enumerate()
            .filter(|(i, _)| i == idx)
            .map(|(_, x)| x)
            .next();

        match optional_dbtype {
            Some(dbtype) => {
                setup_db_connection(s, dbtype);
            }
            None => {}
        }
    };

    let dbtypes = all::<DbType>()
        .enumerate()
        .map(|(idx, dbtype)| (idx, dbtype.to_string()))
        .collect::<Vec<(usize, String)>>();

    let select = components::selector::select_component(dbtypes, "select_dbtype", on_select);

    s.add_layer(
        Dialog::new()
            .title("select database type")
            .content(select)
            .button("quit", Cursive::quit),
    );
}

fn setup_db_connection(s: &mut Cursive, dbtype: DbType) {
    let on_submit = move |s: &mut Cursive| {
        let dbpath = get_data_from_refname::<EditView>(s, "dbpath")
            .get_content()
            .to_string();

        let conn = database::Conn::new(dbtype.clone(), &dbpath);

        match conn.err {
            Some(e) => {
                s.add_layer(Dialog::info(e));
            }
            None => {
                let model = get_current_mut_model(s);
                model.conn = Some(conn);

                s.pop_layer();
                s.pop_layer();

                dashboard::display_dashboard(s);
            }
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
