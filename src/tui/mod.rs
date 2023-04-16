use std::rc::Rc;
use std::{alloc::Layout, collections::HashMap};

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use async_std::channel;
use async_std::prelude::FutureExt;
use cursive::backends::crossterm::Backend;
use cursive::views::{Button, Checkbox, DebugView, ListChild, ListView, RadioGroup, SelectView};
use cursive::{
    direction::Orientation,
    view::{Nameable, Resizable},
    views::{
        Dialog, EditView, LinearLayout, NamedView, OnLayoutView, Panel, ScreensView, StackView,
        TextArea, TextView,
    },
    Cursive, ScreenId, View,
};
use cursive::{logger, With};
use tokio::runtime::{Handle, Runtime};

use crate::database;

use self::model::{DbType, ExecType, QueryList, RoleAccess, RoleList};
use self::utils::{get_current_model, get_current_mut_model, get_data_from_refname};

mod components;
pub mod jsondb;
mod model;
mod style;
mod utils;

pub fn run() {
    let mut app = cursive::default();

    app.set_theme(style::get_theme());

    let model = futures::executor::block_on(jsondb::from_json());

    app.set_user_data(model.clone());

    match model.conn {
        model::Conn::None => {
            select_dbtype(&mut app);
        }
        _ => {
            display_dashboard(&mut app);
        }
    }

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

                async {
                    let conn = database::sqlite::Sqlite::new(&dbpath).await;

                    (dbpath, model::Conn::SQLITE(conn))
                }
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
                // model::Db::MYSQL {
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
                // model::Db::POSTGRES {
                //     host,
                //     username,
                //     port,
                //     password,
                //     database: Some(database),
                // }
            }
        };

        let (dbpath, conn) = futures::executor::block_on(conn);
        s.with_user_data(|m: &mut model::Model| {
            m.db = model::Db::SQLITE { dbpath };
            m.conn = conn;
        });

        let model = get_current_model(s);
        jsondb::to_json(model);

        // TODO: check if we need to pop layer
        // found out that if we don't pop layer then for some terminal there will be some flickering when switching between stackview
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
        model::Sidebar::ROLE.to_string(),
        model::Sidebar::QUERY.to_string(),
        model::Sidebar::EDITOR.to_string(),
    ];

    let on_select = |s: &mut Cursive, idx: &usize| {
        let mut dashboards = utils::get_data_from_refname::<StackView>(s, "dashboards");

        let sidebar_items = vec![
            model::Sidebar::ROLE.to_string(),
            model::Sidebar::QUERY.to_string(),
            model::Sidebar::EDITOR.to_string(),
        ];

        let layerpos = dashboards
            .find_layer_from_name(sidebar_items.get(*idx).unwrap())
            .unwrap();
        dashboards.move_to_front(layerpos);
    };

    let sidebar = Dialog::new().content(components::selector::select_component(
        sidebar_items,
        "sidebar_items",
        on_select,
    ));

    let mut dashboards = StackView::default();

    dashboards.add_fullscreen_layer(editor_dashboard(s));
    dashboards.add_fullscreen_layer(query_dashboard(s));
    dashboards.add_fullscreen_layer(role_dashboard(s));

    s.add_layer(
        LinearLayout::new(Orientation::Horizontal)
            .child(sidebar)
            .child(dashboards.with_name("dashboards")),
    );
}

fn role_dashboard(s: &mut Cursive) -> impl View {
    let model = get_current_model(s);

    let role_list_items = model
        .rolelist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_role(s, *idx);
    };

    let role_list = components::selector::select_component(role_list_items, "role_list", on_select);

    let on_add_role = |s: &mut Cursive| add_role(s);

    Dialog::new()
        .title("role")
        .content(role_list)
        .button("Add Role", on_add_role)
        .full_screen()
        .with_name(model::Sidebar::ROLE.to_string())
}

fn edit_role(s: &mut Cursive, idx: usize) {
    let role = get_current_model(s).rolelist.get(idx).unwrap().to_owned();

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new().content(role.label).with_name("edit_label"),
    );

    let mut boolean_group: RadioGroup<bool> = RadioGroup::new();
    list.add_child(
        "approval required",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(false, "false"))
            .child(
                boolean_group
                    .button(true, "true")
                    .with_if(role.approval_required, |b| {
                        b.select();
                    }),
            ),
    );

    let all_roles = vec![
        model::RoleAccess::READ,
        model::RoleAccess::CREATE,
        model::RoleAccess::DELETE,
        model::RoleAccess::UPDATE,
    ];
    let mut role_access_list = vec![];
    for ra in all_roles {
        if role.role_access.contains(&ra) {
            role_access_list.push((ra.to_string(), true));
        } else {
            role_access_list.push((ra.to_string(), false));
        }
    }

    let check_box =
        components::checkbox_group::checkbox_group_component("role_access", role_access_list);
    list.add_child("role access", check_box);

    let on_submit = move |s: &mut Cursive| {
        let all_roles = vec![
            model::RoleAccess::READ,
            model::RoleAccess::CREATE,
            model::RoleAccess::DELETE,
            model::RoleAccess::UPDATE,
        ];

        let roleaccess = components::checkbox_group::get_checked_role_access_data(s, all_roles);

        let label = s
            .call_on_name("edit_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let rolelist = RoleList {
            label: label.to_string(),
            approval_required: *boolean_group.selection(),
            role_access: roleaccess,
        };

        let model = get_current_mut_model(s);
        model.rolelist[idx] = rolelist;

        update_role_with_model(s);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("edit role")
            .content(list)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}

fn add_role(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let data = get_data_from_refname::<EditView>(s, "add_role_text");

        s.with_user_data(|m: &mut model::Model| {
            m.rolelist.push(model::RoleList {
                label: data.get_content().to_string(),
                approval_required: false,
                role_access: vec![],
            });
        })
        .unwrap();

        update_role_with_model(s);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let textedit = EditView::new();

    s.add_layer(
        Dialog::new()
            .title("add role")
            .content(textedit.with_name("add_role_text"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}

fn query_dashboard(s: &mut Cursive) -> impl View {
    let model = get_current_model(s);

    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        edit_query(s, *idx);
    };

    let query_list =
        components::selector::select_component(query_list_items, "query_list", on_select);

    let on_add_query = |s: &mut Cursive| add_query(s);

    Dialog::new()
        .title("query")
        .content(query_list)
        .button("Add Query", on_add_query)
        .full_screen()
        .with_name(model::Sidebar::QUERY.to_string())
}

fn edit_query(s: &mut Cursive, idx: usize) {
    let query = get_current_model(s).querylist.get(idx).unwrap().to_owned();

    let mut list = ListView::new();
    list.add_child(
        "label",
        EditView::new()
            .content(&query.label.to_string())
            .with_name("edit_query_label"),
    );

    let mut boolean_group: RadioGroup<model::ExecType> = RadioGroup::new();
    list.add_child(
        "approval required",
        LinearLayout::new(Orientation::Horizontal)
            .child(boolean_group.button(ExecType::QUERY, "Query"))
            .child(
                boolean_group
                    .button(ExecType::EXECUTION, "Execution")
                    .with_if(query.exec_type.to_owned() == ExecType::EXECUTION, |b| {
                        b.select();
                    }),
            ),
    );

    let model = get_current_model(s);
    let all_rolelist = model
        .rolelist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let mut role_list = vec![];
    for ra in all_rolelist {
        if query.clone().role_access.contains(&ra) {
            role_list.push((ra.to_string(), true));
        } else {
            role_list.push((ra.to_string(), false));
        }
    }

    let check_box = components::checkbox_group::checkbox_group_component("role_list", role_list);
    list.add_child("role list", check_box);

    let on_submit = move |s: &mut Cursive| {
        let model = get_current_model(s);
        let all_rolelist = model
            .rolelist
            .into_iter()
            .map(|r| r.label)
            .collect::<Vec<String>>();

        let rolelist = components::checkbox_group::get_checked_data(s, all_rolelist);

        let label = s
            .call_on_name("edit_query_label", |view: &mut EditView| view.get_content())
            .unwrap()
            .to_string();

        let query = get_current_model(s).querylist.get(idx).unwrap().to_owned();
        let querylist = model::QueryList {
            label: label.to_string(),
            exec_type: boolean_group.selection().as_ref().to_owned(),
            role_access: rolelist,
            query: query.query,
        };

        let model = get_current_mut_model(s);
        model.querylist[idx] = querylist;

        update_query_with_model(s);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    s.add_layer(
        Dialog::new()
            .title("edit role")
            .content(list)
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}

fn add_query(s: &mut Cursive) {
    let on_submit = |s: &mut Cursive| {
        let data = get_data_from_refname::<EditView>(s, "add_query_text");

        s.with_user_data(|m: &mut model::Model| {
            m.querylist.push(model::QueryList {
                label: data.get_content().to_string(),
                role_access: vec![],
                exec_type: model::ExecType::QUERY,
                query: "".to_string(),
            });
        })
        .unwrap();

        update_query_with_model(s);

        s.pop_layer();
    };

    let on_cancel = |s: &mut Cursive| {
        s.pop_layer();
    };

    let textedit = EditView::new();

    s.add_layer(
        Dialog::new()
            .title("add query")
            .content(textedit.with_name("add_query_text"))
            .button("submit", on_submit)
            .button("cancel", on_cancel),
    );
}

fn editor_dashboard(s: &mut Cursive) -> impl View {
    let model = get_current_model(s);
    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    let on_select = |s: &mut Cursive, idx: &usize| {
        let i = Rc::new(idx.to_owned());
        let query = get_current_model(s).querylist.get(*i).unwrap().to_owned();

        let on_submit = move |s: &mut Cursive| {
            let data = get_data_from_refname::<TextArea>(s, "query_editor");

            let model = get_current_mut_model(s);
            model.querylist[*i].query = data.get_content().to_string();

            update_query_with_model(s);

            s.pop_layer();
        };

        s.add_fullscreen_layer(components::editor::editor_componant(
            "query_editor".to_string(),
            "editor",
            on_submit,
            query.query,
        ));
    };

    let query_list =
        components::selector::select_component(query_list_items, "query_editor_list", on_select);

    Dialog::new()
        .title("editor")
        .content(query_list)
        .full_screen()
        .with_name(model::Sidebar::EDITOR.to_string())
}

fn update_role_with_model(s: &mut Cursive) {
    let model = get_current_model(s);
    jsondb::to_json(model.clone());

    let role_list_items = model
        .rolelist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    components::selector::update_select_component(s, "role_list", role_list_items);
}

fn update_query_with_model(s: &mut Cursive) {
    let model = get_current_model(s);
    jsondb::to_json(model.clone());

    let query_list_items = model
        .querylist
        .into_iter()
        .map(|r| r.label)
        .collect::<Vec<String>>();

    components::selector::update_select_component(s, "query_list", query_list_items.clone());
    components::selector::update_select_component(s, "query_editor_list", query_list_items);
}
