use std::{alloc::Layout, collections::HashMap};

use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{
        Dialog, EditView, LinearLayout, NamedView, OnLayoutView, Panel, ScreensView, StackView,
        TextArea, TextView,
    },
    Cursive, ScreenId, View,
};

use self::model::DbType;

mod components;
mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    app.set_user_data(model::Model::default());

    select_dbtype(&mut app);

    // display_dashboard(&mut app);

    app.run();
}

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

    let select = components::selector::select_component(dbtype, on_select);

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
        let db = match dbtype {
            DbType::SQLITE => {
                let dbpath = utils::get_data_from_refname::<EditView>(s, "dbpath")
                    .get_content()
                    .to_string();

                model::Db::SQLITE { dbpath: dbpath }
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

                model::Db::MYSQL {
                    host,
                    username,
                    port,
                    password,
                    database: Some(database),
                }
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

                model::Db::POSTGRES {
                    host,
                    username,
                    port,
                    password,
                    database: Some(database),
                }
            }
        };

        let mut data: &mut model::Model = s.user_data().unwrap();
        data.db = db;

        // TODO: check if we need to pop layer
        // found out that if we don't pop layer then on some terminal there will be some flickering when switching between stackview
        s.pop_layer();
        s.pop_layer();
        display_dashboard(s);
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

fn display_dashboard(s: &mut Cursive) {
    let sidebar_items = vec![
        model::Sidebar::STATS.to_string(),
        model::Sidebar::AUTH.to_string(),
        model::Sidebar::QUERY.to_string(),
        model::Sidebar::DOCS.to_string(),
    ];

    let on_select = |s: &mut Cursive, idx: &usize| {
        let mut dashboards = utils::get_data_from_refname::<StackView>(s, "dashboards");

        let sidebar_items = vec![
            model::Sidebar::STATS.to_string(),
            model::Sidebar::AUTH.to_string(),
            model::Sidebar::QUERY.to_string(),
            model::Sidebar::DOCS.to_string(),
        ];

        let layerpos = dashboards
            .find_layer_from_name(sidebar_items.get(*idx).unwrap())
            .unwrap();
        dashboards.move_to_front(layerpos);
    };

    let sidebar = Dialog::new().content(components::selector::select_component(
        sidebar_items,
        on_select,
    ));

    let mut dashboards = StackView::default();

    dashboards.add_fullscreen_layer(docs_dashboard(s));
    dashboards.add_fullscreen_layer(query_dashboard(s));
    dashboards.add_fullscreen_layer(auth_dashboard(s));
    dashboards.add_fullscreen_layer(stats_dashboard(s));

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}

fn stats_dashboard(s: &mut Cursive) -> impl View {
    Dialog::new()
        .title("stats")
        .full_screen()
        .with_name(model::Sidebar::STATS.to_string())
}

fn auth_dashboard(s: &mut Cursive) -> impl View {
    Dialog::new()
        .title("auth")
        .full_screen()
        .with_name(model::Sidebar::AUTH.to_string())
}

fn query_dashboard(s: &mut Cursive) -> impl View {
    Dialog::new()
        .title("query")
        .full_screen()
        .with_name(model::Sidebar::QUERY.to_string())
}

fn docs_dashboard(s: &mut Cursive) -> impl View {
    Dialog::new()
        .title("docs")
        .full_screen()
        .with_name(model::Sidebar::DOCS.to_string())
}
